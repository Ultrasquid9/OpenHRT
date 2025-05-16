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
	name: String,
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
		let handle = play_or_load(&music);

		Self {
			name,
			screen: Texture2D::from_image(&load_img(screen).await),
			handle,
		}
	}

	pub fn update(&mut self) {
		if self.handle.state() == PlaybackState::Stopped {
			tracing::info!("Race finished");
			todo!("End the program properly")
		}
	}

	pub fn draw(&self) {
		render_texture_fullscreen(&self.screen);

		draw_text(
			&self.name,
			screen_width() * 0.05,
			screen_height() * 0.95,
			(screen_width() + screen_height()) / 60.,
			WHITE,
		);
	}
}
