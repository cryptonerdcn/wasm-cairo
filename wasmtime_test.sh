#!/bin/bash

# Check if $1 (command) is provided
if [ -z "$2" ]; then
    echo "No command. Please provide a command  as the first argument."
    exit 1
fi

# Check if $2 (input file name) is provided
if [ -z "$2" ]; then
    echo "No input file. Please provide an input file as the second argument."
    exit 1
fi

# Read the entire content of the file into a variable
input_program_string=$(<"$2")

# Check if $3 (output file name) is provided
if [ -z "$3" ]; then
    # If no $3, execute command without output redirection
    wasmtime target/wasm32-wasi/release/wasm-cairo.wasm -- --command "$1" --input-program-string "$input_program_string"
else
    # If $3 is provided, execute command with output redirection
    wasmtime target/wasm32-wasi/release/wasm-cairo.wasm -- --command "$1" --input-program-string "$input_program_string" > "$3"
fi
