#!/bin/bash

#pnpm dlx tailwindcss -i templates/assets/tailwind.css -o templates/assets/main.css --watch
pnpx tailwindcss -i templates/assets/tailwind.css -o templates/assets/main.css --minify --watch