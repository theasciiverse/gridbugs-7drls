# Boat Journey

**It's time for you to head to the ocean.** Everyone in town says so.
There is a boat at the jetty and a river that runs all the way there.
Steer it through a big generated landscape, turning in eighths of a
circle; land to pick up junk and trade it at the inns for fuel and seats;
take on passengers who each lend you a power (the soldier's blast, the
surgeon's healing, the thief's stealth, the physicist's teleporter, the
surveyor's telescope, the ghost's incorporeal moments), each drawn in a
hand-made ANSI portrait; and be inside before dark, because the beasts
and the ghosts mostly come at night.

Boat Journey is Stephen Sherratt's (gridbugs) roguelike from the 7DRL
Challenge 2023. It is part of **gridbugs' 7DRLs** in Asciiverse, with
Meters Below the Ground, slime99, Orbital Decay, Rain Forest, Electric
Organ and Road Closed.

## Playing

The game is laid out for an 80x60 terminal with true colour (`--rgb`,
`--ansi` or `--greyscale` for terminals without it).

- Arrows: walk; walk into doors to open them and into people to talk
- e: take the wheel standing on it, or let go
- At the wheel: Up and Down drive, Left and Right turn
- 1 to 9: passengers' abilities; Space: wait; Escape: menu

Saves, settings and controls live in `$XDG_STATE_HOME/boat-journey`
(else `~/.local/state/boat-journey`).

## This edition

Built from the game's own terminal frontend with no changes to the game.
What we changed is in PATCHES.md, where it came from in UPSTREAM.md.
Session log: `RUST_LOG=debug boat-journey 2>boat-journey.log`.

Source: https://github.com/theasciiverse/gridbugs-7drls (folder
`boat-journey`), from https://github.com/gridbugs/boat-journey.
Licence: MIT, Stephen Sherratt (LICENSE).
