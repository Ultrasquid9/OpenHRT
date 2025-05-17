#![warn(clippy::pedantic)]
#![allow(clippy::cast_lossless)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::semicolon_if_nothing_returned)]

use macroquad::{miniquad::conf::Platform, prelude::*};

mod audio;
mod data;
mod race;
mod utils;

#[macroquad::main(conf)]
async fn main() {
	utils::init_log();

	let mut race = race();

	loop {
		race.draw();
		race.update();

		if race.should_finish() {
			tracing::info!("Race finished");
			return;
		}

		next_frame().await
	}
}

fn race() -> race::Race {
	let runtime = tokio::runtime::Runtime::new().unwrap_or_else(|e| {
		tracing::error!("Could not create runtime: {e}");
		panic!()
	});

	runtime.block_on(async {
		data::RaceData::load("./data/race.toml")
			.set_seed()
			.into_race()
			.await
	})
}

fn conf() -> Conf {
	Conf {
		window_title: "OpenHRT".into(),
		platform: Platform {
			//swap_interval: Some(0),
			..Default::default()
		},
		..Default::default()
	}
}
