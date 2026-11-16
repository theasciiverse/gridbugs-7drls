# Upstream

Every game here is Stephen Sherratt's (gridbugs, https://gridbugs.org),
MIT licensed, each from its own repository on GitHub. They came into this
repo with `git subtree add`, unsquashed, so each folder keeps its full
upstream history (`git log -- orbital-decay`). The remotes are named
`up-<folder>`; a later upstream change comes in with
`git subtree pull --prefix=<folder> up-<folder> <branch>`.

| Game | Folder | Upstream | Branch at |
|---|---|---|---|
| Meters Below the Ground (7DRL 2018) | `meters-below-the-ground` | https://github.com/gridbugs/meters-below-the-ground | `master` 6f4be80 (2019-02-04) |
| slime99 (7DRL 2020) | `slime99` | https://github.com/gridbugs/slime99 | `master` f0e5dd2 (2022-03-26) |
| Orbital Decay (7DRL 2021) | `orbital-decay` | https://github.com/gridbugs/orbital-decay | `main` 08eb7e2 (2023-03-17) |
| Rain Forest (7DRL 2022) | `rainforest` | https://github.com/gridbugs/rainforest | `main` 152afd1 (2024-10-29) |
| Boat Journey (7DRL 2023) | `boat-journey` | https://github.com/gridbugs/boat-journey | `main` ec06291 (2023-05-27) |
| Electric Organ (7DRL 2024) | `electric-organ` | https://github.com/gridbugs/electric-organ | `main` 6e5af91 (2026-01-28) |
| Road Closed (7DRL 2026) | `road-closed` | https://github.com/gridbugs/road-closed | `main` 74a9c9d (2026-03-07) |

Each game also has graphical (SDL2, wgpu, ggez) and web frontends; the
Asciiverse build uses only its terminal frontend (`ansi-terminal`, or
`unix` for Meters Below the Ground), and those folders are otherwise
untouched.

## Left out

- **RIP** (https://github.com/gridbugs/rip, 2019 to 2020): an unfinished
  prototype, one level of a station with no title screen, goal or ending.
  It came in with the others and was removed in its own commit; its
  history is still in this repo's.

## Offered back

Every change in PATCHES.md is written so it could go upstream: the state
folder falls back to upstream's folder next to the executable, the sound
features are on by default, and the Rain Forest fix is a plain bug fix.
Nothing has been sent.
