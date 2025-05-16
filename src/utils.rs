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

pub async fn load_img(path: String) -> Image {
	match load_image(&path).await {
		Ok(ok) => {
			tracing::info!("Image \"{path}\" loaded!");
			ok
		}
		Err(e) => {
			tracing::warn!("Image \"{path}\" failed to load: {e}");
			DEBUG_IMG.clone()
		}
	}
}

/// An alternative to `load_img` which can be multithreaded
pub fn load_img_2(path: String) -> Image {
	let bytes = match std::fs::read(&path) {
		Ok(ok) => ok,
		Err(e) => {
			tracing::warn!("Image \"{path}\" could not be read: {e}");
			return DEBUG_IMG.clone();
		}
	};

	let img = match Image::from_file_with_format(&bytes, None) {
		Ok(ok) => ok,
		Err(e) => {
			tracing::warn!("Image \"{path}\" failed to load: {e}");
			return DEBUG_IMG.clone();
		}
	};
	tracing::info!("Image \"{path}\" loaded!");
	img
}

pub async fn load_img_blocking(path: String) -> Image {
	// Because Macroquad panics when multithreaded, using `spawn` does not work.
	// `spawn_blocking` must be used instead.
	match tokio::task::spawn_blocking(async move || load_img(path).await).await {
		Ok(ok) => ok.await,
		Err(e) => {
			tracing::warn!("{e}");
			DEBUG_IMG.clone()
		}
	}
}

pub fn debug_img() -> Image {
	DEBUG_IMG.clone()
}

pub fn render_texture_fullscreen(texture: &Texture2D) {
	draw_texture_ex(
		texture,
		0.,
		0.,
		WHITE,
		DrawTextureParams {
			dest_size: Some(vec2(screen_width(), screen_height())),
			..Default::default()
		},
	);
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
