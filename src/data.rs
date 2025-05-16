use std::path::{Path, PathBuf};

use macroquad::prelude::*;
use serde::{Deserialize, de::DeserializeOwned};
use tracing::error;

use crate::{
	race::{
		Race,
		horse::Horse,
		victory::{Carrots, Victory},
	},
	utils::load_img,
};

#[derive(Deserialize, Default)]
pub struct RaceData {
	foreground: PathBuf,
	background: PathBuf,
	seed: Option<u64>,
	skip_intro: Option<bool>,
	horses: Vec<(Vec2, PathBuf)>,
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
}

#[derive(Deserialize, Default)]
pub struct CarrotData {
	pos: Vec2,
	sprite: PathBuf,
}

#[derive(Deserialize, Default, Clone, PartialEq)]
pub struct WinData {
	name: String,
	music: PathBuf,
	screen: PathBuf,
}

impl RaceData {
	pub fn load(path: impl AsRef<Path>) -> Self {
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
		let mut horses = vec![];
		for (pos, path) in self.horses {
			let horse = read::<HorseData>(path);
			horses.push(horse.into_horse(pos).await);
		}

		let mut race = Race::new(
			stringify(self.foreground),
			stringify(self.background),
			horses,
			self.gate,
			self.carrots,
		)
		.await;
		race.skip_intro(self.skip_intro.unwrap_or(false));
		race
	}
}

impl HorseData {
	pub async fn into_horse(self, pos: Vec2) -> Horse {
		Horse::new(pos, stringify(self.sprite), self.win_data).await
	}
}

impl GateData {
	pub fn into_pos_size(self) -> (Vec2, Vec2) {
		(self.start, self.end - self.start)
	}
}

impl CarrotData {
	pub async fn into_carrots(self) -> Carrots {
		Carrots::new(self.pos, &load_img(stringify(self.sprite)).await)
	}
}

impl WinData {
	pub async fn into_victory(self) -> Victory {
		Victory::new(self.name, stringify(self.screen), stringify(self.music)).await
	}
}

fn read<Out>(path: impl AsRef<Path>) -> Out
where
	Out: DeserializeOwned + Default,
{
	let str = match std::fs::read_to_string(path) {
		Ok(ok) => ok,
		Err(e) => {
			tracing::error!("{e}");
			return Out::default();
		}
	};

	match toml::from_str(&str) {
		Ok(ok) => ok,
		Err(e) => {
			tracing::error!("{e}");
			Out::default()
		}
	}
}

fn stringify(pth: PathBuf) -> String {
	match pth.as_os_str().to_str() {
		Some(str) => str.into(),
		None => {
			error!("{:?} is not valid unicode!", pth);
			String::new()
		}
	}
}
