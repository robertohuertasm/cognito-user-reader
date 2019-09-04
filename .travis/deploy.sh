#!/bin/bash
echo 'Deploying to crates.io'
git stash --all
cargo login $crates_key
cargo publish
