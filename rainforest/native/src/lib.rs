use gridbugs::storage::{FileStorage, IfDirectoryMissing, Storage};

pub use meap;
use rainforest_app::{AppStorage, InitialRngSeed};
use std::path::PathBuf;

const DEFAULT_SAVE_FILE: &str = "save";
const DEFAULT_NEXT_TO_EXE_STORAGE_DIR: &str = "save";
const DEFAULT_CONTROLS_FILE: &str = "controls.json";

/// Asciiverse: the folder for saves, settings and controls, `$XDG_STATE_HOME/<STATE_ID>`, else
/// `~/.local/state/<STATE_ID>`, or `%LOCALAPPDATA%\<STATE_ID>` on Windows. None when none of
/// those is set; the game then keeps upstream's `save` folder next to the executable.
const STATE_ID: &str = "rain-forest";

fn state_dir() -> Option<std::path::PathBuf> {
    use std::{env, path::PathBuf};
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
    pub storage: AppStorage,
    pub initial_rng_seed: InitialRngSeed,
    pub omniscient: bool,
    pub new_game: bool,
}

impl NativeCommon {
    pub fn parser() -> impl meap::Parser<Item = Self> {
        meap::let_map! {
            let {
                rng_seed = opt_opt::<u64, _>("INT", 'r').name("rng-seed").desc("rng seed to use for first new game");
                save_file = opt_opt("PATH", 's').name("save-file").desc("save file")
                    .with_default(DEFAULT_SAVE_FILE.to_string());
                controls_file = opt_opt("PATH", "controls-file").desc("controls file")
                    .with_default(DEFAULT_CONTROLS_FILE.to_string());
                storage_dir = opt_opt::<String, _>("PATH", 'd').name("storage-dir")
                    .desc("directory next to the executable that will contain state (default: the state folder)");
                delete_save = flag("delete-save").desc("delete save game file");
                new_game = flag("new-game").desc("start a new game, skipping the menu");
                omniscient = flag("omniscient").desc("enable omniscience");
            } in {{
                let initial_rng_seed = rng_seed.map(InitialRngSeed::U64).unwrap_or(InitialRngSeed::Random);
                let state = match storage_dir {
                    Some(dir) => FileStorage::next_to_exe(&dir, IfDirectoryMissing::Create).map_err(|_| dir),
                    None => match state_dir() {
                        Some(dir) => FileStorage::new(dir, IfDirectoryMissing::Create)
                            .map_err(|_| DEFAULT_NEXT_TO_EXE_STORAGE_DIR.to_string()),
                        None => FileStorage::next_to_exe(DEFAULT_NEXT_TO_EXE_STORAGE_DIR, IfDirectoryMissing::Create)
                            .map_err(|_| DEFAULT_NEXT_TO_EXE_STORAGE_DIR.to_string()),
                    },
                };
                let file_storage = match state {
                    Ok(fs) => fs,
                    Err(storage_dir) => {
                        log::warn!("Couldn't create the save dir. Will use temporary directory instead.");
                        FileStorage::temp(PathBuf::from("rainforest").join(storage_dir), IfDirectoryMissing::Create)
                            .expect("failed to open directory")
                    }
                };
                log::info!("state folder: {}", file_storage.full_path("").display());
                let mut file_storage = Storage::new(file_storage);
                if delete_save {
                    let result = file_storage.remove(&save_file);
                    if result.is_err() {
                        log::warn!("couldn't find save file to delete");
                    }
                }
                let storage = AppStorage {
                    handle: file_storage,
                    save_game_key: save_file,
                    controls_key: controls_file,
                };
                Self {
                    initial_rng_seed,
                    storage,
                    omniscient,
                    new_game,
                }
            }}
        }
    }
}
