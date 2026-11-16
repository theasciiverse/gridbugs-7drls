# Changes from upstream

This is the Asciiverse packaging of gridbugs' (Stephen Sherratt's) seven
day roguelikes, in their terminal editions: one repo, one release, one
archive and one catalog title per game. Where each comes from is in
UPSTREAM.md. The games themselves, their rules, maps, monsters and every
line of their text, are as he wrote them; the changes below are packaging,
one commit per game per change.

## Every game

- **State folder.** Saves, settings and controls live in
  `$XDG_STATE_HOME/<id>`, else `~/.local/state/<id>` (or
  `%LOCALAPPDATA%\<id>` on Windows), instead of a `save` (or `user`)
  folder next to the executable, which a package manager may replace or
  may not let the game write to. `--storage-dir` (`--save-dir` in
  slime99) still picks a folder next to the executable, and with none of
  those variables set the game keeps upstream's folder.
  (`<game>/native/src/lib.rs`, `meters-below-the-ground/unix/src/main.rs`)
- **Session logging** with `log` and `env_logger`, on stderr:
  `RUST_LOG=debug orbital-decay 2>orbital-decay.log` logs the start, the
  arguments, the state folder and everything the games already log (save
  and load errors, audio, generation). Errors only without `RUST_LOG`.
  (`<game>/ansi-terminal/src/main.rs`, `meters-below-the-ground/unix`)

## Per game

- **Orbital Decay, slime99: sound as a cargo feature.** Both games embed
  their music and sound effects in the binary and play them through the
  system's sound device. A new `sound` feature, on by default so every
  upstream frontend is unchanged, carries the audio files and the audio
  backend; our terminal builds use `--no-default-features`, so they hold
  no audio at all and play silently (`--mute` is still accepted). The
  sources and licences of Orbital Decay's sounds, and of slime99's
  explosion, are not stated anywhere upstream, so we do not ship them.
  slime99's music is Nihilore's (http://www.nihilore.com, CC BY 4.0) and
  could come back with that credit once the explosion sound is settled.
  (`app/Cargo.toml`, `app/src/audio.rs`, `native/`, `ansi-terminal/`)
- **Rain Forest: the terminal frontend builds again.** Upstream moved the
  game to gridbugs 0.7 in 2024 but left `ansi-terminal` on 0.2, so it
  failed to compile (two chargrid versions). It now uses 0.7 like the rest
  of the game. Rain Forest has no sound, so the unused native audio
  backend (and with it ALSA) is dropped. (`rainforest/ansi-terminal`,
  `rainforest/native`, `rainforest/app/Cargo.toml`, `Cargo.lock`)
- **Electric Organ** keeps its sound: its music and effects are
  synthesised live by gridbugs' own `caw` library (MIT) from code in
  `app/src/music`; there are no recordings. Linux builds need libasound2.
  Upstream opened the audio output on the first tick and stopped with a
  panic where there was none (a server, an SSH session), even with
  `--mute`; now `--mute` opens no output at all, and a failure to open
  one is logged once and the game plays on silent.
  (`electric-organ/app/src/music/mod.rs`, `app/src/game_loop.rs`)
- **Meters Below the Ground:** upstream ignores `Cargo.lock`; ours is
  committed so the build is repeatable (`--locked`).

## Packaging (new files)

- `asciiverse/games.txt`: the games, their folders, cargo packages and
  flags.
- `asciiverse/package.sh`: builds one game or all and packs
  `dist/<id>-<target>.tar.gz` with the binary (named `<id>`), the game's
  `LICENSE`, its README, this file and UPSTREAM.md.
- `.github/workflows/release.yml`: runs `package.sh` for Linux (glibc,
  x86-64 and ARM64) and macOS (Intel and Apple silicon), and attaches the
  archives and SHA256SUMS to the release. No Windows: the terminal
  frontend uses termios.
- `asciiverse/<id>/README.md`: each game's package README.
- `asciiverse/<id>/<id>.toml`, `.md`, `.cast`, `record-preview.sh`: the
  catalog manifest, store page and preview, drafts for hub.
- `README.md`, `UPSTREAM.md`, this file, `CLAUDE.md`.
