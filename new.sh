#!/bin/bash

name="$1"
if [ -z "$name" ]; then
    read -p "Input name of the new project: " name
    if [ -z "$name" ]; then
        echo "No name given"
        exit 1
    fi
fi

curidx=$(ls | awk -F. '$1~/^[0-9]+$/{print $1}' | sort | tail -n 1)
nextidx=$((curidx + 1))
# check strlen of nextidx and pad with 0
if [ ${#nextidx} -eq 1 ]; then
    nextidx="0$nextidx"
fi

echo "Creating new project $name"
echo "[Exec] cargo new \"$nextidx.$name\" --name \"$name\" --vcs none"
cargo new "$nextidx.$name" --name "$name" --vcs none
echo "/target" > "$nextidx.$name/.gitignore"
