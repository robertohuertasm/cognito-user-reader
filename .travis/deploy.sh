#!/bin/bash
echo 'Deploying to crates.io'
git stash --all
if [[ "$TRAVIS_OS_NAME" == "linux" ]]
then
  cargo login $crates_key
  cargo publish
fi

