#!/bin/bash

cargo -V
is_cargo_installed=$?
docker -v
is_docker_installed=$?

if [ $is_cargo_installed -a $is_docker_installed ]; then
    CARGO_DATA_DIR="${HOME}/.local/share/cargo/data/code_runner/"
    
    mkdir -p "${CARGO_DATA_DIR}"
    cp -r "docker/" "${CARGO_DATA_DIR}"
    
    cargo install --force
    
    cd "${CARGO_DATA_DIR}"
    wget "https://raw.githubusercontent.com/github/linguist/master/lib/linguist/languages.yml"
    
    echo "You need to place ${CARGO_DATA_DIR}token.json"
else
    echo "Install Cargo!"
fi
