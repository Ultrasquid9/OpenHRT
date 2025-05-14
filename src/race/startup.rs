use kira::sound::{static_sound::StaticSoundHandle, PlaybackState};
use macroquad::prelude::*;

use crate::audio::play_or_load;

pub struct Startup {
	handle: StaticSoundHandle,
	countdown_pos: Vec2,
	countdown_size: Vec2,
	countdown_texture: Texture2D,
	gate_pos: Vec2,
	gate_size: Vec2,
	gate_texture: Texture2D,
}

impl Startup {
	pub async fn new() -> Self {
		let x = Self {
			handle: play_or_load("../assets/audio/place-your-bets-in.flac"),
			countdown_pos: vec2(0., 0.),
			countdown_size: vec2(0., 0.),
			countdown_texture: Texture2D::from_image(
				&load_image("./assets/gate.png").await.unwrap(),
			),
			gate_pos: vec2(0., 0.),
			gate_size: vec2(0., 0.),
			gate_texture: Texture2D::from_image(&load_image("./assets/gate.png").await.unwrap()),
		};
		println!("done");
		x
	}

	pub fn update(&mut self) {}

	pub fn draw(&self) {
		draw_texture_ex(
			&self.gate_texture,
			self.gate_pos.x,
			self.gate_pos.y,
			WHITE,
			DrawTextureParams {
				dest_size: Some(self.gate_size),
				..Default::default()
			},
		);

		draw_texture_ex(
			&self.countdown_texture,
			self.countdown_pos.x,
			self.countdown_pos.y,
			WHITE,
			DrawTextureParams {
				dest_size: Some(self.countdown_size),
				..Default::default()
			},
		);
	}

	pub fn done(&self) -> bool {
		matches!(self.handle.state(), PlaybackState::Stopped)
	}
}
