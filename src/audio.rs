use std::sync::LazyLock;
use parking_lot::RwLock;

use kira::{
	AudioManager, AudioManagerSettings, DefaultBackend, sound::static_sound::StaticSoundData,
};

// TODO: Less unwrap

type AMan = RwLock<AudioManager<DefaultBackend>>;

static MANAGER: LazyLock<AMan> = LazyLock::new(audio_manager);
// TODO: System for loading other audio 
static BOUNCE: LazyLock<StaticSoundData> =
	LazyLock::new(|| StaticSoundData::from_file("./assets/bounce.flac").unwrap());

pub fn play_bounce() {
	MANAGER.write().play(BOUNCE.clone()).unwrap();
}

fn audio_manager() -> AMan {
	RwLock::new(AudioManager::new(AudioManagerSettings::default()).unwrap())
}
