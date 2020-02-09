#!/bin/bash

set -e
echo "installing picopik cli....."

cd /picopik-cli
cargo watch -s "cargo install --path ." 
echo "hello"

exec "$@"