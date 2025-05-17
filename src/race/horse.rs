use core::f32;

use macroquad::prelude::*;

use crate::{
	audio::play_or_load,
	data::WinData,
	dirs,
	utils::{Dirs, load_img_blocking},
};

use super::victory::Carrots;

pub type Collisions = u8;

pub const NO_COLLISION: Collisions = 0;
pub const DIR_WIDTH: f32 = 36.;

#[rustfmt::skip]
pub const DIRS: Dirs<8> = dirs![
	( normal(DIR_WIDTH),-normal(DIR_WIDTH)),
	(-normal(DIR_WIDTH), normal(DIR_WIDTH)),
	( normal(DIR_WIDTH), normal(DIR_WIDTH)),
	(-normal(DIR_WIDTH),-normal(DIR_WIDTH)),
	( 0.,-DIR_WIDTH),
	( 0., DIR_WIDTH),
	(-DIR_WIDTH, 0.),
	( DIR_WIDTH, 0.),
];

#[derive(Clone)]
pub struct Horse {
	pub pos: Vec2,
	pub dir: Vec2,
	pub texture: Texture2D,
	pub speed: f32,
	pub win_data: WinData,
}

impl Horse {
	pub fn new(pos: Vec2, img_path: &str, win_data: WinData) -> Self {
		let image = load_img_blocking(img_path);

		Self {
			pos,
			dir: vec2(rand::gen_range(-1., 1.), rand::gen_range(-1., 1.)).normalize(),
			texture: Texture2D::from_image(&image),
			speed: 1.,
			win_data,
		}
	}

	pub fn update(&mut self) {
		// Should be somewhere around 1 when game is running 60 FPS.
		// Clamped to 4 to prevent stutters from breaking physics.
		// Hopefully the game will never have to go below 15 FPS. 
		let approx_one = (get_frame_time() * 60.).clamp(0., 4.);

		self.pos += self.dir * self.speed * approx_one * 2.;

		if self.speed < 3.5 {
			self.speed += approx_one / 100.;
		}
	}

	pub fn collision_wall(&self, image: &Image) -> Collisions {
		let mut collisions = NO_COLLISION;

		for (i, Vec2 { x, y }) in DIRS.iter().enumerate() {
			let x = (self.pos.x + x) as u32;
			let y = (self.pos.y + y) as u32;

			if image.get_pixel(x, y).a > 0.75 {
				collisions |= 1 << i;
			}
		}

		collisions
	}

	pub fn collision_honses(&self, honses: &[Horse]) -> Collisions {
		let mut collisions = NO_COLLISION;

		for (i, dir) in DIRS.iter().enumerate() {
			let pos = self.pos + *dir;

			for honse in honses {
				if honse == self {
					continue;
				}
				let bit = (honse.pos.distance(pos) <= DIR_WIDTH) as u8;
				collisions |= bit << i;
			}
		}

		collisions
	}

	pub fn collision_carrots(&self, carrots: &Carrots) -> bool {
		let mut collision = false;
		for dir in DIRS {
			let pos = self.pos + dir;
			collision |= carrots.pos.distance(pos) <= DIR_WIDTH;
		}
		collision
	}

	pub fn bounce(&mut self, collisions: Collisions) {
		let mut new_dir = Vec2::ZERO;

		for i in 0..u8::BITS {
			let bit = 1 << i;
			if collisions & bit != bit {
				continue;
			}

			new_dir = DIRS[i as usize];
		}

		new_dir.x += rand::gen_range(-24., 24.);
		new_dir.y += rand::gen_range(-24., 24.);

		self.dir = new_dir.normalize() * -1.;
		self.speed = rand::gen_range(1., 2.);

		play_or_load("./assets/audio/bounce.flac");
	}
}

impl PartialEq for Horse {
	fn eq(&self, other: &Self) -> bool {
		self.dir == other.dir && self.pos == other.pos && self.speed == other.speed
	}
}

const fn normal(num: f32) -> f32 {
	num * f32::consts::FRAC_1_SQRT_2
}

mod tests {
	#[allow(unused)]
	use super::*;

	#[test]
	fn normal_test() {
		assert_eq!(11., normal(16.).round())
	}
}
