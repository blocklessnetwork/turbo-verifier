use risc0_zkvm::{
    default_prover,
    serde::{from_slice, to_vec},
    ExecutorEnv,
};
use std::fs::File;
use std::io::Read;

use wasm_methods::{WASM_INTERP_ELF, WASM_INTERP_ID};

fn run_guest(iters: i32) -> i32 {

        // Load Wasm File
        let mut wasm_file: Vec<u8> = Vec::new();
        let mut file = File::open("./tank_verifier.wasm")
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