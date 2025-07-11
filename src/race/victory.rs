use std::{
	fmt::Debug,
	path::Path,
	thread::{JoinHandle, spawn},
};

use kira::sound::PlaybackState;
use macroquad::prelude::*;

use crate::{
	audio::{StreamHandle, stream},
	utils::{debug_img, load_img_blocking, render_texture_fullscreen},
};

const ZOOM_TIME: f32 = 4.5;
const FULL_TIME: f32 = 13.;

pub struct Carrots {
	pub texture: Texture2D,
	pub pos: Vec2,
}

pub struct Victory {
	time: f32,
	name: String,
	zoom: Texture2D,
	screen: FileLoad,
	mus: StreamHandle,
}

enum FileLoad {
	/// A thread containing the operation
	Handle(JoinHandle<Image>),
	/// A texture produced by the operation
	Texture(Texture2D),
	/// Used as a temporary value, should only be seen if something goes wrong
	Poisoned,
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
	pub fn new<Dir>(name: String, screen: Dir, music: Dir) -> Self
	where
		Dir: AsRef<Path> + Debug + Send + 'static,
	{
		let zoom = Texture2D::from_image(&get_screen_data());
		zoom.set_filter(FilterMode::Nearest);

		Self {
			time: 0.,
			name,
			zoom,
			screen: FileLoad::new(screen),
			mus: stream(music),
		}
	}

	pub fn update(&mut self) {
		self.time += get_frame_time();

		if self.time >= ZOOM_TIME {
			self.screen.join();
		}
	}

	pub fn should_finish(&self) -> bool {
		self.mus.state() == PlaybackState::Stopped && self.time > FULL_TIME
	}

	pub fn draw(&self) {
		if self.time <= ZOOM_TIME {
			self.zoom();
			return;
		}
		let Some(screen) = self.screen.try_get() else {
			self.zoom();
			return;
		};

		render_texture_fullscreen(screen);

		let (width, height) = (screen_width(), screen_height());
		let current = ((width + height) / 196.) * self.time * 1.25;
		let max = ((width + height) / 28.).round();

		let size = current.clamp(0., max);

		draw_text_ex(
			&self.name,
			width * 0.05,
			height * 0.95,
			TextParams {
				// Macroquad caches fonts every time it sees a font with a
				// different size, which leads to severe performance issues
				// if the font size regularly changes.
				//
				// Rounding the font size, then adjusting the scale, allows
				// for reduced caching while preserving font sharpness.
				font_size: size.round() as u16,
				font_scale: 1. + (size / (max / 5. * 4.)).fract(),
				..Default::default()
			},
		);
	}

	fn zoom(&self) {
		let (width, height) = (screen_width(), screen_height());
		let time_pos = -(self.time / ZOOM_TIME);
		let time_size = self.time + 1.;

		draw_texture_ex(
			&self.zoom,
			width * time_pos,
			height * time_pos,
			WHITE,
			DrawTextureParams {
				flip_y: true,
				dest_size: Some(vec2(width, height) * time_size),
				..Default::default()
			},
		);
	}
}

impl FileLoad {
	fn new<Dir>(path: Dir) -> Self
	where
		Dir: AsRef<Path> + Debug + Send + 'static,
	{
		Self::Handle(spawn(move || load_img_blocking(&path)))
	}

	fn try_get(&self) -> Option<&Texture2D> {
		if let Self::Texture(t) = self {
			Some(t)
		} else {
			None
		}
	}

	fn join(&mut self) {
		if let Self::Handle(_) = self {
			let owned = std::mem::replace(self, Self::Poisoned);

			let Self::Handle(handle) = owned else {
				unreachable!("Owned is known to be `Self::Handle(_)`")
			};

			let img = match handle.join() {
				Ok(ok) => ok,
				Err(e) => {
					tracing::error!("{:?}", e);
					debug_img()
				}
			};

			*self = Self::Texture(Texture2D::from_image(&img));
		}
	}
}
