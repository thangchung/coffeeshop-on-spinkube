cargo component build --release --manifest-path ../counter-core/Cargo.toml 
cargo component build --release

# because of `error: `counter_core` is not in kebab case (at offset 0x0)`
mv -f ../counter-core/target/wasm32-wasi/release/counter_core.wasm ../counter-core/target/wasm32-wasi/release/counter-core.wasm

# compose it
wasm-tools compose target/wasm32-wasi/release/counter_api.wasm -d ../counter-core/target/wasm32-wasi/release/counter-core.wasm -o service.wasm
