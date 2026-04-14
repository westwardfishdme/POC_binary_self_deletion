#!/usr/bin/env bash

# Tests the race conditions for moving the binary.
error="Race condition failed!"
echo -e "\e[33m[ starting the race condition test... ]\e[0m"
cargo run & sleep 10 && mv ../target/debug/happytime .


if [[ $? -ne 0 ]]; then
  echo "[FROM TEST]: \e[31m$error\e[0m"
fi

# tests to see if you can catch the binary before deletion
echo -e "\e[33m[ seeing if you can grab the binary from a race condition... ]\e[0m"
cargo run & sleep 10 && cat ../target/debug/happytime > ./captured_binary

CAPTURED_BINSIZE= echo $(wc -c ./captured_binary) | awk '{ print $1 }' > /dev/null
error="Binary was captured!"

if [[ CAPTURED_BINSIZE != "0" ]]; then
  echo -e "[FROM TEST]: \e[31m$error\e[0m"
fi
