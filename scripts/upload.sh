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
ADDR_FILE="${DIR}/server_address.txt"
DIR_FILE="${DIR}/server_directory.txt"

# Get the upload address
if [[ -f ${ADDR_FILE} && -f ${DIR_FILE} ]]; then
  ADDR=`cat ${ADDR_FILE}`
  DIRECTORY=`cat ${DIR_FILE}`
  LOCATION="${ADDR}:${DIRECTORY}"
else
  echo "Server address? (e.g. user@server.com:~/path/to/saemanga-rust)"
  read LOCATION
fi
echo "Uploading to ${LOCATION}..."

# Start the rsync
rsync -ar \
  --include='/Cargo.lock' \
  --include='/Cargo.toml' \
  --include='/Rocket.toml' \
  --include='/package.json' \
  --include='/process_prod.json' \
  --include='/lib/***' \
  --include='/public/***' \
  --include='/scripts/***' \
  --include='/services/***' \
  --include='/src/***' \
  --include='/templates/***' \
  --exclude='*' \
  ./ $LOCATION