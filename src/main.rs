#![warn(clippy::pedantic)]
#![allow(clippy::cast_lossless)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::semicolon_if_nothing_returned)]

use macroquad::prelude::*;

mod audio;
mod data;
mod race;
mod utils;

#[macroquad::main("OpenHRT")]
async fn main() {
	utils::init_log();

	let mut race = race();

	loop {
		race.draw();
		race.update();

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
