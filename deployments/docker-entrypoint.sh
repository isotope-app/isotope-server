!/bin/bash

set -e
echo "installing picopik cli....."

cargo install --path picopik-cli

exec "$@"
