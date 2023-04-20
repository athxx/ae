#!/bin/bash

# cargo run --package prisma-cli -- generate
cargo prisma generate # outputs client to src/prisma.rs

#cargo prisma db push  # outputs sqlite db to prisma/dev.db (specified in schema.prisma)

# "cargo run --bin seed --"  ==  "cargo seed"

#cargo prisma migrate dev --create-only # outputs migration to migrations/dev.sql
