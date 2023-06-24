#!/bin/bash

# Read the entire content of the file into a variable
input_program_string=$(<"$2")

# Use the variable in your command
wasmtime target/wasm32-wasi/release/wasm-cairo.wasm -- --command "$1" --input-program-string "$input_program_string" > "$3"
