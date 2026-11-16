use general_audio_static::{AudioPlayer, StaticAudioPlayer, StaticHandle, StaticSound};
use maplit::hashmap;
use std::collections::HashMap;

pub type AppAudioPlayer = Option<StaticAudioPlayer>;
pub type AppSound = Option<StaticSound>;
pub type AppHandle = Option<StaticHandle>;

#[cfg(feature = "sound")]
mod data {
    pub const GAMEPLAY0: &[u8] = include_bytes!("./audio/Terminant.ogg");
    pub const GAMEPLAY1: &[u8] = include_bytes!("./audio/Disconnected.ogg");
    pub const GAMEPLAY2: &[u8] = include_bytes!("./audio/Absolute+Terror.ogg");
    pub const BOSS: &[u8] = include_bytes!("./audio/Panthalassa.ogg");
    pub const END_TEXT: &[u8] = include_bytes!("./audio/Bush+Week.ogg");
    pub const MENU: &[u8] = include_bytes!("./audio/10,000+People+Chanting,+-I'm+an+Individual-.ogg");
    pub const EXPLOSION: &[u8] = include_bytes!("./audio/explosion.ogg");
}
#[cfg(not(feature = "sound"))]
mod data {
    pub const GAMEPLAY0: &[u8] = &[];
    pub const GAMEPLAY1: &[u8] = &[];
    pub const GAMEPLAY2: &[u8] = &[];
    pub const BOSS: &[u8] = &[];
    pub const END_TEXT: &[u8] = &[];
    pub const MENU: &[u8] = &[];
    pub const EXPLOSION: &[u8] = &[];
}
use data::*;

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub enum Audio {
    Gameplay0,
    Gameplay1,
    Gameplay2,
    Boss,
    EndText,
    Menu,
    Explosion,
}

pub struct AudioTable {
    map: HashMap<Audio, AppSound>,
}

impl AudioTable {
    pub fn new(audio_player: &AppAudioPlayer) -> Self {
        let map = hashmap![
            Audio::Gameplay0 => audio_player.load_sound(GAMEPLAY0),
            Audio::Gameplay1 => audio_player.load_sound(GAMEPLAY1),
            Audio::Gameplay2=> audio_player.load_sound(GAMEPLAY2),
            Audio::Boss => audio_player.load_sound(BOSS),
            Audio::EndText => audio_player.load_sound(END_TEXT),
            Audio::Menu => audio_player.load_sound(MENU),
            Audio::Explosion => audio_player.load_sound(EXPLOSION),
        ];
        Self { map }
    }
    pub fn get(&self, audio: Audio) -> &AppSound {
        self.map.get(&audio).unwrap()
    }
}
