#!/bin/bash

/home/emilyblunt/.cache/.wasm-pack/wasm-opt-4d7a65327e9363b7/wasm-opt --debug \
  /media/emilyblunt/scratch1/zephyr/zwl-lib/lib/pkg/zecwalletlitelib_bg.wasm \
  -o \
  /media/emilyblunt/scratch1/zephyr/zwl-lib/lib/pkg/zecwalletlitelib_bg.wasm-opt.wasm \
  -O
