use std::f32::consts::PI;

use kira::sound::{PlaybackState, static_sound::StaticSoundHandle};
use macroquad::prelude::*;

use crate::{
	audio::play_or_load,
	utils::{load_img, render_texture_fullscreen},
};

pub struct Carrots {
	pub texture: Texture2D,
	pub pos: Vec2,
}

pub struct Victory {
	time: f32,
	name: String,
	zoom: Texture2D,
	screen: Texture2D,
	handle: StaticSoundHandle,
}

impl Carrots {
	pub fn new(pos: Vec2, img: &Image) -> Self {
		Self {
			pos,
			texture: Texture2D::from_image(img),
		}
	}
}

impl Victory {
	pub async fn new(name: String, screen: String, music: String) -> Self {
		// TODO: Reduce/remove stutter that occurs here
		Self {
			time: 0.,
			name,
			zoom: Texture2D::from_image(&get_screen_data()),
			screen: Texture2D::from_image(&load_img(screen).await),
			handle: play_or_load(&music),
		}
	}

	pub fn update(&mut self) {
		self.time += get_frame_time();

		if self.handle.state() == PlaybackState::Stopped && self.time > 10. {
			tracing::info!("Race finished");
			todo!("End the program properly")
		}
	}

	pub fn draw(&self) {
		let (width, height) = (screen_width(), screen_height());

		if self.time <= 6. {
			let size = vec2(width * (self.time), height * (self.time));

			draw_texture_ex(
				&self.zoom,
				-width * 0.3,
				-height * 0.3,
				WHITE,
				DrawTextureParams {
					rotation: PI,
					flip_x: true,
					dest_size: Some(size),
					..Default::default()
				},
			);

			return;
		}

		render_texture_fullscreen(&self.screen);

		let current = ((width + height) / 196.) * self.time;
		let max = (width + height) / 16.;

		draw_text(
			&self.name,
			screen_width() * 0.05,
			screen_height() * 0.95,
			current.clamp(0., max),
			WHITE,
		);
	}
}
