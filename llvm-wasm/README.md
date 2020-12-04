// from https://github.com/jayphelps/using-llvm-from-rust-to-generate-webassembly/blob/master/main.rs

```
llc -march=wasm32 main.bc -o main.s
```

```
s2wasm main.s -o main.wast
```

```
s2wasm --emit-binary main.s -o main.wasm
```
