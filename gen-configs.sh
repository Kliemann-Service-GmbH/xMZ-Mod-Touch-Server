#!/bin/bash

cargo run --bin xmz_generate_config -- -e production -k 7 -c share/xMZ-Mod-Touch.json.production
cargo run --bin xmz_generate_config -- -e development -c share/xMZ-Mod-Touch.json.development -d true
cargo run --bin xmz_generate_config -- -e development -d true
