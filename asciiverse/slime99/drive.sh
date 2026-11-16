# slime99 preview: the briefing, then the sewers on a fixed seed (-r 4) and
# a fixed walk found by trying: the first level, down the stairs, the
# commander's reward (an ability, chosen with 1), and on into the next
# floor. Recorded at the game's own 60x40, cropped to 100x30 by hub.
COLS=60 ROWS=40 TITLE="slime99" ARGS="-r 4"
WALK="Down Right Down Right Down Left Right Down Right Right Down Right 
Right Down Up Up Left Right Right Up Down Down Down Down Up Right 
Down Right Right Right Up Left Right Right Down Up Down Left Up 
Down Down Up Up Up Right Down Right Down Left Right Right Down Down 
Down Up Right Down Up Down Down Down Down Right Down Up Up Right 
Right Right Right Down Left Down Right Right Left Left Right Right 
Right Left Right Right Left Up Up Down Down Down Right Down Down 
Down Right Right Down Up Up Down Down Down Down Down Down Down Down 
Down Up Up Up Up Right Right Right Right Right Right Right Right 
Down Down Down Down Up Up Up Up Down Left Left Left Left Left Right 
Right Right 1 Left Left Down Down Down Down Up Up Up Up Up Right 
Right Right Up Up Up Up Up Up Up Up Up Up Down Down Down Down Down 
Left Left Right Right Down Down Down Down Right Right Left Left 
Right Right Right Right"
drive() {
  sleep 2.5
  sleep 3; k Space 1.5            # the briefing, read for a moment
  for m in $WALK; do
    case $m in 1) sleep 2.5; k 1 1.5 ;; *) k $m 0.13 ;; esac
  done
  sleep 1.5
  k Escape 1.5; k q 2
}
