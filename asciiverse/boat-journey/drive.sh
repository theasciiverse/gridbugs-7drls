# Boat Journey preview: a new game on a fixed seed (-r 9): out of the inn
# into a townsperson ("It's time for you to head to the ocean."), across
# the town to the jetty, taking on a thief who wants to escape to the
# ocean, aboard the boat, and away up the river. The walk was found once
# by explore.py (GOAL='░', KEYLOG) and is replayed key for key; the game
# is turn-based, so the same keys on the same seed make the same journey.
# Recorded at the game's own 80x60 and cropped to 100x30 by hub.
COLS=80 ROWS=60 TITLE="Boat Journey" ARGS="-r 9"
WALK="
  Up Right Up Up Up Left Up Up Up Up 1 Right Right Up Up Up Up Up Up 
  Up Up Up Up Up Up Left Left Left Down Down Left Down Down Down Down 
  Down Right Right Right Right Right Up Up Up Up Right Up Up Up Up 
  Left Up Up Up Down Down Down Right Right Right Right Up Up Right 
  Right Right Up 1 Up Up Down Down Right Right Right Right Up Up Up 
  Up Right Right Up Up Up Up Up 1 Up Up Up Up Up Up Right"
drive() {
  sleep 2.5
  k n 2.5
  for m in $WALK; do
    case $m in 1) sleep 2; k 1 1 ;; *) k $m 0.17 ;; esac
  done
  k e 1.5
  for m in Up Up Up Up Left Up Up Up Up Up Right Up Up Up Up Up Up Up Up; do k $m 0.4; done
  sleep 1
  k Escape 1.5; k q 2
}
