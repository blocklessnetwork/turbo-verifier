import pywasm

# Load the WebAssembly module
runtime = pywasm.load(
    './target/wasm32-unknown-unknown/release/tank_verifier.wasm')

# Test 1: Expected to pass
# Both tanks and shots are within the play area, tanks are not colliding, and shots have valid velocity.
result1 = runtime.exec('verify_game_state', [
                       10, 10, 30, 30, 5, 5, 1, 1, 15, 15, 1, 1])
print(result1)  # Should print 1 (ok)

# Test 2: Expected to fail
# The second tank is placed outside the play area.
result2 = runtime.exec('verify_game_state', [
                       10, 10, 10, 300, 5, 5, 1, 1, 15, 15, 1, 1])
print(result2)  # Should print 0 (fail)
