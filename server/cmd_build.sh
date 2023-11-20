#!/bin/bash

#!/bin/bash

if [ -z "$1" ]; then
  echo "Error: no argument provided"
  exit 1
fi

if [ "$1" = "api" ]; then
  cargo prisma generate
  cargo build --release -p api
elif [ "$1" = "web" ]; then
  cargo prisma generate
  cargo build --release -p home
elif [ "$1" = "admin" ]; then
  cargo prisma generate
  cargo build --release -p admin
elif [ "$1" = "all" ]; then
  cargo prisma generate
  cargo build --release -p admin
else
  echo "Invalid argument: $1"
  exit 1
fi
