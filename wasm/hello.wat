(module
  ;; Import our myprint function
  (import "env" "jsprint" (func $jsprint (param i32)))
  ;; Define a single page memory of 64KB.
  (memory $0 1)
  ;; Store the Hello World (null terminated) string at byte offset 0
  (data (i32.const 0) "Hello World!\00")
  ;; Export the memory so it can be access in the host environment.
  (export "pagememory" (memory $0))
  ;; Define a function to be called from our host
  (func $helloworld
    (call $jsprint (i32.const 0))
  )
  ;; Export the wasmprint function for the host to call.
  (export "helloworld" (func $helloworld))
)

