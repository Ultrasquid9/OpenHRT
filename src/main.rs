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
		race.update().await;
		race.draw();

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
