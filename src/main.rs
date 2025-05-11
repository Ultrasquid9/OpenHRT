use horse::Horse;
use macroquad::prelude::*;

mod audio;
mod game;
mod horse;

#[macroquad::main("OpenHRT")]
async fn main() {
	let horses = [
		Horse::new(vec2(380., 380.), "./assets/purble.png").await,
		Horse::new(vec2(410., 410.), "./assets/purble.png").await,
		Horse::new(vec2(440., 440.), "./assets/purble.png").await,
		Horse::new(vec2(470., 470.), "./assets/purble.png").await,
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
