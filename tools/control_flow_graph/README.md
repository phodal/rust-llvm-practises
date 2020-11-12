my path: `~/llvm/llvm-10.0.1.src/build/bin/opt`

```bash
opt -dot-callgraph main.ll
```

```bash
opt -dot-cfg lab.ll
```


```
dot callgraph.dot -Tpng -o callgraph.png
```

![CallGraph](callgraph.png)
