# slime99

**Go into the sewers and ELIMINATE THE SOURCE OF SLIME!** It is the
year 1999, fallout from THE WAR has filled the sewers of THE CITY with
radioactive mutant slimes, and you are a genetically-modified precog
super-soldier who can see the outcome of every fight before it happens.

There are no dice: beside the map are the stacks of every attack, defence
and tech you will use, in order, Hit 7 and Cleave 3 and Armour 2 and,
somewhere down there, Die. Abilities let you stash, swap or discard the
top of a stack; techs let you blink, attract, repel, crit, skip. Six
floors of divide, swap, teleport, goo, curse and upgrade slimes, toxic
sludge and bridges, and the Source of Slime at the bottom.

slime99 is Stephen Sherratt's (gridbugs) roguelike from the 7DRL
Challenge 2020. It is part of **gridbugs' 7DRLs** in Asciiverse, with
Meters Below the Ground, Orbital Decay, Rain Forest, Boat Journey,
Electric Organ and Road Closed.

## Playing

Drawn for a 60x40 terminal with true colour (`--rgb`, `--ansi` or
`--greyscale` for terminals without it).

- Arrows, WASD or HJKL: move, and walk into slimes to fight
- t: use the top tech; Escape: cancel aiming, or the menu
- 1 to 8: abilities; Space: wait; x: examine
- A gamepad works too

Saves live in `$XDG_STATE_HOME/slime99` (else `~/.local/state/slime99`).

## This edition

Built from the game's own terminal frontend with no changes to the game.
It is silent: the music (Nihilore, CC BY 4.0) and the explosion sound
are left out, as the explosion's source is not stated upstream. What we
changed is in PATCHES.md, where it came from in UPSTREAM.md. Session log:
`RUST_LOG=debug slime99 2>slime99.log`.

Source: https://github.com/theasciiverse/gridbugs-7drls (folder
`slime99`), from https://github.com/gridbugs/slime99.
Licence: MIT, Stephen Sherratt (LICENSE).
