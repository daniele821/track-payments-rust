#!/bin/bash

SCRIPT_PWD="$(realpath "${BASH_SOURCE[0]}")"
SCRIPT_DIR="$(dirname "${SCRIPT_PWD}")"

cd "$SCRIPT_DIR" || exit 1

RUST_BIN="${SCRIPT_DIR}/.target/release/decrypt"

if [[ "$#" -eq "1" ]]; then
    BIN_PATH="$1"
else
    echo 'invalid amount of parameters'
    exit 1
fi

cargo build --release --bin decrypt
cp "${RUST_BIN}" "${BIN_PATH}"
