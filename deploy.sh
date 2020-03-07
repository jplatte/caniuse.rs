#!/bin/bash
set -e

wasm-pack build --target web
rollup src/main.js --format iife --file public/caniuse_rs.js
rsync -rzz ./public caniuse.rs:/tmp/caniuse/

ssh caniuse.rs '
    set -e
    sudo chown root: /tmp/caniuse/public
    sudo rsync -r --delete /tmp/caniuse/public/* /srv/http/caniuse.rs/
    sudo rm -r /tmp/caniuse/public
'
