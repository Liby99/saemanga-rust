#!/bin/bash

# To run this script, starting from saemanga-rust root directory and type
#  $ ./scripts/upload.sh
# Any type in the server address
#
# This command will not create a specific folder so please include a folder name
# e.g. user@server.com:/path/to/saemanga-rust/
#
# Also please first do the front-end compilation in PRODUCTION mode and then
# do the upload

# Get the file containing server_address
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
FILE="${DIR}/server_address.txt"

# Get the upload address
if test -f "$FILE"; then
  addr=`cat ${FILE}`
else
  echo "Server address? (e.g. user@server.com:~/path/to/saemanga-rust)"
  read addr
fi
echo "Uploading to ${addr}..."

# Start the rsync
rsync -ar \
  --include='/Cargo.lock' \
  --include='/Cargo.toml' \
  --include='/Rocket.toml' \
  --include='/package.json' \
  --include='/process_prod.json' \
  --include='/lib/***' \
  --include='/public/***' \
  --include='/services/***' \
  --include='/src/***' \
  --include='/templates/***' \
  --exclude='*' \
  ./ $addr