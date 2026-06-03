# Chapter 4: Functions

## Learning Objectives

By the end of this chapter, you will:

- Declare functions with parameters and return types
- Understand the critical difference between expressions and statements in Rust
- Use implicit returns (no semicolon) and explicit `return`
- Write functions that compose well together
- Build a modular CLI Calculator

---

## Theory

### 4.1 Function Declaration

Functions are declared with the `fn` keyword:

```rust
fn function_name(param1: Type1, param2: Type2) -> ReturnType {
    // body
}
```

Every Rust program starts with `main()`, which takes no arguments and returns nothing:

```rust
fn main() {
    // entry point
}
```

#### Naming Convention

Rust uses **snake_case** for function names (all lowercase, words separated by underscores):

```rust
fn calculate_area() {}       // correct
fn calculateArea() {}        // incorrect (camelCase — Rust compiler warns)
fn CalculateArea() {}        // incorrect (PascalCase — reserved for types)
```

#### Basic Functions

```rust
fn greet(name: &str) {
    println!("Hello, {}!", name);
}

fn add(a: i32, b: i32) -> i32 {
    a + b  // implicit return — no semicolon
}

fn main() {
    greet("Alice");
    let sum = add(3, 4);
    println!("sum = {}", sum);
}
```

#### Function Order

Rust does not care about declaration order. You can call a function before it's defined:

```rust
fn main() {
    let result = multiply(3, 7);  // calling multiply before it's defined
    println!("{}", result);
}

fn multiply(x: i32, y: i32) -> i32 {
    x * y
}
```

This is different from C, where you need forward declarations.

---

### 4.2 Parameters

Every parameter must have an explicit type annotation. Rust does not infer parameter types (unlike local variables):

```rust
fn describe(name: &str, age: u32, score: f64) {
    println!("{} is {} years old with score {:.1}", name, age, score);
}

fn main() {
    describe("Alice", 25, 92.5);
}
```

#### Passing by Value vs by Reference

When you pass a value to a function, Rust either **moves** or **copies** it depending on the type:

```rust
fn print_number(n: i32) {        // i32 implements Copy → value is copied
    println!("{}", n);
}

fn print_string(s: String) {     // String does NOT implement Copy → value is moved
    println!("{}", s);
}

fn print_str(s: &str) {          // &str is a reference → no move or copy, just borrow
    println!("{}", s);
}

fn main() {
    let x = 42;
    print_number(x);
    println!("{}", x);  // x still usable — was copied

    let greeting = String::from("hello");
    print_string(greeting);
    // println!("{}", greeting);  // ERROR: greeting was moved into print_string

    let word = "world";
    print_str(word);
    println!("{}", word);  // word still usable — reference was passed
}
```

We'll fully explore ownership and borrowing in Part 2. For now, know that:
- Primitive types (i32, f64, bool, char, etc.) are **copied** when passed
- `String` and other heap types are **moved** when passed (unless you pass a reference `&`)
- Pass `&str` or `&T` to lend data without transferring ownership

#### Multiple Return via Tuples

Functions return a single value, but you can use a tuple to return multiple values:

```rust
fn min_max(data: &[i32]) -> (i32, i32) {
    let mut min = data[0];
    let mut max = data[0];
    for &x in &data[1..] {
        if x < min { min = x; }
        if x > max { max = x; }
    }
    (min, max)
}

fn main() {
    let scores = [78, 92, 55, 88, 63, 97];
    let (lowest, highest) = min_max(&scores);
    println!("min={}, max={}", lowest, highest);
}
```

---

### 4.3 Return Values

The return type is declared after `->`:

```rust
fn square(n: i32) -> i32 {
    n * n
}

fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn full_name(first: &str, last: &str) -> String {
    format!("{} {}", first, last)
}
```

#### Implicit Return (No Semicolon)

In Rust, the **last expression** in a function is automatically returned if it has no semicolon:

```rust
fn double(x: i32) -> i32 {
    x * 2           // no semicolon → this is the return value
}

fn triple(x: i32) -> i32 {
    let y = x * 3;
    y               // no semicolon → return y
}
```

This is the idiomatic Rust style. Explicit `return` is for early returns.

#### Explicit return

Use `return` to exit early from a function:

```rust
fn divide(a: f64, b: f64) -> f64 {
    if b == 0.0 {
        return 0.0;  // early return
    }
    a / b  // implicit return for the normal case
}

fn find_first_negative(nums: &[i32]) -> Option<i32> {
    for &n in nums {
        if n < 0 {
            return Some(n);  // found — exit immediately
        }
    }
    None  // nothing found
}
```

#### Functions That Return Nothing

Functions without a `->` clause implicitly return the **unit type** `()`:

```rust
fn say_hello() {           // returns ()
    println!("Hello!");
}

fn say_hello_explicit() -> () {  // same, but explicit
    println!("Hello!");
}
```

The unit type `()` is like `void` in C/Java, but it's a real type in Rust with exactly one value: `()`.

---

### 4.4 Expressions vs Statements

This distinction is critical in Rust and confuses many beginners.

#### Statements

Statements **perform an action** and **do not return a value**:

```rust
let x = 5;       // statement — variable declaration
let y = 6;       // statement
x + y;           // statement — expression followed by semicolon (value discarded)
```

Statements end with a semicolon (`;`).

#### Expressions

Expressions **evaluate to a value**:

```rust
5            // expression — evaluates to 5
x + y        // expression — evaluates to the sum
{            // a block is also an expression!
    let x = 3;
    x + 1    // no semicolon — this is the value of the block
}            // this block evaluates to 4
```

#### The Semicolon Rule

Adding a semicolon to an expression turns it into a statement (discards the value):

```rust
x + 1    // expression — evaluates to some integer
x + 1;   // statement  — evaluates to ()  (value discarded)
```

This is why the last line of a function should NOT have a semicolon if you want to return its value:

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b    // expression — RETURNED
}

fn broken(a: i32, b: i32) -> i32 {
    a + b;   // statement — value discarded!
    // ERROR: mismatched types — expected i32, found ()
}
```

#### Blocks as Expressions

Block scopes `{ }` are expressions:

```rust
fn main() {
    let y = {
        let x = 3;
        x + 1   // no semicolon — this is the value of the block
    };           // note semicolon here — the let statement ends here

    println!("y = {}", y);  // y = 4
}
```

This lets you compute complex values inline:

```rust
fn main() {
    let score = 78;
    let grade = {
        if score >= 90 { "A" }
        else if score >= 80 { "B" }
        else if score >= 70 { "C" }
        else if score >= 60 { "D" }
        else { "F" }
    };
    println!("Grade: {}", grade);
}
```

#### Why This Design?

Most languages have a strict statement/expression divide. In C: `if` is a statement (can't be on the right of `=`). In Rust: nearly everything is an expression. This eliminates a whole class of code patterns like:

```c
// C — needs variable, can't use if as value
char grade;
if (score >= 90) grade = 'A';
else if (score >= 80) grade = 'B';
else grade = 'F';
```

```rust
// Rust — clean assignment with if expression
let grade = if score >= 90 { 'A' }
            else if score >= 80 { 'B' }
            else { 'F' };
```

---

## Code Example

### Practice: Calculator Functions and Temperature Converter

```rust
fn add(a: f64, b: f64) -> f64 { a + b }
fn subtract(a: f64, b: f64) -> f64 { a - b }
fn multiply(a: f64, b: f64) -> f64 { a * b }
fn divide(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 { None } else { Some(a / b) }
}
fn power(base: f64, exp: u32) -> f64 {
    base.powi(exp as i32)
}

fn celsius_to_fahrenheit(c: f64) -> f64 { c * 9.0 / 5.0 + 32.0 }
fn fahrenheit_to_celsius(f: f64) -> f64 { (f - 32.0) * 5.0 / 9.0 }
fn celsius_to_kelvin(c: f64) -> f64 { c + 273.15 }

fn main() {
    println!("-- Calculator --");
    println!("3 + 4 = {}", add(3.0, 4.0));
    println!("10 - 3 = {}", subtract(10.0, 3.0));
    println!("6 * 7 = {}", multiply(6.0, 7.0));
    println!("10 / 3 = {:.4}", divide(10.0, 3.0).unwrap_or(f64::NAN));
    println!("10 / 0 = {:?}", divide(10.0, 0.0));
    println!("2^8 = {}", power(2.0, 8));

    println!("\n-- Temperature --");
    let boiling_c = 100.0_f64;
    println!("{:.1}°C = {:.1}°F = {:.2}K",
        boiling_c,
        celsius_to_fahrenheit(boiling_c),
        celsius_to_kelvin(boiling_c)
    );

    let body_f = 98.6_f64;
    println!("{:.1}°F = {:.2}°C",
        body_f,
        fahrenheit_to_celsius(body_f)
    );
}
```

---

### Mini Project: CLI Calculator

```rust
use std::io;
use std::io::Write;

fn add(a: f64, b: f64) -> f64 { a + b }
fn subtract(a: f64, b: f64) -> f64 { a - b }
fn multiply(a: f64, b: f64) -> f64 { a * b }

fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Cannot divide by zero"))
    } else {
        Ok(a / b)
    }
}

fn modulo(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Cannot modulo by zero"))
    } else {
        Ok(a % b)
    }
}

fn read_number(prompt: &str) -> Result<f64, String> {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input.trim().parse::<f64>().map_err(|e| format!("Invalid number: {}", e))
}

fn read_operator() -> String {
    print!("Operator (+, -, *, /, %, q=quit): ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn calculate(a: f64, op: &str, b: f64) -> Result<f64, String> {
    match op {
        "+" => Ok(add(a, b)),
        "-" => Ok(subtract(a, b)),
        "*" => Ok(multiply(a, b)),
        "/" => divide(a, b),
        "%" => modulo(a, b),
        _   => Err(format!("Unknown operator: '{}'", op)),
    }
}

fn format_result(n: f64) -> String {
    if n == n.floor() && n.abs() < 1e15_f64 {
        format!("{}", n as i64)  // display as integer if it's a whole number
    } else {
        format!("{:.6}", n)
    }
}

fn main() {
    println!("=== CLI Calculator ===");
    println!("Press 'q' at the operator prompt to quit.\n");

    loop {
        let a = match read_number("First number: ") {
            Ok(n)  => n,
            Err(e) => { println!("Error: {}", e); continue; }
        };

        let op = read_operator();
        if op == "q" || op == "quit" {
            println!("Goodbye!");
            break;
        }

        let b = match read_number("Second number: ") {
            Ok(n)  => n,
            Err(e) => { println!("Error: {}", e); continue; }
        };

        match calculate(a, &op, b) {
            Ok(result) => println!("= {}\n", format_result(result)),
            Err(e)     => println!("Error: {}\n", e),
        }
    }
}
```

### Line-by-Line Explanation

```rust
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Cannot divide by zero"))
    } else {
        Ok(a / b)
    }
}
```
- `Result<f64, String>` is the return type — either success (`Ok(f64)`) or failure (`Err(String)`)
- `Result` is Rust's primary error handling type (covered fully in Part 5)
- `Err(...)` and `Ok(...)` are returned as expressions (no `return` keyword, no semicolons)
- The entire `if/else` is an expression — each branch produces a `Result`

```rust
fn read_number(prompt: &str) -> Result<f64, String> {
    ...
    input.trim().parse::<f64>().map_err(|e| format!("Invalid number: {}", e))
}
```
- `parse::<f64>()` attempts to parse the string as an f64
- `.map_err(|e| ...)` transforms the error type from `ParseFloatError` to `String`
- The `|e| format!(...)` is a **closure** (anonymous function) — covered in Part 9

```rust
fn calculate(a: f64, op: &str, b: f64) -> Result<f64, String> {
    match op {
        "+" => Ok(add(a, b)),
        ...
        _   => Err(format!("Unknown operator: '{}'", op)),
    }
}
```
- Dispatch to the right function using `match` on a string
- Wraps each result in `Ok(...)` so the return type is consistent
- Unknown operators produce an `Err`

```rust
fn format_result(n: f64) -> String {
    if n == n.floor() && n.abs() < 1e15_f64 {
        format!("{}", n as i64)
    } else {
        format!("{:.6}", n)
    }
}
```
- `n.floor()` rounds down; comparing `n == n.floor()` checks if the float is a whole number
- `n.abs() < 1e15` guards against very large numbers that can't be exactly represented as `i64`
- If it's a whole number, show it as an integer; otherwise show 6 decimal places

---

## Common Mistakes

### Mistake 1: Semicolon on return expression

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b;  // WRONG: semicolon turns expression into statement → returns ()
            // ERROR: mismatched types: expected `i32`, found `()`
}

fn add(a: i32, b: i32) -> i32 {
    a + b   // CORRECT: no semicolon → this is the return value
}
```

### Mistake 2: Missing type annotations on parameters

```rust
fn multiply(a, b) -> i32 {  // ERROR: missing types
    a * b
}

fn multiply(a: i32, b: i32) -> i32 {  // CORRECT
    a * b
}
```

Unlike local variables, parameter types are always required.

### Mistake 3: Returning different types from branches

```rust
fn classify(n: i32) -> &'static str {
    if n > 0 {
        "positive"
    } else if n < 0 {
        "negative"
    } else {
        42  // ERROR: expected &str, found integer
    }
}

// Fix: consistent types
fn classify(n: i32) -> &'static str {
    if n > 0 { "positive" }
    else if n < 0 { "negative" }
    else { "zero" }
}
```

### Mistake 4: Calling `.unwrap()` carelessly

```rust
let result = divide(10.0, 0.0).unwrap(); // PANIC: called `unwrap()` on `Err`
```

Always handle the error case. In simple programs, `.unwrap_or(default)` or `match` is safer:

```rust
let result = divide(10.0, 0.0).unwrap_or(0.0); // returns 0.0 on error
```

---

## Best Practices

1. **Keep functions short** — if a function is longer than ~30 lines, consider splitting it
2. **Single responsibility** — each function should do one thing well
3. **Use implicit returns** (no `return`) for the main return path; use `return` only for early exits
4. **Return `Result` or `Option`** instead of using special sentinel values like `-1` or `""` to indicate failure
5. **Name functions as verbs** — `calculate_area`, `find_user`, `validate_input`
6. **Prefer `&str` over `String` for parameters** when you only need to read the string (no ownership needed)

---

## Exercises

### Exercise 1: Factorial

Write a function `factorial(n: u64) -> u64` that computes n!. Test with 0, 1, 5, 10.

### Exercise 2: Fibonacci

Write `fibonacci(n: u32) -> u64` that returns the nth Fibonacci number (0-indexed: fib(0)=0, fib(1)=1, fib(2)=1, fib(3)=2...).

### Exercise 3: String Functions

Write three functions:
- `count_vowels(s: &str) -> usize` — count vowels in a string
- `reverse_words(s: &str) -> String` — reverse the order of words in a sentence
- `is_palindrome(s: &str) -> bool` — check if a string reads the same forwards and backwards

### Exercise 4: Body Mass Index

Write `bmi(weight_kg: f64, height_m: f64) -> (f64, &'static str)` that returns the BMI value and category:
- Under 18.5: "Underweight"
- 18.5–24.9: "Normal"
- 25.0–29.9: "Overweight"
- 30.0+: "Obese"

### Exercise 5: Expression Blocks

Rewrite this code to use expression blocks instead of temporary variables:

```rust
let a = 5;
let b = 3;
let temp1 = a * a;
let temp2 = b * b;
let temp3 = temp1 + temp2;
let hypotenuse = temp3 as f64;
let hypotenuse = hypotenuse.sqrt();
println!("{}", hypotenuse);
```

---

## Solutions

### Solution 1

```rust
fn factorial(n: u64) -> u64 {
    if n == 0 { return 1; }
    let mut result = 1u64;
    for i in 2..=n {
        result *= i;
    }
    result
}

fn main() {
    for n in [0, 1, 5, 10] {
        println!("{}! = {}", n, factorial(n));
    }
}
```

### Solution 2

```rust
fn fibonacci(n: u32) -> u64 {
    if n == 0 { return 0; }
    if n == 1 { return 1; }

    let mut a = 0u64;
    let mut b = 1u64;
    for _ in 2..=n {
        let c = a + b;
        a = b;
        b = c;
    }
    b
}

fn main() {
    for i in 0..=10 {
        print!("{} ", fibonacci(i));
    }
    println!();
}
```

### Solution 3

```rust
fn count_vowels(s: &str) -> usize {
    s.chars().filter(|c| "aeiouAEIOU".contains(*c)).count()
}

fn reverse_words(s: &str) -> String {
    s.split_whitespace()
     .rev()
     .collect::<Vec<&str>>()
     .join(" ")
}

fn is_palindrome(s: &str) -> bool {
    let cleaned: String = s.chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect();
    cleaned == cleaned.chars().rev().collect::<String>()
}

fn main() {
    println!("{}", count_vowels("Hello World"));        // 3
    println!("{}", reverse_words("the quick brown fox")); // "fox brown quick the"
    println!("{}", is_palindrome("A man a plan a canal Panama")); // true
    println!("{}", is_palindrome("hello"));              // false
}
```

### Solution 4

```rust
fn bmi(weight_kg: f64, height_m: f64) -> (f64, &'static str) {
    let bmi_value = weight_kg / (height_m * height_m);
    let category = if bmi_value < 18.5 { "Underweight" }
                   else if bmi_value < 25.0 { "Normal" }
                   else if bmi_value < 30.0 { "Overweight" }
                   else { "Obese" };
    (bmi_value, category)
}

fn main() {
    let (value, category) = bmi(70.0, 1.75);
    println!("BMI: {:.1} — {}", value, category);
}
```

### Solution 5

```rust
fn main() {
    let a = 5;
    let b = 3;
    let hypotenuse = {
        let sum_squares = (a * a + b * b) as f64;
        sum_squares.sqrt()
    };
    println!("{}", hypotenuse);
}
```

---

## Quiz

**Q1.** What is the implicit return value of the last expression in a function?

a) `null`  
b) `0`  
c) The value of the expression if it has no trailing semicolon  
d) The value of the expression regardless of semicolons  

**Q2.** What does adding a semicolon to an expression do?

a) Returns the expression's value  
b) Turns it into a statement that returns `()`  
c) Causes a compile error  
d) Has no effect  

**Q3.** When should you use explicit `return` in Rust?

a) Always — for clarity  
b) Never — implicit return is required  
c) Only for the last expression  
d) For early returns within a function  

**Q4.** Why are type annotations required on function parameters?

a) Performance optimization  
b) So Rust can infer types elsewhere in the program  
c) Historical accident  
d) For documentation only  

**Q5.** What is the type of `()` (unit)?

a) `null`  
b) `void`  
c) A real type with exactly one value, `()`  
d) An error type  

---

## Quiz Answers

**A1.** c) The value of the expression if it has no trailing semicolon  
*No semicolon = expression = return value. Semicolon = statement = returns `()`.*

**A2.** b) Turns it into a statement that returns `()`  
*The semicolon "discards" the value. `a + b` evaluates to a number; `a + b;` evaluates to `()`.*

**A3.** d) For early returns within a function  
*Idiomatic Rust uses implicit returns (no semicolon) for the main return value and `return` only to exit a function early.*

**A4.** b) So Rust can infer types elsewhere in the program  
*Function signatures are the "boundary" of type inference. Rust can infer local variable types, but function parameters need explicit types so callers know what to pass.*

**A5.** c) A real type with exactly one value, `()`  
*Unlike `void` in C (which is not a real type), `()` in Rust is a first-class type. You can store it in a variable: `let x: () = ();`. Functions that "return nothing" actually return `()`.*

---

## Chapter Summary

- Functions are declared with `fn`, parameters need explicit types, return type follows `->`
- Rust uses **snake_case** for function names
- **Parameters** require type annotations (unlike local variables where Rust infers types)
- **Implicit return**: the last expression in a function (without semicolon) is the return value
- **Explicit `return`**: use for early exits only; idiomatic Rust avoids it for the main return path
- **Expressions** evaluate to a value; **statements** perform actions and return `()`
- A semicolon transforms an expression into a statement (discards the value)
- **Blocks `{}`** are expressions — they return the value of their last expression
- `if`, `match`, and `loop` are all expressions and can be used in `let` bindings
- **`Result<T, E>`** is the idiomatic way to return values that can fail (`Ok(value)` or `Err(error)`)
- Functions should be short, single-purpose, and clearly named as verbs

---

**Part 1 Complete!**

You now know the fundamentals: variables and types, control flow, and functions. In Part 2, you'll learn Rust's most distinctive feature — the ownership system — which is what makes Rust both fast and memory-safe. This is the hardest conceptual jump in Rust learning, but once you understand it, everything else clicks into place.
