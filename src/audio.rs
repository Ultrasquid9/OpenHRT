use hashbrown::HashMap;
use parking_lot::RwLock;
use std::sync::LazyLock;

use kira::{
	AudioManager, AudioManagerSettings,
	sound::{
		FromFileError,
		static_sound::{StaticSoundData, StaticSoundHandle},
		streaming::{StreamingSoundData, StreamingSoundHandle},
	},
};

pub type StreamHandle = StreamingSoundHandle<FromFileError>;
type Global<T> = LazyLock<RwLock<T>>;
type AudioCache = HashMap<String, StaticSoundData>;

static MANAGER: Global<AudioManager> = manager();
static AUDIO: Global<AudioCache> = audio();

pub fn stream(path: &str) -> StreamHandle {
	let data = match StreamingSoundData::from_file(path) {
		Ok(ok) => ok,
		Err(e) => {
			tracing::error!("Failed to stream file \"{path}\": {e}");
			panic!()
		}
	};

	let handle = MANAGER
		.write()
		.play(data)
		.expect("Should be able to play audio!");
	tracing::info!("Audio {path} is streaming!");
	handle
}

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
