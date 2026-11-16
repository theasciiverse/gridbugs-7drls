# Road Closed preview: the title, slipping past the cordon (the car over
# the opening text), driving on through the zone, stopping at the next
# place and walking it (explore.py) among the grass, trees and what lives
# there, the help, then saving and quitting. 100x30; the game draws 80x30.
COLS=100 ROWS=30 TITLE="Road Closed" ARGS=""
drive() {
  sleep 2.5
  k n 4                         # the car and the opening text
  k d 3; k d 3                  # drive on
  k s 2                         # stop and get out
  python3 -I "$here/explore.py" "$sock" rec 0 0 50 25 45 0.25 '#&/=\|_[]'
  k i 3; k Escape 1
  python3 -I "$here/explore.py" "$sock" rec 0 0 50 25 25 0.25 '#&/=\|_[]'
  sleep 1
  k Escape 1.5; k q 2
}
