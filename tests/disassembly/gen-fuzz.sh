#!/bin/bash
set -euo pipefail

# Bunch of random bytes related to REX and MOV
python3 - << EOF > ./sources/005_fuzz_rex_mov.s 
import random
random.seed(5)
bytes = "4a 40 4c 47 4f b0 b3 b8 b9 bf".split()
print(*(random.choice(bytes) for i in range(999)))
EOF

# Bunch of random bytes related to REX, MOV, and addr/data prefixes
python3 - << EOF > ./sources/010_fuzz_prefix_mov.s 
import random
random.seed(9)
bytes = (
    "4a 40 4c 47 4f 66 66 66 66 67 67 67 67 "
    + "88 88 8A 8A b0 b3 b8 b9 bf c6 c6 c7 c7"
).lower().split()
print(*(random.choice(bytes) for i in range(1001)))
EOF
