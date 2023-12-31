#![no_main]
#![allow(unused_imports)]

risc0_zkvm::guest::entry!(main);

use risc0_zkvm::guest::env;
use wasmi::{Caller, Engine, Func, Linker, Module, Store};

pub fn main() {
    let engine = Engine::default();

    let wasm: Vec<u8> = env::read();
    let iters: i32 = env::read();

    // Derived from the wasmi example: https://docs.rs/wasmi/0.29.0/wasmi/#example
    let module = Module::new(&engine, &mut &wasm[..]).expect("Failed to create module");
    type HostState = u32;

    let linker = <Linker<HostState>>::new(&engine);
    let mut store = Store::new(&engine, 0);
    let instance = linker
        .instantiate(&mut store, &module)
        .expect("failed to instantiate")
        .start(&mut store)
        .expect("Failed to start");

    let verify_game_state = instance
        .get_typed_func::<(i32, i32, i32,i32,i32,i32,i32,i32,i32,i32,i32,i32), i32>(&store, "verify_game_state")
        .expect("Failed to get typed_func");


    let res = verify_game_state.call(&mut store, (10, 10, 30, 30, 5, 5, 1, 1, 15, 15, 1, 1)).expect("Failed to call");

    env::log(&format!("fib {}", res));
    env::commit(&res);
}
