# Chapter 1: Introduction to Rust

## Learning Objectives

By the end of this chapter, you will:

- Understand what Rust is and where it came from
- Know why Rust was created and what problems it solves
- Understand the Rust ecosystem and its key tools
- Have Rust installed and verified on your machine
- Be able to create, build, and run a Rust project with Cargo
- Understand what memory safety without a garbage collector means

---

## Theory

### 1.1 What is Rust?

Rust is a systems programming language focused on three goals: **performance**, **reliability**, and **productivity**. It was created to fill a gap that no other language had successfully filled: writing fast, low-level code that is also provably memory-safe.

#### History of Rust

| Year | Event |
|------|-------|
| 2006 | Graydon Hoare starts Rust as a personal side project at Mozilla |
| 2009 | Mozilla officially sponsors the project |
| 2010 | Rust announced publicly at Mozilla Summit |
| 2012 | Rust 0.1 pre-alpha released |
| 2014 | Rust 1.0 alpha released |
| 2015 | **Rust 1.0 stable released** — May 15, 2015 |
| 2021 | Rust Foundation formed (AWS, Google, Microsoft, Mozilla, Huawei) |
| 2022 | Linux kernel officially accepts Rust as a second language |
| 2023 | US government recommends Rust for memory-safe software |
| 2024 | Rust appears in the Windows kernel |

#### Why Rust Was Created

Mozilla needed to rewrite Firefox's rendering engine, called **Servo**, to:

1. Take advantage of modern multi-core CPUs
2. Eliminate entire classes of security bugs found in C++
3. Match C/C++ performance without a garbage collector

The problem Mozilla faced is the same problem the entire systems programming world faces:

```
The Fundamental Dilemma:

  C / C++         ──→  Fast + Low-level  +  UNSAFE
  Java / Python   ──→  Safe + Easy       +  SLOW (garbage collector)

  Nobody had:         Fast + Low-level  +  SAFE
```

Rust's answer: enforce memory safety rules **at compile time**, not at runtime. No garbage collector needed. The compiler checks your memory usage before your program ever runs.

#### Memory Safety Without Garbage Collection

Most languages handle memory in one of two ways:

**Manual management (C, C++):**
- You call `malloc` / `free` (C) or `new` / `delete` (C++)
- You are responsible for every allocation and deallocation
- If you forget to free → memory leak
- If you free twice → crash or security exploit
- If you use after free → undefined behavior (dangerous)

**Garbage collector (Java, Python, Go, C#):**
- A background process periodically scans memory
- Automatically frees memory no longer in use
- Safe, but introduces unpredictable pauses (GC pauses)
- Uses more memory (objects live longer than needed)
- Not suitable for real-time systems or embedded devices

**Rust's approach: Ownership System**
- The compiler tracks who owns every value
- Memory is freed automatically when the owner goes out of scope
- The compiler **rejects programs** that would cause memory bugs
- Zero runtime overhead — no GC, no reference counting by default
- No pauses, no overhead, fully deterministic memory management

This is enforced through three concepts you will master in Part 2:
- **Ownership** — every value has exactly one owner
- **Borrowing** — temporary references without ownership transfer
- **Lifetimes** — compile-time proof that references are valid

#### The Rust Ecosystem

| Tool | Purpose |
|------|---------|
| `rustc` | The Rust compiler — converts `.rs` source to binary |
| `cargo` | Build system + package manager (like npm + make combined) |
| `rustup` | Toolchain manager — installs/updates Rust versions |
| `crates.io` | Official package registry (like npm registry for Rust) |
| `docs.rs` | Automatic documentation hosting for all crates |
| `rustfmt` | Automatic code formatter |
| `clippy` | Linter — catches common mistakes and style issues |
| `rust-analyzer` | Language server for IDE integration |

---

### 1.2 Why Learn Rust?

#### Performance

Rust compiles directly to native machine code. There is no virtual machine, no interpreter, no JIT compiler, and no garbage collector. Rust programs run at the same speed as equivalent C and C++ programs.

```
Approximate Performance Comparison (lower is better):

C            ████ 1.0x  (baseline)
C++          ████ 1.0x
Rust         ████ 1.0x - 1.05x
Go           ██████ 1.5x - 2x
Java         ████████ 2x - 3x
C# (.NET)    ████████ 2x - 3x
JavaScript   ████████████ 3x - 10x
Python       ████████████████████████████████████████ 20x - 100x
```

This makes Rust suitable for:
- Operating system kernels
- Game engines
- Database engines
- Embedded systems
- WebAssembly (fastest WASM language)
- Anywhere C/C++ would be used

#### Safety

Microsoft reported in 2019 that ~70% of CVEs (security vulnerabilities) in their products were memory safety bugs. Google reported similar numbers for Chrome. Rust eliminates this entire category of bugs.

Rust prevents at compile time:
- **Null pointer dereferences** — use `Option<T>` instead of null
- **Buffer overflows** — bounds checking enforced
- **Use-after-free** — ownership system prevents this
- **Double-free** — ownership system prevents this
- **Data races** — type system prevents concurrent mutations

#### Concurrency

Rust's ownership system makes concurrent programming safe. The compiler prevents data races — two threads writing to the same memory simultaneously — at **compile time**. This is what Rustaceans call **"Fearless Concurrency"**.

In C++, you can write a data race and your program will compile and run, but produce incorrect results or crash unpredictably. In Rust, the program **won't compile** if you introduce a data race.

#### Modern Tooling

- **Cargo** handles everything: building, testing, documentation, publishing, dependency management
- **Integrated test framework** — write tests right next to your code
- **Excellent error messages** — Rust's compiler gives the most helpful error messages of any language
- **Strong type inference** — you rarely need to write types explicitly

#### Where Rust is Used in Production

| Company/Project | Usage |
|----------------|-------|
| Mozilla Firefox | CSS engine (Stylo), WebAssembly runtime |
| AWS | Firecracker (VM hypervisor for Lambda) |
| Microsoft | Azure IoT, Windows kernel components |
| Google | Android kernel drivers, Chromium |
| Cloudflare | Proxies and networking |
| Discord | Replaced Go services, reduced CPU/latency |
| Dropbox | Storage backend |
| Linux Kernel | Device drivers (official since 6.1) |
| npm | Package registry backend |

---

### 1.3 Installing Rust

#### Installing via rustup

`rustup` is the official Rust toolchain installer and version manager. It installs `rustc`, `cargo`, and the standard library, and lets you manage multiple Rust versions.

**Linux / macOS:**

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

This downloads and runs the installer. Follow the on-screen prompts. Choose option 1 (default installation) unless you have specific requirements.

**Windows:**

Download `rustup-init.exe` from https://rustup.rs and run it.

You may also need the Microsoft C++ Build Tools (Visual Studio Build Tools) for the MSVC toolchain.

#### After Installation

The installer adds Cargo's bin directory to your `PATH`. Restart your terminal (or run `source ~/.cargo/env` on Linux/macOS), then verify:

```bash
rustc --version
# rustc 1.78.0 (9b00956e5 2024-04-29)  ← you'll see the current version

cargo --version
# cargo 1.78.0 (54d8815d0 2024-03-26)

rustup --version
# rustup 1.27.0 (2024-03-08)
```

#### What Gets Installed

```
~/.cargo/
├── bin/
│   ├── rustc       ← compiler
│   ├── cargo       ← build system / package manager
│   ├── rustup      ← toolchain manager
│   ├── rustfmt     ← code formatter
│   └── clippy      ← linter (cargo clippy)
└── ...

~/.rustup/
├── toolchains/     ← installed Rust versions
└── ...
```

#### Toolchains

Rust has three release channels:

| Channel | Description | Use Case |
|---------|-------------|----------|
| `stable` | Fully tested, released every 6 weeks | Production code (recommended) |
| `beta` | Testing ground for next stable | Testing upcoming features |
| `nightly` | Bleeding edge, may have bugs | Experimental features |

```bash
# Update Rust to the latest stable
rustup update

# Install a specific toolchain
rustup install nightly

# Switch the default to nightly
rustup default nightly

# Use nightly only in one specific project directory
rustup override set nightly

# See all installed toolchains
rustup toolchain list
```

#### Opening the Offline Docs

```bash
rustup doc          # opens The Rust Book
rustup doc --std    # opens standard library documentation
```

This is extremely useful — the official Rust Book is available offline after installation.

---

### 1.4 Understanding Cargo

Cargo is Rust's build system and package manager. Every Rust project is a Cargo project. Cargo handles:

- Creating new projects with the correct structure
- Building your code
- Running your code
- Running tests
- Managing external dependencies (called **crates**)
- Publishing your own crates to crates.io

#### Creating a New Project

```bash
# Create a binary (executable) project
cargo new hello_world

# Create a library project
cargo new my_library --lib

# Create a project in the current directory
cargo init
```

#### Project Structure

```
hello_world/
├── Cargo.toml    ← project manifest (metadata + dependencies)
├── Cargo.lock    ← exact dependency versions (auto-generated)
└── src/
    └── main.rs   ← source code entry point
```

#### Cargo.toml

The `Cargo.toml` file is the project manifest. It describes your project and its dependencies.

```toml
[package]
name = "hello_world"       # project name
version = "0.1.0"          # semantic version
edition = "2021"           # Rust edition (2015, 2018, or 2021)

[dependencies]
# external crates (libraries) go here
# example:
# rand = "0.8"
# serde = { version = "1.0", features = ["derive"] }
```

**Rust Editions** are a way to introduce breaking changes without breaking old code. Edition 2021 is the current standard. When you create a new project with `cargo new`, it defaults to the latest edition automatically.

#### Key Cargo Commands

```bash
cargo new <name>       # create a new project
cargo build            # compile in debug mode
cargo build --release  # compile in release mode (optimized)
cargo run              # compile and run
cargo run --release    # compile and run with optimizations
cargo check            # check for errors WITHOUT producing binary (fast)
cargo test             # run all tests
cargo doc              # generate HTML documentation from code comments
cargo doc --open       # generate and open docs in browser
cargo fmt              # format all source files
cargo clippy           # run the linter
cargo add <crate>      # add a dependency to Cargo.toml
cargo update           # update dependencies to latest compatible versions
cargo clean            # delete the target/ build directory
```

#### Debug vs Release Builds

```bash
cargo build           # debug: fast compile, slow binary, includes debug info
cargo build --release # release: slow compile, fast binary, optimized
```

| Aspect | Debug | Release |
|--------|-------|---------|
| Compile time | Fast (seconds) | Slow (minutes for large projects) |
| Binary speed | Slow | Fast (can be 10-100x faster) |
| Binary size | Large | Small (after stripping) |
| Overflow checks | Yes (panics) | No (wraps) |
| Debug symbols | Included | Not included |
| Use when | Developing | Deploying / benchmarking |

The built binary is placed in:
- Debug: `target/debug/<name>`
- Release: `target/release/<name>`

#### Adding Dependencies

Dependencies are called **crates** in Rust. You can find them at [crates.io](https://crates.io).

```bash
# Add a dependency using cargo add (cargo 1.62+)
cargo add rand

# Or add manually to Cargo.toml
```

```toml
[dependencies]
rand = "0.8"
```

Then run `cargo build` — Cargo automatically downloads and compiles the dependency.

---

## Code Example

### Mini Project: CLI Hello World

This project goes beyond a simple `println!`. We'll build a greeting CLI that reads the user's name and prints a personalized welcome message.

```rust
use std::io;
use std::io::Write;

fn main() {
    print!("Enter your name: ");
    io::stdout().flush().unwrap();

    let mut name = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    let name = name.trim();

    if name.is_empty() {
        println!("Hello, stranger!");
    } else {
        println!("Hello, {}! Welcome to Rust.", name);
        println!("You have {} characters in your name.", name.len());
    }
}
```

### Line-by-Line Explanation

```rust
use std::io;
```
- `use` brings a module into scope so you can refer to it without the full path
- `std::io` is the standard library's input/output module
- Without this, you'd have to write `std::io::stdin()` every time

```rust
use std::io::Write;
```
- The `Write` trait must be in scope to call `.flush()` on stdout
- This is a Rust pattern: traits must be imported to use their methods

```rust
fn main() {
```
- Every Rust binary starts execution at `main()`
- `fn` is the keyword for function declaration
- `main` takes no parameters and returns nothing (implicitly returns `()`)

```rust
    print!("Enter your name: ");
```
- `print!` prints without a newline (unlike `println!` which adds `\n`)
- The `!` means it's a **macro**, not a regular function
- Macros in Rust have special compile-time expansion behavior

```rust
    io::stdout().flush().unwrap();
```
- Terminal output is **buffered** by default — text is held in a buffer before printing
- `flush()` forces the buffer to actually write to the terminal now
- Without this, the prompt might not appear before the user starts typing
- `.unwrap()` — `flush()` returns a `Result` (can fail); `.unwrap()` says "I expect success; panic if failure"

```rust
    let mut name = String::new();
```
- `let` declares a variable
- `mut` makes it mutable (by default, Rust variables are **immutable**)
- `String::new()` creates a new, empty, heap-allocated string
- We need mutability because `read_line` will write into this string

```rust
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");
```
- `io::stdin()` gets a handle to standard input
- `.read_line(&mut name)` reads a line from the user and **appends** it to `name`
- `&mut name` is a mutable reference — we're lending `name` to `read_line` temporarily
- `.expect("...")` — like `.unwrap()` but shows a custom message on failure
- `read_line` includes the newline character `\n` in the string

```rust
    let name = name.trim();
```
- **Shadowing**: we declare a new `name` variable that shadows (replaces) the old one
- `.trim()` removes leading and trailing whitespace, including the `\n` from `read_line`
- This new `name` is of type `&str` (a string slice), not `String`

```rust
    if name.is_empty() {
```
- `.is_empty()` returns `true` if the string has zero length
- `if` in Rust does NOT require parentheses around the condition (unlike C/Java)

```rust
        println!("Hello, {}! Welcome to Rust.", name);
```
- `{}` is the format placeholder — replaced with the value of `name`
- `println!` always ends with a newline

```rust
        println!("You have {} characters in your name.", name.len());
```
- `.len()` returns the number of **bytes** (for ASCII characters = number of characters)
- For Unicode, use `.chars().count()` to get the actual character count

---

## Common Mistakes

### Mistake 1: Forgetting `mut`

```rust
// WRONG — this will not compile
let name = String::new();
io::stdin().read_line(&name).unwrap(); // ERROR: cannot borrow as mutable

// CORRECT
let mut name = String::new();
io::stdin().read_line(&mut name).unwrap();
```

Rust's error message will say: `cannot borrow 'name' as mutable, as it is not declared as mutable`. This is clear — add `mut`.

### Mistake 2: Not trimming the newline

```rust
let mut input = String::new();
io::stdin().read_line(&mut input).unwrap();

// input is now "hello\n" — includes the newline!
// Comparisons will fail:
if input == "hello" { // false! because input is "hello\n"
    println!("Match!");
}

// Fix: trim before comparing
let input = input.trim();
if input == "hello" { // now works
    println!("Match!");
}
```

### Mistake 3: Confusing `print!` and `println!`

```rust
print!("Enter name: ");   // NO newline — cursor stays on same line
println!("Enter name: "); // WITH newline — cursor moves to next line
```

### Mistake 4: Ignoring the `flush()` requirement

```rust
// This might not show the prompt before the user types
print!("Enter name: ");
// cursor might be on a new line, prompt might appear after input

// Always flush when using print! before reading input
print!("Enter name: ");
io::stdout().flush().unwrap(); // now the prompt is guaranteed to show
```

---

## Best Practices

1. **Always use `cargo new`** to create projects — never create files manually
2. **Use `cargo check` frequently** during development — it's faster than `cargo build` and catches errors early
3. **Use `cargo clippy`** before committing — it catches common mistakes
4. **Use `cargo fmt`** to keep code consistently formatted
5. **Use `.expect("message")` over `.unwrap()`** in application code — better error messages
6. **Use `--release` only when benchmarking or deploying** — debug builds are faster to compile

---

## Exercises

### Exercise 1: Hello, World

Create a new Cargo project called `hello_rust`. Write a program that prints "Hello, Rust!" to the terminal.

### Exercise 2: Personalized Greeting

Extend the mini project to also print the length of the name and whether it has more than 5 characters.

### Exercise 3: Cargo Exploration

Run the following commands on your project and observe what each produces:
1. `cargo build`
2. `cargo check`
3. `cargo doc --open`
4. Find the compiled binary and run it directly

### Exercise 4: Multi-line Output

Print the following pattern using multiple `println!` calls:
```
*
**
***
****
*****
```

---

## Solutions

### Solution 1

```bash
cargo new hello_rust
cd hello_rust
```

```rust
// src/main.rs
fn main() {
    println!("Hello, Rust!");
}
```

```bash
cargo run
```

### Solution 2

```rust
use std::io;
use std::io::Write;

fn main() {
    print!("Enter your name: ");
    io::stdout().flush().unwrap();

    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read");
    let name = name.trim();

    if name.is_empty() {
        println!("Hello, stranger!");
    } else {
        println!("Hello, {}!", name);
        println!("Your name has {} characters.", name.len());

        if name.len() > 5 {
            println!("That's a long name!");
        } else {
            println!("That's a short name!");
        }
    }
}
```

### Solution 3

After `cargo build`, the binary is at `./target/debug/hello_rust`. Run it with:
```bash
./target/debug/hello_rust
```

### Solution 4

```rust
fn main() {
    println!("*");
    println!("**");
    println!("***");
    println!("****");
    println!("*****");
}
```

---

## Quiz

**Q1.** What is the main innovation that allows Rust to be both fast and memory-safe?

a) A very fast garbage collector  
b) The ownership system enforced at compile time  
c) Manual memory management with smart pointers  
d) A reference counting runtime  

**Q2.** What does `cargo check` do differently from `cargo build`?

a) It runs tests  
b) It compiles with optimizations  
c) It checks for errors without producing a binary (faster)  
d) It checks formatting  

**Q3.** Why must variables be declared with `mut` to be modifiable in Rust?

a) It's a performance optimization  
b) It prevents accidental mutations and makes intent explicit  
c) The compiler requires it for garbage collection  
d) It's required for thread safety only  

**Q4.** What does `.trim()` do when called on a String read with `read_line`?

a) Removes all spaces inside the string  
b) Removes leading/trailing whitespace including the newline character  
c) Converts uppercase to lowercase  
d) Removes the first and last characters  

**Q5.** What does the `!` in `println!` signify?

a) It's a required syntax for all function calls  
b) It means the function can panic  
c) It marks a macro rather than a regular function  
d) It means the function is unsafe  

---

## Quiz Answers

**A1.** b) The ownership system enforced at compile time  
*The borrow checker validates memory safety before the program runs — no runtime overhead needed.*

**A2.** c) It checks for errors without producing a binary (faster)  
*`cargo check` skips code generation, making it 2-3x faster than `cargo build`. Use it during development for rapid feedback.*

**A3.** b) It prevents accidental mutations and makes intent explicit  
*Immutability by default is a safety feature. If a variable doesn't need to change, Rust forces you to prove it — reducing bugs from unexpected mutations.*

**A4.** b) Removes leading/trailing whitespace including the newline character  
*`read_line` always includes `\n` at the end. Forgetting to trim is one of the most common bugs in beginner Rust programs.*

**A5.** c) It marks a macro rather than a regular function  
*Macros in Rust are expanded at compile time and can accept variable numbers of arguments and types, which regular functions cannot do in the same way.*

---

## Chapter Summary

- Rust is a systems programming language created by Mozilla in 2006, stable since 2015
- Its core innovation is memory safety without a garbage collector, enforced by the **ownership system** at compile time
- Rust occupies a unique position: as fast as C/C++, as safe as Java/Python
- **rustup** installs and manages Rust toolchains
- **Cargo** is the all-in-one tool: creates projects, builds, runs, tests, manages dependencies
- Variables are **immutable by default** — you need `mut` to allow mutation
- `cargo check` is faster than `cargo build` and catches errors during development
- Use `cargo clippy` and `cargo fmt` to write idiomatic, well-formatted Rust code
- The `!` suffix marks macros (`println!`, `print!`, `vec!`, etc.)

In Chapter 2, we dive into Rust's type system: how variables work, what types are available, and how Rust handles compound data.
