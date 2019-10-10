#!/bin/bash

set -e

file="/isotope/isotope-cli"

if [ -e "$file" ]; then
	echo "isotope cli already installed"
else
	echo "installing isotope cli....."
	cargo install --path isotope-cli
fi

exec "$@"