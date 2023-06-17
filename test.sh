#!/usr/bin/env bash

if [ ! -d data ]; then
    mkdir -p data
fi

rm -rf data/test-*.txt
touch data/test-1.txt

while true; do
    sleep 1
    if [ -f data/test-1.txt ]; then
        mv data/test-1.txt data/test-2.txt
    else
        mv data/test-2.txt data/test-1.txt
    fi
done
