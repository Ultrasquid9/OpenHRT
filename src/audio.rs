use hashbrown::HashMap;
use parking_lot::RwLock;
use std::sync::LazyLock;

use kira::{
	AudioManager, AudioManagerSettings,
	sound::static_sound::{StaticSoundData, StaticSoundHandle},
};

type Global<T> = LazyLock<RwLock<T>>;
type AudioCache = HashMap<String, StaticSoundData>;

static MANAGER: Global<AudioManager> = manager();
static AUDIO: Global<AudioCache> = audio();

pub fn play_or_load(key: &str) -> StaticSoundHandle {
	let mut writer = AUDIO.write();

	let data = if let Some(data) = writer.get(key) {
		data.clone()
	} else {
		let data = read(key);
		writer.insert(key.into(), data.clone());
		data
	};

	MANAGER
		.write()
		.play(data)
		.expect("Should be able to play audio!")
}

const fn manager() -> Global<AudioManager> {
	LazyLock::new(|| {
		RwLock::new(
			AudioManager::new(AudioManagerSettings::default()).unwrap_or_else(|e| {
				tracing::error!("Could not create audio manager: {e}");
				panic!()
			}),
		)
	})
}

const fn audio() -> Global<AudioCache> {
	LazyLock::new(|| RwLock::new(HashMap::new()))
}

fn read(path: &str) -> StaticSoundData {
	match StaticSoundData::from_file(path) {
		Ok(ok) => {
			tracing::info!("Audio \"{path}\" loaded!");
			ok
		}
		Err(e) => {
			tracing::error!("Failed to load audio from \"{path}\": {e}");
			panic!()
		}
	}
}
