cargo component build --release --manifest-path ../product-core/Cargo.toml 

cargo component build --release

# because of `error: `product_core` is not in kebab case (at offset 0x0)`
mv -f ../product-core/target/wasm32-wasi/release/product_core.wasm ../product-core/target/wasm32-wasi/release/product-core.wasm

# compose it
wasm-tools compose target/wasm32-wasi/release/product_api.wasm -d ../product-core/target/wasm32-wasi/release/product-core.wasm -o service.wasm
