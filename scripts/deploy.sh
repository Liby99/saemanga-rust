#!/bin/bash

# Setting up files
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
UPLOAD_SCRIPT="${DIR}/upload.sh"
ADDR_FILE="${DIR}/server_address.txt"
DIR_FILE="${DIR}/server_directory.txt"

# Check if the files exist
if [[ -f ${ADDR_FILE} && -f ${DIR_FILE} ]]; then
  ADDR=`cat ${ADDR_FILE}`
  DIRECTORY=`cat ${DIR_FILE}`

  # First build the project locally
  # npm run build-prod # NO NEED

  # Upload the files to the server
  ${UPLOAD_SCRIPT}

  # SSH to the server and run the prod
  ssh ${ADDR} "${DIRECTORY}/scripts/deploy_in_server.sh"
else
  echo "DEPLOY FAILED: Need address file"
fi