#!/bin/bash
~/Tools/tp/target/release/tp $@ | while IFS= read -r a; do
    # If the input starts with "tp::", handle it as a command
    if [[ $a == "tp::"* ]]; then
        # Extract the directory path
        cmd="${a#tp::}"

        # Change directory
        eval "$cmd"

    else
      echo $a
    fi
done
