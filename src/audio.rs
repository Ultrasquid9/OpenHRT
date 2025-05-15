use hashbrown::HashMap;
use parking_lot::RwLock;
use std::sync::LazyLock;

use kira::{
	AudioManager, AudioManagerSettings,
	sound::static_sound::{StaticSoundData, StaticSoundHandle},
};

// TODO: Less unwrap

type Global<T> = LazyLock<RwLock<T>>;
type AudioCache = HashMap<String, StaticSoundData>;

static MANAGER: Global<AudioManager> = manager();
static AUDIO: Global<AudioCache> = audio();

pub fn play_or_load(key: &str) -> StaticSoundHandle {
	let mut writer = AUDIO.write();

	let data = match writer.get(key) {
		Some(data) => data.clone(),
		None => {
			let path = "./assets/".to_string() + key;
			let data = StaticSoundData::from_file(path).unwrap();
			writer.insert(key.into(), data.clone());
			data
		}
	};

	MANAGER
		.write()
		.play(data)
		.expect("Should be able to play audio!")
}

const fn manager() -> Global<AudioManager> {
	LazyLock::new(|| RwLock::new(AudioManager::new(AudioManagerSettings::default()).unwrap()))
}

const fn audio() -> Global<AudioCache> {
	LazyLock::new(|| RwLock::new(HashMap::new()))
}
