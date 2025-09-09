#!/bin/sh

# Build an updated README
cat docs/readme/head.md > README.md
cat docs/lib.md >> README.md
cat docs/main.md >> README.md
cat docs/readme/tail.md >> README.md   

# Build Changelog
gen-changelog