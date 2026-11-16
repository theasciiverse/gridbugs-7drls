# Meters Below the Ground preview: the title, a new game, and the first
# level walked by explore.py: larvae and beetoids, eggs counting down to
# hatching, ammunition and medkits on the floor; then save and quit from
# the menu. Recorded at the game's own 60x44, cropped to 100x30 by hub.
COLS=60 ROWS=44 TITLE="Meters Below the Ground" ARGS=""
drive() {
  sleep 4
  k Enter 2
  python3 -I "$here/explore.py" "$sock" rec 0 5 29 29 60 0.25 '#'
  k 1 1.5
  python3 -I "$here/explore.py" "$sock" rec 0 5 29 29 50 0.25 '#'
  sleep 1
  k Escape 1.5; k Down 0.8; k Enter 2
}
