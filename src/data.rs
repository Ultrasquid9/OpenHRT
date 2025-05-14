use std::path::{Path, PathBuf};

use macroquad::{math::Vec2, miniquad};
use serde::Deserialize;

use crate::race::{Race, horse::Horse};

#[derive(Deserialize)]
pub struct RaceData {
	foreground: PathBuf,
	background: PathBuf,
	seed: Option<u64>,
	skip_intro: Option<bool>,
	horses: Vec<(PathBuf, Vec2)>,
}

#[derive(Deserialize)]
pub struct HorseData {
	sprite: PathBuf,
}

impl HorseData {
	pub async fn into_horse(self, pos: Vec2) -> Horse {
		Horse::new(pos, &stringify(self.sprite)).await
	}
}

impl RaceData {
	pub fn load<Dir: AsRef<Path>>(path: Dir) -> Self {
		toml::from_str(&std::fs::read_to_string(path).unwrap()).unwrap()
	}

	pub fn set_seed(self) -> Self {
		macroquad::rand::srand(match self.seed {
			Some(seed) => seed,
			None => miniquad::date::now().to_bits(),
		});

		self
	}

	pub async fn into_race(self) -> Race {
		let mut horses = vec![];
		for (path, pos) in self.horses {
			let horse: HorseData = toml::from_str(&std::fs::read_to_string(path).unwrap()).unwrap();
			horses.push(horse.into_horse(pos).await);
		}

		let mut race = Race::new(
			&stringify(self.foreground),
			&stringify(self.background),
			&horses,
		)
		.await;
		race.skip_intro(self.skip_intro.unwrap_or(false));
		race
	}
}

fn stringify(pth: PathBuf) -> String {
	pth.as_os_str().to_str().unwrap().into()
}
