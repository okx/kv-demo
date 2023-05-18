build:
	RUSTFLAGS='-C link-arg=-s' cargo build --release --lib --target=wasm32-unknown-unknown
	wasm-opt -O3 -o pkg/kv_demo_opt.wasm target/wasm32-unknown-unknown/release/kv_demo.wasm