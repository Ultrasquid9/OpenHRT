use std::{
	fmt::Debug,
	path::{Path, PathBuf},
};

use macroquad::prelude::*;
use serde::{Deserialize, de::DeserializeOwned};

use crate::{
	audio::{StreamHandle, stream},
	race::{
		Race,
		horse::Horse,
		victory::{Carrots, Victory},
	},
	utils::{load_img, load_img_blocking},
};

#[derive(Deserialize, Default)]
pub struct RaceData {
	foreground: PathBuf,
	background: PathBuf,
	seed: Option<u64>,
	skip_intro: Option<bool>,
	horses: Vec<(Vec2, PathBuf)>,
	countdown: CountdownData,
	gate: GateData,
	carrots: CarrotData,
}

#[derive(Deserialize, Default)]
pub struct HorseData {
	sprite: PathBuf,
	win_data: WinData,
}

#[derive(Deserialize, Default)]
pub struct GateData {
	start: Vec2,
	end: Vec2,
	sprite: PathBuf,
}

#[derive(Deserialize, Default)]
pub struct CountdownData {
	audio: PathBuf,
	sprite: PathBuf,
}

#[derive(Deserialize, Default)]
pub struct CarrotData {
	pos: Vec2,
	sprite: PathBuf,
}

#[derive(Deserialize, Default, Clone)]
pub struct WinData {
	name: String,
	music: PathBuf,
	screen: PathBuf,
}

impl RaceData {
	pub fn load(path: impl AsRef<Path> + Debug) -> Self {
		read(path)
	}

	pub fn set_seed(self) -> Self {
		macroquad::rand::srand(match self.seed {
			Some(seed) => seed,
			None => miniquad::date::now().to_bits(),
		});

		self
	}

	pub async fn into_race(self) -> Race {
		let foreground = load_img(self.foreground);
		let background = load_img(self.background);

		let mut horses = vec![];
		for (pos, path) in self.horses {
			let horse = read::<HorseData>(path);
			horses.push(horse.into_horse(pos));
		}

		let mut race = Race::new(
			foreground,
			background,
			horses,
			self.gate,
			self.countdown,
			self.carrots,
		)
		.await;
		race.skip_intro(self.skip_intro.unwrap_or(false));
		race
	}
}

impl HorseData {
	pub fn into_horse(self, pos: Vec2) -> Horse {
		Horse::new(pos, self.sprite, self.win_data)
	}
}

impl GateData {
	pub fn into_pos_size(self) -> (Vec2, Vec2) {
		(self.start, self.end - self.start)
	}

	pub async fn texture(&self) -> Texture2D {
		Texture2D::from_image(&load_img(self.sprite.clone()).await)
	}
}

impl CountdownData {
	pub fn play_sound(&self) -> StreamHandle {
		stream(&self.audio)
	}

	pub async fn image(&self) -> Image {
		load_img(self.sprite.clone()).await
	}
}

impl CarrotData {
	pub fn into_carrots(self) -> Carrots {
		Carrots::new(self.pos, &load_img_blocking(self.sprite))
	}
}

impl WinData {
	pub fn into_victory(self) -> Victory {
		Victory::new(self.name, self.screen, self.music)
	}
}

fn read<Out>(path: impl AsRef<Path> + Debug) -> Out
where
	Out: DeserializeOwned + Default,
{
	let str = match std::fs::read_to_string(&path) {
		Ok(ok) => ok,
		Err(e) => {
			tracing::error!("Failed to read file {path:?}: {e}");
			return Out::default();
		}
	};

	match toml::from_str(&str) {
		Ok(ok) => ok,
		Err(e) => {
			tracing::error!("Failed to decode file {path:?}: {e}");
			Out::default()
		}
	}
}
