#!/bin/bash

set -e
echo "installing isotope cli....."

cargo install --path isotope-cli

exec "$@"