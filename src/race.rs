use macroquad::prelude::*;

use horse::{Collisions, Horse, NO_COLLISION};
use startup::Startup;
use victory::{Carrots, Victory};

use crate::{
	data::{CarrotData, GateData},
	utils::{load_img_blocking, render_texture_fullscreen},
};

pub mod horse;
mod startup;
pub mod victory;

pub struct Race {
	time: f32,
	foreground: Image,
	background: Texture2D,
	horses: Vec<Horse>,
	carrots: Carrots,
	startup: Option<Startup>,
	victory: Option<Victory>,
}

impl Race {
	pub async fn new(
		foreground_path: String,
		background_path: String,
		horses: Vec<Horse>,
		gate: GateData,
		carrots: CarrotData,
	) -> Self {
		let (foreground, background) = tokio::join!(
			load_img_blocking(foreground_path),
			load_img_blocking(background_path),
		);

		Self {
			time: 0.,
			foreground: foreground.clone(),
			background: Texture2D::from_image(&background),
			horses,
			carrots: carrots.into_carrots().await,
			startup: Some(Startup::new(&background, gate).await),
			victory: None,
		}
	}

	pub fn skip_intro(&mut self, skip: bool) {
		if skip {
			self.startup = None;
		}
	}

	pub async fn update(&mut self) {
		if let Some(startup) = &mut self.startup {
			startup.update();

			if startup.done() {
				self.startup = None;
			}

			return;
		} else if let Some(victory) = &mut self.victory {
			victory.update();
			return;
		} else {
			self.time += get_frame_time();
		}

		let collisions = self
			.horses
			.iter()
			.map(|horse| {
				horse.collision_wall(&self.foreground) | horse.collision_honses(&self.horses)
			})
			.collect::<Vec<Collisions>>();

		for (i, collision) in collisions.iter().enumerate() {
			let honse = &mut self.horses[i];
			honse.update();

			if *collision != NO_COLLISION {
				honse.bounce(*collision);
			}
		}

		for horse in &self.horses {
			if horse.collision_carrots(&self.carrots) {
				self.victory = Some(horse.win_data.clone().into_victory().await)
			}
		}
	}

	pub fn draw(&self) {
		render_texture_fullscreen(&self.background);
		render_texture_fullscreen(&Texture2D::from_image(&self.foreground));

		self.little_guy(self.carrots.pos, &self.carrots.texture);
		for horse in &self.horses {
			self.little_guy(horse.pos, &horse.texture);
		}

		draw_text(
			&parse_time(self.time),
			screen_width() * 0.85,
			screen_height() * 0.95,
			(screen_width() + screen_height()) / 70.,
			WHITE,
		);

		if let Some(startup) = &self.startup {
			startup.draw();
		} else if let Some(victory) = &self.victory {
			victory.draw();
		}
	}

	fn little_guy(&self, pos: Vec2, texture: &Texture2D) {
		let size = (screen_width() + screen_height()) / 40.;
		let params = DrawTextureParams {
			dest_size: Some(vec2(size, size)),
			..Default::default()
		};

		let pos_x = pos.x / self.foreground.width() as f32 * screen_width();
		let pos_y = pos.y / self.foreground.height() as f32 * screen_height();

		draw_texture_ex(
			texture,
			pos_x - (size / 2.),
			pos_y - (size / 2.),
			WHITE,
			params,
		);
	}
}

fn parse_time(mut time: f32) -> String {
	let mut minutes = 0u8;
	while time >= 60. {
		minutes += 1;
		time -= 60.;
	}

	let mut seconds = format!("{:.2}", time);
	if time < 10. {
		seconds = "0".to_string() + &seconds;
	}

	let start = if minutes > 10 { "" } else { "0" };

	format!("{start}{minutes}:{seconds}")
}
