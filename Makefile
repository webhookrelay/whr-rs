
build:
	cargo build

build_examples:
	$(MAKE) -C examples/hello_world build

run_hello_world:
	$(MAKE) -C examples/hello_world build
	rc add examples/hello_world/target/wasm32-unknown-unknown/release/hello_world.wasm
	rc invoke hello_world test_data/req_1.json

install_rust_toolchain:
	rustup toolchain add nightly-2019-11-24
	rustup target add wasm32-unknown-unknown --toolchain nightly-2019-11-24