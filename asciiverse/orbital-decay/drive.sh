# Orbital Decay preview: the prologue, then the first floor of the station
# on a fixed seed (-r 7) and a fixed walk: through the door marked DON'T
# OPEN, past credits, an upgrade and a map, into a skeleton; then saving
# and quitting. Recorded at the game's own 80x60, cropped to 100x30 by hub.
COLS=80 ROWS=60 TITLE="Orbital Decay" ARGS="-r 7"
drive() {
  sleep 3
  k Space 4                     # the prologue, read for a moment
  k n 1.5
  for m in Right Right Right Right Right Up Up; do k $m 0.3; done
  for m in Up Down Down Up Left Down Right Down Up Down Up Right Left Down Up Up Right Down Down Right Right Up Up Up Down Right Up Up Up Down Up Left Up Left Right Down Right Right Right Down Right Up Left Up Up Up Right Up Left Right Left Right Down Right Down Left Down Down Right Down Up Left Up Left Down Up Left Down Down Down Up Up Down Left Left Up Up Right Right Up Left Up Right Up Up Left Right Right Up Up Down Down Up Right Down Left Down Left Down Up Up Left Up Up Up Down Down Up Up Right Left Down Left Up Up Left Left Left Up Right; do k $m 0.22; done
  sleep 2
  k Escape 1.5; k q 2
}
