# Rust Cheat Sheet

## Core Ideas

**Ownership** — Every value has exactly one owner. When the owner goes out of scope, the value is dropped.

```rust
{
    let s = String::from("hello");  // s owns the String
}                                   // s goes out of scope → memory freed
```

**Move** — Assigning or passing a value transfers ownership; the original binding is invalidated (unless the type is `Copy`).

```rust
let a = String::from("hello");
let b = a;           // ownership moves to b
// println!("{a}");   // ✗ compile error: a is no longer valid
println!("{b}");      // ✓ b owns it now
```

**Borrowing** — References (`&T` / `&mut T`) let you access data without taking ownership. Rule: many shared readers *or* one exclusive writer, never both.

```rust
let mut s = String::from("hello");

let r1 = &s;         // ✓ shared borrow
let r2 = &s;         // ✓ multiple shared borrows OK
println!("{r1} {r2}");

let r3 = &mut s;     // ✓ exclusive borrow (no other refs active)
r3.push_str(" world");
```

**Lifetimes** — The compiler tracks how long references are valid to guarantee no dangling pointers. Usually inferred; annotate (`'a`) when the compiler needs help.

```rust
// Annotation tells the compiler: the returned ref lives as long as
// both inputs — the caller must keep both alive.
fn longer<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() { a } else { b }
}
```

**Zero-cost abstractions** — Iterators, generics, and traits compile down to the same code you'd write by hand; no runtime overhead.

```rust
// This iterator chain compiles to the same loop a C programmer would write
let sum: i32 = (1..=100).filter(|n| n % 2 == 0).sum();
```

---

## 0. Structs vs Enums

Use a struct when:

 - All fields are always present
 - There is only one valid “state”
 - The data represents a cohesive object

Use an enum when:

 - There are multiple valid states
 - Each state has different data
 - You want the compiler to prevent impossible combinations
 - You’ll pattern-match on behavior

Enums are the foundation of sum types, structs are product types:

 - struct { A, B } → has A and B
 - enum { A, B } → has A or B


## 1. Structs & Methods

Define data with `struct`, attach behavior with `impl`:

```rust
struct Tree {
    value: i32,
    left:  Option<Box<Tree>>,   // Box for recursive types
    right: Option<Box<Tree>>,
}

impl Tree {
    // Associated function (no self) — used as a constructor
    fn leaf(value: i32) -> Self {
        Tree { value, left: None, right: None }
    }

    fn new(value: i32, left: Tree, right: Tree) -> Self {
        Tree { value, left: Some(Box::new(left)), right: Some(Box::new(right)) }
    }

    // Method: &self = shared borrow, &mut self = exclusive borrow
    fn count(&self) -> usize {
        1 + self.left.as_ref().map_or(0, |n| n.count())
          + self.right.as_ref().map_or(0, |n| n.count())
    }

    fn contains(&self, target: i32) -> bool {
        self.value == target
            || self.left.as_ref().is_some_and(|n| n.contains(target))
            || self.right.as_ref().is_some_and(|n| n.contains(target))
    }
}

// Usage
let tree = Tree::new(
    1,
    Tree::leaf(2),
    Tree::new(3, Tree::leaf(4), Tree::leaf(5)),
);
tree.count();       // 5
tree.contains(4);   // true
```

Key points:
 - `Self` is an alias for the type being implemented
 - `&self` borrows immutably, `&mut self` borrows mutably, `self` takes ownership
 - Associated functions (no `self`) are called with `Type::func()`
 - Methods (have `self`) are called with `value.method()`
 - Use `Box<T>` for heap allocation / recursive types

------------------------------------------------------------

## 2. Core Definitions
```rust
// Option<T>: value may or may not exist
// Some(T) | None
// Used when "missing" is normal

// Result<T, E>: success or error with diagnostic info
// Ok(T) | Err(E)
```

------------------------------------------------------------

## 3. Conversions Between Option and Result

### Option → Result
```rust
option.ok_or(err)
option.ok_or_else(|| err)

// Example:
let user = maybe_user.ok_or("user missing")?;
```

### Result → Option
```rust
result.ok()      // drop error, keep success
result.err()     // drop success, keep error

// Example:
let maybe_int = "123".parse::<i32>().ok();
```

------------------------------------------------------------

## 4. Transforming Values

```rust
option.map(|v| f(v))          // transform Some(v)
result.map(|v| f(v))          // transform Ok(v)
result.map_err(|e| g(e))      // transform Err(e)
```

### map_or / map_or_else
```rust
option.map_or(default, |v| f(v))
option.map_or_else(|| default(), |v| f(v))
```

------------------------------------------------------------

## 5. Unwrapping With Defaults (Safe)

```rust
option.unwrap_or(default)
option.unwrap_or_else(|| default)
option.unwrap_or_default()       // uses T: Default

result.unwrap_or(default)
result.unwrap_or_else(|_| default)
```

------------------------------------------------------------

## 6. Early Returns With `?`

### Option example
```rust
fn first_char(s: &str) -> Option<char> {
    let c = s.chars().first()?;  // returns None early
    Some(c)
}
```

### Result example
```rust
fn load() -> Result<String, std::io::Error> {
    let txt = std::fs::read_to_string("file.txt")?;  // returns Err early
    Ok(txt)
}
```

------------------------------------------------------------

## 7. Combine Option<Result> Using transpose()

```rust
let x: Option<Result<i32, _>> = Some("5".parse());
let y: Result<Option<i32>, _> = x.transpose()?;  // Option<Result> -> Result<Option>
```

------------------------------------------------------------

## 8. Pattern Matching

### Option
```rust
match val {
    Some(v) => {
        // use v
    }
    None => {
        // handle missing
    }
}
```

### Result
```rust
match res {
    Ok(v) => {
        // use v
    }
    Err(e) => {
        // handle error
    }
}
```

------------------------------------------------------------

## 9. Useful Option Helpers

```rust
option.is_some();
option.is_none();
option.as_ref();
option.as_mut();
option.take();                  // take value, replace with None
option.filter(|v| condition(v)) // keep only if predicate true
option.and(other_option);
option.or(other_option);
option.xor(other_option);
option.flatten();               // Option<Option<T>> -> Option<T>
option.copied();                // Option<&T> -> Option<T> if T: Copy
option.cloned();                // Option<&T> -> Option<T> clone
```

### Option zip
```rust
Some(1).zip(Some(2));   // Some((1,2))
```

### Option iter
```rust
for x in option.iter() { println!("{x}"); }
```

### bool.then(|| value)
```rust
let o = condition.then(|| compute_value());
```

------------------------------------------------------------

## 10. Useful Result Helpers

```rust
result.is_ok();
result.is_err();
result.as_ref();
result.as_mut();
result.or_else(|e| f(e));
result.and_then(|v| g(v));      // chain fallible operations
result.map(|v| f(v));           // transform Ok(v)
result.map_err(|e| g(e));       // transform Err(e)
result.inspect(|v| println!("value = {v}"));
result.inspect_err(|e| println!("error = {e}"));
```

### Result zip
```rust
let a: Result<i32, _> = Ok(3);
let b: Result<i32, _> = Ok(4);
let both = a.zip(b)?;   // (3,4)
```

------------------------------------------------------------

## 11. Common Real-World Patterns

### Optional lookup → error
```rust
let last = v.last().copied().ok_or("empty slice")?;
```

### Try parse → Option
```rust
let n = "42".parse::<i32>().ok();
```

### Fallible chaining
```rust
fn run() -> Result<()> {
    let txt = std::fs::read_to_string("config.toml")?;
    let port: u16 = txt.trim().parse()?;
    Ok(())
}
```

### Add context to errors (anyhow)
```rust
use anyhow::{Result, Context};

fn load(path: &str) -> Result<String> {
    std::fs::read_to_string(path)
        .with_context(|| format!("failed reading config: {path}"))
}
```

### Convert Option<T> <-> Result<T, ()>
```rust
let r: Result<i32, ()> = option.ok_or(());
let o: Option<i32> = result.ok();
```

------------------------------------------------------------

## 12. When to Use Which

```rust
// Use Option<T> when:
// - Missing is normal
// - No error info needed
// - Natural optionality (searching, lookup, optional fields)

// Use Result<T, E> when:
// - Operation should usually succeed
// - Diagnostics matter
// - Failure is exceptional or worth reporting
```

------------------------------------------------------------

## 13. Avoid Common Mistakes

```rust
// Don't hide real errors by returning Option.
// Don't wrap everything in Result unnecessarily.
// Don't call unwrap() except in guaranteed-safe situations (tests, invariants).
```
