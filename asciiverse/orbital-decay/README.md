# Orbital Decay

**DON'T OPEN! DEAD INSIDE!** say the words smeared in blood on the airlock
door, and you are going through it anyway. The station is out of fuel and
falling towards Earth; you carry the fuel cell that can save it, five
floors down to the fuel bay, past what is left of the crew.

Orbital Decay is Stephen Sherratt's (gridbugs) turn-based tactical
roguelike from the 7DRL Challenge 2021, about ranged combat where every
gun is also a way to put a hole in the hull. Shots pass through enemies
weaker than their penetration and fly on; the ones that reach the wall
may breach it, and then the air, the loot and you go rushing towards the
vacuum. Rifle, shotgun, railgun, chainsaw, gaus cannon, oxidiser and life
stealer; credits and upgrades; a map if you can find one.

It is part of **gridbugs' 7DRLs** in Asciiverse, with Meters Below the
Ground, slime99, Rain Forest, Boat Journey, Electric Organ and Road
Closed.

## Playing

The game is drawn for an 80x60 terminal with true colour (`--rgb`,
`--ansi` or `--greyscale` for terminals without it).

- Arrows, WASD or HJKL: walk and aim
- 1 to 3: fire a gun; Escape: cancel the aim, or open the menu
- Space: wait; G: pick up a weapon; X: examine
- A gamepad works too

Saves, settings and controls live in `$XDG_STATE_HOME/orbital-decay`
(else `~/.local/state/orbital-decay`). `--help` lists every option.

## This edition

Built from the game's own terminal frontend with no changes to the game.
It is silent: the music and sound effects are left out, as their
licences are not stated upstream. What we changed is in PATCHES.md, where
it came from in UPSTREAM.md. Session log: `RUST_LOG=debug orbital-decay
2>orbital-decay.log`.

Source: https://github.com/theasciiverse/gridbugs-7drls (folder
`orbital-decay`), from https://github.com/gridbugs/orbital-decay.
Licence: MIT, Stephen Sherratt (LICENSE).
