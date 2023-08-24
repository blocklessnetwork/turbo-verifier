import pywasm

# Load the WebAssembly module
runtime = pywasm.load(
    './target/wasm32-unknown-unknown/release/tank_verifier.wasm')

# Test 1: Expected to pass
# Both tanks and shots are within the play area, tanks are not colliding, and shots have valid velocity.
runtime.exec('verify_game_state', [10, 10, 30, 30, 5, 5, 1, 1, 15, 15, 1, 1])

success1 = runtime.exec('get_success', [])
tank1_health1 = runtime.exec('get_tank1_health', [])
tank2_health1 = runtime.exec('get_tank2_health', [])

print(
    f"Test 1 - Success: {success1}, Tank 1 Health: {tank1_health1}, Tank 2 Health: {tank2_health1}")

# Test 2: Expected to fail
# The second tank is placed outside the play area.
runtime.exec('verify_game_state', [10, 10, 10, 300, 5, 5, 1, 1, 15, 15, 1, 1])

success2 = runtime.exec('get_success', [])
tank1_health2 = runtime.exec('get_tank1_health', [])
tank2_health2 = runtime.exec('get_tank2_health', [])

print(
    f"Test 2 - Success: {success2}, Tank 1 Health: {tank1_health2}, Tank 2 Health: {tank2_health2}")
