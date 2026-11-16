# Rain Forest

**You've booked five days at a cabin in the forest.** You arrive close
to midnight, exhausted, looking forward to falling asleep to the sound of
rain, and see the cabin's lights through the trees.

Rain Forest is a chill turn-based game with no enemies: your one meter is
motivation, worn down by rain, flood water and the dark, and built back
up by sheltering under trees, hopping across stepping stones, and the
small pleasures of a stay in the woods: a cozy chair, a cup of tea, a
flower for the abandoned altar, a lake to contemplate, a bed. Find an
umbrella, gumboots, a lantern, a crowbar for the rocks, a shovel for
ditches, a map and a weather report, and last the five days.

Rain Forest is Stephen Sherratt's (gridbugs) game from the 7DRL
Challenge 2022. It is part of **gridbugs' 7DRLs** in Asciiverse, with
Meters Below the Ground, slime99, Orbital Decay, Boat Journey, Electric
Organ and Road Closed.

## Playing

Drawn for an 80x60 terminal with true colour (`--rgb`, `--ansi` or
`--greyscale` for terminals without it).

- Arrows, WASD or HJKL: walk; Shift with WASD or HJKL: walk quickly
- Space: wait 2 minutes; .: wait an hour
- g: pick up or put down; x: examine; m: map; r: weather report
- f: lantern; p: push rocks; e: dig; Escape: menu (save and quit)
- A gamepad works too

Saves, settings and controls live in `$XDG_STATE_HOME/rain-forest`
(else `~/.local/state/rain-forest`).

## This edition

Built from the game's own terminal frontend, which upstream had left on
an older version of its library so that it no longer compiled; it builds
again here, with no changes to the game. What we changed is in
PATCHES.md, where it came from in UPSTREAM.md. Session log:
`RUST_LOG=debug rain-forest 2>rain-forest.log`.

Source: https://github.com/theasciiverse/gridbugs-7drls (folder
`rainforest`), from https://github.com/gridbugs/rainforest.
Licence: MIT, Stephen Sherratt (LICENSE).
