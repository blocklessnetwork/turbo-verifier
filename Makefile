# Name of the Rust project (same as the name in Cargo.toml)
PROJECT_NAME = tank_verifier

# Target WebAssembly format
WASM_TARGET = wasm32-unknown-unknown

# Directory where the wasm output will be placed
WASM_DIR = ./target/$(WASM_TARGET)/release

# Name of the output wasm and wat files
WASM_FILE = $(WASM_DIR)/$(PROJECT_NAME).wasm

all: $(WASM_FILE)

# Build the Rust project
$(WASM_FILE):
	cargo build --target $(WASM_TARGET) --release

# Clean the project
clean:
	cargo clean
	rm -f $(WAT_FILE)

.PHONY: all clean
