This assumes using [fork](https://github.com/rajatjindal/wasmtime/tree/repro-string-issue) of wasmtime to run

- build using `cargo build --component`
- run using `wasmtime run target/wasm32-wasi/debug/my_lib.wasm`

```
$ wasmtime run target/wasm32-wasi/debug/my_lib.wasm
just hello justhello from wasmtime
hello with result Ok("hello from wasmtime")
```