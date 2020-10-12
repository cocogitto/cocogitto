#!/usr/bin/env bash
set -euo pipefail

export COCOGITTO_HOME="$(cd "$(dirname "$0")/.." && pwd)"

echoerr() {
   echo "$@" 1>&2
}

release() {

   TAR_DIR="${COCOGITTO_HOME}/target/tar"

   target="${1:-}"
   if [[ $target == *"osx"* ]]; then
      echoerr "OSX cross-compile is impossible. Fallbacking to cargo..."
      target=""
   fi

   cd "$COCOGITTO_HOME"

   rm -rf "${COCOGITTO_HOME}/target" 2> /dev/null || true

   if [ -n "$target" ]; then
      cargo install --version 0.1.16 cross 2> /dev/null || true
      cross build --release --target "$target"
      bin_folder="${target}/release"
   else
      cargo build --release
      bin_folder="release"
   fi

   coco_bin_path="${COCOGITTO_HOME}/target/${bin_folder}/coco"
   cog_bin_path="${COCOGITTO_HOME}/target/${bin_folder}/cog"
   chmod +x "$coco_bin_path"
   chmod +x "$cog_bin_path"
   mkdir -p "$TAR_DIR" 2> /dev/null || true

   cp "$coco_bin_path" "$TAR_DIR"
   cp "$cog_bin_path" "$TAR_DIR"
   cp "COCOGITTO_HOME/LICENSE" "$TAR_DIR"

   cd "$TAR_DIR"
   tar -czf cocogitto.tar.gz *

}

cmd="$1"
shift

release "$@"