#!/usr/bin/env bash
set -euo pipefail

if [[ $# -lt 1 ]]; then
  echo "Usage: $0 <day> [input-file]" >&2
  exit 1
fi

day=$(printf "%02d" "$1")
input_file=${2:-"inputs/day${day}.txt"}

cargo run -q -p "day${day}" -- "$input_file"
