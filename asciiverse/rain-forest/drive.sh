# Rain Forest preview: a new game on a fixed seed (-r 3): the arrival in
# the rain at night ("You've booked five days at a cabin in the forest"),
# the walk south through the trees to the lit cabin, into bed for eight
# hours of rain, and quitting. Recorded at the
# game's own 80x60, cropped to 100x30 by hub.
COLS=80 ROWS=60 TITLE="Rain Forest" ARGS="-r 3"
WALK="Down Down Down Down Down Down Down Down Down Right Right Right Left 
Left Left Left Right Right Right Right Right Right Right Right 
Right Right Down Down Down Down Down Down Down Down Down Left Left 
Right Right Down Down Down Down Down Down Down Down Down Right 
Right Left Left Left Left Right Right Right Right Right Right Down 
Down Down Down Down Down Down Down Left Left Left Left Left Left 
Left Left Left Left Left Left Down Down Down Down"
drive() {
  sleep 2.5
  k n 6                           # the arrival, read in the rain
  k Space 1.5
  slept=
  for m in $WALK Down Down Right Right Down Left Left Left Up; do
    k $m 0.3
    if [ -z "$slept" ] && tmux -L $sock capture-pane -p -t rec | grep -q 'Go to sleep?'; then
      sleep 1.5; k y 3; k Space 1.5; slept=1   # into bed: eight hours of rain
    fi
  done
  sleep 1
  k Escape 1.5; k q 2
}
