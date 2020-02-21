#!/bin/bash

set -e
echo "installing picopik cli....."

cd /picopik/picopik-server/picopik-cli
cargo watch -w . -w ./../picopik-core -s "cargo install --path ."


exec "$@"l
