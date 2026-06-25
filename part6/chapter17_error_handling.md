# Chapter 17: Error Handling

## Learning Objectives

By the end of this chapter, you will be able to:

- Understand why Rust handles errors differently from Java, Python, and C++
- Know the difference between recoverable and unrecoverable errors
- Use `panic!` appropriately and understand when it is the wrong choice
- Use `Result<T, E>` to represent operations that may fail
- Match on `Result` variants to handle both success and failure paths
- Propagate errors up the call stack using the `?` operator
- Define custom error types using enums
- Use the `thiserror` crate to derive error implementations
- Use the `anyhow` crate for ergonomic application-level error handling
- Avoid common beginner mistakes such as overusing `unwrap` in production code

---

## Theory

### Why Error Handling Matters

Every program that interacts with the outside world — files, network, user input, databases — must deal with the possibility of failure. A file might not exist. A network request might time out. A user might enter invalid data.

In many languages, errors are handled through **exceptions**: an error is "thrown" at the point of failure, and "caught" somewhere up the call stack. This is convenient but has a serious drawback — exceptions are **invisible in the type system**. Looking at a function signature in Java or Python, you cannot tell whether that function might throw an exception. You have to read the documentation or discover it at runtime.

Rust takes a radically different approach. Errors are **values**. If a function can fail, its return type says so explicitly using the `Result<T, E>` enum. If you call a function that returns `Result`, the compiler forces you to handle it. You cannot accidentally ignore an error.

This is one of the key reasons Rust programs tend to be more robust than programs in other languages — the compiler is your partner in ensuring that error paths are handled.

---

### Comparison: Rust vs Other Languages

**Python (exceptions):**
```python
def read_file(path):
    with open(path, "r") as f:  # raises FileNotFoundError invisibly
        return f.read()

content = read_file("config.txt")  # might crash with no warning
```

**Java (checked exceptions):**
```java
public String readFile(String path) throws IOException {
    // at least Java forces you to declare throws
    return Files.readString(Path.of(path));
}
```
Java's checked exceptions are closer to Rust's philosophy, but still use exception-throwing mechanics which unwind the stack in an unpredictable way.

**C++ (undefined behavior / error codes):**
```cpp
int divide(int a, int b) {
    return a / b;  // undefined behavior if b == 0
}
```

**Rust (Result):**
```rust
fn read_file(path: &str) -> Result<String, std::io::Error> {
    std::fs::read_to_string(path)  // compiler forces caller to handle error
}
```

In Rust, the function's return type is the contract. There are no surprises.

---

### 17.1 `panic!`

A `panic!` in Rust represents an **unrecoverable error** — something has gone so wrong that the program cannot continue meaningfully. When a panic occurs, Rust:

1. Prints an error message with the file and line number
2. Unwinds the stack (by default), running destructors for all values
3. Terminates the program (or the thread)

**When to use `panic!`:**
- A bug in your own code that should never happen (violated invariant)
- Prototype/test code where you want to fail fast
- Situations where continuing would cause data corruption or worse

**When NOT to use `panic!`:**
- Expected failure conditions (file not found, invalid user input, network timeout)
- Library code (your panic kills the caller's program — very rude)
- Any situation where the caller could reasonably handle the error

**Examples of panic!:**
```rust
fn main() {
    // Explicit panic
    panic!("Something went terribly wrong!");

    // Index out of bounds — automatic panic
    let v = vec![1, 2, 3];
    println!("{}", v[99]);  // panics at runtime

    // Integer overflow (in debug mode)
    let x: u8 = 255;
    let _y = x + 1;  // panics in debug, wraps in release

    // Unwrap on None
    let name: Option<&str> = None;
    let _n = name.unwrap();  // panics: "called `Option::unwrap()` on a `None` value"
}
```

**The `unwrap` and `expect` methods** are shortcuts that panic if the value is `None` or `Err`:

```rust
let result: Result<i32, &str> = Err("something failed");
let value = result.unwrap();   // panics: "called `Result::unwrap()` on an `Err` value: \"something failed\""
let value = result.expect("Expected a number"); // panics with your custom message
```

`expect` is slightly better than `unwrap` because the message gives context, but both are panic-on-failure. Reserve them for test code or situations where the error is truly impossible.

**Panic vs Abort:**

By default, panics unwind the stack. You can configure Rust to abort immediately instead:

```toml
# Cargo.toml
[profile.release]
panic = "abort"
```

Abort is faster and produces smaller binaries, but you lose the stack unwind (destructors do not run). This is common in embedded or WebAssembly targets.

---

### 17.2 `Result<T, E>`

`Result` is a standard library enum defined as:

```rust
enum Result<T, E> {
    Ok(T),   // Success: contains a value of type T
    Err(E),  // Failure: contains an error of type E
}
```

`T` is the type of the successful value. `E` is the type of the error value. Both are generic — you choose what types to use.

**A simple example:**
```rust
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Cannot divide by zero"))
    } else {
        Ok(a / b)
    }
}
```

The return type `Result<f64, String>` says: "This function either gives you an `f64` wrapped in `Ok`, or a `String` error message wrapped in `Err`."

**Why an enum and not a special type?**

Because `Result` is just a regular Rust enum. There is no magic. The error handling system is built on top of ordinary Rust features — pattern matching, generics, traits. This means you can use all the tools you already know with `Result`.

**The standard library uses Result everywhere:**

```rust
use std::fs;
use std::io;

fn main() {
    let result: Result<String, io::Error> = fs::read_to_string("hello.txt");
    // Must handle both Ok and Err
}
```

**Result has many useful methods:**

```rust
let ok: Result<i32, &str> = Ok(42);
let err: Result<i32, &str> = Err("fail");

// is_ok() and is_err()
assert!(ok.is_ok());
assert!(err.is_err());

// unwrap_or — provide a default value instead of panicking
let val = err.unwrap_or(0);  // val == 0

// unwrap_or_else — compute a default lazily
let val = err.unwrap_or_else(|e| {
    println!("Error was: {}", e);
    -1
});

// map — transform the Ok value, leave Err unchanged
let doubled = ok.map(|v| v * 2);  // Ok(84)

// map_err — transform the Err value, leave Ok unchanged
let better_err = err.map_err(|e| format!("Got error: {}", e));

// and_then — chain operations that also return Result
let chained = ok.and_then(|v| if v > 10 { Ok(v) } else { Err("too small") });
```

---

### 17.3 Matching Errors

The most explicit and readable way to handle a `Result` is with `match`:

```rust
use std::fs;

fn main() {
    let result = fs::read_to_string("config.txt");

    match result {
        Ok(content) => {
            println!("File contents:\n{}", content);
        }
        Err(error) => {
            println!("Failed to read file: {}", error);
        }
    }
}
```

**Matching on specific error kinds:**

The `std::io::Error` type has a `kind()` method that returns an `io::ErrorKind` enum. You can match on specific kinds of IO errors:

```rust
use std::fs;
use std::io;

fn main() {
    match fs::read_to_string("config.txt") {
        Ok(content) => println!("{}", content),
        Err(e) => match e.kind() {
            io::ErrorKind::NotFound => {
                println!("File not found. Creating default config...");
                // create a default config
            }
            io::ErrorKind::PermissionDenied => {
                println!("Permission denied reading config.");
            }
            other => {
                println!("Unexpected IO error: {:?}", other);
            }
        },
    }
}
```

**Using `if let` for simple cases:**

When you only care about the `Ok` case (or only the `Err` case), `if let` is cleaner:

```rust
if let Ok(content) = fs::read_to_string("config.txt") {
    println!("Content: {}", content);
}

if let Err(e) = fs::write("output.txt", "hello") {
    eprintln!("Write failed: {}", e);
}
```

---

### 17.4 Propagating Errors

"Propagating" means: instead of handling an error in the current function, pass it up to the caller to deal with. This is extremely common in Rust — most functions that can fail simply return `Result` and let the caller decide what to do.

**Without propagation helpers (verbose):**

```rust
use std::fs;
use std::io;

fn read_username_from_file() -> Result<String, io::Error> {
    let result = fs::read_to_string("username.txt");

    match result {
        Ok(s) => Ok(s),
        Err(e) => Err(e),
    }
}
```

This is correct but tedious. You are just passing the error straight through.

**The evolution toward `?`:**

First, many beginners discover `unwrap` as a shortcut:

```rust
fn read_username_from_file() -> String {
    fs::read_to_string("username.txt").unwrap()  // PANICS on error — wrong!
}
```

This changes the return type to `String` (no longer a `Result`), which means the caller cannot handle the error — the program just crashes. This is the wrong approach for anything that might legitimately fail.

The correct solution for propagation is the `?` operator, covered in the next section.

---

### 17.5 The `?` Operator

The `?` operator is syntactic sugar for "if this is `Err`, return the error from the current function; if it is `Ok`, unwrap the value and continue."

**Equivalent code:**
```rust
// Without ?
let content = match fs::read_to_string("file.txt") {
    Ok(s) => s,
    Err(e) => return Err(e),
};

// With ?
let content = fs::read_to_string("file.txt")?;
```

Both do exactly the same thing. The `?` version is far more readable.

**Full example with `?`:**

```rust
use std::fs;
use std::io;

fn read_username_from_file() -> Result<String, io::Error> {
    let content = fs::read_to_string("username.txt")?;
    Ok(content.trim().to_string())
}

fn main() {
    match read_username_from_file() {
        Ok(name) => println!("Username: {}", name),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

**Chaining `?` calls:**

```rust
use std::fs;
use std::io;
use std::io::Read;

fn read_first_line(path: &str) -> Result<String, io::Error> {
    let mut file = fs::File::open(path)?;           // ? here
    let mut content = String::new();
    file.read_to_string(&mut content)?;              // and here
    let first_line = content.lines().next().unwrap_or("").to_string();
    Ok(first_line)
}
```

**`?` performs type conversion:**

The `?` operator does something subtle and powerful: it calls `From::from(e)` on the error before returning it. This means if your function returns `Result<T, MyError>` and the operation produces a `std::io::Error`, Rust will automatically convert the `io::Error` into `MyError` — as long as you have implemented `From<io::Error> for MyError`.

This is what allows you to use `?` with multiple different error types in the same function, as long as they can all be converted into your function's error type.

**`?` in `main`:**

Since Rust 2018, `main` can return `Result`:

```rust
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = fs::read_to_string("config.txt")?;
    println!("{}", content);
    Ok(())
}
```

`Box<dyn std::error::Error>` is a trait object that can hold any error type. It is often used in `main` and quick scripts.

**ASCII Diagram — Error Propagation Flow:**

```
read_config()                   parse_config()                  main()
     |                               |                              |
     | fs::read_to_string()?         | read_config()?               | parse_config()?
     |                               |                              |
     v                               v                              v
  io::Error -----> ? ----------> ConfigError -----> ? ---------> printed to stderr
                  |  From impl               |  From impl
                  v                          v
            return Err(e)             return Err(e)
```

Each `?` passes the error upward, converting it along the way using `From` implementations.

---

### 17.6 Custom Errors

For real applications, you need your own error types. This lets you:
- Give the caller precise information about what went wrong
- Carry context (like which file caused the error, or what value was invalid)
- Convert different underlying error types into a unified error type for your module

**Defining a custom error enum:**

```rust
use std::fmt;
use std::num::ParseIntError;
use std::io;

#[derive(Debug)]
enum ConfigError {
    IoError(io::Error),
    ParseError(ParseIntError),
    MissingField(String),
    InvalidValue { field: String, value: String },
}

// Implement Display for human-readable messages
impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::IoError(e) => write!(f, "IO error: {}", e),
            ConfigError::ParseError(e) => write!(f, "Parse error: {}", e),
            ConfigError::MissingField(field) => write!(f, "Missing required field: '{}'", field),
            ConfigError::InvalidValue { field, value } => {
                write!(f, "Invalid value '{}' for field '{}'", value, field)
            }
        }
    }
}

// Implement the std::error::Error trait
impl std::error::Error for ConfigError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ConfigError::IoError(e) => Some(e),
            ConfigError::ParseError(e) => Some(e),
            _ => None,
        }
    }
}

// From implementations allow ? to convert automatically
impl From<io::Error> for ConfigError {
    fn from(e: io::Error) -> Self {
        ConfigError::IoError(e)
    }
}

impl From<ParseIntError> for ConfigError {
    fn from(e: ParseIntError) -> Self {
        ConfigError::ParseError(e)
    }
}
```

**Using the custom error:**

```rust
fn load_config(path: &str) -> Result<Config, ConfigError> {
    let content = std::fs::read_to_string(path)?;  // io::Error auto-converted via From
    let port: u16 = content.trim().parse()?;        // ParseIntError auto-converted via From
    Ok(Config { port })
}
```

**The three required pieces for a proper error type:**

1. `#[derive(Debug)]` — so you can use `{:?}` formatting
2. `impl fmt::Display` — for human-readable messages
3. `impl std::error::Error` — to work with the broader error handling ecosystem

Writing all this boilerplate by hand is tedious. That is why `thiserror` exists.

---

### 17.7 `thiserror`

`thiserror` is a popular crate that eliminates the boilerplate of writing `Display` and `Error` trait implementations for custom error types. It uses a procedural macro to generate the implementations from annotations.

**Add to Cargo.toml:**

```toml
[dependencies]
thiserror = "1.0"
```

**The same `ConfigError` with `thiserror`:**

```rust
use thiserror::Error;
use std::num::ParseIntError;
use std::io;

#[derive(Debug, Error)]
enum ConfigError {
    #[error("IO error: {0}")]
    IoError(#[from] io::Error),

    #[error("Parse error: {0}")]
    ParseError(#[from] ParseIntError),

    #[error("Missing required field: '{0}'")]
    MissingField(String),

    #[error("Invalid value '{value}' for field '{field}'")]
    InvalidValue { field: String, value: String },
}
```

That is it. `thiserror` generates:
- `impl fmt::Display for ConfigError` using the `#[error("...")]` strings
- `impl std::error::Error for ConfigError` with `.source()` pointing to the inner error
- `impl From<io::Error> for ConfigError` because of `#[from]`
- `impl From<ParseIntError> for ConfigError` because of `#[from]`

**Format strings in `#[error]`:**
- `{0}` refers to the first field of a tuple variant
- `{field_name}` refers to named fields in a struct variant
- `{0}` with `#[from]` generates a `From` impl automatically

`thiserror` is ideal for **library code** where you want a well-defined, structured error type that callers can match on.

---

### 17.8 `anyhow`

`anyhow` takes a different approach to error handling. Instead of defining a specific error type, `anyhow` provides a single `anyhow::Error` type that can wrap **any** error that implements `std::error::Error`. It is designed for **application code** where you typically just want to propagate errors, log them, and exit — not match on specific variants.

**Add to Cargo.toml:**

```toml
[dependencies]
anyhow = "1.0"
```

**Using anyhow:**

```rust
use anyhow::{Context, Result};
use std::fs;

fn read_config(path: &str) -> Result<String> {
    // anyhow::Result<T> is shorthand for Result<T, anyhow::Error>
    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read config file at '{}'", path))?;

    Ok(content)
}

fn parse_port(config: &str) -> Result<u16> {
    let port: u16 = config
        .trim()
        .parse()
        .with_context(|| format!("Expected a port number, got: '{}'", config.trim()))?;
    Ok(port)
}

fn main() -> Result<()> {
    let config = read_config("config.txt")?;
    let port = parse_port(&config)?;
    println!("Listening on port {}", port);
    Ok(())
}
```

**Key `anyhow` features:**

- `anyhow::Result<T>` = `Result<T, anyhow::Error>` — convenient type alias
- `.context("message")` — attach a human-readable message to an error
- `.with_context(|| format!(...))` — lazy context (only computed if there is an error)
- `anyhow::bail!("message")` — macro to return early with an error: `return Err(anyhow::anyhow!("message"))`
- `anyhow::ensure!(condition, "message")` — like `assert!` but returns `Err` instead of panicking
- `anyhow::anyhow!("message")` — create an ad-hoc error from a string

**anyhow error chains:**

```rust
use anyhow::{Context, Result};

fn load_app() -> Result<()> {
    let content = std::fs::read_to_string("app.conf")
        .context("loading application configuration")?;
    println!("{}", content);
    Ok(())
}

fn main() {
    if let Err(e) = load_app() {
        eprintln!("Error: {:#}", e);
        // {:#} prints the full error chain:
        // "loading application configuration: No such file or directory (os error 2)"
    }
}
```

**`thiserror` vs `anyhow` — when to use which:**

| Situation | Use |
|-----------|-----|
| Writing a library | `thiserror` — callers need to match on specific errors |
| Writing an application | `anyhow` — you just want to propagate and display errors |
| Mixed: library errors in an application | Both — `thiserror` in library, `anyhow` in app |

---

### Code Example

Below is a complete, self-contained program that demonstrates all of the concepts in this chapter. It simulates reading a configuration file, parsing its values, and using a chain of error types.

**Cargo.toml:**

```toml
[package]
name = "error_handling_demo"
version = "0.1.0"
edition = "2021"

[dependencies]
thiserror = "1.0"
anyhow = "1.0"
```

**src/main.rs:**

```rust
// ============================================================
// Chapter 17 — Error Handling Demo
// ============================================================
// This program demonstrates:
//  - panic! (briefly, in a comment)
//  - Custom error types with thiserror
//  - Result<T, E> and the ? operator
//  - Error propagation and From conversions
//  - anyhow for application-level error handling
//  - A simulated config file parser
// ============================================================

use std::collections::HashMap;
use std::fmt;
use thiserror::Error;
use anyhow::{Context, Result as AnyResult};

// ─────────────────────────────────────────────
// 1. Custom error type for our config module
// ─────────────────────────────────────────────

#[derive(Debug, Error)]
enum ConfigError {
    // #[from] generates: impl From<std::io::Error> for ConfigError
    #[error("Failed to read configuration file: {0}")]
    Io(#[from] std::io::Error),

    // #[from] generates: impl From<std::num::ParseIntError> for ConfigError
    #[error("Failed to parse integer value: {0}")]
    ParseInt(#[from] std::num::ParseIntError),

    // A custom error variant with named fields
    #[error("Missing required configuration key: '{key}'")]
    MissingKey { key: String },

    // Another custom variant
    #[error("Value '{value}' for key '{key}' is out of allowed range [{min}, {max}]")]
    OutOfRange {
        key: String,
        value: i64,
        min: i64,
        max: i64,
    },
}

// ─────────────────────────────────────────────
// 2. Config struct — what we want to produce
// ─────────────────────────────────────────────

#[derive(Debug)]
struct AppConfig {
    host: String,
    port: u16,
    max_connections: u32,
    timeout_seconds: u64,
}

// ─────────────────────────────────────────────
// 3. Parsing functions — each returns Result
// ─────────────────────────────────────────────

/// Parse a key=value formatted string into a HashMap.
/// Lines starting with '#' are treated as comments and ignored.
/// Returns ConfigError::Io if reading fails (though here we take &str).
fn parse_key_value(content: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for line in content.lines() {
        let line = line.trim();
        // Skip empty lines and comments
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some((key, value)) = line.split_once('=') {
            map.insert(key.trim().to_string(), value.trim().to_string());
        }
    }
    map
}

/// Get a required string value from the map.
fn get_required<'a>(map: &'a HashMap<String, String>, key: &str) -> Result<&'a str, ConfigError> {
    map.get(key)
        .map(String::as_str)
        .ok_or_else(|| ConfigError::MissingKey { key: key.to_string() })
}

/// Get a required integer value and validate its range.
fn get_int_in_range(
    map: &HashMap<String, String>,
    key: &str,
    min: i64,
    max: i64,
) -> Result<i64, ConfigError> {
    let raw = get_required(map, key)?;
    let parsed: i64 = raw.parse()?;  // ParseIntError auto-converted via From

    if parsed < min || parsed > max {
        return Err(ConfigError::OutOfRange {
            key: key.to_string(),
            value: parsed,
            min,
            max,
        });
    }

    Ok(parsed)
}

/// Parse the full config from a &str.
fn parse_config(content: &str) -> Result<AppConfig, ConfigError> {
    let map = parse_key_value(content);

    let host = get_required(&map, "host")?.to_string();
    let port = get_int_in_range(&map, "port", 1, 65535)? as u16;
    let max_connections = get_int_in_range(&map, "max_connections", 1, 10_000)? as u32;
    let timeout_seconds = get_int_in_range(&map, "timeout_seconds", 1, 3600)? as u64;

    Ok(AppConfig {
        host,
        port,
        max_connections,
        timeout_seconds,
    })
}

/// Read a config file from disk and parse it.
/// Returns ConfigError, which can wrap io::Error via From.
fn load_config_file(path: &str) -> Result<AppConfig, ConfigError> {
    let content = std::fs::read_to_string(path)?;  // io::Error → ConfigError::Io via From
    parse_config(&content)
}

// ─────────────────────────────────────────────
// 4. Application layer — using anyhow
// ─────────────────────────────────────────────

/// Application startup using anyhow for ergonomic propagation.
/// anyhow::Result<T> is Result<T, anyhow::Error>
fn run_application(config_path: &str) -> AnyResult<()> {
    // .context() attaches a human-readable message to any error
    let config = load_config_file(config_path)
        .with_context(|| format!("Failed to load config from '{}'", config_path))?;

    println!("✓ Configuration loaded successfully:");
    println!("  Host              : {}", config.host);
    println!("  Port              : {}", config.port);
    println!("  Max Connections   : {}", config.max_connections);
    println!("  Timeout (seconds) : {}", config.timeout_seconds);

    Ok(())
}

// ─────────────────────────────────────────────
// 5. Demonstrate panic! (only in safe contexts)
// ─────────────────────────────────────────────

fn demonstrate_panics() {
    println!("\n--- Panic demonstrations ---");

    // This is safe: std::panic::catch_unwind lets us catch panics in demo code.
    // In real code, you would NOT catch panics to handle business logic.
    let result = std::panic::catch_unwind(|| {
        let v: Vec<i32> = vec![1, 2, 3];
        v[99]  // out-of-bounds access — panics
    });
    match result {
        Ok(_) => println!("  (no panic)"),
        Err(_) => println!("  Caught panic: vector index out of bounds (as expected)"),
    }

    // unwrap on None
    let result = std::panic::catch_unwind(|| {
        let x: Option<i32> = None;
        x.unwrap()
    });
    match result {
        Ok(_) => println!("  (no panic)"),
        Err(_) => println!("  Caught panic: called unwrap() on a None value (as expected)"),
    }

    println!("--- End of panic demonstrations ---\n");
}

// ─────────────────────────────────────────────
// 6. Demonstrate the evolution: match → ? operator
// ─────────────────────────────────────────────

fn demo_verbose_propagation(content: &str) -> Result<u16, ConfigError> {
    // Step 1: Using match — explicit but verbose
    let map = parse_key_value(content);
    let raw = match map.get("port") {
        Some(v) => v,
        None => return Err(ConfigError::MissingKey { key: "port".to_string() }),
    };
    let port: i64 = match raw.parse() {
        Ok(n) => n,
        Err(e) => return Err(ConfigError::ParseInt(e)),
    };
    Ok(port as u16)
}

fn demo_question_mark_propagation(content: &str) -> Result<u16, ConfigError> {
    // Step 2: Using ? — clean and idiomatic
    let map = parse_key_value(content);
    let port = get_int_in_range(&map, "port", 1, 65535)? as u16;
    Ok(port)
}

// ─────────────────────────────────────────────
// 7. main — ties everything together
// ─────────────────────────────────────────────

fn main() {
    println!("=== Chapter 17: Error Handling Demo ===\n");

    // ── Demo 1: panic demonstrations ──────────
    demonstrate_panics();

    // ── Demo 2: verbose vs ? propagation ──────
    println!("--- Propagation comparison ---");
    let good_content = "port = 8080";
    let bad_content  = "port = not_a_number";

    match demo_verbose_propagation(good_content) {
        Ok(p)  => println!("  Verbose OK: port = {}", p),
        Err(e) => println!("  Verbose Err: {}", e),
    }
    match demo_question_mark_propagation(bad_content) {
        Ok(p)  => println!("  ? OK: port = {}", p),
        Err(e) => println!("  ? Err: {}", e),
    }
    println!();

    // ── Demo 3: inline config string (no file needed) ──
    println!("--- Config parsing from inline string ---");
    let valid_config = r#"
# Application configuration
host             = localhost
port             = 4000
max_connections  = 100
timeout_seconds  = 30
"#;

    match parse_config(valid_config) {
        Ok(cfg) => println!("  Parsed config: {:?}", cfg),
        Err(e)  => println!("  Error: {}", e),
    }

    // ── Demo 4: config with out-of-range value ──
    println!("\n--- Config with out-of-range port ---");
    let bad_range_config = r#"
host             = localhost
port             = 99999
max_connections  = 100
timeout_seconds  = 30
"#;
    match parse_config(bad_range_config) {
        Ok(cfg) => println!("  Parsed: {:?}", cfg),
        Err(e)  => println!("  Error: {}", e),
    }

    // ── Demo 5: config with missing key ──
    println!("\n--- Config with missing key ---");
    let missing_key_config = r#"
host = localhost
port = 8080
# max_connections is missing!
timeout_seconds = 30
"#;
    match parse_config(missing_key_config) {
        Ok(cfg) => println!("  Parsed: {:?}", cfg),
        Err(e)  => println!("  Error: {}", e),
    }

    // ── Demo 6: attempt to load from a nonexistent file (uses anyhow) ──
    println!("\n--- Attempting to load nonexistent file (anyhow) ---");
    match run_application("nonexistent_config.txt") {
        Ok(())  => println!("  Application started."),
        Err(e)  => {
            // {:#} prints the full error chain including context
            eprintln!("  Application error: {:#}", e);
        }
    }

    // ── Demo 7: load from an actual file ──
    println!("\n--- Writing and loading a real config file ---");
    let path = "/tmp/demo_config.txt";
    std::fs::write(path, valid_config).expect("Could not write temp config");

    match run_application(path) {
        Ok(())  => {},
        Err(e)  => eprintln!("  Error: {:#}", e),
    }
}
```

---

### Line-by-Line Explanation

**Lines 1–10 — Imports**

```rust
use std::collections::HashMap;
use std::fmt;
use thiserror::Error;
use anyhow::{Context, Result as AnyResult};
```

- `HashMap` is used to store parsed key-value pairs from the config.
- `fmt` is imported for the `Display` trait (though `thiserror` handles it).
- `thiserror::Error` is the derive macro from the `thiserror` crate.
- `anyhow::Context` adds the `.context()` and `.with_context()` methods to `Result`.
- `anyhow::Result` is aliased to `AnyResult` to avoid conflicting with `std::result::Result`.

**Lines 12–35 — ConfigError enum**

```rust
#[derive(Debug, Error)]
enum ConfigError {
    #[error("Failed to read configuration file: {0}")]
    Io(#[from] std::io::Error),
    ...
}
```

- `#[derive(Debug, Error)]` makes `thiserror` generate the `Display` and `Error` trait impls.
- Each `#[error("...")]` attribute defines the `Display` string for that variant.
- `{0}` in the format string refers to the first (and only) unnamed field of the tuple variant.
- `#[from]` on a field tells `thiserror` to generate `impl From<std::io::Error> for ConfigError`.
- The `OutOfRange` variant uses named fields `{key}`, `{value}`, `{min}`, `{max}` in its format string.

**Lines 38–45 — AppConfig struct**

A plain data struct with `#[derive(Debug)]` so it can be printed with `{:?}`.

**Lines 48–62 — parse_key_value**

Iterates lines of the config string. Skips blank lines and `#` comments. Uses `split_once('=')` (stable since Rust 1.52) to split each line on the first `=` sign into a key-value pair.

**Lines 65–71 — get_required**

Returns a reference to the map value or a `MissingKey` error. The `'a` lifetime ties the returned `&str` to the lifetime of the map. `ok_or_else` is a lazy version of `ok_or` — the error value is only constructed if there is actually an error, avoiding unnecessary allocation.

**Lines 74–93 — get_int_in_range**

1. Calls `get_required` and uses `?` to propagate `MissingKey` errors.
2. Calls `.parse::<i64>()` on the raw string. The `?` operator converts `ParseIntError` into `ConfigError::ParseInt` via the `From` impl generated by `thiserror`.
3. Checks the range manually and returns `ConfigError::OutOfRange` if violated.

**Lines 96–109 — parse_config**

Calls the helper functions in sequence, using `?` to propagate any error. Returns an `AppConfig` wrapped in `Ok` on success.

**Lines 112–116 — load_config_file**

Reads a file with `std::fs::read_to_string`. The `?` operator converts the `io::Error` to `ConfigError::Io` via the `From` impl. Then calls `parse_config`.

**Lines 119–133 — run_application**

Uses `anyhow::Result`. The `.with_context(|| ...)` call attaches a descriptive message to any error that comes out of `load_config_file`. The closure is lazy — it is only called if there is an error.

**Lines 136–156 — demonstrate_panics**

Uses `std::panic::catch_unwind` to safely demonstrate panics without crashing the program. In real code you would never catch panics for business logic — this is only for demonstration.

---

### Common Mistakes

**Mistake 1: Using `unwrap()` in production code**

```rust
// WRONG — panics if the file doesn't exist
let content = std::fs::read_to_string("config.txt").unwrap();

// RIGHT — handle the error
let content = match std::fs::read_to_string("config.txt") {
    Ok(c) => c,
    Err(e) => {
        eprintln!("Cannot read config: {}", e);
        std::process::exit(1);
    }
};

// OR — propagate with ?
let content = std::fs::read_to_string("config.txt")?;
```

`unwrap` is fine in:
- Test code (`#[test]` functions)
- Examples and prototypes
- Situations where the error is genuinely impossible (and even then, prefer `expect` with an explanation)

**Mistake 2: Using `Box<dyn Error>` in library code**

```rust
// WRONG for a library — callers cannot match on specific error types
pub fn parse(s: &str) -> Result<Config, Box<dyn std::error::Error>> { ... }

// RIGHT — use a specific error type
pub fn parse(s: &str) -> Result<Config, ConfigError> { ... }
```

`Box<dyn Error>` is appropriate in `main` and application code. In library code, it prevents callers from handling specific error variants.

**Mistake 3: Missing `From` implementations**

```rust
fn load() -> Result<Config, ConfigError> {
    let content = std::fs::read_to_string("config.txt")?;
    // ERROR if there is no impl From<io::Error> for ConfigError
    // The ? operator needs From to convert the error type
    ...
}
```

When you see an error like "the `?` operator can only be used in a function that returns `Result`" or "the trait `From<io::Error>` is not implemented for `MyError`", it means you need a `From` impl (or use `map_err`).

**Mistake 4: Ignoring errors silently**

```rust
// WRONG — error is completely ignored
let _ = std::fs::write("output.txt", "data");

// RIGHT — at minimum, log the error
if let Err(e) = std::fs::write("output.txt", "data") {
    eprintln!("Warning: could not write output: {}", e);
}
```

**Mistake 5: Forgetting to return `Ok(())` at the end of a function**

```rust
fn do_work() -> Result<(), MyError> {
    some_operation()?;
    // WRONG — missing Ok(())
}

fn do_work() -> Result<(), MyError> {
    some_operation()?;
    Ok(())  // RIGHT
}
```

**Mistake 6: Using `?` in a function that returns `()`**

```rust
fn main() {
    std::fs::read_to_string("file.txt")?;  // ERROR — main() returns ()
}

// FIX: change main to return Result
fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::fs::read_to_string("file.txt")?;
    Ok(())
}
```

**Mistake 7: Creating error variants that are too coarse**

```rust
// TOO COARSE — callers cannot distinguish between different failures
enum AppError {
    SomethingWentWrong(String),
}

// BETTER — callers can match on specific cases
enum AppError {
    ConfigNotFound { path: String },
    InvalidPort { value: String },
    DatabaseConnectionFailed { host: String, port: u16 },
}
```

---

### Best Practices

1. **Prefer `Result` over `panic!` for anything a caller could handle.** Use `panic!` only for programming bugs (violated invariants), not for expected runtime failures.

2. **Use `thiserror` for library crates.** It gives you well-structured error types with minimal boilerplate.

3. **Use `anyhow` for application crates.** It lets you focus on the happy path and attach context to errors as they propagate.

4. **Add context to errors as they cross abstraction boundaries.** Use `.context()` or `.with_context()` to explain what the program was trying to do.

5. **Use `?` instead of `match` when you just want to propagate.** `?` is cleaner and idiomatic.

6. **Implement `From` (or let `thiserror`/`#[from]` do it) so `?` can do type conversion automatically.**

7. **Never silently ignore errors.** At minimum, log them. The `let _ = expr` pattern discards errors — use it only if you have thought carefully about why the error does not matter.

8. **Use `expect("explanation")` instead of `unwrap()` when you must use a panic-on-fail approach.** The explanation makes debugging easier.

9. **Make error types implement `Send + Sync` when they might cross thread boundaries.** `anyhow::Error` does this for you. For custom types, ensure all fields are `Send + Sync`.

10. **Test both the success path and the error paths.** Write tests that deliberately trigger error conditions.

---

## Practice: File Reader

This practice exercise builds a file reading utility that demonstrates all the error handling concepts in a realistic scenario.

**Cargo.toml:**

```toml
[package]
name = "file_reader"
version = "0.1.0"
edition = "2021"

[dependencies]
thiserror = "1.0"
```

**src/main.rs:**

```rust
use std::fs;
use std::io;
use std::path::Path;
use thiserror::Error;

// ── Custom error type ──────────────────────────────────────────────────────

#[derive(Debug, Error)]
enum FileReaderError {
    #[error("File not found: '{path}'")]
    NotFound { path: String },

    #[error("Permission denied reading file: '{path}'")]
    PermissionDenied { path: String },

    #[error("IO error reading '{path}': {source}")]
    Io {
        path: String,
        #[source]  // marks this as the "source" of the error for error chains
        source: io::Error,
    },

    #[error("File is too large: {size} bytes (max {max} bytes)")]
    FileTooLarge { size: u64, max: u64 },

    #[error("File is not valid UTF-8: '{path}'")]
    InvalidEncoding { path: String },
}

// ── File reading functions ─────────────────────────────────────────────────

const MAX_FILE_SIZE: u64 = 10 * 1024 * 1024; // 10 MB

/// Read a text file, with detailed error handling.
fn read_text_file(path: &str) -> Result<String, FileReaderError> {
    let p = Path::new(path);

    // Check metadata before reading
    let metadata = fs::metadata(p).map_err(|e| match e.kind() {
        io::ErrorKind::NotFound => FileReaderError::NotFound {
            path: path.to_string(),
        },
        io::ErrorKind::PermissionDenied => FileReaderError::PermissionDenied {
            path: path.to_string(),
        },
        _ => FileReaderError::Io {
            path: path.to_string(),
            source: e,
        },
    })?;

    // Check file size before reading
    let size = metadata.len();
    if size > MAX_FILE_SIZE {
        return Err(FileReaderError::FileTooLarge {
            size,
            max: MAX_FILE_SIZE,
        });
    }

    // Read the file
    let bytes = fs::read(p).map_err(|e| FileReaderError::Io {
        path: path.to_string(),
        source: e,
    })?;

    // Convert to UTF-8
    String::from_utf8(bytes).map_err(|_| FileReaderError::InvalidEncoding {
        path: path.to_string(),
    })
}

/// Count the words in a string (split on whitespace).
fn count_words(text: &str) -> usize {
    text.split_whitespace().count()
}

/// Count the lines in a string.
fn count_lines(text: &str) -> usize {
    text.lines().count()
}

/// Find the longest line in a string.
fn longest_line(text: &str) -> Option<&str> {
    text.lines().max_by_key(|line| line.len())
}

fn main() {
    let files = [
        "README.md",
        "nonexistent_file.txt",
        "/etc/shadow",  // likely permission denied
    ];

    for path in &files {
        println!("Reading '{}'...", path);
        match read_text_file(path) {
            Ok(content) => {
                println!("  Lines  : {}", count_lines(&content));
                println!("  Words  : {}", count_words(&content));
                if let Some(longest) = longest_line(&content) {
                    println!("  Longest: {} chars", longest.len());
                }
            }
            Err(e) => {
                eprintln!("  Error  : {}", e);
            }
        }
        println!();
    }
}
```

---

## Mini Project: Configuration Loader

### Project Overview

Build a complete configuration loader that reads INI-style config files, validates the values, provides meaningful errors, and exposes a clean API using proper Rust error handling idioms.

**The config file format:**

```ini
# Server configuration
host = 0.0.0.0
port = 8080
max_connections = 200
timeout_seconds = 60
log_level = info
debug = false
```

### Functional Requirements

1. Read a config file from a given path
2. Parse `key = value` lines (ignore comments and blank lines)
3. Extract typed values: string, integer, boolean
4. Validate that required keys are present
5. Validate that integer values are within expected ranges
6. Return a structured `AppConfig` on success
7. Return a descriptive `ConfigError` on failure
8. Use `thiserror` for the error type
9. Use `anyhow` in the application layer for ergonomic error chaining

### Project Structure

```
config_loader/
├── Cargo.toml
└── src/
    ├── main.rs          ← application entry point
    ├── config.rs        ← AppConfig struct and loader
    └── error.rs         ← ConfigError type
```

### Step 1: Set up Cargo.toml

```toml
[package]
name = "config_loader"
version = "0.1.0"
edition = "2021"

[dependencies]
thiserror = "1.0"
anyhow = "1.0"
```

### Step 2: Define the error type (src/error.rs)

```rust
// src/error.rs
//
// ConfigError — all the ways loading a config can fail.
// We use thiserror to avoid writing Display and Error impls by hand.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    /// The config file could not be read from disk.
    /// The #[from] attribute generates: impl From<std::io::Error> for ConfigError
    /// This makes the ? operator work on io::Error inside functions that return
    /// Result<_, ConfigError>.
    #[error("Cannot read configuration file '{path}': {source}")]
    IoError {
        path: String,
        #[source]
        source: std::io::Error,
    },

    /// A required key was not present in the config file.
    #[error("Required configuration key '{key}' is missing")]
    MissingKey { key: String },

    /// A value that should be an integer could not be parsed.
    /// The #[from] attribute on ParseIntError would conflict if we have multiple
    /// From impls, so we use #[source] and wrap manually.
    #[error("Configuration key '{key}' has value '{value}' which is not a valid integer")]
    InvalidInteger { key: String, value: String },

    /// A value that should be a boolean could not be parsed.
    #[error("Configuration key '{key}' has value '{value}' which is not a valid boolean (use 'true' or 'false')")]
    InvalidBoolean { key: String, value: String },

    /// An integer value was outside its allowed range.
    #[error("Configuration key '{key}' has value {actual} which is outside the allowed range [{min}, {max}]")]
    OutOfRange {
        key: String,
        actual: i64,
        min: i64,
        max: i64,
    },

    /// The log level was not one of the allowed values.
    #[error("Configuration key 'log_level' has invalid value '{value}'. Allowed values: trace, debug, info, warn, error")]
    InvalidLogLevel { value: String },
}
```

### Step 3: Implement the config module (src/config.rs)

```rust
// src/config.rs
//
// Parses and validates the application configuration from a file.

use std::collections::HashMap;
use std::path::Path;

use crate::error::ConfigError;

// ── Public types ───────────────────────────────────────────────────────────

/// Log level severity.
#[derive(Debug, Clone, PartialEq)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogLevel::Trace => write!(f, "trace"),
            LogLevel::Debug => write!(f, "debug"),
            LogLevel::Info  => write!(f, "info"),
            LogLevel::Warn  => write!(f, "warn"),
            LogLevel::Error => write!(f, "error"),
        }
    }
}

/// The fully-parsed and validated application configuration.
#[derive(Debug)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
    pub max_connections: u32,
    pub timeout_seconds: u64,
    pub log_level: LogLevel,
    pub debug: bool,
}

// ── Internal helpers ────────────────────────────────────────────────────────

/// Parse a "key = value" text into a lookup map.
/// - Strips leading/trailing whitespace from keys and values.
/// - Ignores blank lines and lines starting with '#'.
/// - If a line has no '=' it is silently ignored.
fn parse_raw(content: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();

    for line in content.lines() {
        let line = line.trim();

        // Skip comments and empty lines
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        // Split on the first '=' only, so values can contain '='
        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim().to_lowercase();   // normalize keys to lowercase
            let value = value.trim().to_string();
            map.insert(key, value);
        }
    }

    map
}

/// Extract a required string value.
fn require_string<'a>(
    map: &'a HashMap<String, String>,
    key: &str,
) -> Result<&'a str, ConfigError> {
    map.get(key)
        .map(String::as_str)
        .ok_or_else(|| ConfigError::MissingKey {
            key: key.to_string(),
        })
}

/// Extract a required integer within an inclusive range.
fn require_int_in_range(
    map: &HashMap<String, String>,
    key: &str,
    min: i64,
    max: i64,
) -> Result<i64, ConfigError> {
    let raw = require_string(map, key)?;

    let value: i64 = raw.parse().map_err(|_| ConfigError::InvalidInteger {
        key: key.to_string(),
        value: raw.to_string(),
    })?;

    if value < min || value > max {
        return Err(ConfigError::OutOfRange {
            key: key.to_string(),
            actual: value,
            min,
            max,
        });
    }

    Ok(value)
}

/// Extract a required boolean. Accepts "true" / "false" (case-insensitive).
fn require_bool(map: &HashMap<String, String>, key: &str) -> Result<bool, ConfigError> {
    let raw = require_string(map, key)?;

    match raw.to_lowercase().as_str() {
        "true"  | "1" | "yes" | "on"  => Ok(true),
        "false" | "0" | "no"  | "off" => Ok(false),
        _ => Err(ConfigError::InvalidBoolean {
            key: key.to_string(),
            value: raw.to_string(),
        }),
    }
}

/// Parse a log level string into the LogLevel enum.
fn require_log_level(map: &HashMap<String, String>) -> Result<LogLevel, ConfigError> {
    let raw = require_string(map, "log_level")?;

    match raw.to_lowercase().as_str() {
        "trace" => Ok(LogLevel::Trace),
        "debug" => Ok(LogLevel::Debug),
        "info"  => Ok(LogLevel::Info),
        "warn"  => Ok(LogLevel::Warn),
        "error" => Ok(LogLevel::Error),
        _ => Err(ConfigError::InvalidLogLevel {
            value: raw.to_string(),
        }),
    }
}

// ── Public API ─────────────────────────────────────────────────────────────

/// Parse configuration from a string (useful for testing without touching disk).
pub fn parse_config(content: &str) -> Result<AppConfig, ConfigError> {
    let map = parse_raw(content);

    let host = require_string(&map, "host")?.to_string();
    let port = require_int_in_range(&map, "port", 1, 65535)? as u16;
    let max_connections = require_int_in_range(&map, "max_connections", 1, 10_000)? as u32;
    let timeout_seconds = require_int_in_range(&map, "timeout_seconds", 1, 3600)? as u64;
    let log_level = require_log_level(&map)?;
    let debug = require_bool(&map, "debug")?;

    Ok(AppConfig {
        host,
        port,
        max_connections,
        timeout_seconds,
        log_level,
        debug,
    })
}

/// Load configuration from a file on disk.
/// Returns a ConfigError::IoError if the file cannot be read.
pub fn load_config<P: AsRef<Path>>(path: P) -> Result<AppConfig, ConfigError> {
    let path_str = path.as_ref().display().to_string();

    let content = std::fs::read_to_string(&path).map_err(|source| ConfigError::IoError {
        path: path_str,
        source,
    })?;

    parse_config(&content)
}
```

### Step 4: Write the main entry point (src/main.rs)

```rust
// src/main.rs
//
// Application entry point. Uses anyhow for ergonomic error handling
// in the top-level code. Library code (config.rs) uses the specific
// ConfigError type.

mod config;
mod error;

use anyhow::{Context, Result};
use config::{load_config, parse_config};

fn print_config(cfg: &config::AppConfig) {
    println!("  Host            : {}", cfg.host);
    println!("  Port            : {}", cfg.port);
    println!("  Max Connections : {}", cfg.max_connections);
    println!("  Timeout (s)     : {}", cfg.timeout_seconds);
    println!("  Log Level       : {}", cfg.log_level);
    println!("  Debug Mode      : {}", cfg.debug);
}

fn run() -> Result<()> {
    println!("=== Configuration Loader Demo ===\n");

    // ── Scenario 1: Valid inline config ───────────────────────────────────
    println!("--- Scenario 1: Valid configuration ---");
    let valid = r#"
# Server settings
host            = 0.0.0.0
port            = 8080
max_connections = 200
timeout_seconds = 60
log_level       = info
debug           = false
"#;

    let cfg = parse_config(valid).context("parsing valid config")?;
    print_config(&cfg);
    println!();

    // ── Scenario 2: Missing key ────────────────────────────────────────────
    println!("--- Scenario 2: Missing required key ---");
    let missing_key = r#"
host            = 0.0.0.0
port            = 8080
# max_connections is intentionally missing
timeout_seconds = 60
log_level       = info
debug           = false
"#;

    match parse_config(missing_key) {
        Ok(_)  => println!("  (unexpected success)"),
        Err(e) => println!("  Expected error: {}", e),
    }
    println!();

    // ── Scenario 3: Invalid integer ────────────────────────────────────────
    println!("--- Scenario 3: Invalid integer value ---");
    let invalid_int = r#"
host            = 0.0.0.0
port            = eight-thousand
max_connections = 200
timeout_seconds = 60
log_level       = info
debug           = false
"#;

    match parse_config(invalid_int) {
        Ok(_)  => println!("  (unexpected success)"),
        Err(e) => println!("  Expected error: {}", e),
    }
    println!();

    // ── Scenario 4: Out-of-range integer ──────────────────────────────────
    println!("--- Scenario 4: Out-of-range port ---");
    let out_of_range = r#"
host            = 0.0.0.0
port            = 99999
max_connections = 200
timeout_seconds = 60
log_level       = info
debug           = false
"#;

    match parse_config(out_of_range) {
        Ok(_)  => println!("  (unexpected success)"),
        Err(e) => println!("  Expected error: {}", e),
    }
    println!();

    // ── Scenario 5: Invalid boolean ────────────────────────────────────────
    println!("--- Scenario 5: Invalid boolean ---");
    let invalid_bool = r#"
host            = 0.0.0.0
port            = 8080
max_connections = 200
timeout_seconds = 60
log_level       = info
debug           = yes_please
"#;

    match parse_config(invalid_bool) {
        Ok(_)  => println!("  (unexpected success)"),
        Err(e) => println!("  Expected error: {}", e),
    }
    println!();

    // ── Scenario 6: Invalid log level ─────────────────────────────────────
    println!("--- Scenario 6: Invalid log level ---");
    let bad_log_level = r#"
host            = 0.0.0.0
port            = 8080
max_connections = 200
timeout_seconds = 60
log_level       = verbose
debug           = false
"#;

    match parse_config(bad_log_level) {
        Ok(_)  => println!("  (unexpected success)"),
        Err(e) => println!("  Expected error: {}", e),
    }
    println!();

    // ── Scenario 7: Load from a real file with anyhow context ─────────────
    println!("--- Scenario 7: Load from disk ---");
    let tmp_path = "/tmp/app_config_demo.ini";
    std::fs::write(tmp_path, valid).context("writing demo config file to /tmp")?;

    let cfg = load_config(tmp_path)
        .with_context(|| format!("loading config from '{}'", tmp_path))?;

    println!("  Loaded from disk successfully:");
    print_config(&cfg);
    println!();

    // ── Scenario 8: File not found (anyhow error chain) ───────────────────
    println!("--- Scenario 8: File not found (anyhow chain) ---");
    let result: anyhow::Result<config::AppConfig> = load_config("nonexistent.ini")
        .with_context(|| "loading production config".to_string())
        .map_err(anyhow::Error::from);

    if let Err(e) = result {
        // {:#} prints the full chain: "loading production config: Cannot read..."
        println!("  Error chain: {:#}", e);
    }

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Fatal error: {:#}", e);
        std::process::exit(1);
    }
}
```

### Complete Source Code Listing

The three files above constitute the full project. To run it:

```bash
cargo new config_loader
cd config_loader

# Create src/error.rs, src/config.rs, replace src/main.rs with the code above
# Update Cargo.toml with the dependencies

cargo run
```

**Expected output:**

```
=== Configuration Loader Demo ===

--- Scenario 1: Valid configuration ---
  Host            : 0.0.0.0
  Port            : 8080
  Max Connections : 200
  Timeout (s)     : 60
  Log Level       : info
  Debug Mode      : false

--- Scenario 2: Missing required key ---
  Expected error: Required configuration key 'max_connections' is missing

--- Scenario 3: Invalid integer value ---
  Expected error: Configuration key 'port' has value 'eight-thousand' which is not a valid integer

--- Scenario 4: Out-of-range port ---
  Expected error: Configuration key 'port' has value 99999 which is outside the allowed range [1, 65535]

--- Scenario 5: Invalid boolean ---
  Expected error: Configuration key 'debug' has value 'yes_please' which is not a valid boolean (use 'true' or 'false')

--- Scenario 6: Invalid log level ---
  Expected error: Configuration key 'log_level' has invalid value 'verbose'. Allowed values: trace, debug, info, warn, error

--- Scenario 7: Load from disk ---
  Loaded from disk successfully:
  Host            : 0.0.0.0
  Port            : 8080
  Max Connections : 200
  Timeout (s)     : 60
  Log Level       : info
  Debug Mode      : false

--- Scenario 8: File not found (anyhow chain) ---
  Error chain: loading production config: Cannot read configuration file 'nonexistent.ini': No such file or directory (os error 2)
```

### Code Explanation

**`src/error.rs`** — The error module defines every way the config loading can fail as an enum variant. Using `thiserror` means:
- No manual `impl Display` — the `#[error("...")]` strings handle it.
- The `#[source]` attribute on `std::io::Error` fields connects the error chain so callers can traverse the chain with `.source()`.
- Each variant carries exactly the context needed to write a useful message.

**`src/config.rs`** — The config module separates concerns cleanly:
- `parse_raw` is a pure function: string in, `HashMap` out. It never fails.
- `require_string`, `require_int_in_range`, `require_bool`, `require_log_level` are small focused helpers. Each validates one thing and returns a specific error if validation fails.
- `parse_config` orchestrates the helpers. It reads like a list of requirements.
- `load_config` is the only function that touches the filesystem. Separating IO from parsing makes both easier to test.

**`src/main.rs`** — The application layer uses `anyhow`:
- `run() -> Result<()>` uses `anyhow::Result` for convenience.
- `.context("...")` adds human-readable context when errors cross from the library into the application.
- `main` calls `run()` and handles the top-level error gracefully (print and exit with code 1).

### Refactoring Suggestions

**Suggestion 1: Add an optional key helper**

The current design requires all keys. Add an `optional_string` helper and use it for keys that have sensible defaults:

```rust
fn optional_string<'a>(
    map: &'a HashMap<String, String>,
    key: &str,
) -> Option<&'a str> {
    map.get(key).map(String::as_str)
}

// In parse_config:
let debug = optional_string(&map, "debug")
    .map(|raw| require_bool_raw("debug", raw))
    .transpose()?
    .unwrap_or(false);  // default to false if not specified
```

**Suggestion 2: Support environment variable overrides**

A production config loader should allow env vars to override file values:

```rust
pub fn load_config_with_env_overrides<P: AsRef<Path>>(path: P) -> Result<AppConfig, ConfigError> {
    let mut map = {
        let content = std::fs::read_to_string(&path).map_err(|source| ConfigError::IoError {
            path: path.as_ref().display().to_string(),
            source,
        })?;
        parse_raw(&content)
    };

    // Override with environment variables
    for (key, env_var) in &[
        ("host",            "APP_HOST"),
        ("port",            "APP_PORT"),
        ("max_connections", "APP_MAX_CONNECTIONS"),
        ("log_level",       "APP_LOG_LEVEL"),
    ] {
        if let Ok(val) = std::env::var(env_var) {
            map.insert(key.to_string(), val);
        }
    }

    validate_and_build(map)
}
```

**Suggestion 3: Use a builder pattern**

Instead of one giant `parse_config` function, consider an `AppConfigBuilder` that accumulates the config and validates at the end:

```rust
#[derive(Default)]
struct AppConfigBuilder {
    host: Option<String>,
    port: Option<u16>,
    // ...
}

impl AppConfigBuilder {
    fn host(mut self, h: impl Into<String>) -> Self { self.host = Some(h.into()); self }
    fn port(mut self, p: u16) -> Self { self.port = Some(p); self }
    fn build(self) -> Result<AppConfig, ConfigError> {
        Ok(AppConfig {
            host: self.host.ok_or_else(|| ConfigError::MissingKey { key: "host".into() })?,
            port: self.port.ok_or_else(|| ConfigError::MissingKey { key: "port".into() })?,
            // ...
        })
    }
}
```

**Suggestion 4: Write unit tests for each error case**

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::ConfigError;

    fn valid_config() -> &'static str {
        "host=localhost\nport=8080\nmax_connections=100\ntimeout_seconds=30\nlog_level=info\ndebug=false"
    }

    #[test]
    fn test_valid_config_parses() {
        let cfg = parse_config(valid_config()).expect("should parse");
        assert_eq!(cfg.host, "localhost");
        assert_eq!(cfg.port, 8080);
    }

    #[test]
    fn test_missing_key() {
        let input = "port=8080"; // host is missing
        let err = parse_config(input).unwrap_err();
        assert!(matches!(err, ConfigError::MissingKey { key } if key == "host"));
    }

    #[test]
    fn test_out_of_range_port() {
        let input = "host=localhost\nport=0\nmax_connections=100\ntimeout_seconds=30\nlog_level=info\ndebug=false";
        let err = parse_config(input).unwrap_err();
        assert!(matches!(err, ConfigError::OutOfRange { key, .. } if key == "port"));
    }

    #[test]
    fn test_invalid_log_level() {
        let input = "host=localhost\nport=8080\nmax_connections=100\ntimeout_seconds=30\nlog_level=verbose\ndebug=false";
        let err = parse_config(input).unwrap_err();
        assert!(matches!(err, ConfigError::InvalidLogLevel { .. }));
    }
}
```

### Challenge Exercises

1. **Multi-file config:** Extend the loader to read a base config file and then merge an optional override file on top of it. Keys in the override file replace keys from the base file.

2. **TOML support:** Add a `parse_toml(content: &str) -> Result<AppConfig, ConfigError>` function that parses TOML format. Use the `toml` crate. Add a `ConfigError::TomlError(toml::de::Error)` variant.

3. **Schema validation:** Add a `ConfigSchema` struct that describes which keys are required, their types, and their valid ranges. Have `parse_config` accept a schema and validate against it generically, rather than having hardcoded field names.

4. **Reload on signal:** On Unix systems, extend the application to reload the config when it receives `SIGUSR1`. Think about how you would safely handle the case where the new config is invalid — should the old config remain active?

5. **Config diff:** Write a function `diff_configs(old: &AppConfig, new: &AppConfig) -> Vec<String>` that returns a human-readable list of what changed between two configs.

---

## Exercises

**Exercise 1:** Write a function `parse_positive_integer(s: &str) -> Result<u32, String>` that parses a string as a `u32`. Return `Err("empty input".to_string())` if the string is empty, and `Err(format!("not a valid number: '{}'", s))` if it cannot be parsed.

**Exercise 2:** Write a function `read_lines(path: &str) -> Result<Vec<String>, std::io::Error>` that reads a file and returns its lines as a `Vec<String>`. Use `?` for error propagation.

**Exercise 3:** Define a custom error enum `MathError` with variants `DivisionByZero`, `NegativeSquareRoot`, and `Overflow`. Implement `Display` manually (without `thiserror`). Write functions `safe_divide(a: f64, b: f64) -> Result<f64, MathError>` and `safe_sqrt(x: f64) -> Result<f64, MathError>`.

**Exercise 4:** Convert `MathError` from Exercise 3 to use `thiserror`.

**Exercise 5:** Write a function `load_and_count(path: &str) -> anyhow::Result<usize>` that reads a file and returns the number of words. Use `.with_context()` to add the filename to any IO errors.

---

## Solutions

### Solution 1

```rust
fn parse_positive_integer(s: &str) -> Result<u32, String> {
    if s.is_empty() {
        return Err("empty input".to_string());
    }
    s.parse::<u32>().map_err(|_| format!("not a valid number: '{}'", s))
}

#[test]
fn test_parse_positive_integer() {
    assert_eq!(parse_positive_integer("42"), Ok(42));
    assert_eq!(parse_positive_integer(""), Err("empty input".to_string()));
    assert!(parse_positive_integer("abc").is_err());
    assert!(parse_positive_integer("-5").is_err());
}
```

### Solution 2

```rust
fn read_lines(path: &str) -> Result<Vec<String>, std::io::Error> {
    let content = std::fs::read_to_string(path)?;
    Ok(content.lines().map(String::from).collect())
}
```

### Solution 3

```rust
use std::fmt;

#[derive(Debug)]
enum MathError {
    DivisionByZero,
    NegativeSquareRoot,
    Overflow,
}

impl fmt::Display for MathError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MathError::DivisionByZero     => write!(f, "division by zero"),
            MathError::NegativeSquareRoot => write!(f, "square root of a negative number"),
            MathError::Overflow           => write!(f, "arithmetic overflow"),
        }
    }
}

impl std::error::Error for MathError {}

fn safe_divide(a: f64, b: f64) -> Result<f64, MathError> {
    if b == 0.0 {
        Err(MathError::DivisionByZero)
    } else {
        Ok(a / b)
    }
}

fn safe_sqrt(x: f64) -> Result<f64, MathError> {
    if x < 0.0 {
        Err(MathError::NegativeSquareRoot)
    } else {
        Ok(x.sqrt())
    }
}
```

### Solution 4

```rust
use thiserror::Error;

#[derive(Debug, Error)]
enum MathError {
    #[error("division by zero")]
    DivisionByZero,

    #[error("square root of a negative number")]
    NegativeSquareRoot,

    #[error("arithmetic overflow")]
    Overflow,
}

fn safe_divide(a: f64, b: f64) -> Result<f64, MathError> {
    if b == 0.0 { Err(MathError::DivisionByZero) } else { Ok(a / b) }
}

fn safe_sqrt(x: f64) -> Result<f64, MathError> {
    if x < 0.0 { Err(MathError::NegativeSquareRoot) } else { Ok(x.sqrt()) }
}
```

### Solution 5

```rust
use anyhow::{Context, Result};

fn load_and_count(path: &str) -> Result<usize> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read file '{}'", path))?;
    Ok(content.split_whitespace().count())
}
```

---

## Quiz

**Question 1:** What is the difference between `panic!` and returning `Err(...)` from a function?

**Question 2:** Why does Rust use `Result<T, E>` instead of exceptions?

**Question 3:** What does the `?` operator do? Write the equivalent code without `?`.

**Question 4:** What three things must a custom error type in Rust implement to be considered a proper error type?

**Question 5:** What is the difference between `thiserror` and `anyhow`? When should you use each?

**Question 6:** What does `#[from]` in a `thiserror`-annotated enum do?

**Question 7:** What does `.context("message")` (from `anyhow`) do?

**Question 8:** Why is `unwrap()` dangerous in production code?

**Question 9:** What does `From::from(e)` have to do with the `?` operator?

**Question 10:** What is the difference between `unwrap_or(value)` and `unwrap_or_else(|| compute_value())`?

---

## Quiz Answers

**Answer 1:** `panic!` terminates the program (or thread) immediately — it is unrecoverable. The caller has no opportunity to handle it. Returning `Err(...)` is a recoverable error — the caller receives the error as a normal value and can decide what to do with it: log it, retry, use a default, or propagate it further.

**Answer 2:** Exceptions are invisible in the type system. Looking at a function signature with exceptions, you cannot tell whether it might throw. Rust's `Result<T, E>` makes errors part of the function's type — the compiler forces callers to acknowledge that the operation might fail. This prevents an entire class of unhandled-error bugs at compile time.

**Answer 3:** The `?` operator early-returns from the current function with the `Err` value (after converting it with `From::from`) if the expression is `Err`. If it is `Ok`, it unwraps the value and the expression evaluates to the inner value. Equivalent code:
```rust
// With ?
let x = some_fn()?;

// Without ?
let x = match some_fn() {
    Ok(v) => v,
    Err(e) => return Err(From::from(e)),
};
```

**Answer 4:** A proper Rust error type should implement: (1) `std::fmt::Debug` (usually via `#[derive(Debug)]`), (2) `std::fmt::Display` (for human-readable messages), and (3) `std::error::Error` (the standard trait that makes it work with the error handling ecosystem, including `Box<dyn Error>` and `anyhow`).

**Answer 5:** `thiserror` helps you define **specific, structured error types** using a derive macro. It generates `Display` and `Error` impls for you. Use it in library code where callers need to match on specific error variants. `anyhow` provides a single **opaque error type** (`anyhow::Error`) that can wrap any error, with support for adding context messages. Use it in application code where you primarily want to propagate and display errors, not match on specific variants.

**Answer 6:** `#[from]` on a field inside a `thiserror`-annotated enum variant automatically generates `impl From<FieldType> for YourErrorEnum`. This enables the `?` operator to automatically convert errors of `FieldType` into your enum when used in functions that return `Result<_, YourErrorEnum>`.

**Answer 7:** `.context("message")` (from `anyhow`) attaches a human-readable description to an error as it propagates. The result is an `anyhow::Error` that has the original error as its `source` and the context string as its description. When printed with `{:#}`, the output shows the full chain: `"context message: original error message"`.

**Answer 8:** `unwrap()` panics if the `Result` is `Err` or the `Option` is `None`. In production code, many errors are expected (file not found, network timeout, invalid input) and should be handled gracefully — logging, retrying, or returning an error to the user. Panicking crashes the program (or the thread), losing any in-flight work and potentially leaving the system in an inconsistent state. Use `?` or explicit error handling instead.

**Answer 9:** When the `?` operator encounters an `Err(e)`, it does not return `Err(e)` directly. It first calls `From::from(e)` to convert `e` into the error type expected by the current function's return type. This is what allows you to use `?` on an `io::Error` in a function that returns `Result<_, MyError>`, as long as `impl From<io::Error> for MyError` exists. `thiserror`'s `#[from]` attribute generates exactly these `From` implementations.

**Answer 10:** `unwrap_or(value)` always evaluates `value`, even if the `Result` or `Option` is `Ok`/`Some`. `unwrap_or_else(|| compute_value())` only evaluates the closure if the value is actually `Err`/`None`. This matters when the default value is expensive to compute (e.g., allocating a string, making a DB call) — in those cases, use `unwrap_or_else` to avoid unnecessary work.

---

## Chapter Summary

This chapter covered Rust's complete error handling story, from the simplest cases to production-grade patterns.

**Key concepts:**

- Rust treats errors as **values**, not exceptions. This makes error handling explicit and compiler-enforced.
- **`panic!`** is for unrecoverable bugs — violations of invariants that should never happen. It terminates the program.
- **`Result<T, E>`** is for recoverable errors — anything a caller might want to handle. It is a plain enum with `Ok(T)` and `Err(E)` variants.
- **Matching on `Result`** with `match`, `if let`, or the many combinator methods (`map`, `and_then`, `unwrap_or`) gives you full control over error paths.
- **The `?` operator** eliminates the boilerplate of propagating errors manually. It is syntactic sugar for `match` with an early return, plus automatic type conversion via `From`.
- **Custom error types** should be enums with variants for each failure mode. They need `Debug`, `Display`, and `std::error::Error` implementations.
- **`thiserror`** generates the `Display`, `Error`, and `From` impls for you using derive macros. It is ideal for library code.
- **`anyhow`** provides a single flexible error type for application code. Its `.context()` / `.with_context()` methods make error messages more informative as errors propagate up the call stack.
- **Common mistakes** to avoid: `unwrap` in production, `Box<dyn Error>` in library return types, silent error discarding, and missing `From` implementations.

**The mental model:**

```
Is this error something the caller could handle?
    YES → Return Result<T, E>
    NO  → panic! (or use expect with an explanation)

Is this library code?
    YES → Use thiserror for structured error types
    NO  → Use anyhow for ergonomic propagation + context

Is this a chain of operations that can all fail?
    YES → Use ? to propagate through the chain
    NO  → Match, unwrap_or, or other combinators
```

Rust's error handling requires slightly more initial thought than throwing exceptions, but the result is code that is explicit about its failure modes, resistant to overlooked error paths, and much easier to debug when things do go wrong.
