use macroquad::prelude::*;

mod audio;
mod data;
mod race;

#[macroquad::main("OpenHRT")]
async fn main() {
	let mut game = data::RaceData::load("./data/race.toml")
		.set_seed()
		.into_race()
		.await;

	loop {
		game.update();
		game.draw();

		next_frame().await
	}
}
