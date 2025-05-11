use core::f32;

use macroquad::prelude::*;

use crate::audio::play_bounce;

pub type Collision = [bool; 8];
pub type Dirs = [(i32, i32); 8];

pub const NO_COLLISION: Collision = [false; 8];
pub const DIR_WIDTH: i32 = 22;

#[rustfmt::skip]
pub const DIRS: Dirs = [
	( normal(DIR_WIDTH),-normal(DIR_WIDTH)),
	(-normal(DIR_WIDTH), normal(DIR_WIDTH)),
	( normal(DIR_WIDTH), normal(DIR_WIDTH)),
	(-normal(DIR_WIDTH),-normal(DIR_WIDTH)),
	( 0,-DIR_WIDTH),
	( 0, DIR_WIDTH),
	(-DIR_WIDTH, 0),
	( DIR_WIDTH, 0),
];

#[derive(Clone, PartialEq)]
pub struct Horse {
	pub pos: Vec2,
	pub dir: Vec2,
	pub texture: Texture2D,
	pub speed: f32,
}

impl Horse {
	pub async fn new(pos: Vec2, img_path: &str) -> Self {
		let image = match load_image(img_path).await {
			Ok(ok) => ok,
			Err(_) => todo!(),
		};

		Self {
			pos,
			dir: vec2(rand::gen_range(-1., 1.), rand::gen_range(-1., 1.)).normalize(),
			texture: Texture2D::from_image(&image),
			speed: 1.,
		}
	}

	pub fn update(&mut self) {
		self.pos += self.dir * self.speed;

		if self.speed < 3.5 {
			self.speed += 0.01;
		}
	}

	pub fn collision_wall(&self, image: &Image) -> Collision {
		let mut collision = NO_COLLISION;

		for i in 0..DIRS.len() {
			let (dir_x, dir_y) = DIRS[i];

			let x = (self.pos.x as i32 + dir_x) as u32;
			let y = (self.pos.y as i32 + dir_y) as u32;

			if matches!(image.get_pixel(x, y), pixel if pixel.a > 0.75) {
				collision[i] = true;
			}
		}

		collision
	}

	pub fn collision_honses(&self, honses: &[Horse]) -> Collision {
		let mut collision = NO_COLLISION;

		for i in 0..DIRS.len() {
			let (dir_x, dir_y) = DIRS[i];

			let pos = vec2(
				(self.pos.x as i32 + dir_x) as f32,
				(self.pos.y as i32 + dir_y) as f32,
			);

			for honse in honses {
				if honse == self { continue; }
				collision[i] |= honse.pos.distance(pos) <= DIR_WIDTH as f32;
			}
		}

		collision
	}

	pub fn bounce(&mut self, collision: Collision) {
		let mut new_dir = Vec2::ZERO;

		for i in 0..collision.len() {
			if !collision[i] {
				continue;
			}

			let (dir_x, dir_y) = DIRS[i];
			new_dir.x += dir_x as f32;
			new_dir.y += dir_y as f32;
		}

		new_dir.x += rand::gen_range(-10., 10.);
		new_dir.y += rand::gen_range(-10., 10.);

		self.dir = new_dir.normalize() * -1.;
		self.speed = rand::gen_range(1., 2.);

		play_bounce();
	}
}

pub fn fuse_collisions(c1: Collision, c2: Collision) -> Collision {
	let mut out = NO_COLLISION;

	for i in 0..c1.len() {
		out[i] |= c1[i];
		out[i] |= c2[i];
	}

	out
}

const fn normal(num: i32) -> i32 {
	(num as f32 * f32::consts::FRAC_1_SQRT_2) as i32
}

#[test]
fn normal_test() {
	assert_eq!(11, normal(16))
}

#[test]
fn fuse_collisions_test() {
	let c1 = [false, true, true, true, false, false, false, false];
	let c2 = [false, false, true, true, true, false, false, false];

	assert_eq!(fuse_collisions(c1, c2), [false, true, true, true, true, false, false, false])
}
