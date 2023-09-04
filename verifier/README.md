# Tank Verifier

This project contains code for verifying the game state of a two-player tank game. It ensures the validity of the game state based on various rules and constraints.

## Table of Contents

- [Overview](#overview)
- [Features](#features)
- [How to Use](#how-to-use)
- [Build Instructions](#build-instructions)

## Overview

The project includes:

- Rust code for verifying the game state.
- A Makefile for building the Rust code into a WebAssembly (WASM) module.

## Features

The verifier checks several conditions to ensure the validity of the game state:

1.  Both tanks must be within the play area.
2.  Tanks must not collide with each other.
3.  Shots (if active) must be within the play area.
4.  Shots must not exceed a maximum allowed velocity.
5.  Tank health is updated based on shot impacts.
6.  A success status is returned to indicate the validity of the game state.

## How to Use

### In a Web Environment

To use the compiled WASM module in a web environment:

    WebAssembly.instantiateStreaming(fetch('path/to/your.wasm'))
        .then(obj => {
            // Call exported functions
            const result = obj.instance.exports.verify_game_state(...);
            const tank1_health = obj.instance.exports.get_tank1_health();
            const tank2_health = obj.instance.exports.get_tank2_health();
        });

### In a Python Environment

To use the WASM module with Python, you can utilize the `pywasm` library:

    import pywasm

    runtime = pywasm.load('path/to/your.wasm')
    result = runtime.exec('verify_game_state', [/* game state parameters */])
    tank1_health = runtime.exec('get_tank1_health', [])
    tank2_health = runtime.exec('get_tank2_health', [])

## Build Instructions

To build the project, you'll need to have Rust and Cargo installed:

1.  Navigate to the project directory.
2.  Run `cargo build --target wasm32-unknown-unknown --release` in the terminal.

This will compile the Rust code into WebAssembly. You can find the compiled WASM file in the `./target/wasm32-unknown-unknown/release/` directory.
