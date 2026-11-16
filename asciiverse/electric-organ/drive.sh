# Electric Organ preview: the title, then the first level of Retro-Future
# City, walked by explore.py out of the evac zone and through the streets
# and buildings, a look at your organs (o), more walking, then saving and
# quitting. Sound off (--mute). 100x30; the game draws 80x30.
COLS=100 ROWS=30 TITLE="Electric Organ" ARGS="--mute"
drive() {
  sleep 3
  k n 2
  python3 -I "$here/explore.py" "$sock" rec 0 0 50 25 45 0.25
  k o 4; k Escape 1
  python3 -I "$here/explore.py" "$sock" rec 0 0 50 25 35 0.25
  sleep 1
  k Escape 1.5; k q 2
}
