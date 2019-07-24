#!/bin/bash

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
cd "${DIR}/../"

# Build cargo
~/.cargo/bin/cargo build --release

# Start process in pm2
pm2 start process_prod.json