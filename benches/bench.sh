#!/bin/bash

# Clones a C++ enigma cli project
# Generates a 5MB file with all 'A'
# Times the encryption

git clone https://github.com/jeffrey-chan/enigma.git
cd enigma
make all

dd if=/dev/zero bs=1M count=5 | tr '\0' 'A' > input_5MB.txt

start_time=$(python -c "import time; print(int(time.time() * 1000))")
./enigma -r QWR -f input_5MB.txt -o encrypted_5MB.txt
end_time=$(python -c "import time; print(int(time.time() * 1000))")

elapsed_time=$((end_time - start_time))
echo "Time taken to encrypt 5MB: $elapsed_time milliseconds"