#!/bin/bash
# Build the Docker image
docker build -t rust-web-alpine .
# Run the Docker container
docker run --rm -p 3030:3030 --name server rust-web-alpine