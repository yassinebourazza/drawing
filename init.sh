#!/bin/bash
set -e

cargo update -p rayon --precise 1.10.0
cargo update -p rayon-core --precise 1.12.1
cargo run -q