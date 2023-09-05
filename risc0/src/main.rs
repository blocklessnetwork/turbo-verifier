// Copyright 2023 RISC Zero, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use risc0_zkvm::{
    default_prover,
    serde::{from_slice, to_vec},
    ExecutorEnv,
};
use std::fs::File;
use std::io::Read;

use wasm_methods::{WASM_INTERP_ELF, WASM_INTERP_ID};

fn wat2wasm(wat: &str) -> Result<Vec<u8>, wat::Error> {
    wat::parse_str(wat)
}

fn run_guest(iters: i32) -> i32 {

        // Load Wasm File
        let mut wasm_file: Vec<u8> = Vec::new();
        let mut file = File::open("/Users/derekanderson/Projects/wasm/tank_verifier.wasm")
            .expect("Failed to open WASM file.");
        file.read_to_end(&mut wasm_file)
            .expect("Failed to load WASM file.");

    // generate proof
    // (10, 10, 30, 30, 5, 5, 1, 1, 15, 15, 1, 1)
    let env = ExecutorEnv::builder()
        .add_input(&to_vec(&wasm_file).unwrap())
        .add_input(&to_vec(&10).unwrap())
        .add_input(&to_vec(&10).unwrap())
        .add_input(&to_vec(&30).unwrap())
        .add_input(&to_vec(&30).unwrap())
        .add_input(&to_vec(&5).unwrap())
        .add_input(&to_vec(&5).unwrap())
        .add_input(&to_vec(&1).unwrap())
        .add_input(&to_vec(&1).unwrap())
        .add_input(&to_vec(&15).unwrap())
        .add_input(&to_vec(&15).unwrap())
        .add_input(&to_vec(&1).unwrap())
        .add_input(&to_vec(&1).unwrap())
        .build()
        .unwrap();

    // Obtain the default prover.
    let prover = default_prover();

    // Produce a receipt by proving the specified ELF binary.
    let receipt = prover.prove_elf(env, WASM_INTERP_ELF).unwrap();

    // Verify the receipt.
    receipt.verify(WASM_INTERP_ID).expect(
        "Code you have proven should successfully verify; did you specify the correct image ID?",
    );

    let result: i32 = from_slice(&receipt.journal).unwrap();

    result
}

fn main() {
    let fib_iters: i32 = 100;
    let _ = run_guest(fib_iters);
}

#[cfg(test)]
mod tests {
    fn fibonacci(n: i32) -> i32 {
        let (mut a, mut b) = (0, 1);
        for _ in 0..n {
            let c = a;
            a = b;
            b += c;
        }
        a
    }

    #[test]
    fn wasm_fib() {
        let fib_iters: i32 = 10;
        let result = super::run_guest(fib_iters);
        assert_eq!(
            result,
            fibonacci(fib_iters),
            "We expect the zkVM output to be the product of the inputs"
        )
    }
}