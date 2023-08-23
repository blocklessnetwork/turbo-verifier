# Path to the wasm2wat utility
WASM2WAT = /Users/derekanderson/Downloads/wabt-1.0.33/bin/wasm2wat

# Name of the Rust project (same as the name in Cargo.toml)
PROJECT_NAME = tank_verifier

# Target WebAssembly format
WASM_TARGET = wasm32-unknown-unknown

# Directory where the wasm output will be placed
WASM_DIR = ./target/$(WASM_TARGET)/release

# Name of the output wasm and wat files
WASM_FILE = $(WASM_DIR)/$(PROJECT_NAME).wasm
WAT_FILE = $(WASM_DIR)/$(PROJECT_NAME).wat

all: $(WAT_FILE)

# Build the Rust project
$(WASM_FILE):
	cargo build --target $(WASM_TARGET) --release

# Convert the wasm file to wat
$(WAT_FILE): $(WASM_FILE)
	$(WASM2WAT) $(WASM_FILE) -o $(WAT_FILE)

# Clean the project
clean:
	cargo clean
	rm -f $(WAT_FILE)

.PHONY: all clean
