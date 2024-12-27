#!/bin/bash
set -euo pipefail

# Bunch of random bytes related to REX and MOV
python3 - << EOF > ./sources/005_fuzz_rex_mov.s 
import random
random.seed(5)
bytes = "4a 40 4c 47 4f b0 b3 b8 b9 bf".split()
print(*(random.choice(bytes) for i in range(1000)))
EOF
