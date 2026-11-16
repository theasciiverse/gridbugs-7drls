extern crate env_logger;
#[macro_use]
extern crate log;
extern crate meters_prototty;
extern crate prototty_unix;
extern crate rand;

use meters_prototty::*;
use prototty_unix::{Context, FileStorage};
use rand::Rng;
use std::env;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;

const USER_DIR: &'static str = "user";
/// Asciiverse: saves and settings live in `$XDG_STATE_HOME/<STATE_ID>`, else
/// `~/.local/state/<STATE_ID>`, or `%LOCALAPPDATA%\<STATE_ID>` on Windows; upstream's
/// `user` folder next to the executable when none of those is set.
const STATE_ID: &'static str = "meters-below-the-ground";
const TICK_MILLIS: u64 = 33;

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

fn main() {
    // Session log on stderr (Asciiverse): RUST_LOG=debug for everything, errors only by default.
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("error")).init();
    info!(
        "Meters Below the Ground {} started: {:?}",
        env!("CARGO_PKG_VERSION"),
        env::args().skip(1).collect::<Vec<_>>()
    );
    let storage = match state_dir() {
        Some(dir) => {
            info!("state folder: {}", dir.display());
            FileStorage::new(dir, true)
        }
        None => FileStorage::next_to_exe(USER_DIR, true),
    }
    .expect("Failed to find user dir");

    let mut context = Context::new().unwrap();

    let mut app = App::new(Frontend::Unix, storage, rand::thread_rng().gen());

    let mut view = AppView::new(context.size().unwrap());

    loop {
        view.set_size(context.size().unwrap());

        context.render(&mut view, &app).unwrap();
        thread::sleep(Duration::from_millis(TICK_MILLIS));

        if let Some(control_flow) = app.tick(
            context.drain_input().unwrap(),
            Duration::from_millis(TICK_MILLIS),
            &view,
        ) {
            match control_flow {
                ControlFlow::Quit => break,
            }
        }
    }
}
