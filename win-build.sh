#!/bin/sh
cargo build --target x86_64-pc-windows-gnu &&
cp target/x86_64-pc-windows-gnu/debug/bevy_ecosystem_simulator.exe . &&
exec ./bevy_ecosystem_simulator.exe "$@"
