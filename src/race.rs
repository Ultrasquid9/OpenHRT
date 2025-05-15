use macroquad::prelude::*;

use horse::{Collisions, Horse, NO_COLLISION};
use startup::Startup;
use victory::Carrots;

use crate::{
	data::{CarrotData, GateData},
	utils::load_img_blocking,
};

pub mod horse;
mod startup;
pub mod victory;

pub struct Race {
	foreground: Image,
	background: Texture2D,
	horses: Vec<Horse>,
	carrots: Carrots,
	startup: Option<Startup>,
}

impl Race {
	pub async fn new(
		foreground_path: &str,
		background_path: &str,
		horses: &[Horse],
		gate: GateData,
		carrots: CarrotData,
	) -> Self {
		let foreground = load_img_blocking(foreground_path.into()).await;
		let background = load_img_blocking(background_path.into()).await;

		Self {
			foreground: foreground.clone(),
			background: Texture2D::from_image(&background),
			horses: horses.to_vec(),
			startup: Some(Startup::new(&background, gate).await),
			carrots: carrots.into_carrots().await,
		}
	}

	pub fn skip_intro(&mut self, skip: bool) {
		if skip {
			self.startup = None;
		}
	}

	pub fn update(&mut self) {
		if let Some(startup) = &mut self.startup {
			startup.update();

			if startup.done() {
				self.startup = None;
			}

			return;
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
	}

	pub fn draw(&self) {
		render_texture_fullscreen(&self.background);
		render_texture_fullscreen(&Texture2D::from_image(&self.foreground));

		for horse in &self.horses {
			self.little_guy(horse.pos, &horse.texture);
		}

		self.little_guy(self.carrots.pos, &self.carrots.texture);

		if let Some(startup) = &self.startup {
			startup.draw();
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

fn render_texture_fullscreen(texture: &Texture2D) {
	draw_texture_ex(
		texture,
		0.,
		0.,
		WHITE,
		DrawTextureParams {
			dest_size: Some(vec2(screen_width(), screen_height())),
			..Default::default()
		},
	);
}
