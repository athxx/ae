#!/bin/bash

if [ ! "$1" ]; then 
    echo -e "\e[91mError: Must choose one project you want to run.\e[0m"
    exit 1
fi

if [ ! -d "$1/src" ]; then 
    echo -e "\e[91mError: path '$1/src' not exist, please check your directory.\e[0m"
    exit 1
fi

# need to check cargo-watch
cargo watch -q -c -w "$1/src" -x "run -p $1"
