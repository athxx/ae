#!/bin/bash
#cargo run -p api

# cargo watch -q -c -x "run -p api"

cargo watch -q -c -w "api/src" -x "run -p api"