use std::sync::LazyLock;

use hex_literal::hex;
use macroquad::prelude::*;

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
		Err(_) => DEBUG_IMG.clone(),
	}
}
