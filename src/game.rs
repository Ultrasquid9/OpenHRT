use macroquad::prelude::*;

use crate::horse::{Horse, NO_COLLISION};

pub struct Game {
	image: Image,
	horses: Vec<Horse>,
}

impl Game {
	pub async fn new(img_path: &str, horses: &[Horse]) -> Self {
		let image = match load_image(img_path).await {
			Ok(ok) => ok,
			Err(_) => todo!(),
		};

		Self {
			image,
			horses: horses.to_vec(),
		}
	}

	pub fn update(&mut self) {
		for horse in &mut self.horses {
			horse.update();

			match horse.collision(&self.image) {
				collision if collision != NO_COLLISION => {
					horse.bounce(collision);
				}
				_ => (),
			}
		}
	}

	pub fn draw(&mut self) {
		clear_background(GRAY);

		draw_texture_ex(
			&Texture2D::from_image(&self.image),
			0.,
			0.,
			WHITE,
			DrawTextureParams {
				dest_size: Some(vec2(screen_width(), screen_height())),
				..Default::default()
			},
		);

		let horse_size = (screen_width() + screen_height()) / 56.;

		for horse in &self.horses {
			let horse_pos_x = horse.pos.x / self.image.width() as f32 * screen_width();
			let horse_pos_y = horse.pos.y / self.image.height() as f32 * screen_height();

			draw_texture_ex(
				&horse.texture, 
				horse_pos_x - (horse_size / 2.), 
				horse_pos_y - (horse_size / 2.), 
				WHITE, 
				DrawTextureParams { 
					dest_size: Some(vec2(horse_size, horse_size)), 
					..Default::default()
				}
			);
		}
	}
}
