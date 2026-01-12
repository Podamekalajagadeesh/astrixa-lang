# STEP 36: Error Diagnostics Testing Guide

## 🧪 Testing Strategy

This guide shows how to test and verify the error diagnostic system.

---

## 🎯 Test Categories

### 1. **Syntax Errors** ✅
- Missing identifiers
- Unexpected tokens
- Malformed structures

### 2. **Position Accuracy** ✅
- Correct line numbers
- Correct column numbers
- Multi-line tracking

### 3. **Help Messages** ✅
- Appropriate suggestions
- Clear guidance
- Actionable advice

---

## 📝 Manual Test Cases

### Test 1: Valid Function
```astrixa
fn greet {
}
```

**Expected Output:**
```
✅ Parsing successful
```

---

### Test 2: Missing Function Name
```astrixa
fn {
}
```

**Expected Output:**
```
Error: Expected function name
 → line 1, column 4
 Help: Function names must be valid identifiers
```

**Verify:**
- [ ] Error message is clear
- [ ] Line number is correct (1)
- [ ] Column number is correct (4)
- [ ] Help text is present
- [ ] No panic or crash

---

### Test 3: Function Name on New Line
```astrixa
fn 
{
}
```

**Expected Output:**
```
Error: Expected function name
 → line 2, column 1
 Help: Function names must be valid identifiers
```

**Verify:**
- [ ] Detects error on correct line (2)
- [ ] Column tracking works across newlines

---

### Test 4: Multiple Functions
```astrixa
fn hello {
}

fn world {
}
```

**Expected Output:**
```
✅ Parsing successful
Functions parsed: 2
```

---

### Test 5: Error in Second Function
```astrixa
fn hello {
}

fn {
}
```

**Expected Output:**
```
Error: Expected function name
 → line 4, column 4
 Help: Function names must be valid identifiers
```

**Verify:**
- [ ] Parses first function successfully
- [ ] Catches error in second function
- [ ] Line number references second function (4)

---

## 🔧 Automated Tests

### Unit Test Structure

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_valid_function() {
        let source = "fn greet { }";
        let lexer = Lexer::new(source);
        let mut parser = Parser::new(lexer);
        
        let result = parser.parse();
        assert!(result.is_ok());
        
        let ast = result.unwrap();
        assert_eq!(ast.len(), 1);
    }
    
    #[test]
    fn test_missing_function_name() {
        let source = "fn { }";
        let lexer = Lexer::new(source);
        let mut parser = Parser::new(lexer);
        
        let result = parser.parse();
        assert!(result.is_err());
        
        if let Err(err) = result {
            assert_eq!(err.message, "Expected function name");
            assert_eq!(err.line, 1);
            assert!(err.help.is_some());
        }
    }
    
    #[test]
    fn test_position_tracking() {
        let source = "\n\nfn { }"; // Error on line 3
        let lexer = Lexer::new(source);
        let mut parser = Parser::new(lexer);
        
        let result = parser.parse();
        assert!(result.is_err());
        
        if let Err(err) = result {
            assert_eq!(err.line, 3);
        }
    }
}
```

---

## 🎮 Interactive Testing

### Option 1: Use the Demo
```bash
cd /workspaces/astrixa-lang/compiler
cargo run --example error_demo
```

### Option 2: Modify main.rs
Edit `compiler/src/main.rs` to test different inputs:

```rust
fn main() {
    let source = r#"
        fn {  // Your test case here
        }
    "#;
    
    let lexer = Lexer::new(source);
    let mut parser = Parser::new(lexer);
    
    match parser.parse() {
        Ok(ast) => println!("✅ Success"),
        Err(err) => display_error(err),
    }
}
```

Then run:
```bash
cargo run
```

---

## 📊 Verification Checklist

### Error Quality
- [ ] Messages are clear and specific
- [ ] No technical jargon
- [ ] No blame language
- [ ] Suggests fixes when possible

### Position Tracking
- [ ] Line numbers are 1-based (human-friendly)
- [ ] Column numbers are 1-based
- [ ] Newlines update line counter
- [ ] Column resets after newline

### Error Handling
- [ ] No panics on invalid input
- [ ] Errors propagate correctly
- [ ] Help text is optional but used
- [ ] Multiple errors handled gracefully

### User Experience
- [ ] Output is easy to read
- [ ] Format is consistent
- [ ] stderr used for errors
- [ ] stdout used for success messages

---

## 🔍 Debugging Tips

### If line numbers are wrong:
1. Check `advance()` in lexer.rs
2. Verify newline detection
3. Ensure line starts at 1

### If column numbers are wrong:
1. Check column increment in `advance()`
2. Verify column reset on newline
3. Ensure column starts at 1

### If errors aren't caught:
1. Verify parser returns `Result`
2. Check error propagation with `?`
3. Ensure all error paths return `Err`

### If help text doesn't show:
1. Check `.help()` call on error creation
2. Verify `display_error` shows help
3. Ensure help is `Some(text)`

---

## 🎯 Success Criteria

Step 36 is successful if:

✅ **No panics** - All errors are handled gracefully  
✅ **Clear messages** - Errors are easy to understand  
✅ **Precise location** - Line and column are accurate  
✅ **Helpful suggestions** - Help text guides users  
✅ **Professional output** - Consistent formatting  
✅ **Extensible** - Easy to add new error types

---

## 🚀 Next Steps

After verifying Step 36:

1. **Add More Error Types**
   - Type mismatch errors
   - Undefined variable errors
   - Syntax errors for other constructs

2. **Enhance Diagnostics**
   - Show code snippets
   - Add color coding
   - Support multiple errors

3. **IDE Integration**
   - Output parseable by language servers
   - Quick fixes for common errors
   - Code actions for suggestions

4. **Documentation**
   - Error code reference
   - Common error solutions
   - Troubleshooting guide

---

## 📚 Related Files

- [STEP_36_ERROR_DIAGNOSTICS_COMPLETE.md](STEP_36_ERROR_DIAGNOSTICS_COMPLETE.md)
- [STEP_36_QUICK_REFERENCE.md](STEP_36_QUICK_REFERENCE.md)
- [compiler/src/error.rs](compiler/src/error.rs)
- [compiler/src/diagnostics.rs](compiler/src/diagnostics.rs)
- [compiler/examples/error_demo.rs](compiler/examples/error_demo.rs)

---

**Status:** ✅ Testing Framework Complete  
**Last Updated:** January 12, 2026
