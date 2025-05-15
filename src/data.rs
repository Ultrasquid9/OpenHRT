use std::path::{Path, PathBuf};

use macroquad::{math::Vec2, miniquad};
use serde::{Deserialize, de::DeserializeOwned};
use tracing::error;

use crate::race::{Race, horse::Horse};

#[derive(Deserialize)]
pub struct RaceData {
	foreground: PathBuf,
	background: PathBuf,
	seed: Option<u64>,
	skip_intro: Option<bool>,
	horses: Vec<(PathBuf, Vec2)>,
	gate: GateData,
}

#[derive(Deserialize)]
pub struct HorseData {
	sprite: PathBuf,
}

#[derive(Deserialize)]
pub struct GateData {
	start: Vec2,
	end: Vec2,
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
		for (path, pos) in self.horses {
			let horse = read::<HorseData>(path);
			horses.push(horse.into_horse(pos).await);
		}

		let mut race = Race::new(
			&stringify(self.foreground),
			&stringify(self.background),
			&horses,
			self.gate,
		)
		.await;
		race.skip_intro(self.skip_intro.unwrap_or(false));
		race
	}
}

impl Default for RaceData {
	fn default() -> Self {
		Self {
			foreground: PathBuf::new(),
			background: PathBuf::new(),
			seed: None,
			skip_intro: None,
			horses: vec![],
			gate: GateData::default(),
		}
	}
}

impl HorseData {
	pub async fn into_horse(self, pos: Vec2) -> Horse {
		Horse::new(pos, &stringify(self.sprite)).await
	}
}

impl Default for HorseData {
	fn default() -> Self {
		Self {
			sprite: PathBuf::new(),
		}
	}
}

impl GateData {
	pub fn into_pos_size(self) -> (Vec2, Vec2) {
		(self.start, self.end - self.start)
	}
}

impl Default for GateData {
	fn default() -> Self {
		Self {
			start: Vec2::ZERO,
			end: Vec2::ZERO,
		}
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
