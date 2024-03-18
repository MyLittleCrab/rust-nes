#!/bin/bash

tail -c +17 ./target/mos-nes-cnrom/debug/brickgame | head -c 32768 > target/PRG.bin

cp scripts/symbols.info target/symbols.info
node scripts/get-symbols.js >> target/symbols.info

da65 -i target/symbols.info --comments 3 target/PRG.bin > target/PRG.s
