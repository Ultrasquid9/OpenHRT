use kira::{
	Tween,
	sound::{PlaybackState, static_sound::StaticSoundHandle},
};
use macroquad::prelude::*;

use crate::audio::play_or_load;

pub struct Startup {
	handle: StaticSoundHandle,
	countdown: Countdown,
	gate_pos: Vec2,
	gate_size: Vec2,
	gate_texture: Texture2D,
}

struct Countdown {
	pos: Vec2,
	size: Vec2,
	texture: Texture2D,
	source: Rect,
	direction: u8,
	time: f32,
}

impl Startup {
	pub async fn new() -> Self {
		Self {
			handle: play_or_load("../assets/audio/place-your-bets-in.flac"),
			countdown: Countdown::new().await,
			gate_pos: vec2(0., 0.),
			gate_size: vec2(0., 0.),
			gate_texture: Texture2D::from_image(&load_image("./assets/gate.png").await.unwrap()),
		}
	}

	pub fn update(&mut self) {
		self.countdown.update();
	}

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

		self.countdown.draw();
	}

	pub fn done(&self) -> bool {
		matches!(self.handle.state(), PlaybackState::Stopped)
	}
}

impl Countdown {
	const DIRS: [Vec2; 4] = [
		vec2(24., 24.),
		vec2(-24., 24.),
		vec2(-24., -24.),
		vec2(24., -24.),
	];

	async fn new() -> Self {
		let img = load_image("./assets/countdown.png").await.unwrap();
		let rect = Rect::new(0., 0., img.width as f32, (img.height / 12) as f32);

		Self {
			pos: vec2(screen_width() / 2., screen_height() / 2.),
			size: Vec2::ZERO,
			texture: Texture2D::from_image(&img),
			source: rect,
			direction: rand::gen_range(0, 3),
			time: 0.,
		}
	}

	fn update(&mut self) {
		self.source();
		self.size();
		self.bounce();
	}

	fn source(&mut self) {
		self.time += get_frame_time();

		let offset = ((self.time / 13. * 12.) - 1.2).floor().clamp(0., 12.);
		self.source.y = self.source.h * offset;
	}

	fn bounce(&mut self) {
		if self.pos.x <= 0.
			|| self.pos.y <= 0.
			|| self.pos.x + self.size.x >= screen_width()
			|| self.pos.y + self.size.y >= screen_height()
		{
			self.center();
			self.direction += 1;
			if self.direction >= 4 {
				self.direction = 0;
			}
		}

		let dir = Self::DIRS[self.direction as usize];
		self.pos += dir * get_frame_time() * ((screen_width() + screen_height()) / 300.);
	}

	fn size(&mut self) {
		self.size = vec2(screen_width() / 4., screen_height() / 12.)
	}

	fn center(&mut self) {
		while self.pos.x <= 0. {
			self.pos.x += 1.;
		}
		while self.pos.y <= 0. {
			self.pos.y += 1.;
		}
		while self.pos.x + self.size.x >= screen_width() {
			self.pos.x -= 1.;
		}
		while self.pos.y + self.size.y >= screen_height() {
			self.pos.y -= 1.;
		}
	}

	fn draw(&self) {
		draw_texture_ex(
			&self.texture,
			self.pos.x,
			self.pos.y,
			WHITE,
			DrawTextureParams {
				dest_size: Some(self.size),
				source: Some(self.source),
				..Default::default()
			},
		);
	}
}

impl Drop for Startup {
	fn drop(&mut self) {
		self.handle.stop(Tween::default());
	}
}
