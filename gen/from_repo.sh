#!/bin/sh

if [ -d heroicons ]
then
  git submodule update --remote
  # (cd $PWD/heroicons; npm install)
else
  git submodule init
fi

cargo run outline
cargo run solid

elm-format *.elm --yes

mv *.elm ../src/heroicons
