use std::sync::LazyLock;

use hex_literal::hex;
use macroquad::prelude::*;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

pub type Dirs<const AMOUNT: usize> = [Vec2; AMOUNT];

static DEBUG_IMG: LazyLock<Image> = LazyLock::new(|| {
	Image {
		width: 3,
		height: 3,
		bytes: hex!(
			"ff0000ff 00ff00ff 0000ffff" // red     green   blue
			"ffff00ff 00ffffff ff00ffff" // yellow  cyan    magenta
			"ffffffff 000000ff 666666ff" // white   black   gray
		)
		.to_vec(),
	}
});

pub async fn load_img(path: &str) -> Image {
	match load_image(path).await {
		Ok(ok) => ok,
		Err(e) => {
			tracing::warn!("{e}");
			DEBUG_IMG.clone()
		}
	}
}

pub async fn load_img_blocking(path: String) -> Image {
	match tokio::task::spawn_blocking(async move || load_img(&path).await).await {
		Ok(ok) => ok.await,
		Err(e) => {
			tracing::warn!("{e}");
			DEBUG_IMG.clone()
		}
	}
}

pub fn init_log() {
	let subscriber = FmtSubscriber::builder()
		.with_max_level(Level::INFO)
		.finish();

	tracing::subscriber::set_global_default(subscriber).unwrap_or_else(|_| {
		tracing::error!("Tried to set global default more than once");
	})
}

/// Creates an array of [Vec2] from an array of tuples
#[macro_export]
macro_rules! dirs { ( $( ($x:expr, $y:expr) ),* $(,)? ) => { [ $( vec2($x, $y), )* ] } }
