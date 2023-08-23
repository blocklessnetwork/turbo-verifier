# Tank Verifier

This project contains code for verifying the game state of a two-player tank game similar to Atari's classic "Tanks" game.

## Table of Contents

- [Overview](#overview)
- [What is WAT?](#wat-explanation)
- [How to Use](#how-to-use)
- [Build Instructions](#build-instructions)

## Overview

The project includes:

- Rust code for verifying the game state.
- A Makefile for building the Rust code and generating a WAT (WebAssembly Text) file.
- A WAT file that represents the compiled WebAssembly code in a readable text format.

## What is WAT?

WAT (WebAssembly Text) is a textual representation of WebAssembly bytecode. WebAssembly is a binary format for executing code on web browsers, and WAT serves as its human-readable version.

The generated WAT file contains all the logic for verifying the game state, as written in Rust, but in a format that is easier to read and edit manually, if needed.

## How to Use

To use the WAT file in a web environment, you would typically convert it back to WebAssembly (WASM) using the `wat2wasm` utility. Then you can load the WASM file into a web page using JavaScript.

        WebAssembly.instantiateStreaming(fetch('path/to/your.wasm'))
            .then(obj => {
                // Call exported functions
                obj.instance.exports.verify_game_state(...);
            });


## Build Instructions

To build the project, you'll need to have Rust and Cargo installed, along with the wasm2wat utility. Follow these steps:

1.  Update the `WASM2WAT` variable in the Makefile to point to your wasm2wat utility.
2.  Run `make` in the terminal.

This will compile the Rust code into WebAssembly and then generate a WAT file. You can find the WAT file in the `./target/wasm32-unknown-unknown/release/` directory.
