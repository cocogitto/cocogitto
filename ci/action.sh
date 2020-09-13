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
      cargo install cross 2> /dev/null || true
      cross build --release --target "$target" --bin cocogitto
      bin_folder="${target}/release"
   else
      cargo build --release
      bin_folder="release"
   fi

   bin_path="${COCOGITTO_HOME}/target/${bin_folder}/cocogitto"
   chmod +x "$bin_path"
   mkdir -p "$TAR_DIR" 2> /dev/null || true

   cp "$bin_path" "$TAR_DIR"

   cd "$TAR_DIR"
   tar -czf cocogitto.tar.gz *

}

cmd="$1"
shift

release "$@"