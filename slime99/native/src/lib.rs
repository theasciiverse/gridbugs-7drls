#[cfg(feature = "sound")]
use general_audio_static::{
    backend::{Error as NativeAudioError, NativeAudioPlayer},
    StaticAudioPlayer,
};
use general_storage_static::backend::{FileStorage, IfDirectoryMissing};
pub use general_storage_static::StaticStorage;
pub use meap;
use slime99_app::{AppAudioPlayer, Controls, GameConfig, Omniscient, RngSeed};
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

const DEFAULT_SAVE_FILE: &str = "save";
const DEFAULT_NEXT_TO_EXE_SAVE_DIR: &str = "save";
const DEFAULT_NEXT_TO_EXE_CONTROLS_FILE: &str = "controls.json";

/// Asciiverse: the folder for saves, `$XDG_STATE_HOME/<STATE_ID>`, else
/// `~/.local/state/<STATE_ID>`, or `%LOCALAPPDATA%\<STATE_ID>` on Windows. None when none of
/// those is set; the game then keeps upstream's `save` folder next to the executable.
const STATE_ID: &str = "slime99";

fn state_dir() -> Option<PathBuf> {
    let abs = |name: &str| env::var_os(name).map(PathBuf::from).filter(|p| p.is_absolute());
    let base = abs("XDG_STATE_HOME").or_else(|| {
        if cfg!(windows) {
            abs("LOCALAPPDATA")
        } else {
            abs("HOME").map(|home| home.join(".local").join("state"))
        }
    });
    base.map(|base| base.join(STATE_ID))
}

pub struct NativeCommon {
    pub rng_seed: RngSeed,
    pub save_file: String,
    pub file_storage: StaticStorage,
    pub controls: Controls,
    pub audio_player: AppAudioPlayer,
    pub game_config: GameConfig,
}

fn read_controls_file(path: &PathBuf) -> Option<Controls> {
    let mut buf = Vec::new();
    let mut f = File::open(path).ok()?;
    f.read_to_end(&mut buf).ok()?;
    serde_json::from_slice(&buf).ok()
}

impl NativeCommon {
    pub fn parser() -> impl meap::Parser<Item = Self> {
        meap::let_map! {
            let {
                rng_seed = opt_opt::<u64, _>("INT", 'r').name("rng-seed").desc("rng seed to use for first new game");
                save_file = opt_opt("PATH", 's').name("save-file").desc("save file")
                    .with_default(DEFAULT_SAVE_FILE.to_string());
                save_dir = opt_opt::<String, _>("PATH", 'd').name("save-dir")
                    .desc("save dir next to the executable (default: the state folder)");
                controls_file = opt_opt::<String, _>("PATH", 'c').name("controls-file").desc("controls file");
                delete_save = flag("delete-save").desc("delete save game file");
                omniscient = flag("omniscient").desc("enable omniscience");
                mute = flag('m').name("mute").desc("mute audio");
            } in {{
                let rng_seed = rng_seed.map(RngSeed::U64).unwrap_or(RngSeed::Random);
                let controls_file = if let Some(controls_file) = controls_file {
                    controls_file.into()
                } else {
                    env::current_exe().unwrap().parent().unwrap().join(DEFAULT_NEXT_TO_EXE_CONTROLS_FILE)
                        .to_path_buf()
                };
                let controls = read_controls_file(&controls_file).unwrap_or_else(Controls::default);
                let file_storage = match (save_dir, state_dir()) {
                    (Some(dir), _) => FileStorage::next_to_exe(dir, IfDirectoryMissing::Create),
                    (None, Some(dir)) => FileStorage::new(dir, IfDirectoryMissing::Create),
                    (None, None) => FileStorage::next_to_exe(DEFAULT_NEXT_TO_EXE_SAVE_DIR, IfDirectoryMissing::Create),
                }
                .expect("failed to open directory");
                log::info!("state folder: {}", file_storage.full_path("").display());
                let mut file_storage = StaticStorage::new(file_storage);
                if delete_save {
                    let result = file_storage.remove(&save_file);
                    if result.is_err() {
                        log::warn!("couldn't find save file to delete");
                    }
                }
                #[cfg(not(feature = "sound"))]
                let audio_player = {
                    let _ = mute;
                    None
                };
                #[cfg(feature = "sound")]
                let audio_player = if mute {
                    None
                } else {
                    match NativeAudioPlayer::try_new_default_device() {
                        Ok(audio_player) => Some(StaticAudioPlayer::new(audio_player)),
                        Err(NativeAudioError::FailedToCreateOutputStream) => {
                            log::warn!("no output audio device - continuing without audio");
                            None
                        }
                    }
                };
                let game_config = GameConfig {
                    omniscient: if omniscient {
                        Some(Omniscient)
                    } else {
                        None
                    }
                };
                Self {
                    rng_seed,
                    save_file,
                    file_storage,
                    controls,
                    audio_player,
                    game_config,
                }
            }}
        }
    }
}
