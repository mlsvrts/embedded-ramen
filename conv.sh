#!/bin/bash

set -u # Unset vars
set -e # Exit on error

python3 uf2/uf2conv.py -i -f 0xe48bff56 -b 0x10000000 target/thumbv6m-none-eabi/$1/rp2040 -o target/thumbv6m-none-eabi/$1/rp2040.uf2
