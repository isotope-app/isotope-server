#!/bin/bash

set -e
file="/picopik/picopik-cli"

if [ -e "$file" ]; then
	echo "picopik cli already installed"
else
	echo "installing picopik cli....."
	cargo install --path picopik-cli
fi

exec "$@"
