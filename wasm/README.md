# WASM


### WASM

Refs:

 - [Using WebAssembly in LLVM](https://gist.github.com/yurydelendik/4eeff8248aeb14ce763e)
 - [Using LLVM from Rust to generate WebAssembly binaries](https://medium.com/@jayphelps/using-llvm-from-rust-to-generate-webassembly-93e8c193fdb4)

```
-DLLVM_EXPERIMENTAL_TARGETS_TO_BUILD=WebAssembly
```

```
llc -march=wasm32 main.bc -o main.s
```

Using solang: [https://solang.readthedocs.io/en/latest/installing.html#installing-llvm-on-mac](https://solang.readthedocs.io/en/latest/installing.html#installing-llvm-on-mac)

```
cmake -G Ninja -DLLVM_ENABLE_ASSERTIONS=On '-DLLVM_ENABLE_PROJECTS=clang;lld'  \
        -DLLVM_ENABLE_TERMINFO=Off -DCMAKE_BUILD_TYPE=Release \
        -DCMAKE_INSTALL_PREFIX=installdir -B build llvm
cmake --build build --target install
```

### Run Rasm

```
curl https://get.wasmer.io -sSfL | sh
```

```
$ wasmer qjs.wasm
```

```
rustup target add wasm32-unknown-emscripten
```
