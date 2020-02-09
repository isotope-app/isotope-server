#!/bin/bash

set -e
echo "installing picopik cli....."

cd /picopik/picopik-server/picopik-cli
cargo watch -s "cargo install --path ."

exec "$@"l
