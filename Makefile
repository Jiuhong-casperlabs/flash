prepare:
	rustup target add wasm32-unknown-unknown

build-contracts:
	cargo build --release -p erc20 --target wasm32-unknown-unknown
	wasm-strip 

clippy:
	cargo clippy --all-targets --all -- -A clippy::ptr_arg

check-lint: clippy
	cargo fmt --all -- --check

clean:
	cargo clean