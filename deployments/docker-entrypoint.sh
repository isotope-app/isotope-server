#!/bin/bash

set -e
echo "installing picopik cli....."

cargo install --path picopik-cli
cd /picopik-cli
cargo watch -x build 
echo "hello"

exec "$@"