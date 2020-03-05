#!/bin/bash
set -e

js_patch='
28c28
<             var file = fetch( "caniuse-rs.wasm", {credentials: "same-origin"} );
---
>             var file = fetch( "/caniuse-rs.wasm", {credentials: "same-origin"} );
'

cargo web deploy --release
wasm-opt -o ./target/deploy/caniuse-rs.wasm ./target/deploy/caniuse-rs.wasm
patch ./target/deploy/caniuse-rs.js <<< "$js_patch"
rsync -rzz ./target/deploy caniuse.rs:/tmp/caniuse/

ssh caniuse.rs '
    set -e
    sudo chown root: /tmp/caniuse/deploy
    sudo rsync -r --delete /tmp/caniuse/deploy/* /srv/http/caniuse.rs/
    sudo rm -r /tmp/caniuse/deploy
'
