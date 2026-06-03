# Chapter 2: Variables and Data Types

## Learning Objectives

By the end of this chapter, you will:

- Understand how variables work in Rust (let, mut, shadowing)
- Know all of Rust's primitive scalar types and when to use each
- Understand compound types: tuples, arrays, and slices
- Build a working Student Grade Calculator

---

## Theory

### 2.1 Variables

In Rust, variables are declared with the `let` keyword. The most important thing to understand about Rust variables is that **they are immutable by default**.

#### let — Immutable Variables

```rust
fn main() {
    let x = 5;
    println!("x = {}", x);

    x = 6; // ERROR: cannot assign twice to immutable variable `x`
}
```

This is intentional. Immutability is a safety feature. If a value should never change, the compiler enforces it. If you later accidentally try to change it, the compiler catches the mistake.

This is different from constants in other languages — in Rust, immutable variables are the **default**, not the exception.

#### mut — Mutable Variables

When you do need a variable to change, you explicitly opt in with `mut`:

```rust
fn main() {
    let mut x = 5;
    println!("x = {}", x);  // x = 5

    x = 6;
    println!("x = {}", x);  // x = 6
}
```

The `mut` keyword signals to both the compiler and the reader: "this value is expected to change."

#### Type Inference

Rust can infer types from context. You don't always need to annotate types:

```rust
let x = 5;        // Rust infers: i32
let y = 3.14;     // Rust infers: f64
let z = true;     // Rust infers: bool
let name = "Bob"; // Rust infers: &str
```

But you can (and sometimes must) annotate types explicitly:

```rust
let x: i32 = 5;
let y: f64 = 3.14;
let z: bool = true;
let name: &str = "Bob";
```

You must annotate when Rust cannot infer the type from context — for example, when parsinglet n: i32 = "42".parse().unwrap(); // must tell Rust what type to parse to a string:

```rust
let n: i32 = "42".parse().unwrap(); // must tell Rust what type to parse to
```

#### Constants

Constants are different from immutable variables:

```rust
const MAX_SCORE: u32 = 100;        // must have type annotation
const PI: f64 = 3.14159265358979;
```

| Aspect | `let` (immutable) | `const` |
|--------|-------------------|---------|
| Type annotation | Optional | Required |
| `mut` allowed | Yes | No |
| Set at runtime | Yes | No (compile-time only) |
| Scope | Block scope | Any scope (including global) |
| Shadowing allowed | Yes | No |

#### Shadowing

Shadowing lets you declare a new variable with the same name as a previous one:

```rust
fn main() {
    let x = 5;
    println!("x = {}", x);  // x = 5

    let x = x + 1;          // new variable 'x', shadows the old one
    println!("x = {}", x);  // x = 6

    {
        let x = x * 2;      // shadows only within this block
        println!("x = {}", x);  // x = 12
    }

    println!("x = {}", x);  // x = 6  (inner shadow gone)
}
```

Shadowing is **not the same** as mutation. When you shadow, you create an entirely new variable. The old one is gone from that point. This allows you to change the type:

```rust
// With shadowing — allowed, changing type
let spaces = "   ";        // &str
let spaces = spaces.len(); // usize — this is a new variable

// With mut — NOT allowed, cannot change type
let mut spaces = "   ";
spaces = spaces.len(); // ERROR: mismatched types
```

**Why shadowing is useful:**

```rust
let input = "42";           // user input as a string
let input: i32 = input.parse().unwrap(); // convert in place, same name
// now input is an integer
```

This avoids naming things like `input_str` and `input_int`. The old string is consumed, and the name `input` is repurposed for the integer.

---

### 2.2 Scalar Types

Rust has four primary scalar types: integers, floating-point numbers, booleans, and characters.

#### Integer Types

An integer is a whole number. Rust provides both **signed** (positive and negative) and **unsigned** (positive only) integers of various sizes:

```
Signed integers:
  i8    → -128 to 127                     (8 bits)
  i16   → -32,768 to 32,767               (16 bits)
  i32   → -2,147,483,648 to 2,147,483,647 (32 bits) ← DEFAULT
  i64   → -9.2 × 10^18 to 9.2 × 10^18    (64 bits)
  i128  → enormous range                  (128 bits)
  isize → pointer size (32 or 64 bits depending on architecture)

Unsigned integers:
  u8    → 0 to 255                        (8 bits)
  u16   → 0 to 65,535                     (16 bits)
  u32   → 0 to 4,294,967,295             (32 bits)
  u64   → 0 to 18.4 × 10^18              (64 bits)
  u128  → enormous range                  (128 bits)
  usize → pointer size (for indexing and sizes)
```

**Which to use?**
- Default to `i32` — it's the fastest integer type on most 64-bit systems
- Use `u8` for byte data (file contents, images)
- Use `usize` for array indices and collection sizes (required by Rust's indexing)
- Use `i64` / `u64` when you need large numbers
- Use `i128` / `u128` only for cryptography or huge numbers

```rust
fn main() {
    let a: i32 = -42;
    let b: u8 = 255;
    let c: i64 = 9_000_000_000; // underscores for readability
    let d: usize = 42;

    // Integer literals in different bases:
    let decimal     = 98_222;    // decimal (underscores ignored)
    let hex         = 0xff;      // hexadecimal
    let octal       = 0o77;      // octal
    let binary      = 0b1111_0000; // binary
    let byte        = b'A';      // byte literal — u8 only

    println!("{} {} {} {} {} {} {} {} {}", a, b, c, d, decimal, hex, octal, binary, byte);
}
```

#### Integer Overflow

In debug mode, Rust panics on integer overflow:

```rust
let x: u8 = 255;
let y = x + 1; // PANIC in debug: attempt to add with overflow
```

In release mode, it wraps around (255 + 1 = 0 for u8). For intentional wrapping, use the explicit wrapping methods:

```rust
let x: u8 = 255;
let y = x.wrapping_add(1);    // y = 0
let z = x.saturating_add(1);  // z = 255 (saturates at max)
let w = x.checked_add(1);     // w = None (returns Option)
```

#### Floating-Point Types

Rust has two floating-point types following the IEEE 754 standard:

```
f32   → 32-bit float, ~7 decimal digits of precision
f64   → 64-bit float, ~15 decimal digits of precision ← DEFAULT
```

```rust
fn main() {
    let x = 2.0;        // f64 by default
    let y: f32 = 3.0;   // explicit f32

    // All standard operations:
    let sum = x + 2.5;
    let difference = x - 1.0;
    let product = x * 3.0;
    let quotient = x / 2.0;
    let remainder = 10.0_f64 % 3.0;

    println!("{} {} {} {} {}", sum, difference, product, quotient, remainder);

    // Math functions:
    let pi = std::f64::consts::PI;
    println!("sin(π/2) = {}", (pi / 2.0).sin());    // 1.0
    println!("sqrt(2) = {}", 2.0_f64.sqrt());        // 1.4142...
    println!("2^10 = {}", 2.0_f64.powi(10));         // 1024.0
}
```

**Why two float types?**
- `f64` is default because on modern 64-bit CPUs, it's the same speed as `f32` but twice as precise
- Use `f32` for GPU computing, graphics, or when memory is extremely limited (embedded systems)

#### Boolean

```rust
fn main() {
    let t: bool = true;
    let f: bool = false;

    // Logical operators:
    println!("{}", t && f);  // AND → false
    println!("{}", t || f);  // OR  → true
    println!("{}", !t);      // NOT → false

    // Booleans in conditions:
    if t {
        println!("it is true");
    }

    // Boolean from comparison:
    let x = 5;
    let is_big = x > 3;   // true
    let is_ten = x == 10; // false
}
```

A `bool` is exactly 1 byte in memory (not 1 bit, due to alignment requirements).

#### Character

Rust's `char` type is 4 bytes (32 bits) and represents a **Unicode scalar value**. This is very different from C/Java where `char` is typically 1 byte (ASCII).

```rust
fn main() {
    let c = 'z';           // single quotes for char (NOT double quotes)
    let z: char = 'ℤ';    // mathematical integer sign
    let heart = '❤';      // emoji — valid Rust char!
    let kanji = '猫';      // Japanese for "cat"

    println!("{} {} {} {}", c, z, heart, kanji);

    // char operations:
    println!("{}", 'a'.is_alphabetic());  // true
    println!("{}", '5'.is_numeric());     // true
    println!("{}", 'A'.to_lowercase().next().unwrap()); // 'a'

    // char to/from u32:
    let unicode_val = 'A' as u32;  // 65
    let back = char::from(65_u8);  // 'A'
}
```

---

### 2.3 Compound Types

Compound types group multiple values into one type.

#### Tuple

A tuple is a fixed-length collection of values of potentially different types:

```rust
fn main() {
    let tup: (i32, f64, bool, char) = (500, 6.4, true, 'z');

    // Access by index (0-based):
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let is_true = tup.2;

    println!("{} {} {}", five_hundred, six_point_four, is_true);

    // Destructuring:
    let (x, y, z, w) = tup;
    println!("{} {} {} {}", x, y, z, w);

    // Nested tuple:
    let nested = ((1, 2), (3, 4));
    println!("{}", nested.0.0);  // 1
    println!("{}", nested.1.1);  // 4

    // Unit tuple — the empty type
    let unit: () = ();  // returned by functions that return "nothing"
}
```

**When to use tuples:**
- Returning multiple values from a function
- Temporary grouping of related values
- When the fields don't have names (use a struct if they do)

```rust
fn min_max(numbers: &[i32]) -> (i32, i32) {
    let min = *numbers.iter().min().unwrap();
    let max = *numbers.iter().max().unwrap();
    (min, max)
}

fn main() {
    let nums = [3, 1, 4, 1, 5, 9, 2, 6];
    let (min, max) = min_max(&nums);
    println!("min={}, max={}", min, max);
}
```

#### Array

Arrays in Rust are fixed-length collections of values of the **same type**. They are stack-allocated (not heap-allocated like `Vec`).

```rust
fn main() {
    // Type annotation: [type; length]
    let arr: [i32; 5] = [1, 2, 3, 4, 5];

    // Access by index:
    println!("{}", arr[0]);  // 1
    println!("{}", arr[4]);  // 5

    // Array with repeated value:
    let zeros = [0; 10];    // [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
    let fives = [5; 3];     // [5, 5, 5]

    // Length:
    println!("length = {}", arr.len());  // 5

    // Iteration:
    for element in arr {
        print!("{} ", element);
    }
    println!();

    // Index bounds checking:
    // arr[10] → PANIC: index out of bounds
    // (Rust checks bounds at runtime; C/C++ would silently corrupt memory)

    // Mutable array:
    let mut grades = [85, 90, 78, 92, 88];
    grades[2] = 80;  // update third element
}
```

**Array vs Vec:**

| Aspect | Array `[T; N]` | Vector `Vec<T>` |
|--------|---------------|-----------------|
| Size | Fixed at compile time | Dynamic, grows/shrinks |
| Location | Stack | Heap |
| Performance | Faster (no heap allocation) | Slightly slower |
| Use when | Size is known, small, fixed | Size varies at runtime |

#### Slice

A slice is a **reference to a contiguous sequence** of elements in an array or vector. It does NOT own the data — it borrows it.

```rust
fn main() {
    let arr = [1, 2, 3, 4, 5];

    // Slice syntax: &arr[start..end]  (start inclusive, end exclusive)
    let slice = &arr[1..4];      // [2, 3, 4]
    let from_start = &arr[..3];  // [1, 2, 3]  (0..3)
    let to_end = &arr[2..];      // [3, 4, 5]  (2..len)
    let whole = &arr[..];        // [1, 2, 3, 4, 5]

    println!("{:?}", slice);      // [2, 3, 4]
    println!("len = {}", slice.len());  // 3

    // Slices work with functions:
    fn sum(s: &[i32]) -> i32 {
        let mut total = 0;
        for &x in s {
            total += x;
        }
        total
    }

    println!("sum = {}", sum(&arr));       // sum of whole array
    println!("sum = {}", sum(&arr[1..4])); // sum of slice

    // String slices:
    let hello = "Hello, World!";
    let hello_only = &hello[0..5];  // "Hello"
    let world = &hello[7..12];      // "World"
    println!("{} {}", hello_only, world);
}
```

**Memory layout of a slice:**

```
arr: [1, 2, 3, 4, 5]  ← stored in memory
      ^  ^  ^  ^  ^
      0  1  2  3  4

&arr[1..4]:
  pointer → element at index 1
  length  → 3
```

A slice is a **fat pointer** — it holds both a pointer to the data and the length. This allows bounds checking without knowing the original array's size.

---

## Code Example

### Mini Project: Student Grade Calculator

```rust
use std::io;
use std::io::Write;

fn read_line(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn calculate_average(scores: &[f64]) -> f64 {
    if scores.is_empty() {
        return 0.0;
    }
    let sum: f64 = scores.iter().sum();
    sum / scores.len() as f64
}

fn letter_grade(average: f64) -> &'static str {
    match average as u32 {
        90..=100 => "A",
        80..=89  => "B",
        70..=79  => "C",
        60..=69  => "D",
        _        => "F",
    }
}

fn pass_fail(average: f64) -> &'static str {
    if average >= 60.0 { "PASS" } else { "FAIL" }
}

fn main() {
    println!("=== Student Grade Calculator ===\n");

    let name = read_line("Student name: ");
    let count_str = read_line("Number of subjects: ");
    let count: usize = count_str.parse().unwrap_or(0);

    if count == 0 {
        println!("Invalid number of subjects.");
        return;
    }

    let mut scores: Vec<f64> = Vec::new();
    let mut subjects: Vec<String> = Vec::new();

    for i in 0..count {
        let subject = read_line(&format!("  Subject {} name: ", i + 1));
        let score_str = read_line(&format!("  Subject {} score (0-100): ", i + 1));
        let score: f64 = score_str.parse().unwrap_or(0.0);
        subjects.push(subject);
        scores.push(score);
    }

    let average = calculate_average(&scores);
    let grade = letter_grade(average);
    let result = pass_fail(average);

    println!("\n=== Report Card ===");
    println!("Student: {}", name);
    println!("-------------------");
    for (subject, score) in subjects.iter().zip(scores.iter()) {
        println!("  {:<20} {:>6.1}", subject, score);
    }
    println!("-------------------");
    println!("  {:<20} {:>6.1}", "Average", average);
    println!("  Grade: {}", grade);
    println!("  Result: {}", result);
}
```

### Line-by-Line Explanation

```rust
fn read_line(prompt: &str) -> String {
```
- Helper function to avoid repeating input code
- Takes `prompt: &str` — a string slice (borrowed, doesn't take ownership)
- Returns `String` — an owned, heap-allocated string

```rust
    let sum: f64 = scores.iter().sum();
    sum / scores.len() as f64
```
- `scores.iter()` creates an iterator over the slice
- `.sum()` adds all elements (returns f64 because scores is `&[f64]`)
- `scores.len() as f64` — `len()` returns `usize`, must cast to `f64` for division
- No semicolon on last line → this expression is the return value

```rust
fn letter_grade(average: f64) -> &'static str {
    match average as u32 {
        90..=100 => "A",
```
- `match` is Rust's powerful pattern matching (covered fully in Chapter 3)
- `average as u32` truncates the float to an unsigned integer
- `90..=100` is an inclusive range pattern (90 to 100 inclusive)
- String literals like `"A"` have type `&'static str` — references to data baked into the binary

```rust
    for (subject, score) in subjects.iter().zip(scores.iter()) {
```
- `.zip()` combines two iterators into pairs: `(subjects[0], scores[0])`, `(subjects[1], scores[1])`, etc.
- Destructuring in the `for` loop extracts each pair into `subject` and `score`

---

## Common Mistakes

### Mistake 1: Using `=` instead of `==` for comparison

```rust
let x = 5;
if x = 6 {      // COMPILE ERROR in Rust (unlike C where this is a silent bug)
    println!("six");
}
if x == 6 {     // correct
    println!("six");
}
```

Rust's type system saves you here: `=` is an assignment, not an expression that returns a bool.

### Mistake 2: Array index out of bounds

```rust
let arr = [1, 2, 3];
println!("{}", arr[5]); // PANIC at runtime: index out of bounds: len=3, index=5
```

Rust checks bounds at runtime and panics rather than silently reading garbage memory like C.

### Mistake 3: Integer and float mixing

```rust
let x: i32 = 5;
let y: f64 = 3.14;
let sum = x + y; // ERROR: cannot add `f64` to `i32`

// Fix: explicit cast
let sum = x as f64 + y;
```

Rust never implicitly converts between numeric types. You must cast explicitly.

### Mistake 4: Mutable reference to a tuple field

```rust
let tup = (1, 2, 3);
tup.0 = 10; // ERROR: cannot assign to immutable field

let mut tup = (1, 2, 3); // make the whole tuple mutable
tup.0 = 10; // now works
```

---

## Best Practices

1. **Default to `i32` for integers** unless you have a specific reason for another type
2. **Default to `f64` for floats** — same performance as `f32` on 64-bit systems, more precise
3. **Use `_` as a numeric separator** for readability: `1_000_000` instead of `1000000`
4. **Use `usize` for indices** — it's the type Rust arrays and vectors expect
5. **Prefer slices over arrays in function parameters** — `&[T]` accepts both arrays and vectors
6. **Name tuples with structs** once they have more than 2-3 elements — tuples with many fields become confusing

---

## Exercises

### Exercise 1: Type Exploration

Declare one variable of each scalar type (`i32`, `u8`, `f64`, `bool`, `char`). Print each with a descriptive label.

### Exercise 2: Temperature Converter

Write a program that declares temperature in Celsius as a variable and converts it to Fahrenheit and Kelvin.
- Formula: F = C × 9/5 + 32
- Formula: K = C + 273.15

### Exercise 3: Array Statistics

Create an array of 6 test scores. Calculate and print:
- Minimum score
- Maximum score
- Sum of all scores
- Average score

### Exercise 4: Tuple Swap

Create a tuple `(a, b)` with two integers. Swap the values using destructuring. Print before and after.

### Exercise 5: Shadowing Type Change

Start with `let data = "1234";` (a string). Using shadowing, convert it to:
1. An integer
2. A float
3. Back to a string using `format!("{}", data)`

---

## Solutions

### Solution 1

```rust
fn main() {
    let integer: i32 = -42;
    let unsigned: u8 = 200;
    let float: f64 = 3.14159;
    let boolean: bool = true;
    let character: char = '🦀'; // Rust's mascot, Ferris the crab

    println!("i32:   {}", integer);
    println!("u8:    {}", unsigned);
    println!("f64:   {}", float);
    println!("bool:  {}", boolean);
    println!("char:  {}", character);
}
```

### Solution 2

```rust
fn main() {
    let celsius: f64 = 100.0;
    let fahrenheit = celsius * 9.0 / 5.0 + 32.0;
    let kelvin = celsius + 273.15;

    println!("{}°C = {}°F = {}K", celsius, fahrenheit, kelvin);
}
```

### Solution 3

```rust
fn main() {
    let scores: [i32; 6] = [85, 92, 78, 95, 60, 88];

    let min = scores.iter().min().unwrap();
    let max = scores.iter().max().unwrap();
    let sum: i32 = scores.iter().sum();
    let avg = sum as f64 / scores.len() as f64;

    println!("Min: {}", min);
    println!("Max: {}", max);
    println!("Sum: {}", sum);
    println!("Avg: {:.2}", avg);
}
```

### Solution 4

```rust
fn main() {
    let pair = (10, 20);
    println!("Before: ({}, {})", pair.0, pair.1);

    let (b, a) = pair; // destructure in reverse order
    let swapped = (a, b);
    println!("After:  ({}, {})", swapped.0, swapped.1);
}
```

### Solution 5

```rust
fn main() {
    let data = "1234";
    println!("String: {}", data);

    let data: i64 = data.parse().unwrap();
    println!("Integer: {}", data);

    let data = data as f64 + 0.5;
    println!("Float: {}", data);

    let data = format!("{}", data);
    println!("String again: {}", data);
}
```

---

## Quiz

**Q1.** What is the default integer type when you write `let x = 5;`?

a) `i8`  
b) `i16`  
c) `i32`  
d) `i64`  

**Q2.** What is the key difference between shadowing and mutation?

a) There is no difference  
b) Shadowing creates a new variable (can change type); mutation modifies the existing variable (same type only)  
c) Mutation creates a new variable; shadowing modifies existing  
d) Shadowing is only for strings  

**Q3.** What type does `'a'` (single quotes) produce in Rust?

a) `&str`  
b) `String`  
c) `u8`  
d) `char`  

**Q4.** Which statement about arrays in Rust is correct?

a) Arrays can grow dynamically  
b) Arrays are heap-allocated  
c) Arrays have a fixed size known at compile time  
d) Arrays can hold values of different types  

**Q5.** What is a slice (`&[T]`)?

a) An owned copy of a portion of an array  
b) A reference to a contiguous sequence of elements  
c) A type that can only be used with strings  
d) A heap-allocated collection  

**Q6.** Why does Rust not allow implicit type conversion between numeric types?

a) It's slower to implement  
b) To prevent subtle bugs caused by unexpected precision loss or overflow  
c) Because Rust's type system is too simple  
d) For backwards compatibility with C  

---

## Quiz Answers

**A1.** c) `i32`  
*Rust defaults to `i32` for integer literals because it's efficient on most 64-bit systems.*

**A2.** b) Shadowing creates a new variable (can change type); mutation modifies the existing variable  
*`let x = x + 1` creates a new `x`. `x = x + 1` (with `mut`) modifies the same `x`. Only shadowing allows type changes.*

**A3.** d) `char`  
*Single quotes denote char literals (4-byte Unicode). Double quotes `"a"` produce `&str`.*

**A4.** c) Arrays have a fixed size known at compile time  
*The size is part of the type: `[i32; 5]` and `[i32; 6]` are different types. Use `Vec<T>` for dynamic sizing.*

**A5.** b) A reference to a contiguous sequence of elements  
*A slice is a fat pointer (pointer + length) that borrows data from an array or vector without owning it.*

**A6.** b) To prevent subtle bugs caused by unexpected precision loss or overflow  
*C allows `int + double` silently. In Rust, you must write `x as f64 + y`, making the conversion visible and intentional.*

---

## Chapter Summary

- Variables are **immutable by default** in Rust — add `mut` to allow changes
- **Shadowing** creates a new variable with the same name — the type can change; this is different from mutation
- **Constants** (`const`) must have type annotations, are truly compile-time constants, and can be in global scope
- Integer types: signed (`i8` to `i128`) and unsigned (`u8` to `u128`); default is `i32`
- Floating-point types: `f32` and `f64`; default is `f64`
- `bool` is `true` or `false`, exactly 1 byte
- `char` is 4 bytes and represents any Unicode scalar value (emoji, kanji, etc.)
- **Tuples**: fixed-length, heterogeneous, accessed by index (`tup.0`) or destructured
- **Arrays**: fixed-length, homogeneous, stack-allocated, bounds-checked at runtime
- **Slices**: borrowed references to contiguous sequences; the type `&[T]` accepts both arrays and vectors
- Rust **never implicitly converts** between numeric types — use `as` for explicit casting

In Chapter 3, we explore how to control program flow with `if`, loops, and pattern matching.
