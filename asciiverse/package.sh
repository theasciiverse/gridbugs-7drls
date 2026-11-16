#!/bin/sh
# Builds the terminal edition of every game in games.txt (or only the ids
# given after the target) and packs one archive per game for av:
# dist/<id>-<target>.tar.gz holding the binary, named <id>, with the game's
# LICENSE, our README.md for it, PATCHES.md and UPSTREAM.md.
#   asciiverse/package.sh x86_64-unknown-linux-gnu [id ...]
# With CARGO_BUILD_TARGET unset the host's toolchain builds (local checks);
# the workflow sets it for the macOS cross targets. Each game is its own
# Cargo workspace with its own Cargo.lock, built with --locked.
set -eu
target=$1; shift
root=$(cd "$(dirname "$0")/.." && pwd)
mkdir -p "$root/dist"
grep -v '^#' "$root/asciiverse/games.txt" | while read -r id dir pkg flags; do
  [ -n "$id" ] || continue
  if [ $# -gt 0 ]; then
    case " $* " in *" $id "*) ;; *) continue ;; esac
  fi
  echo "== $id"
  (cd "$root/$dir" && cargo build --release --locked -j"${JOBS:-4}" -p "$pkg" $flags)
  out=$root/$dir/target/${CARGO_BUILD_TARGET:+$CARGO_BUILD_TARGET/}release/$pkg
  stage=$root/dist/stage-$id
  rm -rf "$stage" && mkdir -p "$stage"
  cp "$out" "$stage/$id"
  strip "$stage/$id" 2>/dev/null || true
  cp "$root/asciiverse/$id/README.md" "$stage/README.md"
  cp "$root/$dir/LICENSE" "$stage/LICENSE"
  cp "$root/PATCHES.md" "$root/UPSTREAM.md" "$stage/"
  tar -czf "$root/dist/$id-$target.tar.gz" -C "$stage" .
  rm -rf "$stage"
done
