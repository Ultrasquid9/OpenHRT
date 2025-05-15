use macroquad::{
	math::Vec2,
	texture::{Image, Texture2D},
};

pub struct Carrots {
	pub texture: Texture2D,
	pub pos: Vec2,
}

impl Carrots {
	pub fn new(pos: Vec2, img: &Image) -> Self {
		Self {
			pos,
			texture: Texture2D::from_image(img),
		}
	}
}
