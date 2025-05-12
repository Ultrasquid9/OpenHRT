use macroquad::prelude::*;

use crate::horse::{Collisions, Horse, NO_COLLISION};

pub struct Game {
	foreground: Image,
	background: Texture2D,
	horses: Vec<Horse>,
}

impl Game {
	pub async fn new(foreground_path: &str, background_path: &str, horses: &[Horse]) -> Self {
		let foreground = match load_image(foreground_path).await {
			Ok(ok) => ok,
			Err(_) => todo!(),
		};
		let background = match load_image(background_path).await {
			Ok(ok) => ok,
			Err(_) => todo!(),
		};

		Self {
			foreground,
			background: Texture2D::from_image(&background),
			horses: horses.to_vec(),
		}
	}

	pub fn update(&mut self) {
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

	pub fn draw(&mut self) {
		render_texture_fullscreen(&self.background);
		render_texture_fullscreen(&Texture2D::from_image(&self.foreground));

		let horse_size = (screen_width() + screen_height()) / 56.;

		for horse in &self.horses {
			let horse_pos_x = horse.pos.x / self.foreground.width() as f32 * screen_width();
			let horse_pos_y = horse.pos.y / self.foreground.height() as f32 * screen_height();

			draw_texture_ex(
				&horse.texture,
				horse_pos_x - (horse_size / 2.),
				horse_pos_y - (horse_size / 2.),
				WHITE,
				DrawTextureParams {
					dest_size: Some(vec2(horse_size, horse_size)),
					..Default::default()
				},
			);
		}
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
