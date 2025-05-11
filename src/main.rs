use macroquad::prelude::*;
use horse::Horse;

mod audio;
mod game;
mod horse;

#[macroquad::main("OpenHRT")]
async fn main() {
	let horses = [
		Horse::new(vec2(400., 400.), "./assets/purble.png").await,
		Horse::new(vec2(420., 420.), "./assets/purble.png").await,
		Horse::new(vec2(400., 440.), "./assets/purble.png").await,
		Horse::new(vec2(440., 400.), "./assets/purble.png").await,
	];

	let mut game = game::Game::new(
		"./assets/arenatest1.png",
		"./assets/backgroundtest1.png",
		&horses,
	)
	.await;

	loop {
		game.update();
		game.draw();

		next_frame().await
	}
}
