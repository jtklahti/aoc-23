#!/usr/bin/env bash

pwd_dir="$(dirname "${BASH_SOURCE[0]}")"

function getInput() {
  DAY="$1"
  SESSION=$(getSessionKey)

  curl -H "Cookie: session=$SESSION" https://adventofcode.com/2023/day/$DAY/input
}

function getSessionKey() {
  FILE="$pwd_dir/.session_key"
  if [ ! -f "$FILE" ]; then
    echo "$FILE missing or empty"
    exit 1
  fi
  cat "$FILE"
}

if [ -z "$1" ]; then
  echo "Usage: $0 getInput DAY > output.txt"
  echo "Note: need to have session key in .session_key"
  exit 1
fi

"$@"
