# Meters Below the Ground

**Kill the eggs before they hatch!** You are sixty metres down in a
facility overrun by insects, and the way out is up. Each level gives you
an order (kill the eggs, activate the emergency beacon, kill the boss),
and everything in the game is a progress meter: your health, armour and
stamina, your quadgun, railgun, push, blink, medkit and metabol, the
eggs' hatching, and the metres still between you and the surface. Eggs
hatch into larvae, larvae become chrysalises, chrysalises break open as
aracnoids and beetoids; super eggs hatch into queens.

Meters Below the Ground is Stephen Sherratt's (gridbugs) roguelike from
the 7DRL Challenge 2018, the first of his seven-day roguelikes. It is
part of **gridbugs' 7DRLs** in Asciiverse, with slime99, Orbital Decay,
Rain Forest, Boat Journey, Electric Organ and Road Closed.

## Playing

Needs a 60x44 terminal with 256 colours.

- Arrows: move, and walk into enemies to punch
- 0 to 9: use a meter, then a direction to aim it
- Space: wait; Escape: menu (save and quit)

Saves live in `$XDG_STATE_HOME/meters-below-the-ground` (else
`~/.local/state/meters-below-the-ground`).

## This edition

Built from the game's own terminal frontend (prototty's unix context)
with no changes to the game; the fonts of its window and web editions
are not part of it. What we changed is in PATCHES.md, where it came from
in UPSTREAM.md. Session log: `RUST_LOG=debug meters-below-the-ground
2>meters.log`.

Source: https://github.com/theasciiverse/gridbugs-7drls (folder
`meters-below-the-ground`), from
https://github.com/gridbugs/meters-below-the-ground.
Licence: MIT, Stephen Sherratt (LICENSE).
