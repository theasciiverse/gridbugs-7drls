#!/bin/sh
# Records a game's catalog preview, asciiverse/<id>/<id>.cast (hub's
# RECORDING.md): the game runs in a tmux pane (own socket, no status line,
# no user config) and asciinema records that pane while
# asciiverse/<id>/drive.sh sends it keys. drive.sh sets COLS, ROWS, ARGS
# and TITLE and defines drive(). A throwaway HOME and XDG_STATE_HOME, so
# no real save is touched; sound off. Needs asciinema 3 and tmux.
#   asciiverse/record-preview.sh <id> path/to/binary
set -eu
id=$1
binary=$(cd "$(dirname "$2")" && pwd)/$(basename "$2")
here=$(cd "$(dirname "$0")" && pwd)
out=$here/$id/$id.cast
. "$here/$id/drive.sh"
home=$(mktemp -d)
sock=avrec-$id
trap 'tmux -L $sock kill-server 2>/dev/null; rm -rf "$home"' EXIT
mkdir -p "$home/bin" "$home/state"
cp "$binary" "$home/bin/$id"
cd "$home"  # nothing in the recording names a local path
export HOME="$home" XDG_STATE_HOME="$home/state" \
  PATH="$home/bin:$PATH" USER=player LOGNAME=player \
  TERM=xterm-256color COLORTERM=truecolor LANG=C.UTF-8
unset VTE_VERSION TERM_PROGRAM TERM_PROGRAM_VERSION
tmux -L $sock -f /dev/null new-session -d -s rec -x "$COLS" -y "$ROWS" \
  "$id $ARGS 2>/dev/null" \; set -g status off

keys() { tmux -L $sock send-keys -t rec "$@"; }
say() { keys -l "$1"; }
# k KEY [pause]: one key, then a pause (default 0.35 s)
k() { keys "$1"; sleep "${2:-0.35}"; }
drive &
drv=$!
asciinema rec --headless --overwrite -q \
  --window-size "${COLS}x${ROWS}" --idle-time-limit 2 \
  --capture-env "TERM,COLORTERM,LANG" --title "$TITLE" \
  --command "tmux -L $sock attach -t rec" "$out"
wait "$drv"
