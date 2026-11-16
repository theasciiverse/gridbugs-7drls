# gridbugs' 7DRLs

Every March, game makers around the world take the Seven Day Roguelike
Challenge: one week, one complete roguelike. Stephen Sherratt, who makes
games as gridbugs, has taken it most years since 2018, each time on his
own Rust library for games drawn in characters, and each time with a
game that runs in a terminal as well as in a window or a browser. This
repository brings seven of them to Asciiverse, in their terminal
editions, each its own title in the catalog:

- **Meters Below the Ground** (2018): climb sixty metres out of an
  insect-infested facility, where everything, your health, your guns and
  the eggs about to hatch, is a progress meter
- **slime99** (2020): a precog super-soldier in the sewers of 1999, who
  can see every attack and defence to come and bends fate to make them
  count
- **Orbital Decay** (2021): carry a fuel cell down a falling space
  station full of undead crew, where every shot that misses may breach
  the hull
- **Rain Forest** (2022): five days at a cabin in the rain, with no
  enemies but the weather and your own motivation
- **Boat Journey** (2023): steer a boat down a river to the ocean, with
  passengers who each lend you a power
- **Electric Organ** (2024): fight down through Retro-Future City and
  rebuild your body from the organs of the dead, to a synthesised
  soundtrack
- **Road Closed** (2026): a road trip two thousand kilometres into an
  exclusion zone, towards a room that grants wishes

## Playing

Install them from Asciiverse (`av`), or unpack a release archive and run
the binary: `orbital-decay`, `boat-journey` and so on. Each game's
README.md says how it plays. Saves and settings live in
`$XDG_STATE_HOME/<id>` (else `~/.local/state/<id>`). Linux and macOS.

## This repository

Each game is a folder brought in from its own upstream repository with
its full history (UPSTREAM.md); what we changed, all of it packaging, is
in PATCHES.md. `asciiverse/package.sh <target> [id ...]` builds and packs
the archives (`asciiverse/games.txt` lists the games and their cargo
flags), and `asciiverse/record-preview.sh` records the catalog previews.

All the games are MIT licensed, Copyright Stephen Sherratt (each
folder's LICENSE). RIP, a 2020 prototype, is left out (UPSTREAM.md).
