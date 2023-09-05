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
    let get_success: TypedFunc<(), i32> = instance.get_typed_func::<(), i32>(&store, "get_success")?;
    let get_tank1_health: TypedFunc<(), i32> = instance.get_typed_func::<(), i32>(&store, "get_tank1_health")?;
    let get_tank2_health: TypedFunc<(), i32> = instance.get_typed_func::<(), i32>(&store, "get_tank2_health")?;

    // And finally we can call the wasm!
    // [10, 10, 30, 30, 5, 5, 1, 1, 15, 15, 1, 1]
    let exit_result = verify_game_state.call(&mut store, (10, 10, 30, 30, 5, 5, 1, 1, 15, 15, 1, 1) )?;
    let success_result = get_success.call(&mut store, () )?;
    let tank1_health_result = get_tank1_health.call(&mut store, () )?;
    let tank2_health_result = get_tank2_health.call(&mut store, () )?;

    println!("Exit Result: {:?}", exit_result);
    println!("Success Realst: {:?}", success_result);
    println!("Tank 1 Health Result: {:?}", tank1_health_result);
    println!("Tank 2 Health Result: {:?}", tank2_health_result);

    //fails
    let result = verify_game_state.call(&mut store, (10, 10, 10, 300, 5, 5, 1, 1, 15, 15, 1, 1) )?;
    println!("Result: {:?}", result);

    Ok(())
}