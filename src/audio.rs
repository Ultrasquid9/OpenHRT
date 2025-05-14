use hashbrown::HashMap;
use parking_lot::RwLock;
use std::sync::LazyLock;

use kira::{
	AudioManager, AudioManagerSettings, DefaultBackend,
	sound::static_sound::{StaticSoundData, StaticSoundHandle},
};

// TODO: Less unwrap

type Global<T> = LazyLock<RwLock<T>>;
type AMan = AudioManager<DefaultBackend>;
type ADat = HashMap<String, StaticSoundData>;

static MANAGER: Global<AMan> = LazyLock::new(manager);
static AUDIO: Global<ADat> = LazyLock::new(audio);

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

fn manager() -> RwLock<AMan> {
	RwLock::new(AudioManager::new(AudioManagerSettings::default()).unwrap())
}

fn audio() -> RwLock<ADat> {
	RwLock::new(HashMap::new())
}
