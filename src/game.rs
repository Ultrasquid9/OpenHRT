use macroquad::prelude::*;

use crate::horse::{Horse, NO_COLLISION};

pub struct Game {
	foreground: Image,
	background: Image,
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
			background,
			horses: horses.to_vec(),
		}
	}

	pub fn update(&mut self) {
		for horse in &mut self.horses {
			horse.update();

			match horse.collision(&self.foreground) {
				collision if collision != NO_COLLISION => {
					horse.bounce(collision);
				}
				_ => (),
			}
		}
	}

	pub fn draw(&mut self) {
		render_fullscreen_img(&self.background);
		render_fullscreen_img(&self.foreground);

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

fn render_fullscreen_img(img: &Image) {
	draw_texture_ex(
		&Texture2D::from_image(img),
		0.,
		0.,
		WHITE,
		DrawTextureParams {
			dest_size: Some(vec2(screen_width(), screen_height())),
			..Default::default()
		},
	);
}
