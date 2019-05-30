#!/bin/bash

echo "Server address? "

read addr

rsync -ar \
  --include='/Cargo.lock' \
  --include='/Cargo.toml' \
  --include='/Rocket.toml' \
  --include='/lib/***' \
  --include='/public/***' \
  --include='/services/***' \
  --include='/src/***' \
  --include='/templates/***' \
  --exclude='*' \
  ./ $addr