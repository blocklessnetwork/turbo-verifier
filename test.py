import pywasm
# pywasm.on_debug()

runtime = pywasm.load(
    './target/wasm32-unknown-unknown/release/tank_verifier.wasm')
r = runtime.exec('verify_game_state', [10, 10, 10, 10, 10, 10, 10, 10, 10, 10])
print(r)  # 55
