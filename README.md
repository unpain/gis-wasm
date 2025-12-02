# gis-wasm
A GIS wasm tool compiled from Rust

## Build
- first install wasm-pack if you don't have it yet
```sh
cargo install wasm-pack
```
- then build the project
```sh
wasm-pack build --target web -d ./pkg
```
