# Chapter 19: Testing

## Learning Objectives

By the end of this chapter, you will:

- Understand why Rust has testing built directly into Cargo and the language itself
- Write unit tests using `#[test]`, `assert!`, `assert_eq!`, `assert_ne!`, and `#[should_panic]`
- Understand the difference between unit tests (inline in `src/`) and integration tests (`tests/` directory)
- Organize tests using modules, helper functions, and shared fixtures
- Understand mocking concepts in Rust using trait-based design and the `mockall` crate
- Write benchmarks using the `criterion` crate
- Use `cargo test` and its flags to run specific tests and control output
- Avoid the most common beginner testing mistakes

---

## Theory

### Why Testing is Built Into Cargo

In most ecosystems, testing requires a separate tool. In JavaScript you install Jest or Mocha. In Python you install pytest. In Java you configure JUnit. Rust takes a different philosophy: **testing is a first-class feature of the language and the build tool**.

This means:

- The `#[test]` attribute is part of the Rust language itself
- `cargo test` is built into Cargo — no extra installation needed
- The test runner is compiled from your own code
- Tests live in the same file as the code they test, or in a dedicated `tests/` folder
- Documentation examples (`///`) are also runnable as tests

This design choice reflects Rust's philosophy of making correctness easy. You should not have to set up a testing framework before you can write your first test. The moment you create a Cargo project, you can write and run tests.

---

### 19.1 Unit Testing

A unit test verifies a single function or a small unit of behavior in isolation. In Rust, unit tests live **inside the same file as the code they are testing**, usually at the bottom of the file.

#### The `#[test]` Attribute

The `#[test]` attribute marks a function as a test. Cargo collects all functions marked with `#[test]` and runs them when you execute `cargo test`.

```rust
#[test]
fn it_works() {
    let result = 2 + 2;
    assert_eq!(result, 4);
}
```

A test function:
- Takes no arguments
- Returns nothing (or returns `Result<(), E>` — covered later)
- Panics if it fails
- Is ignored in normal `cargo build` or `cargo run` — only compiled and run with `cargo test`

#### The `#[cfg(test)]` Module

Unit tests are wrapped in a module annotated with `#[cfg(test)]`. This tells the compiler: **only compile this code when running tests**. This keeps test code out of your final binary.

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addition() {
        assert_eq!(add(2, 3), 5);
    }
}
```

The `use super::*;` import brings everything from the parent module into scope. This is how unit tests access private functions — they are in a child module of the code being tested, so they have access to private items by the normal Rust module visibility rules.

This is one of the few places in Rust where you can test private functions directly, and it is intentional. You do not need to make everything `pub` just to test it.

#### Assertion Macros

Rust provides several macros for making assertions in tests:

**`assert!(expression)`**

Passes if `expression` is `true`. Panics otherwise.

```rust
assert!(5 > 3);
assert!(!false);
```

**`assert_eq!(left, right)`**

Passes if `left == right`. On failure, prints both values.

```rust
assert_eq!(2 + 2, 4);
assert_eq!(String::from("hello"), String::from("hello"));
```

**`assert_ne!(left, right)`**

Passes if `left != right`. On failure, prints both values.

```rust
assert_ne!(2 + 2, 5);
```

**Custom failure messages**

All assertion macros accept an optional format message after the required arguments:

```rust
assert_eq!(result, expected, "Expected {} but got {}", expected, result);
```

#### `#[should_panic]`

Sometimes you want to test that a function panics under certain conditions. Use `#[should_panic]`:

```rust
#[test]
#[should_panic]
fn test_divide_by_zero_panics() {
    divide(10, 0); // should panic
}
```

You can make this more precise by requiring a specific panic message:

```rust
#[test]
#[should_panic(expected = "division by zero")]
fn test_divide_by_zero_message() {
    divide(10, 0);
}
```

The test passes only if the function panics AND the panic message contains the string `"division by zero"`.

#### Tests That Return `Result`

Instead of panicking, tests can return `Result<(), E>`. This lets you use the `?` operator inside tests:

```rust
#[test]
fn test_parse_number() -> Result<(), std::num::ParseIntError> {
    let n: i32 = "42".parse()?;
    assert_eq!(n, 42);
    Ok(())
}
```

If the `Result` is `Err`, the test fails with the error message. This is useful when testing functions that return `Result` themselves.

#### `#[ignore]`

Mark slow or incomplete tests with `#[ignore]` to skip them in normal test runs:

```rust
#[test]
#[ignore]
fn expensive_test() {
    // takes 30 seconds
}
```

Run ignored tests explicitly with:

```bash
cargo test -- --ignored
```

---

### 19.2 Integration Testing

Integration tests live in a dedicated `tests/` directory at the root of your crate (next to `src/`). Each file in `tests/` is compiled as a **separate crate** that uses your library as an external dependency.

```
my_project/
├── src/
│   └── lib.rs
├── tests/
│   ├── integration_test.rs
│   └── another_test.rs
└── Cargo.toml
```

Because integration test files are separate crates, they can only access the **public API** of your library. This is by design — integration tests verify that your public interface works correctly, the same way an external user would use it.

```rust
// tests/integration_test.rs
use my_project::add; // must be pub in src/lib.rs

#[test]
fn test_add_from_outside() {
    assert_eq!(my_project::add(2, 3), 5);
}
```

No `#[cfg(test)]` is needed in integration test files because the entire `tests/` directory is only compiled during `cargo test`.

#### Shared Helper Code in Integration Tests

If you need helper functions shared across multiple integration test files, create a `tests/common/mod.rs` file:

```
tests/
├── common/
│   └── mod.rs
├── integration_test.rs
└── another_test.rs
```

```rust
// tests/common/mod.rs
pub fn setup() -> SomeFixture {
    // shared setup code
}
```

```rust
// tests/integration_test.rs
mod common;

#[test]
fn test_something() {
    let fixture = common::setup();
    // ...
}
```

Note: `common/mod.rs` is NOT treated as a test file itself because it does not live directly in `tests/` — it lives in a subdirectory.

---

### 19.3 Test Organization

#### ASCII Diagram: Test File Structure

```
my_crate/
├── src/
│   ├── lib.rs          <-- public API + unit tests at bottom
│   ├── math.rs         <-- module with unit tests at bottom
│   └── utils.rs        <-- module with unit tests at bottom
│
├── tests/
│   ├── common/
│   │   └── mod.rs      <-- shared helpers (NOT a test file itself)
│   ├── math_tests.rs   <-- integration tests for math module
│   └── utils_tests.rs  <-- integration tests for utils module
│
├── benches/
│   └── benchmarks.rs   <-- criterion benchmarks
│
└── Cargo.toml
```

#### Unit Test Module Convention

The standard convention is to put a `tests` module at the bottom of each source file:

```rust
// src/math.rs

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn internal_helper(x: i32) -> i32 {
    x * 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_positive() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn test_add_negative() {
        assert_eq!(add(-1, -2), -3);
    }

    #[test]
    fn test_internal_helper() {
        // private fn accessible here because tests is a child module
        assert_eq!(internal_helper(5), 10);
    }
}
```

#### Naming Tests

Name tests descriptively. The name should tell you what is being tested and what the expected outcome is:

```rust
// Bad
#[test]
fn test1() { ... }

// Good
#[test]
fn add_two_positive_numbers_returns_sum() { ... }

#[test]
fn divide_by_zero_panics() { ... }

#[test]
fn parse_valid_email_returns_ok() { ... }

#[test]
fn parse_invalid_email_returns_err() { ... }
```

#### Grouping Tests with Nested Modules

For complex modules, you can further organize tests into nested submodules:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    mod addition {
        use super::*;

        #[test]
        fn positive_numbers() {
            assert_eq!(add(2, 3), 5);
        }

        #[test]
        fn negative_numbers() {
            assert_eq!(add(-1, -2), -3);
        }
    }

    mod subtraction {
        use super::*;

        #[test]
        fn positive_result() {
            assert_eq!(subtract(5, 3), 2);
        }
    }
}
```

When you run `cargo test`, these appear as `tests::addition::positive_numbers` and `tests::subtraction::positive_result`.

---

### 19.4 Mocking Concepts

Mocking is the practice of replacing real dependencies with fake ones that return controlled values. This lets you test a unit of code in isolation without relying on databases, network calls, file systems, or other external systems.

#### Why Mocking is Different in Rust

In dynamic languages like Python or JavaScript, mocking is easy because you can replace any function or object at runtime. In Rust, the type system prevents this — you cannot replace a concrete type with a different one unless the code was designed for it.

Rust's approach to mocking relies on **traits**. Instead of depending on a concrete type, your code depends on a trait. In tests, you substitute a fake implementation of that trait. In production, you use the real implementation.

This is sometimes called **dependency injection via traits**.

#### Manual Mock via Traits

```rust
// Define a trait for the dependency
trait EmailSender {
    fn send(&self, to: &str, subject: &str, body: &str) -> Result<(), String>;
}

// Real implementation
struct SmtpEmailSender {
    host: String,
}

impl EmailSender for SmtpEmailSender {
    fn send(&self, to: &str, subject: &str, body: &str) -> Result<(), String> {
        // actually connect to SMTP server
        println!("Sending email to {} via {}", to, self.host);
        Ok(())
    }
}

// The function being tested depends on the trait, not the concrete type
fn send_welcome_email(sender: &dyn EmailSender, user_email: &str) -> Result<(), String> {
    sender.send(user_email, "Welcome!", "Thanks for signing up.")
}

// In tests: fake implementation
#[cfg(test)]
mod tests {
    use super::*;

    struct MockEmailSender {
        pub calls: std::cell::RefCell<Vec<String>>,
    }

    impl MockEmailSender {
        fn new() -> Self {
            MockEmailSender {
                calls: std::cell::RefCell::new(Vec::new()),
            }
        }
    }

    impl EmailSender for MockEmailSender {
        fn send(&self, to: &str, _subject: &str, _body: &str) -> Result<(), String> {
            self.calls.borrow_mut().push(to.to_string());
            Ok(())
        }
    }

    #[test]
    fn test_send_welcome_email_calls_sender() {
        let mock = MockEmailSender::new();
        let result = send_welcome_email(&mock, "user@example.com");
        assert!(result.is_ok());
        assert_eq!(mock.calls.borrow().len(), 1);
        assert_eq!(mock.calls.borrow()[0], "user@example.com");
    }
}
```

This pattern — define a trait, implement it for both real and fake types — is idiomatic Rust mocking. No library required.

#### The `mockall` Crate

For more complex mocking needs (verifying call counts, setting return values per call, matching arguments), the `mockall` crate generates mock implementations automatically using a derive macro:

```toml
[dev-dependencies]
mockall = "0.13"
```

```rust
use mockall::automock;

#[automock]
trait Database {
    fn get_user(&self, id: u32) -> Option<String>;
    fn save_user(&mut self, id: u32, name: &str) -> Result<(), String>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;

    #[test]
    fn test_get_user_called_with_correct_id() {
        let mut mock = MockDatabase::new();

        mock.expect_get_user()
            .with(eq(42))
            .times(1)
            .returning(|_| Some("Alice".to_string()));

        let result = mock.get_user(42);
        assert_eq!(result, Some("Alice".to_string()));
    }
}
```

`mockall` is powerful but adds complexity. For simple cases, manual mocks are cleaner. Use `mockall` when you need to verify exact call counts, argument matchers, or complex return sequences.

---

### 19.5 Benchmarking Basics

Rust's standard library provides unstable benchmarking via `#[bench]`, but this requires nightly Rust. The community standard for stable Rust benchmarking is the **`criterion`** crate.

Criterion:
- Works on stable Rust
- Performs statistical analysis across multiple runs
- Detects performance regressions
- Generates HTML reports with graphs

#### Setting Up Criterion

Add to `Cargo.toml`:

```toml
[dev-dependencies]
criterion = { version = "0.5", features = ["html_reports"] }

[[bench]]
name = "benchmarks"
harness = false
```

The `harness = false` tells Cargo not to use the built-in test harness for this benchmark — criterion provides its own.

#### Writing Benchmarks

Create `benches/benchmarks.rs`:

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn benchmark_fibonacci(c: &mut Criterion) {
    c.bench_function("fibonacci 20", |b| {
        b.iter(|| fibonacci(black_box(20)))
    });
}

criterion_group!(benches, benchmark_fibonacci);
criterion_main!(benches);
```

Run benchmarks with:

```bash
cargo bench
```

**`black_box`** prevents the compiler from optimizing away the computation being measured. Without it, the compiler might detect that the result is never used and eliminate the entire call.

---

### Running Tests with `cargo test`

#### Basic Usage

```bash
# Run all tests
cargo test

# Run a specific test by name
cargo test test_addition

# Run all tests in a module
cargo test math::tests

# Run tests matching a pattern (substring match)
cargo test divide

# Show println! output (normally suppressed)
cargo test -- --nocapture

# Run ignored tests
cargo test -- --ignored

# Run only ignored tests
cargo test -- --ignored --include-ignored

# Run tests on a single thread (useful when tests share global state)
cargo test -- --test-threads=1

# List all tests without running them
cargo test -- --list
```

#### Understanding Test Output

```
running 4 tests
test tests::add_positive ... ok
test tests::add_negative ... ok
test tests::divide_by_zero_panics ... ok
test tests::parse_empty_string_returns_err ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

Each line shows the test name and result. `ok` means passed, `FAILED` means the test panicked or an assertion failed.

When a test fails, cargo shows the panic message and location:

```
---- tests::add_negative stdout ----
thread 'tests::add_negative' panicked at 'assertion `left == right` failed
  left: -3
 right: 99', src/math.rs:45:9
```

---

### Code Example: Complete Unit Test Demonstration

```rust
// src/lib.rs

/// Adds two integers and returns their sum.
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Subtracts b from a.
pub fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

/// Multiplies two integers.
pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

/// Divides a by b.
///
/// # Panics
///
/// Panics if b is zero.
pub fn divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("division by zero");
    }
    a / b
}

/// Parses a string as an integer.
pub fn parse_number(s: &str) -> Result<i32, std::num::ParseIntError> {
    s.trim().parse::<i32>()
}

/// Returns the absolute value of an integer.
pub fn absolute(n: i32) -> i32 {
    if n < 0 { -n } else { n }
}

// Private helper — accessible in tests because tests module is a child
fn clamp(value: i32, min: i32, max: i32) -> i32 {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- add ---

    #[test]
    fn add_two_positive_numbers() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn add_positive_and_negative() {
        assert_eq!(add(10, -3), 7);
    }

    #[test]
    fn add_two_negatives() {
        assert_eq!(add(-4, -6), -10);
    }

    #[test]
    fn add_zeros() {
        assert_eq!(add(0, 0), 0);
    }

    // --- subtract ---

    #[test]
    fn subtract_returns_difference() {
        assert_eq!(subtract(10, 3), 7);
    }

    #[test]
    fn subtract_produces_negative() {
        assert_eq!(subtract(3, 10), -7);
    }

    // --- multiply ---

    #[test]
    fn multiply_two_positives() {
        assert_eq!(multiply(4, 5), 20);
    }

    #[test]
    fn multiply_by_zero_returns_zero() {
        assert_eq!(multiply(999, 0), 0);
    }

    #[test]
    fn multiply_two_negatives_returns_positive() {
        assert_eq!(multiply(-3, -4), 12);
    }

    // --- divide ---

    #[test]
    fn divide_evenly() {
        assert_eq!(divide(10, 2), 5);
    }

    #[test]
    fn divide_with_integer_truncation() {
        // 7 / 2 = 3 in integer division
        assert_eq!(divide(7, 2), 3);
    }

    #[test]
    #[should_panic(expected = "division by zero")]
    fn divide_by_zero_panics_with_message() {
        divide(5, 0);
    }

    // --- parse_number ---

    #[test]
    fn parse_valid_integer_string() -> Result<(), std::num::ParseIntError> {
        let n = parse_number("42")?;
        assert_eq!(n, 42);
        Ok(())
    }

    #[test]
    fn parse_negative_integer_string() -> Result<(), std::num::ParseIntError> {
        let n = parse_number("-10")?;
        assert_eq!(n, -10);
        Ok(())
    }

    #[test]
    fn parse_string_with_whitespace() -> Result<(), std::num::ParseIntError> {
        let n = parse_number("  7  ")?;
        assert_eq!(n, 7);
        Ok(())
    }

    #[test]
    fn parse_invalid_string_returns_err() {
        let result = parse_number("hello");
        assert!(result.is_err());
    }

    #[test]
    fn parse_empty_string_returns_err() {
        let result = parse_number("");
        assert!(result.is_err());
    }

    // --- absolute ---

    #[test]
    fn absolute_of_positive_is_same() {
        assert_eq!(absolute(5), 5);
    }

    #[test]
    fn absolute_of_negative_is_positive() {
        assert_eq!(absolute(-8), 8);
    }

    #[test]
    fn absolute_of_zero_is_zero() {
        assert_eq!(absolute(0), 0);
    }

    // --- clamp (private function) ---

    #[test]
    fn clamp_value_within_range_unchanged() {
        assert_eq!(clamp(5, 0, 10), 5);
    }

    #[test]
    fn clamp_below_min_returns_min() {
        assert_eq!(clamp(-5, 0, 10), 0);
    }

    #[test]
    fn clamp_above_max_returns_max() {
        assert_eq!(clamp(20, 0, 10), 10);
    }

    // --- assert_ne example ---

    #[test]
    fn add_does_not_return_wrong_value() {
        assert_ne!(add(2, 2), 5);
    }

    // --- custom message example ---

    #[test]
    fn multiply_negative_and_positive() {
        let result = multiply(-3, 4);
        assert_eq!(
            result, -12,
            "Expected -3 * 4 = -12, but got {}",
            result
        );
    }
}
```

---

### Line-by-Line Explanation

```rust
pub fn add(a: i32, b: i32) -> i32 {
```
Public function. `pub` makes it accessible from integration tests and external crates. `i32` is a 32-bit signed integer.

```rust
    a + b
```
Expression without semicolon — this is the return value. Rust functions return the last expression implicitly.

```rust
pub fn divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("division by zero");
    }
    a / b
}
```
`panic!` terminates the current thread with a message. The `#[should_panic(expected = "division by zero")]` test attribute verifies this exact string appears in the panic message.

```rust
pub fn parse_number(s: &str) -> Result<i32, std::num::ParseIntError> {
    s.trim().parse::<i32>()
}
```
Returns a `Result`. `.trim()` removes whitespace so `"  7  "` parses as `7`. `.parse::<i32>()` uses the turbofish syntax to specify the target type.

```rust
#[cfg(test)]
mod tests {
```
`#[cfg(test)]` is a **conditional compilation attribute**. This entire module is excluded from non-test builds. It does not appear in `cargo build` output. The `mod tests` name is conventional but not required — you could name it anything.

```rust
    use super::*;
```
Imports everything from the parent module (`lib.rs` or whichever file contains this `mod tests`). This includes private functions like `clamp`. Child modules can access private items of their parent.

```rust
    #[test]
    fn add_two_positive_numbers() {
        assert_eq!(add(2, 3), 5);
    }
```
`#[test]` marks this as a test function. `assert_eq!` is a macro that calls `==` on the two arguments. If they are not equal, it panics with a message showing both values.

```rust
    #[test]
    #[should_panic(expected = "division by zero")]
    fn divide_by_zero_panics_with_message() {
        divide(5, 0);
    }
```
Two attributes on one function. `#[should_panic]` inverts the pass/fail logic: if the function does NOT panic, the test fails. The `expected` parameter checks that the panic message contains the given substring.

```rust
    #[test]
    fn parse_valid_integer_string() -> Result<(), std::num::ParseIntError> {
        let n = parse_number("42")?;
        assert_eq!(n, 42);
        Ok(())
    }
```
Test returns `Result`. The `?` operator propagates errors — if `parse_number` returns `Err`, the test function returns `Err` immediately, which cargo treats as a test failure. `Ok(())` at the end signals success.

---

### Practice: Calculator Tests

This section builds tests for a calculator module step by step, demonstrating real testing workflow.

```toml
# Cargo.toml
[package]
name = "calculator"
version = "0.1.0"
edition = "2021"
```

```rust
// src/lib.rs

#[derive(Debug, PartialEq)]
pub enum CalcError {
    DivisionByZero,
    Overflow,
    InvalidInput(String),
}

impl std::fmt::Display for CalcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CalcError::DivisionByZero => write!(f, "Cannot divide by zero"),
            CalcError::Overflow => write!(f, "Arithmetic overflow"),
            CalcError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
        }
    }
}

pub fn add(a: i64, b: i64) -> Result<i64, CalcError> {
    a.checked_add(b).ok_or(CalcError::Overflow)
}

pub fn subtract(a: i64, b: i64) -> Result<i64, CalcError> {
    a.checked_sub(b).ok_or(CalcError::Overflow)
}

pub fn multiply(a: i64, b: i64) -> Result<i64, CalcError> {
    a.checked_mul(b).ok_or(CalcError::Overflow)
}

pub fn divide(a: i64, b: i64) -> Result<i64, CalcError> {
    if b == 0 {
        return Err(CalcError::DivisionByZero);
    }
    a.checked_div(b).ok_or(CalcError::Overflow)
}

pub fn power(base: i64, exp: u32) -> Result<i64, CalcError> {
    base.checked_pow(exp).ok_or(CalcError::Overflow)
}

pub fn sqrt(n: f64) -> Result<f64, CalcError> {
    if n < 0.0 {
        return Err(CalcError::InvalidInput(
            "Cannot take square root of negative number".to_string(),
        ));
    }
    Ok(n.sqrt())
}

#[cfg(test)]
mod tests {
    use super::*;

    // ---- add ----

    #[test]
    fn add_positive_numbers() {
        assert_eq!(add(10, 20), Ok(30));
    }

    #[test]
    fn add_negative_numbers() {
        assert_eq!(add(-5, -3), Ok(-8));
    }

    #[test]
    fn add_overflow_returns_err() {
        assert_eq!(add(i64::MAX, 1), Err(CalcError::Overflow));
    }

    #[test]
    fn add_zero_is_identity() {
        assert_eq!(add(42, 0), Ok(42));
        assert_eq!(add(0, 42), Ok(42));
    }

    // ---- subtract ----

    #[test]
    fn subtract_basic() {
        assert_eq!(subtract(10, 3), Ok(7));
    }

    #[test]
    fn subtract_underflow_returns_err() {
        assert_eq!(subtract(i64::MIN, 1), Err(CalcError::Overflow));
    }

    // ---- multiply ----

    #[test]
    fn multiply_basic() {
        assert_eq!(multiply(6, 7), Ok(42));
    }

    #[test]
    fn multiply_by_zero_returns_zero() {
        assert_eq!(multiply(12345, 0), Ok(0));
    }

    #[test]
    fn multiply_overflow_returns_err() {
        assert_eq!(multiply(i64::MAX, 2), Err(CalcError::Overflow));
    }

    #[test]
    fn multiply_negative_times_negative_is_positive() {
        assert_eq!(multiply(-4, -5), Ok(20));
    }

    // ---- divide ----

    #[test]
    fn divide_basic() {
        assert_eq!(divide(20, 4), Ok(5));
    }

    #[test]
    fn divide_by_zero_returns_err() {
        assert_eq!(divide(10, 0), Err(CalcError::DivisionByZero));
    }

    #[test]
    fn divide_truncates_toward_zero() {
        assert_eq!(divide(7, 2), Ok(3));
        assert_eq!(divide(-7, 2), Ok(-3));
    }

    // ---- power ----

    #[test]
    fn power_basic() {
        assert_eq!(power(2, 10), Ok(1024));
    }

    #[test]
    fn power_of_zero_is_one() {
        assert_eq!(power(5, 0), Ok(1));
    }

    #[test]
    fn power_overflow_returns_err() {
        assert_eq!(power(2, 63), Err(CalcError::Overflow));
    }

    // ---- sqrt ----

    #[test]
    fn sqrt_of_perfect_square() {
        let result = sqrt(16.0).unwrap();
        assert!((result - 4.0).abs() < 1e-10, "sqrt(16) should be 4.0, got {}", result);
    }

    #[test]
    fn sqrt_of_negative_returns_err() {
        match sqrt(-1.0) {
            Err(CalcError::InvalidInput(_)) => {}
            other => panic!("Expected InvalidInput, got {:?}", other),
        }
    }

    #[test]
    fn sqrt_of_zero() {
        assert_eq!(sqrt(0.0), Ok(0.0));
    }
}
```

---

## Mini Project: Tested Utility Library

### Project Overview

Build a utility library called `strutils` that provides string manipulation functions. The library will have:

- A well-organized source structure
- Comprehensive unit tests
- Integration tests using the public API
- Edge case coverage

### Functional Requirements

1. Count words in a string
2. Count characters (excluding whitespace)
3. Reverse a string
4. Capitalize the first letter of each word (title case)
5. Check if a string is a palindrome
6. Truncate a string to a maximum length with ellipsis
7. Count occurrences of a substring

### Project Structure

```
strutils/
├── src/
│   ├── lib.rs           -- re-exports and top-level docs
│   ├── count.rs         -- counting functions
│   ├── transform.rs     -- transformation functions
│   └── analyze.rs       -- analysis functions
├── tests/
│   ├── common/
│   │   └── mod.rs       -- shared test helpers
│   ├── count_tests.rs   -- integration tests for count module
│   ├── transform_tests.rs
│   └── analyze_tests.rs
├── benches/
│   └── string_bench.rs  -- criterion benchmarks
└── Cargo.toml
```

### Step-by-Step Development

#### Step 1: Create the Project

```bash
cargo new strutils --lib
cd strutils
```

#### Step 2: Write `Cargo.toml`

```toml
[package]
name = "strutils"
version = "0.1.0"
edition = "2021"
description = "String utility library with comprehensive tests"

[dev-dependencies]
criterion = { version = "0.5", features = ["html_reports"] }

[[bench]]
name = "string_bench"
harness = false
```

#### Step 3: Write `src/count.rs`

```rust
// src/count.rs

/// Counts the number of words in a string.
///
/// Words are sequences of non-whitespace characters separated by whitespace.
/// An empty string or a string with only whitespace returns 0.
///
/// # Examples
///
/// ```
/// use strutils::count::count_words;
/// assert_eq!(count_words("hello world"), 2);
/// assert_eq!(count_words(""), 0);
/// ```
pub fn count_words(s: &str) -> usize {
    s.split_whitespace().count()
}

/// Counts characters in a string, excluding whitespace.
///
/// # Examples
///
/// ```
/// use strutils::count::count_chars;
/// assert_eq!(count_chars("hello world"), 10);
/// ```
pub fn count_chars(s: &str) -> usize {
    s.chars().filter(|c| !c.is_whitespace()).count()
}

/// Counts the number of non-overlapping occurrences of `pattern` in `text`.
///
/// Returns 0 if `pattern` is empty or not found.
///
/// # Examples
///
/// ```
/// use strutils::count::count_occurrences;
/// assert_eq!(count_occurrences("banana", "an"), 2);
/// ```
pub fn count_occurrences(text: &str, pattern: &str) -> usize {
    if pattern.is_empty() {
        return 0;
    }
    text.matches(pattern).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    // ---- count_words ----

    #[test]
    fn count_words_simple_sentence() {
        assert_eq!(count_words("hello world"), 2);
    }

    #[test]
    fn count_words_single_word() {
        assert_eq!(count_words("hello"), 1);
    }

    #[test]
    fn count_words_empty_string() {
        assert_eq!(count_words(""), 0);
    }

    #[test]
    fn count_words_only_whitespace() {
        assert_eq!(count_words("   \t\n  "), 0);
    }

    #[test]
    fn count_words_multiple_spaces_between_words() {
        assert_eq!(count_words("a   b   c"), 3);
    }

    #[test]
    fn count_words_leading_and_trailing_spaces() {
        assert_eq!(count_words("  hello world  "), 2);
    }

    #[test]
    fn count_words_newlines_as_separators() {
        assert_eq!(count_words("line1\nline2\nline3"), 3);
    }

    #[test]
    fn count_words_tabs_as_separators() {
        assert_eq!(count_words("col1\tcol2\tcol3"), 3);
    }

    // ---- count_chars ----

    #[test]
    fn count_chars_excludes_spaces() {
        assert_eq!(count_chars("hello world"), 10);
    }

    #[test]
    fn count_chars_empty_string() {
        assert_eq!(count_chars(""), 0);
    }

    #[test]
    fn count_chars_all_spaces() {
        assert_eq!(count_chars("   "), 0);
    }

    #[test]
    fn count_chars_no_spaces() {
        assert_eq!(count_chars("hello"), 5);
    }

    #[test]
    fn count_chars_unicode() {
        // "café" has 4 chars, no spaces
        assert_eq!(count_chars("café"), 4);
    }

    // ---- count_occurrences ----

    #[test]
    fn count_occurrences_basic() {
        assert_eq!(count_occurrences("banana", "an"), 2);
    }

    #[test]
    fn count_occurrences_not_found() {
        assert_eq!(count_occurrences("hello", "xyz"), 0);
    }

    #[test]
    fn count_occurrences_empty_pattern_returns_zero() {
        assert_eq!(count_occurrences("hello", ""), 0);
    }

    #[test]
    fn count_occurrences_empty_text() {
        assert_eq!(count_occurrences("", "hi"), 0);
    }

    #[test]
    fn count_occurrences_overlapping_not_counted() {
        // "aa" in "aaa" — matches at position 0, then at position 2 is not matched
        // because matches() is non-overlapping
        assert_eq!(count_occurrences("aaa", "aa"), 1);
    }

    #[test]
    fn count_occurrences_whole_string_is_pattern() {
        assert_eq!(count_occurrences("hello", "hello"), 1);
    }

    #[test]
    fn count_occurrences_single_char_pattern() {
        assert_eq!(count_occurrences("mississippi", "s"), 4);
    }
}
```

#### Step 4: Write `src/transform.rs`

```rust
// src/transform.rs

/// Reverses the characters in a string.
///
/// Works correctly with multi-byte UTF-8 characters.
///
/// # Examples
///
/// ```
/// use strutils::transform::reverse;
/// assert_eq!(reverse("hello"), "olleh");
/// ```
pub fn reverse(s: &str) -> String {
    s.chars().rev().collect()
}

/// Converts a string to title case.
///
/// Each word's first character is uppercased; the rest are lowercased.
/// Words are separated by whitespace.
///
/// # Examples
///
/// ```
/// use strutils::transform::to_title_case;
/// assert_eq!(to_title_case("hello world"), "Hello World");
/// ```
pub fn to_title_case(s: &str) -> String {
    s.split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => {
                    let upper: String = first.to_uppercase().collect();
                    let rest: String = chars.as_str().to_lowercase();
                    upper + &rest
                }
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}

/// Truncates a string to `max_len` characters.
///
/// If the string exceeds `max_len`, it is cut at `max_len - 3` characters
/// and `"..."` is appended. If `max_len` is less than 3, behavior is to
/// return an empty string or the full string depending on length.
///
/// # Examples
///
/// ```
/// use strutils::transform::truncate;
/// assert_eq!(truncate("Hello, World!", 8), "Hello...");
/// assert_eq!(truncate("Hi", 8), "Hi");
/// ```
pub fn truncate(s: &str, max_len: usize) -> String {
    let char_count = s.chars().count();
    if char_count <= max_len {
        return s.to_string();
    }
    if max_len < 3 {
        return String::new();
    }
    let truncated: String = s.chars().take(max_len - 3).collect();
    format!("{}...", truncated)
}

#[cfg(test)]
mod tests {
    use super::*;

    // ---- reverse ----

    #[test]
    fn reverse_basic_string() {
        assert_eq!(reverse("hello"), "olleh");
    }

    #[test]
    fn reverse_empty_string() {
        assert_eq!(reverse(""), "");
    }

    #[test]
    fn reverse_single_char() {
        assert_eq!(reverse("a"), "a");
    }

    #[test]
    fn reverse_palindrome_unchanged() {
        assert_eq!(reverse("racecar"), "racecar");
    }

    #[test]
    fn reverse_with_spaces() {
        assert_eq!(reverse("hello world"), "dlrow olleh");
    }

    #[test]
    fn reverse_unicode() {
        // "café" reversed should be "éfac"
        assert_eq!(reverse("café"), "éfac");
    }

    // ---- to_title_case ----

    #[test]
    fn title_case_basic() {
        assert_eq!(to_title_case("hello world"), "Hello World");
    }

    #[test]
    fn title_case_already_uppercase() {
        assert_eq!(to_title_case("HELLO WORLD"), "Hello World");
    }

    #[test]
    fn title_case_mixed_case() {
        assert_eq!(to_title_case("hElLo WoRlD"), "Hello World");
    }

    #[test]
    fn title_case_empty_string() {
        assert_eq!(to_title_case(""), "");
    }

    #[test]
    fn title_case_single_word() {
        assert_eq!(to_title_case("rust"), "Rust");
    }

    #[test]
    fn title_case_extra_spaces_are_normalized() {
        // split_whitespace collapses multiple spaces
        assert_eq!(to_title_case("hello   world"), "Hello World");
    }

    // ---- truncate ----

    #[test]
    fn truncate_short_string_unchanged() {
        assert_eq!(truncate("Hi", 10), "Hi");
    }

    #[test]
    fn truncate_exact_length_unchanged() {
        assert_eq!(truncate("Hello", 5), "Hello");
    }

    #[test]
    fn truncate_long_string_gets_ellipsis() {
        assert_eq!(truncate("Hello, World!", 8), "Hello...");
    }

    #[test]
    fn truncate_max_len_less_than_3_returns_empty() {
        assert_eq!(truncate("Hello", 2), "");
    }

    #[test]
    fn truncate_empty_string() {
        assert_eq!(truncate("", 5), "");
    }

    #[test]
    fn truncate_max_len_exactly_3_returns_ellipsis() {
        assert_eq!(truncate("Hello", 3), "...");
    }
}
```

#### Step 5: Write `src/analyze.rs`

```rust
// src/analyze.rs

/// Returns true if the string is a palindrome.
///
/// Ignores case and non-alphanumeric characters, so "A man a plan a canal Panama"
/// is considered a palindrome.
///
/// # Examples
///
/// ```
/// use strutils::analyze::is_palindrome;
/// assert!(is_palindrome("racecar"));
/// assert!(is_palindrome("A man a plan a canal Panama"));
/// assert!(!is_palindrome("hello"));
/// ```
pub fn is_palindrome(s: &str) -> bool {
    let cleaned: String = s
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_lowercase().next().unwrap())
        .collect();

    let reversed: String = cleaned.chars().rev().collect();
    cleaned == reversed
}

/// Returns statistics about a string.
#[derive(Debug, PartialEq)]
pub struct StringStats {
    pub char_count: usize,
    pub word_count: usize,
    pub line_count: usize,
    pub unique_chars: usize,
}

/// Computes statistics about the given string.
///
/// # Examples
///
/// ```
/// use strutils::analyze::{analyze, StringStats};
/// let stats = analyze("hello world");
/// assert_eq!(stats.word_count, 2);
/// ```
pub fn analyze(s: &str) -> StringStats {
    let char_count = s.chars().filter(|c| !c.is_whitespace()).count();
    let word_count = s.split_whitespace().count();
    let line_count = if s.is_empty() { 0 } else { s.lines().count() };

    let mut seen = std::collections::HashSet::new();
    for c in s.chars().filter(|c| !c.is_whitespace()) {
        seen.insert(c);
    }
    let unique_chars = seen.len();

    StringStats {
        char_count,
        word_count,
        line_count,
        unique_chars,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ---- is_palindrome ----

    #[test]
    fn simple_palindrome() {
        assert!(is_palindrome("racecar"));
    }

    #[test]
    fn not_a_palindrome() {
        assert!(!is_palindrome("hello"));
    }

    #[test]
    fn palindrome_with_spaces_and_mixed_case() {
        assert!(is_palindrome("A man a plan a canal Panama"));
    }

    #[test]
    fn empty_string_is_palindrome() {
        assert!(is_palindrome(""));
    }

    #[test]
    fn single_char_is_palindrome() {
        assert!(is_palindrome("a"));
    }

    #[test]
    fn palindrome_with_punctuation() {
        assert!(is_palindrome("Was it a car or a cat I saw?"));
    }

    #[test]
    fn even_length_palindrome() {
        assert!(is_palindrome("abba"));
    }

    // ---- analyze ----

    #[test]
    fn analyze_basic_string() {
        let stats = analyze("hello world");
        assert_eq!(stats.char_count, 10);
        assert_eq!(stats.word_count, 2);
        assert_eq!(stats.line_count, 1);
    }

    #[test]
    fn analyze_empty_string() {
        let stats = analyze("");
        assert_eq!(stats.char_count, 0);
        assert_eq!(stats.word_count, 0);
        assert_eq!(stats.line_count, 0);
    }

    #[test]
    fn analyze_multiline_string() {
        let stats = analyze("line one\nline two\nline three");
        assert_eq!(stats.line_count, 3);
        assert_eq!(stats.word_count, 6);
    }

    #[test]
    fn analyze_unique_chars() {
        let stats = analyze("aabb");
        assert_eq!(stats.unique_chars, 2); // only 'a' and 'b'
    }
}
```

#### Step 6: Write `src/lib.rs`

```rust
// src/lib.rs

//! # strutils
//!
//! A collection of string utility functions with comprehensive test coverage.
//!
//! ## Modules
//!
//! - [`count`] — count words, characters, and occurrences
//! - [`transform`] — reverse, title-case, and truncate strings
//! - [`analyze`] — palindrome detection and string statistics

pub mod analyze;
pub mod count;
pub mod transform;
```

#### Step 7: Write Shared Test Helpers — `tests/common/mod.rs`

```rust
// tests/common/mod.rs

/// Returns a set of standard test strings used across integration tests.
pub struct TestFixtures;

impl TestFixtures {
    pub fn empty() -> &'static str {
        ""
    }

    pub fn single_word() -> &'static str {
        "hello"
    }

    pub fn simple_sentence() -> &'static str {
        "hello world"
    }

    pub fn multiline() -> &'static str {
        "line one\nline two\nline three"
    }

    pub fn with_extra_spaces() -> &'static str {
        "  hello   world  "
    }

    pub fn unicode_string() -> &'static str {
        "café résumé naïve"
    }

    pub fn long_string() -> &'static str {
        "The quick brown fox jumps over the lazy dog"
    }
}

/// Helper: assert two floats are approximately equal within epsilon.
pub fn assert_approx_eq(a: f64, b: f64, epsilon: f64) {
    assert!(
        (a - b).abs() < epsilon,
        "Values not approximately equal: {} vs {} (epsilon {})",
        a,
        b,
        epsilon
    );
}
```

#### Step 8: Write `tests/count_tests.rs`

```rust
// tests/count_tests.rs

mod common;
use common::TestFixtures;
use strutils::count::{count_chars, count_occurrences, count_words};

#[test]
fn count_words_via_public_api_empty() {
    assert_eq!(count_words(TestFixtures::empty()), 0);
}

#[test]
fn count_words_via_public_api_simple() {
    assert_eq!(count_words(TestFixtures::simple_sentence()), 2);
}

#[test]
fn count_words_via_public_api_extra_spaces() {
    // split_whitespace handles extra spaces correctly
    assert_eq!(count_words(TestFixtures::with_extra_spaces()), 2);
}

#[test]
fn count_words_multiline_counts_all_words() {
    assert_eq!(count_words(TestFixtures::multiline()), 6);
}

#[test]
fn count_chars_excludes_all_whitespace_types() {
    // tabs and newlines should also be excluded
    let s = "a\tb\nc";
    assert_eq!(count_chars(s), 3);
}

#[test]
fn count_chars_unicode_counts_code_points() {
    // "café" = c, a, f, é = 4 chars
    assert_eq!(count_chars("café"), 4);
}

#[test]
fn count_occurrences_case_sensitive() {
    // "Hello" vs "hello" — should not match
    assert_eq!(count_occurrences("Hello World", "hello"), 0);
    assert_eq!(count_occurrences("Hello World", "Hello"), 1);
}

#[test]
fn count_occurrences_in_long_string() {
    let text = TestFixtures::long_string();
    // "the" appears once (lowercase), "The" appears once (title case)
    assert_eq!(count_occurrences(text, "the"), 1);
    assert_eq!(count_occurrences(text, "The"), 1);
}
```

#### Step 9: Write `tests/transform_tests.rs`

```rust
// tests/transform_tests.rs

mod common;
use common::TestFixtures;
use strutils::transform::{reverse, to_title_case, truncate};

#[test]
fn reverse_empty_string_via_public_api() {
    assert_eq!(reverse(TestFixtures::empty()), "");
}

#[test]
fn reverse_and_reverse_again_is_identity() {
    let original = TestFixtures::simple_sentence();
    let double_reversed = reverse(&reverse(original));
    assert_eq!(double_reversed, original);
}

#[test]
fn reverse_unicode_string() {
    // Each character (including multi-byte ones) should be reversed correctly
    let result = reverse("café");
    assert_eq!(result, "éfac");
}

#[test]
fn title_case_long_string() {
    let result = to_title_case(TestFixtures::long_string());
    assert_eq!(result, "The Quick Brown Fox Jumps Over The Lazy Dog");
}

#[test]
fn title_case_unicode_preserves_structure() {
    // "café résumé" -> "Café Résumé"
    let result = to_title_case("café résumé");
    assert_eq!(result, "Café Résumé");
}

#[test]
fn truncate_long_string_via_public_api() {
    let result = truncate(TestFixtures::long_string(), 10);
    assert_eq!(result, "The quick...");
    assert_eq!(result.chars().count(), 12); // 9 chars + "..."
}

#[test]
fn truncate_at_exact_word_boundary() {
    // "hello world" truncated to 8 = "hello..." (5 chars + "...")
    let result = truncate("hello world", 8);
    assert_eq!(result, "hello...");
}
```

#### Step 10: Write `tests/analyze_tests.rs`

```rust
// tests/analyze_tests.rs

mod common;
use common::TestFixtures;
use strutils::analyze::{analyze, is_palindrome};

#[test]
fn palindrome_check_empty_via_public_api() {
    assert!(is_palindrome(TestFixtures::empty()));
}

#[test]
fn palindrome_check_long_string_not_palindrome() {
    assert!(!is_palindrome(TestFixtures::long_string()));
}

#[test]
fn analyze_long_string() {
    let stats = analyze(TestFixtures::long_string());
    // "The quick brown fox jumps over the lazy dog"
    // 9 words when split... let's count: The/quick/brown/fox/jumps/over/the/lazy/dog = 9
    assert_eq!(stats.word_count, 9);
    assert_eq!(stats.line_count, 1);
}

#[test]
fn analyze_multiline_has_correct_line_count() {
    let stats = analyze(TestFixtures::multiline());
    assert_eq!(stats.line_count, 3);
}

#[test]
fn analyze_unicode_string_counts_code_points() {
    let stats = analyze(TestFixtures::unicode_string());
    // "café résumé naïve" = 3 words
    assert_eq!(stats.word_count, 3);
}
```

#### Step 11: Write `benches/string_bench.rs`

```rust
// benches/string_bench.rs

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use strutils::analyze::{analyze, is_palindrome};
use strutils::count::{count_occurrences, count_words};
use strutils::transform::{reverse, to_title_case, truncate};

static LONG_TEXT: &str = "The quick brown fox jumps over the lazy dog. \
    Rust is a systems programming language focused on three goals: safety, speed, and concurrency. \
    It accomplishes these goals without a garbage collector, making it useful for a number of use cases \
    other languages are not good at: embedding in other languages, programs with specific space and time requirements, \
    and writing low-level code, like device drivers and operating systems.";

fn bench_count_words(c: &mut Criterion) {
    c.bench_function("count_words short", |b| {
        b.iter(|| count_words(black_box("hello world")))
    });

    c.bench_function("count_words long", |b| {
        b.iter(|| count_words(black_box(LONG_TEXT)))
    });
}

fn bench_reverse(c: &mut Criterion) {
    let sizes = [10usize, 100, 1000];
    let mut group = c.benchmark_group("reverse");

    for size in sizes {
        let input: String = "a".repeat(size);
        group.bench_with_input(BenchmarkId::from_parameter(size), &input, |b, s| {
            b.iter(|| reverse(black_box(s)))
        });
    }

    group.finish();
}

fn bench_to_title_case(c: &mut Criterion) {
    c.bench_function("to_title_case", |b| {
        b.iter(|| to_title_case(black_box(LONG_TEXT)))
    });
}

fn bench_is_palindrome(c: &mut Criterion) {
    c.bench_function("is_palindrome true", |b| {
        b.iter(|| is_palindrome(black_box("A man a plan a canal Panama")))
    });

    c.bench_function("is_palindrome false", |b| {
        b.iter(|| is_palindrome(black_box(LONG_TEXT)))
    });
}

fn bench_count_occurrences(c: &mut Criterion) {
    c.bench_function("count_occurrences", |b| {
        b.iter(|| count_occurrences(black_box(LONG_TEXT), black_box("the")))
    });
}

fn bench_analyze(c: &mut Criterion) {
    c.bench_function("analyze long text", |b| {
        b.iter(|| analyze(black_box(LONG_TEXT)))
    });
}

fn bench_truncate(c: &mut Criterion) {
    c.bench_function("truncate long text at 50", |b| {
        b.iter(|| truncate(black_box(LONG_TEXT), black_box(50)))
    });
}

criterion_group!(
    benches,
    bench_count_words,
    bench_reverse,
    bench_to_title_case,
    bench_is_palindrome,
    bench_count_occurrences,
    bench_analyze,
    bench_truncate,
);
criterion_main!(benches);
```

### Running the Full Test Suite

```bash
# Run all unit and integration tests
cargo test

# Run only unit tests (in src/)
cargo test --lib

# Run only integration tests (in tests/)
cargo test --test count_tests
cargo test --test transform_tests
cargo test --test analyze_tests

# Run a specific test by name
cargo test palindrome_check_empty

# Run all palindrome tests
cargo test palindrome

# Show output from println! in tests
cargo test -- --nocapture

# Run benchmarks
cargo bench
```

### Expected `cargo test` Output

```
running 25 tests (in src/count.rs)
test count::tests::count_words_simple_sentence ... ok
test count::tests::count_words_empty_string ... ok
...

running 18 tests (in src/transform.rs)
test transform::tests::reverse_basic_string ... ok
...

running 11 tests (in src/analyze.rs)
test analyze::tests::simple_palindrome ... ok
...

running 8 tests (in tests/count_tests.rs)
test count_words_via_public_api_empty ... ok
...

test result: ok. 62 passed; 0 failed; 0 ignored; 0 measured
```

### Code Explanation

#### Why separate `count.rs`, `transform.rs`, `analyze.rs`?

Each module has a single responsibility. `count.rs` only counts things. `transform.rs` only transforms strings. `analyze.rs` only analyzes structure. This makes it easy to find tests when something fails — a failing `count_*` test is in `count.rs`.

#### Why `tests/common/mod.rs`?

Shared fixtures prevent duplication. If the same test string is used in 10 integration tests, define it once in `common::TestFixtures`. When you want to change the fixture, you change it in one place.

#### Why `black_box` in benchmarks?

```rust
b.iter(|| reverse(black_box("hello world")))
```

Without `black_box`, the compiler might:
1. See that the input is a compile-time constant
2. Evaluate `reverse("hello world")` at compile time
3. Replace the entire loop with a no-op

`black_box` tells the optimizer: "pretend you don't know what this value is." This ensures the benchmark measures actual runtime behavior.

#### Why `criterion` instead of built-in `#[bench]`?

Built-in `#[bench]` requires nightly Rust and provides no statistical analysis. `criterion`:
- Works on stable Rust
- Runs multiple iterations and applies statistical analysis
- Detects outliers
- Generates HTML reports showing variance over time
- Can compare benchmarks before and after a change

---

### Refactoring Suggestions

1. **`to_title_case` with leading/trailing spaces**: The current implementation normalizes extra spaces because it uses `split_whitespace().collect::<Vec<_>>().join(" ")`. If preserving spacing is a requirement, you would need a different approach.

2. **`truncate` and Unicode**: The current truncation counts Unicode code points (`.chars().count()`), not bytes. This is correct for display purposes. If you need byte-level truncation (for fixed-size buffers), use `.as_bytes()` instead.

3. **`count_occurrences` and overlapping matches**: The `str::matches()` method returns non-overlapping matches. If you need overlapping matches ("aa" in "aaa" = 2), you would implement a manual sliding window.

4. **`StringStats` could be a builder**: Instead of computing all stats always, you could use a builder pattern to compute only what is needed. Useful if the string is very large and you only need word count.

5. **Error types**: The current functions return `Option` or plain values. For a production library, many of these functions could benefit from `Result<T, StrUtilsError>` with a custom error type.

---

### Challenge Exercises

1. Add a `count_sentences` function that counts sentences by counting `.`, `!`, and `?` characters. Write unit and integration tests for it, including edge cases (consecutive punctuation, ellipsis `...`).

2. Add a `wrap_at` function that wraps a string at a given column width, inserting newlines without breaking words. Write tests for long words that exceed the column width.

3. Add a `slugify` function that converts a string to a URL slug (`"Hello World!"` → `"hello-world"`). Write tests for special characters, Unicode, and consecutive special characters.

4. Modify `analyze` to also return the most frequent character. Write a test that verifies `"aabbc"` returns `'a'` or `'b'` as the most frequent (since they tie at 2 each — decide how ties should be handled and document it).

5. Write a benchmark comparing your `to_title_case` against a naive implementation that does not use `split_whitespace` (iterate character by character instead). Measure the difference.

---

### Common Mistakes

#### 1. Forgetting `#[cfg(test)]` on the test module

```rust
// Wrong — test code compiled into release binary
mod tests {
    #[test]
    fn it_works() { ... }
}

// Correct
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() { ... }
}
```

Without `#[cfg(test)]`, the `tests` module is included in your production binary. It will still compile, but it adds dead code to the release build.

#### 2. Not importing the module under test

```rust
#[cfg(test)]
mod tests {
    // Forgot: use super::*;
    
    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5); // Error: cannot find function `add`
    }
}
```

Fix: add `use super::*;` as the first line of the `tests` module.

#### 3. Testing only the happy path

```rust
// Only testing what works
#[test]
fn test_divide() {
    assert_eq!(divide(10, 2), Ok(5));
}

// Should also test edge cases
#[test]
fn test_divide_by_zero() { ... }

#[test]
fn test_divide_negative_numbers() { ... }

#[test]
fn test_divide_with_overflow() { ... }
```

Always ask: what happens at the boundaries? What inputs cause different code paths?

#### 4. Using floating-point equality directly

```rust
// Wrong — floating-point arithmetic is imprecise
assert_eq!(sqrt(2.0), 1.4142135623730951);

// Correct — check within an acceptable epsilon
let result = sqrt(2.0).unwrap();
assert!((result - 1.4142135623730951).abs() < 1e-10);
```

#### 5. Making tests depend on each other

```rust
static mut COUNTER: i32 = 0;

#[test]
fn test_first() {
    unsafe { COUNTER = 5; }
}

#[test]
fn test_second() {
    // Wrong: assumes test_first ran first
    // cargo test runs tests in parallel by default
    unsafe { assert_eq!(COUNTER, 5); }
}
```

Tests run in parallel by default. Each test must be completely independent. If you need sequential tests, run with `--test-threads=1`, but redesign the tests so they do not share global mutable state.

#### 6. Writing integration tests without making functions `pub`

```rust
// src/lib.rs
fn internal_function() -> i32 { 42 } // private!

// tests/integration_test.rs
use my_crate::internal_function; // Error: not accessible
```

Integration tests are in a separate crate. They can only access `pub` items. If you need to test private behavior from an integration test, that is a sign the behavior should be tested at the unit test level (inside `src/`) instead.

#### 7. Confusing `assert!` with `assert_eq!`

```rust
// Bad: only tells you "false != true", no values shown
assert!(add(2, 3) == 5);

// Good: on failure, shows you left=4 right=5
assert_eq!(add(2, 3), 5);
```

Always prefer `assert_eq!` and `assert_ne!` over `assert!(a == b)` because they print the actual values on failure.

---

### Best Practices

1. **Test one thing per test function.** A test named `test_user_creation` that tests creation, validation, AND serialization is three tests masquerading as one.

2. **Name tests as specifications.** `returns_err_when_dividing_by_zero` tells you what the behavior should be. `test_divide3` does not.

3. **Test edge cases systematically.** For any function, consider: empty input, single-element input, maximum/minimum values, null/None, negative numbers, Unicode, very long strings.

4. **Keep tests fast.** Unit tests should run in milliseconds. If a test needs a real database or network, it is an integration test and should be labeled accordingly (or placed behind `#[ignore]`).

5. **Use `Result`-returning tests for functions that return `Result`.** The `?` operator makes these tests concise and readable.

6. **Do not use `unwrap()` in tests** without `#[should_panic]`. Use `assert!(result.is_ok())` or return `Result` from the test.

7. **Separate unit tests from integration tests conceptually.** Unit tests verify that individual functions work. Integration tests verify that your public API works correctly together.

8. **Run tests before committing.** Make `cargo test` part of your workflow. Set up a CI pipeline that runs `cargo test` automatically.

9. **Use `cargo test -- --nocapture` during debugging** to see `println!` output from failing tests.

10. **Benchmark before optimizing.** Do not guess where your code is slow. Measure with `criterion`, optimize, then measure again.

---

### Exercises

**Exercise 1**

Write a function `is_anagram(a: &str, b: &str) -> bool` that returns `true` if the two strings are anagrams of each other (contain the same characters, case-insensitive, ignoring spaces). Write at least 6 unit tests covering:
- Two words that are anagrams
- Two words that are not anagrams
- Empty strings
- Strings that are the same word
- Strings with different lengths
- Case-insensitive matching

**Exercise 2**

Write a function `compress(s: &str) -> String` that performs run-length encoding: `"aaabbc"` → `"a3b2c1"`. If the compressed form is longer than the original, return the original. Write unit tests for:
- A string with repeating characters
- A string with no repeating characters (should return original)
- Empty string
- Single character
- All same character

**Exercise 3**

Write a function `find_duplicates(words: &[&str]) -> Vec<String>` that returns all words that appear more than once, in the order they first appear. Write unit tests for:
- A slice with duplicates
- A slice with no duplicates
- An empty slice
- A slice where all words are duplicates
- Case sensitivity (is `"Hello"` the same as `"hello"`?)

**Exercise 4**

Write integration tests (in `tests/`) for a `caesar_cipher` module that provides:
- `encrypt(text: &str, shift: u8) -> String`
- `decrypt(text: &str, shift: u8) -> String`

The tests should verify that `decrypt(encrypt(text, shift), shift) == text` for several inputs.

**Exercise 5**

Add a benchmark (using `criterion`) that compares two implementations of string reversal:
1. Using `.chars().rev().collect()`
2. Using a manual byte swap (only correct for ASCII)

Measure both on a 1000-character ASCII string and report the difference.

---

### Solutions

**Solution 1: `is_anagram`**

```rust
pub fn is_anagram(a: &str, b: &str) -> bool {
    let normalize = |s: &str| -> Vec<char> {
        let mut chars: Vec<char> = s
            .chars()
            .filter(|c| !c.is_whitespace())
            .map(|c| c.to_lowercase().next().unwrap())
            .collect();
        chars.sort_unstable();
        chars
    };
    normalize(a) == normalize(b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn listen_is_anagram_of_silent() {
        assert!(is_anagram("listen", "silent"));
    }

    #[test]
    fn hello_is_not_anagram_of_world() {
        assert!(!is_anagram("hello", "world"));
    }

    #[test]
    fn empty_strings_are_anagrams() {
        assert!(is_anagram("", ""));
    }

    #[test]
    fn same_word_is_anagram_of_itself() {
        assert!(is_anagram("rust", "rust"));
    }

    #[test]
    fn different_lengths_are_not_anagrams() {
        assert!(!is_anagram("abc", "ab"));
    }

    #[test]
    fn case_insensitive() {
        assert!(is_anagram("Listen", "Silent"));
    }
}
```

**Solution 2: `compress`**

```rust
pub fn compress(s: &str) -> String {
    if s.is_empty() {
        return String::new();
    }

    let mut result = String::new();
    let mut chars = s.chars().peekable();
    let mut count = 1usize;
    let mut current = chars.next().unwrap();

    for ch in chars {
        if ch == current {
            count += 1;
        } else {
            result.push(current);
            result.push_str(&count.to_string());
            current = ch;
            count = 1;
        }
    }
    result.push(current);
    result.push_str(&count.to_string());

    if result.len() >= s.len() {
        s.to_string()
    } else {
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compress_repeating_chars() {
        assert_eq!(compress("aaabbc"), "a3b2c1");
    }

    #[test]
    fn compress_no_repeats_returns_original() {
        assert_eq!(compress("abcd"), "abcd");
    }

    #[test]
    fn compress_empty_string() {
        assert_eq!(compress(""), "");
    }

    #[test]
    fn compress_single_char() {
        assert_eq!(compress("a"), "a");
    }

    #[test]
    fn compress_all_same_char() {
        assert_eq!(compress("aaaaa"), "a5");
    }
}
```

**Solution 3: `find_duplicates`**

```rust
pub fn find_duplicates<'a>(words: &[&'a str]) -> Vec<String> {
    use std::collections::{HashMap, HashSet};

    let mut counts: HashMap<&str, usize> = HashMap::new();
    for &word in words {
        *counts.entry(word).or_insert(0) += 1;
    }

    let mut seen: HashSet<&str> = HashSet::new();
    let mut result: Vec<String> = Vec::new();

    for &word in words {
        if counts[word] > 1 && seen.insert(word) {
            result.push(word.to_string());
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_duplicates_in_order() {
        let words = vec!["apple", "banana", "apple", "cherry", "banana"];
        assert_eq!(find_duplicates(&words), vec!["apple", "banana"]);
    }

    #[test]
    fn no_duplicates_returns_empty() {
        let words = vec!["a", "b", "c"];
        assert!(find_duplicates(&words).is_empty());
    }

    #[test]
    fn empty_slice_returns_empty() {
        let words: Vec<&str> = vec![];
        assert!(find_duplicates(&words).is_empty());
    }

    #[test]
    fn all_duplicates() {
        let words = vec!["x", "x", "x"];
        assert_eq!(find_duplicates(&words), vec!["x"]);
    }

    #[test]
    fn case_sensitive() {
        let words = vec!["Hello", "hello"];
        assert!(find_duplicates(&words).is_empty());
    }
}
```

**Solution 4: Caesar cipher integration test (abbreviated structure)**

```rust
// src/cipher.rs
pub fn encrypt(text: &str, shift: u8) -> String {
    text.chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let base = if c.is_uppercase() { b'A' } else { b'a' };
                let shifted = (c as u8 - base + shift) % 26 + base;
                shifted as char
            } else {
                c
            }
        })
        .collect()
}

pub fn decrypt(text: &str, shift: u8) -> String {
    encrypt(text, 26 - (shift % 26))
}
```

```rust
// tests/cipher_tests.rs
use strutils::cipher::{decrypt, encrypt};

#[test]
fn encrypt_then_decrypt_is_identity() {
    let original = "Hello, World!";
    let encrypted = encrypt(original, 13);
    let decrypted = decrypt(&encrypted, 13);
    assert_eq!(decrypted, original);
}

#[test]
fn shift_zero_is_identity() {
    let text = "Rust";
    assert_eq!(encrypt(text, 0), text);
}

#[test]
fn shift_26_wraps_around() {
    let text = "Hello";
    assert_eq!(encrypt(text, 26), text);
}
```

**Solution 5: Benchmark (benches/reverse_bench.rs)**

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn reverse_chars(s: &str) -> String {
    s.chars().rev().collect()
}

fn reverse_bytes_ascii(s: &str) -> String {
    // Only correct for ASCII
    let mut bytes = s.as_bytes().to_vec();
    bytes.reverse();
    String::from_utf8(bytes).unwrap()
}

fn bench_reverse_methods(c: &mut Criterion) {
    let input: String = "abcdefghij".repeat(100); // 1000 chars, ASCII only

    let mut group = c.benchmark_group("reverse_comparison");

    group.bench_function("chars_rev", |b| {
        b.iter(|| reverse_chars(black_box(&input)))
    });

    group.bench_function("bytes_reverse_ascii", |b| {
        b.iter(|| reverse_bytes_ascii(black_box(&input)))
    });

    group.finish();
}

criterion_group!(benches, bench_reverse_methods);
criterion_main!(benches);
```

---

### Quiz

**Question 1**

What does `#[cfg(test)]` do and why is it important?

**Question 2**

What is the difference between `assert!`, `assert_eq!`, and `assert_ne!`? When should you use each?

**Question 3**

Why can unit tests in a `#[cfg(test)] mod tests` block access private functions of the parent module?

**Question 4**

You have a function that is supposed to panic when given invalid input. How do you write a test that verifies it panics with a specific message?

**Question 5**

What is the difference between unit tests (in `src/`) and integration tests (in `tests/`)? What can each access?

**Question 6**

Why do integration tests not need `#[cfg(test)]`?

**Question 7**

What is trait-based mocking and why is it the idiomatic approach to mocking in Rust?

**Question 8**

What does `black_box` do in a criterion benchmark and why is it necessary?

**Question 9**

What command would you use to run only tests whose names contain the word "palindrome"?

**Question 10**

You write a test for floating-point equality like this:

```rust
assert_eq!(sqrt(2.0), 1.4142135623730951);
```

What is wrong with this approach, and how would you fix it?

---

### Quiz Answers

**Answer 1**

`#[cfg(test)]` is a conditional compilation attribute. It tells the Rust compiler to include the annotated item (usually a `mod tests` block) only when compiling in test mode (`cargo test`). Without it, test code would be included in release builds, increasing binary size and potentially exposing test-only dependencies.

**Answer 2**

- `assert!(expr)` — passes if `expr` is `true`. Use for boolean conditions that are not comparisons.
- `assert_eq!(a, b)` — passes if `a == b`. Use when comparing two values. On failure, prints both values.
- `assert_ne!(a, b)` — passes if `a != b`. Use when verifying two values are different. On failure, prints both values.

Prefer `assert_eq!` over `assert!(a == b)` because it shows both values in the failure output, making debugging much easier.

**Answer 3**

In Rust, a child module can access private items of its parent module. The `mod tests` block inside a source file is a child module of that file's module. Therefore, it can see everything in the parent, including private functions and types. This is intentional — it allows unit tests to test internal implementation details without making them `pub`.

**Answer 4**

Use `#[should_panic(expected = "message")]`:

```rust
#[test]
#[should_panic(expected = "division by zero")]
fn test_division_panics() {
    divide(10, 0);
}
```

The `expected` parameter checks that the panic message contains the given string. The test fails if the function does NOT panic, or if it panics with a message that does not contain the expected substring.

**Answer 5**

Unit tests (`src/`) live in the same module as the code they test. They can access private functions via the child module visibility rule. They use `#[cfg(test)] mod tests { use super::*; }`.

Integration tests (`tests/`) are compiled as separate crates. They can only access items marked `pub` in your library. They do not need `#[cfg(test)]` because the entire `tests/` directory is only compiled during `cargo test`.

**Answer 6**

Because the `tests/` directory itself is only compiled when running `cargo test`. Cargo knows these are test files from their location. The `#[cfg(test)]` attribute would be redundant — Cargo never includes `tests/` in a normal build.

**Answer 7**

Rust's type system prevents swapping concrete types at runtime. Instead, you design your code to depend on traits (interfaces) rather than concrete types. In production, you pass a real implementation. In tests, you pass a fake implementation (mock) that returns controlled values. This approach requires designing for testability upfront but results in more modular, loosely-coupled code. The `mockall` crate can automate the generation of mock implementations.

**Answer 8**

`black_box(value)` is a hint to the optimizer that pretends the value might change at runtime, preventing the compiler from constant-folding or eliminating the computation being benchmarked. Without it, the compiler might detect that the benchmark result is never used and optimize the entire measurement loop away, making the benchmark measure nothing.

**Answer 9**

```bash
cargo test palindrome
```

`cargo test` accepts a substring filter. Any test function whose fully-qualified name contains "palindrome" will be run.

**Answer 10**

Floating-point arithmetic is imprecise. The computed result of `sqrt(2.0)` may differ from the literal `1.4142135623730951` by a tiny amount due to rounding errors at the hardware level. The `==` comparison checks for exact bit-for-bit equality, which will fail even if the result is "correct enough."

The fix:

```rust
let result = sqrt(2.0).unwrap();
assert!(
    (result - 1.4142135623730951_f64).abs() < 1e-10,
    "Expected approximately 1.414..., got {}",
    result
);
```

Check that the absolute difference is within an acceptable epsilon (tolerance).

---

## Chapter Summary

In this chapter, you learned how Rust makes testing a first-class citizen by building it into Cargo and the language itself.

**Unit Testing** uses the `#[test]` attribute to mark functions as tests. The `#[cfg(test)]` attribute ensures test code is only compiled during `cargo test`. The assertion macros — `assert!`, `assert_eq!`, `assert_ne!` — verify behavior and print useful diagnostics on failure. `#[should_panic]` tests that functions panic correctly. Tests can return `Result<(), E>` to use the `?` operator.

**Integration Testing** uses the `tests/` directory. Each file there is a separate crate that uses your library's public API. This mirrors how real users interact with your code. Shared fixtures belong in `tests/common/mod.rs`.

**Test Organization** follows the convention of a `#[cfg(test)] mod tests` block at the bottom of each source file. Child modules can access private functions of their parent, which is why unit tests can test private implementation details.

**Mocking** in Rust relies on traits. Design your code to depend on traits rather than concrete types. In tests, substitute a fake implementation. The `mockall` crate automates mock generation for complex scenarios.

**Benchmarking** uses the `criterion` crate on stable Rust. Write benchmarks in `benches/`, configure `[[bench]]` with `harness = false`, and use `black_box` to prevent compiler optimization of your measurements.

**Key commands:**

| Command | Purpose |
|---|---|
| `cargo test` | Run all tests |
| `cargo test pattern` | Run tests matching pattern |
| `cargo test --lib` | Run only unit tests |
| `cargo test --test file` | Run specific integration test file |
| `cargo test -- --nocapture` | Show println! output |
| `cargo test -- --ignored` | Run ignored tests |
| `cargo bench` | Run criterion benchmarks |

The goal of testing is not to achieve 100% coverage for its own sake. It is to build confidence that your code behaves correctly — including at edge cases, under invalid input, and when dependencies fail. Well-tested Rust code is one of the clearest signals of production readiness.
