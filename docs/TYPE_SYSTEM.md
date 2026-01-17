# ASTRIXA Type System (Minimum Viable)

## Overview
This document describes the implementation of ASTRIXA's type system - a predictable, explicit type system that prevents silent failures through static type checking.

## Implemented Features

### 1. Type System Foundation

The type system supports four core types:

```rust
pub enum Type {
    Int,      // 64-bit signed integer
    Float,    // 64-bit floating point
    Bool,     // Boolean value
    String,   // Unicode string
    Void,     // No return value
    Unknown,  // Inferred type (internal use)
}
```

### 2. Type Inference

Variables automatically infer their type from initialization:

```astrixa
let x = 10         // x: Int
let msg = "hello"  // msg: String  
let active = 1     // active: Int
```

### 3. Type Checking

The type checker validates:

#### Variable Assignment Type Safety
```astrixa
let x = 10
x = "hello"   // ❌ Type mismatch error
```

Error message:
```
Type mismatch: cannot assign String to variable of type Int
```

#### Arithmetic Operations (Int-only except Add)
```astrixa
let a = 10
let b = 20
let sum = a + b       // ✅ Both Int → Int result
let diff = a - b      // ✅ Both Int → Int result
let product = a * b   // ✅ Both Int → Int result
```

Type mismatch:
```astrixa
let num = 10
let str = "twenty"
let result = num + str  // ❌ Type error: cannot add Int and String
```

#### String Concatenation
```astrixa
let str1 = "hello"
let str2 = "world"
let combined = str1 + str2  // ✅ Both String → String result
```

#### Comparison Operators (Int-only)
```astrixa
let a = 10
let b = 20
let is_less = a < b   // ✅ Int comparison → Bool result
```

Supported comparison operators:
- `==` (Eq) - equality
- `!=` (Ne) - inequality
- `<` (Lt) - less than
- `<=` (Le) - less or equal
- `>` (Gt) - greater than
- `>=` (Ge) - greater or equal

#### Control Flow Type Checking

If conditions must be Bool or Int:
```astrixa
if x {           // ✅ if x: Int
  print("yes")
}

if "string" {    // ❌ Type error: if condition must be Bool, got String
  print("never")
}
```

While loops:
```astrixa
let i = 0
while i {       // ✅ while condition: Int
  print(i)
  i = i + 1
}
```

#### Function Return Types

Functions infer return type from return statements:

```astrixa
fn get_int {
  return 42      // Inferred return type: Int
}

fn get_string {
  return "hello" // Inferred return type: String
}
```

Return type consistency:
```astrixa
fn inconsistent {
  if 1 {
    return 10    // Int
  } else {
    return "ten" // String
  }
  // ❌ Type error: inconsistent return types in function 'inconsistent'
}
```

### 4. Function Argument Validation

Function parameters are typed (currently Int for V1):

```astrixa
fn add(a b) {
  return a + b
}

let result = add(5 10)      // ✅ Both Int arguments
let bad = add(5 "10")       // ❌ Type error: argument 1 expects Int, got String
```

### 5. Panic Statement Type Checking

Panic requires a String message:

```astrixa
panic("error message")   // ✅ String argument
panic(42)               // ❌ Type error: panic() requires string message, got Int
```

## Error Messages

All type errors use human-readable messages instead of debug output:

```
Type mismatch: cannot assign String to variable of type Int
Type error: arithmetic operator requires Int operands, got String and Int
Type error: if condition must be Bool, got String
Type error: cannot add String and Int (both operands must be Int or both must be String)
Type error: argument 0 of function 'add' expects Int, got String
```

## Implementation Details

### Type Checker Architecture

Located in [compiler/src/typechecker.rs](../compiler/src/typechecker.rs):

```rust
pub struct TypeChecker {
    symbols: HashMap<String, Type>,           // Variable type mappings
    functions: HashMap<String, FunctionSignature>,  // Function signatures
    errors: Vec<String>,                      // Accumulated errors
}
```

#### Key Methods:
- `check(&mut self, stmts: &[Stmt]) -> Result<(), Vec<String>>` - Main entry point
- `check_stmt(&mut self, stmt: &Stmt)` - Type check individual statements
- `check_expr(&mut self, expr: &Expr) -> Type` - Type check expressions
- `type_to_readable_name(t: &Type) -> &'static str` - Human-readable type names

#### Type Inference Flow:
1. Parse source code to AST
2. Create TypeChecker instance
3. For each statement:
   - If Let: infer type from expression, store in symbol table
   - If Assign: lookup variable type, check value type matches
   - If Function: register signature, check body, infer return type
   - If If/While: check condition is Bool/Int, recursively check branches
4. Collect and report all errors

### Expression Type Checking

Expressions return their inferred type:
- `Expr::Number(n)` → `Type::Int`
- `Expr::String(s)` → `Type::String`
- `Expr::Bool(b)` → `Type::Bool`
- `Expr::Float(f)` → `Type::Float`
- `Expr::Identifier(name)` → lookup in symbol table
- Binary expressions → check operand types and return result type

## Testing

Comprehensive test suite created in [tests/test_type_system.ax](../tests/test_type_system.ax):

- Test 1-5: Basic type inference for Int, String, Bool
- Test 6: Type mismatch detection
- Test 7-10: Function return types and argument validation
- Test 11-15: Control flow, loops, and multi-variable declarations

## Future Enhancements

For full type system maturity:

1. **Union/Optional Types**: Handle `null` and optional values
2. **Generic Types**: Support parametric polymorphism
3. **Type Annotations**: Explicit type declarations (`let x: Int = 10`)
4. **Custom Types**: Structs, enums, and type aliases
5. **Type Conversion**: Explicit casting operators
6. **Better Error Recovery**: Continue type checking after first error
7. **Error Locations**: Track line/column information for IDE integration

## No Silent Failures

The type system ensures:
- ✅ All type mismatches are errors, never coerced
- ✅ All operations are type-safe at compile time
- ✅ No undefined behavior or runtime type errors
- ✅ Clear, actionable error messages

This makes ASTRIXA suitable for:
- Smart contracts (safety critical)
- Financial systems (precision important)
- Production code (debugging time precious)
