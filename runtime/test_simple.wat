(module
  (import "env" "println" (func $println (param i32)))

  (func $main (result i32)
    i32.const 42
    call $println
    i32.const 0
    return
  )
  (export "main" (func $main))
)
