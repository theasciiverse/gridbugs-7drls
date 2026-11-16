# gridbugs-7drls

Asciiverse collection repo: Stephen Sherratt's (gridbugs) 7DRLs in their
terminal editions, one archive and one catalog title per game, collection
key `gridbugs-7drls` in hub's `catalog/collections.toml`. Read PATCHES.md
and UPSTREAM.md first.

- Each game is its own folder, its own Cargo workspace and lockfile,
  brought in with `git subtree` (history kept). Change as little as
  possible inside them; every change is listed in PATCHES.md.
- `asciiverse/games.txt` lists id, folder, cargo package and flags;
  `asciiverse/package.sh <target> [id ...]` builds and packs.
- Saves in `$XDG_STATE_HOME/<id>`. Tests, scripted play and recordings
  run with a throwaway HOME and XDG_STATE_HOME, `--mute` for Electric
  Organ, under `systemd-run --user --scope -p MemoryMax=4G`, `-j4`.
- Linux and macOS only (termios). Tags `vX.Y.Z` for the whole collection.
- Commit as `David <sandquist@gmail.com>`; no em or en dashes anywhere.
