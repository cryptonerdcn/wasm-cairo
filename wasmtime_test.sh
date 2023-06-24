#!/bin/bash

# Read the entire content of the file into a variable
input_program_string=$(<test.cairo)

# Use the variable in your command
wasmtime target/wasm32-wasi/release/wasm-cairo.wasm -- --command compileCairoProgram --input-program-string "$input_program_string" > test.sierra
