use macroquad::prelude::*;

mod audio;
mod data;
mod race;
mod utils;

#[macroquad::main("OpenHRT")]
async fn main() {
	let mut race = race();

	loop {
		race.update();
		race.draw();

		next_frame().await
	}
}

fn race() -> race::Race {
	let runtime = tokio::runtime::Runtime::new().unwrap();
	runtime.block_on(async {
		data::RaceData::load("./data/race.toml")
			.set_seed()
			.into_race()
			.await
	})
}
