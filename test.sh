#!/bin/bash
cargo build
for FILE in $(ls ./tests)
do
    echo "Testing $FILE"
    echo ">>>>"
    echo "$(cat ./tests/$FILE)"
    echo "<<<<>>>>"
    cat ./tests/$FILE | ./target/debug/enum-ts
    echo "<<<<"
done