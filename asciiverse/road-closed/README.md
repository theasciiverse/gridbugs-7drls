# Road Closed

**You slip past the cordon in the early hours of the morning.** Somewhere
deep within the zone there is rumoured to be a room that will grant any
wish, two thousand kilometres down a closed road. You drive; you stop
when you must, at a fire trail through a pine plantation gone wild, a
swamp of flooded woodland and ruined cabins, a mountain pass; you
scavenge fuel, food, coffee and firewood, fight zombies that don't stay
dead, slimes that split and drainers that sap your strength, and you
light a fire to sleep through the night, because night stalkers come out
after dark and fear only fire.

Road Closed is Stephen Sherratt's (gridbugs) roguelike from the 7DRL
Challenge 2026, after Roadside Picnic and Pacific Drive. It is part of
**gridbugs' 7DRLs** in Asciiverse, with Meters Below the Ground, slime99,
Orbital Decay, Rain Forest, Boat Journey and Electric Organ.

## Playing

Drawn for an 80x30 terminal with true colour (`--rgb`, `--ansi` or
`--greyscale` for terminals without it).

- Arrows: walk, and walk into enemies to fight
- Walk into the side of the car to drive, into its back for the boot
- g: get; i: inventory; a: apply; d: drop; u: unequip; r: remove armour
- m: message log; ?: help; Escape: menu (save and quit)

Saves, settings, controls and every wish you make live in
`$XDG_STATE_HOME/road-closed` (else `~/.local/state/road-closed`).

## This edition

Built from the game's own terminal frontend with no changes to the game.
What we changed is in PATCHES.md, where it came from in UPSTREAM.md.
Session log: `RUST_LOG=debug road-closed 2>road-closed.log`.

Source: https://github.com/theasciiverse/gridbugs-7drls (folder
`road-closed`), from https://github.com/gridbugs/road-closed.
Licence: MIT, Stephen Sherratt (LICENSE).
