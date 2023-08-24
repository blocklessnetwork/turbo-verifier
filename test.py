import pywasm
# pywasm.on_debug()

runtime = pywasm.load(
    './target/wasm32-unknown-unknown/release/tank_verifier.wasm')


# tank1_x: i32, tank1_y: i32,
# tank2_x: i32, tank2_y: i32,
# shot1_x: i32, shot1_y: i32, shot1_status: i32,
# shot2_x: i32, shot2_y: i32, shot2_status: i32

r = runtime.exec('verify_game_state', [
                 10, 10, 10, 10, 10, 10, 1, 10, 10, 1])
print(r)  # 1, ok

r = runtime.exec('verify_game_state', [
                 10, 10, 10, 300, 200, 0, 10, 10, 10, 1])
print(r)  # 0, fail
