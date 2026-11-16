# Electric Organ

**A corruption is taking over Retro-Future City, and only you can stop
it.** Four levels down waits the CORRUPTOR. To reach it you fight through
zombies that rise again, dividers that split, boomers that explode and
glowers that pour out radiation, and you keep yourself alive by changing
what you are made of: hearts, lungs, stomachs and livers harvested from
the dead and installed at the clinic, cybernetic upgrades that need a
CyberCore, Cronenberg pistols and shotguns grown into your body and fired
with your own health. Radiation brings mutations; tumours copy
themselves. Every note of music and every sound effect is synthesised
live.

Electric Organ is Stephen Sherratt's (gridbugs) roguelike from the 7DRL
Challenge 2024. It is part of **gridbugs' 7DRLs** in Asciiverse, with
Meters Below the Ground, slime99, Orbital Decay, Rain Forest, Boat
Journey and Road Closed.

## Playing

Drawn for an 80x30 terminal with true colour (`--rgb`, `--ansi` or
`--greyscale` for terminals without it). `--mute` turns the sound off.

- Arrows: walk, and walk into enemies to fight
- f: fire; c: fire every Cronenberg weapon; r: reload
- g: get; a: apply; d: drop; u: unequip
- o: organs; m: message log; ?: help; Escape: menu (save and quit)

Saves, settings and controls live in `$XDG_STATE_HOME/electric-organ`
(else `~/.local/state/electric-organ`).

## This edition

Built from the game's own terminal frontend. The sound plays through
ALSA on Linux (libasound2); with no audio device the game now plays on
silently instead of stopping. What we changed is in PATCHES.md, where it
came from in UPSTREAM.md. Session log: `RUST_LOG=debug electric-organ
2>electric-organ.log`.

Source: https://github.com/theasciiverse/gridbugs-7drls (folder
`electric-organ`), from https://github.com/gridbugs/electric-organ.
Licence: MIT, Stephen Sherratt (LICENSE).
