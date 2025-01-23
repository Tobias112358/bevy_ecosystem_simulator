#!/bin/sh
cargo build --target x86_64-pc-windows-gnu &&
exec ./target/x86_64-pc-windows-gnu/debug/bevy_ecosystem_simulator.exe "$@"