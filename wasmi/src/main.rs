use anyhow::{anyhow, Result};
use wasmi::*;

fn main() -> Result<()> {
    let engine = Engine::default();
    let wasm_bytes = std::fs::read("./tank_verifier.wasm")?;

    let module = Module::new(&engine, &*wasm_bytes)?;

    type HostState = u32;

    let mut store = Store::new(&engine, 0);

    let mut linker = <Linker<HostState>>::new(&engine);

    let instance = linker
        .instantiate(&mut store, &module)?
        .start(&mut store)?;

    let verify_game_state: TypedFunc<(i32, i32, i32,i32,i32,i32,i32,i32,i32,i32,i32,i32), i32> = instance.get_typed_func::<(i32, i32, i32,i32,i32,i32,i32,i32,i32,i32,i32,i32), i32>(&store, "verify_game_state")?;

    // And finally we can call the wasm!
    // [10, 10, 30, 30, 5, 5, 1, 1, 15, 15, 1, 1]
    let exit_result = verify_game_state.call(&mut store, (10, 10, 30, 30, 5, 5, 1, 1, 15, 15, 1, 1) )?;
 
    println!("Exit Result: {:?}", exit_result);

    //fails
    let result = verify_game_state.call(&mut store, (10, 10, 10, 300, 5, 5, 1, 1, 15, 15, 1, 1) )?;
    println!("Result: {:?}", result);

    Ok(())
}