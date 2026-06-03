# Chapter 3: Control Flow

## Learning Objectives

By the end of this chapter, you will:

- Use `if` / `else if` / `else` expressions (including as values in assignments)
- Master all three loop types: `loop`, `while`, and `for`
- Use Rust's powerful `match` expression for pattern matching
- Use `if let` and `while let` for concise conditional binding
- Build a Number Guessing Game using all three loop and matching techniques

---

## Theory

### 3.1 if Expression

In Rust, `if` is an **expression**, not just a statement. This means it returns a value.

#### Basic if / else if / else

```rust
fn main() {
    let number = 7;

    if number < 0 {
        println!("negative");
    } else if number == 0 {
        println!("zero");
    } else if number < 10 {
        println!("small positive");
    } else {
        println!("large positive");
    }
}
```

Note: Rust does **not** require parentheses around the condition (unlike C, Java, JavaScript). The curly braces `{}` are required.

#### if as an Expression

Because `if` is an expression, you can use it on the right side of a `let`:

```rust
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("number = {}", number);  // number = 5
}
```

This is Rust's equivalent of the ternary operator (`condition ? a : b`) in other languages. Both arms must return the **same type** — the compiler will reject:

```rust
let x = if condition { 5 } else { "six" }; // ERROR: expected integer, found &str
```

#### No implicit truthy/falsy

Rust only accepts `bool` in `if` conditions. There is no implicit conversion:

```rust
let x = 5;
if x {           // ERROR: expected `bool`, found integer
    println!("truthy");
}

if x != 0 {      // CORRECT: explicit comparison
    println!("nonzero");
}
```

---

### 3.2 Loops

Rust has three loop constructs. Each is designed for a specific use case.

#### loop — Infinite Loop with Explicit Exit

`loop` runs forever until you explicitly break out. It's the right choice when you don't know in advance how many iterations you need.

```rust
fn main() {
    let mut counter = 0;

    loop {
        counter += 1;
        println!("counter = {}", counter);

        if counter == 5 {
            break;
        }
    }

    println!("done");
}
```

#### loop as an Expression (Returning Values)

Unlike loops in most languages, Rust's `loop` can **return a value** via `break`:

```rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;  // return value from loop
        }
    };

    println!("result = {}", result);  // result = 20
}
```

This is useful when you need to retry an operation until it succeeds and use the result:

```rust
let connection = loop {
    match try_connect() {
        Ok(conn) => break conn,   // return the connection when successful
        Err(e)   => {
            println!("retrying... {}", e);
        }
    }
};
```

#### Loop Labels

When you have nested loops, `break` and `continue` only affect the innermost loop by default. Labels let you specify which loop:

```rust
fn main() {
    'outer: for i in 0..5 {
        for j in 0..5 {
            if i + j == 6 {
                println!("breaking outer at i={}, j={}", i, j);
                break 'outer;  // break out of the outer loop
            }
        }
    }
    println!("done");
}
```

Labels start with a single quote `'` and go before the loop keyword.

#### while — Condition-Based Loop

`while` repeats while a condition is true. Use it when you have a clear exit condition to check at the start of each iteration.

```rust
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }

    println!("LIFTOFF!");
}
```

```rust
fn main() {
    let mut x = 1;

    // Doubling: 1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024
    while x <= 1000 {
        print!("{} ", x);
        x *= 2;
    }
    println!();
}
```

#### for — Iterating Over Collections

`for` is the most commonly used loop in Rust. It iterates over anything that implements the `Iterator` trait:

```rust
fn main() {
    // Iterate over array:
    let arr = [10, 20, 30, 40, 50];
    for element in arr {
        println!("{}", element);
    }

    // Iterate over a range:
    for i in 0..5 {    // 0, 1, 2, 3, 4  (exclusive end)
        print!("{} ", i);
    }
    println!();

    for i in 0..=5 {   // 0, 1, 2, 3, 4, 5  (inclusive end)
        print!("{} ", i);
    }
    println!();

    // Reverse range:
    for i in (0..5).rev() {   // 4, 3, 2, 1, 0
        print!("{} ", i);
    }
    println!();
}
```

#### for with Index: enumerate()

```rust
fn main() {
    let fruits = ["apple", "banana", "cherry"];

    for (index, fruit) in fruits.iter().enumerate() {
        println!("{}: {}", index, fruit);
    }
    // 0: apple
    // 1: banana
    // 2: cherry
}
```

#### for with References

When iterating, be aware of ownership:

```rust
fn main() {
    let words = vec!["hello", "world"];

    // iter() — borrows each element (&T)
    for word in words.iter() {
        println!("{}", word);  // word is &&str here
    }
    // words is still usable

    // into_iter() — consumes the collection (T)
    for word in words.into_iter() {
        println!("{}", word);  // word is &str
    }
    // words is MOVED — can't use it here

    let mut nums = vec![1, 2, 3];

    // iter_mut() — mutable borrow (&mut T)
    for num in nums.iter_mut() {
        *num *= 2;  // dereference to modify
    }
    println!("{:?}", nums);  // [2, 4, 6]
}
```

#### Loop Comparison Table

| Loop | When to use | Can return value? |
|------|-------------|-------------------|
| `loop` | Unknown iterations, retry logic | Yes (via `break value`) |
| `while` | Condition-based, clear exit check | No |
| `for` | Iterating collections or ranges | No |

---

### 3.3 Pattern Matching Basics

Pattern matching is one of Rust's most powerful features. It lets you compare a value against a series of patterns and execute code based on which pattern matches.

#### match

```rust
fn main() {
    let x = 3;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        4 => println!("four"),
        5 => println!("five"),
        _ => println!("something else"),  // _ is the catch-all wildcard
    }
}
```

`match` is **exhaustive** — you must cover every possible case, or use `_` as a catch-all. The compiler will error if you miss a case. This is a huge safety feature: you can't accidentally forget to handle a case.

#### match Returns a Value

Like `if`, `match` is an expression:

```rust
fn main() {
    let coin = "quarter";

    let value = match coin {
        "penny"   => 1,
        "nickel"  => 5,
        "dime"    => 10,
        "quarter" => 25,
        _         => 0,
    };

    println!("{} cents", value);  // 25 cents
}
```

#### Multiple Patterns with `|`

```rust
fn main() {
    let x = 3;

    match x {
        1 | 2       => println!("one or two"),
        3 | 4       => println!("three or four"),
        5..=9       => println!("five through nine"),
        10..=100    => println!("ten through one hundred"),
        _           => println!("out of range"),
    }
}
```

#### match with Binding

You can bind the matched value to a variable using `@`:

```rust
fn main() {
    let x = 7;

    match x {
        n @ 1..=10  => println!("{} is between 1 and 10", n),
        n @ 11..=20 => println!("{} is between 11 and 20", n),
        n           => println!("{} is out of range", n),
    }
}
```

#### match on Tuples

```rust
fn main() {
    let point = (0, 1);

    match point {
        (0, 0) => println!("origin"),
        (x, 0) => println!("on x-axis at {}", x),
        (0, y) => println!("on y-axis at {}", y),
        (x, y) => println!("at ({}, {})", x, y),
    }
}
```

#### Guards in match

A guard is an extra condition after the pattern:

```rust
fn main() {
    let pair = (2, -2);

    match pair {
        (x, y) if x == y      => println!("equal"),
        (x, y) if x + y == 0  => println!("opposites"),
        (x, _) if x % 2 == 0  => println!("first is even"),
        _                      => println!("other"),
    }
}
```

#### if let — Concise Single-Pattern Matching

When you only care about **one** pattern and want to ignore all others, `if let` is cleaner than `match`:

```rust
fn main() {
    let some_value: Option<i32> = Some(42);

    // With match (verbose):
    match some_value {
        Some(x) => println!("got {}", x),
        None    => {},  // must handle None even if you don't care
    }

    // With if let (clean):
    if let Some(x) = some_value {
        println!("got {}", x);
    }

    // if let with else:
    if let Some(x) = some_value {
        println!("got {}", x);
    } else {
        println!("got nothing");
    }
}
```

Think of `if let` as "if the value matches this pattern, bind and execute."

#### while let — Loop While Pattern Matches

`while let` loops as long as the value matches the pattern:

```rust
fn main() {
    let mut stack = vec![1, 2, 3, 4, 5];

    // Pop elements until the stack is empty
    while let Some(top) = stack.pop() {
        println!("{}", top);  // prints 5, 4, 3, 2, 1
    }
    // When pop() returns None (empty), the while let exits
}
```

`Vec::pop()` returns `Option<T>` — `Some(value)` if there's an element, `None` if empty. `while let` gracefully handles this.

#### Understanding Option<T>

You'll see `Option<T>` constantly in Rust. It replaces `null` from other languages:

```rust
enum Option<T> {
    Some(T),  // a value exists
    None,     // no value
}
```

```rust
fn find_first_even(numbers: &[i32]) -> Option<i32> {
    for &n in numbers {
        if n % 2 == 0 {
            return Some(n);
        }
    }
    None  // no even number found
}

fn main() {
    let nums = [1, 3, 5, 4, 7];
    match find_first_even(&nums) {
        Some(n) => println!("first even: {}", n),
        None    => println!("no even numbers"),
    }
}
```

`Option<T>` forces you to explicitly handle the "no value" case — you cannot accidentally dereference null.

---

## Code Example

### Mini Project: Number Guessing Game

```rust
use std::io;
use std::io::Write;
use std::cmp::Ordering;

fn get_random_number(min: u32, max: u32) -> u32 {
    // Simple LCG pseudo-random (no external crate needed)
    // In a real project, use the `rand` crate
    use std::time::{SystemTime, UNIX_EPOCH};
    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();
    (seed % (max - min + 1)) + min
}

fn read_guess() -> Option<u32> {
    print!("Your guess: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let trimmed = input.trim();
    if trimmed == "quit" || trimmed == "q" {
        return None;
    }

    match trimmed.parse::<u32>() {
        Ok(n) => Some(n),
        Err(_) => {
            println!("Please enter a valid number (or 'quit' to exit).");
            None
        }
    }
}

fn main() {
    println!("=== Number Guessing Game ===");
    println!("I'm thinking of a number between 1 and 100.");
    println!("Type 'quit' to exit.\n");

    let secret = get_random_number(1, 100);
    let mut attempts = 0;
    let max_attempts = 7;

    loop {
        println!("Attempts left: {}", max_attempts - attempts);

        let guess = loop {
            match read_guess() {
                Some(n) => break n,
                None    => {
                    println!("Game over. The number was {}.", secret);
                    return;
                }
            }
        };

        attempts += 1;

        match guess.cmp(&secret) {
            Ordering::Less    => println!("Too low!"),
            Ordering::Greater => println!("Too high!"),
            Ordering::Equal   => {
                println!("\nCorrect! You got it in {} attempt{}!",
                    attempts,
                    if attempts == 1 { "" } else { "s" }
                );
                break;
            }
        }

        if attempts >= max_attempts {
            println!("\nOut of attempts. The number was {}.", secret);
            break;
        }

        println!();
    }
}
```

### Line-by-Line Explanation

```rust
use std::cmp::Ordering;
```
- `Ordering` is an enum with three variants: `Less`, `Equal`, `Greater`
- Returned by `.cmp()` when comparing values

```rust
fn get_random_number(min: u32, max: u32) -> u32 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();
    (seed % (max - min + 1)) + min
}
```
- A simple pseudo-random number using the nanosecond portion of the current time as seed
- `subsec_nanos()` returns the nanosecond sub-second component — changes rapidly
- The modulo maps the nanoseconds into the range `[0, max-min]`, then adds `min` to shift into `[min, max]`
- **Note**: This is NOT cryptographically secure or uniformly distributed. For production, use the `rand` crate.

```rust
match trimmed.parse::<u32>() {
```
- `.parse::<u32>()` uses a **turbofish** `::<u32>` to specify the type to parse into
- Returns `Result<u32, ParseIntError>` — `Ok(number)` on success, `Err(e)` on failure

```rust
        let guess = loop {
            match read_guess() {
                Some(n) => break n,
                None    => { ... return; }
            }
        };
```
- A `loop` that keeps asking until the user enters a valid number or quits
- `break n` — returns `n` as the value of the loop expression, assigned to `guess`

```rust
        match guess.cmp(&secret) {
            Ordering::Less    => println!("Too low!"),
            Ordering::Greater => println!("Too high!"),
            Ordering::Equal   => { ... break; }
        }
```
- `.cmp()` compares `guess` with `secret` and returns an `Ordering`
- `match` is exhaustive — all three variants of `Ordering` must be handled

```rust
                if attempts == 1 { "" } else { "s" }
```
- Using `if` as an expression inline to choose singular/plural

---

## Common Mistakes

### Mistake 1: Non-exhaustive match

```rust
let x: i32 = 5;

match x {
    1 => println!("one"),
    2 => println!("two"),
    // ERROR: non-exhaustive patterns: `i32::MIN..=0_i32` and `3_i32..=i32::MAX`
}

// Fix: add a wildcard
match x {
    1 => println!("one"),
    2 => println!("two"),
    _ => println!("something else"),
}
```

### Mistake 2: Shadowing in match arms

```rust
let x = Some(5);
let y = 10;

match x {
    Some(y) => println!("matched y = {}", y), // y here is 5 (new binding), NOT 10
    None    => println!("nothing"),
}
// The y in Some(y) creates a NEW variable y that shadows the outer y
```

This is a common source of confusion. If you want to match against an existing variable, use a guard:

```rust
match x {
    Some(n) if n == y => println!("matched outer y"),
    Some(n)           => println!("other value: {}", n),
    None              => println!("nothing"),
}
```

### Mistake 3: Using `while` where `for` is clearer

```rust
// Avoid: verbose, error-prone
let mut i = 0;
while i < arr.len() {
    println!("{}", arr[i]);
    i += 1;
}

// Prefer: clear, idiomatic
for element in arr {
    println!("{}", element);
}
```

### Mistake 4: Forgetting loop returns unit by default

```rust
let x = loop {
    break; // returns ()
};
// x is ()

let x = loop {
    break 42; // returns i32
};
// x is 42
```

---

## Best Practices

1. **Use `for` for iteration** — prefer `for x in collection` over index-based `while`
2. **Use `if let` when you only care about one pattern** — cleaner than `match` with a `_ => {}` arm
3. **Always handle `_` in `match`** unless you can provably enumerate all cases (like with enums you define)
4. **Use `while let` for pop-based loops** — it expresses the pattern clearly
5. **Use loop labels** when breaking out of nested loops — clearer than flag variables
6. **Use `.cmp()` with `match`** for three-way comparisons — don't use a chain of if/else if

---

## Exercises

### Exercise 1: FizzBuzz

Print numbers from 1 to 50. For multiples of 3, print "Fizz". For multiples of 5, print "Buzz". For multiples of both, print "FizzBuzz".

### Exercise 2: Grade Letter

Write a function `grade_letter(score: u32) -> &'static str` that returns a letter grade. Use `match` with ranges. Test it with 95, 82, 71, 65, 45.

### Exercise 3: Loop with Return Value

Use a `loop` to find the first number between 1 and 1000 that is divisible by both 7 and 13. Print it.

### Exercise 4: Pattern Matching Shapes

Define variables for three shapes using tuples:
- Circle: `("circle", 5.0)` where the f64 is the radius
- Rectangle: `("rect", 4.0, 6.0)` where f64s are width and height
- Triangle: `("tri", 3.0, 4.0, 5.0)` where f64s are three sides

Use `match` to calculate and print the area of each. (Circle: πr², Rect: w×h, Triangle: Heron's formula)

### Exercise 5: while let Stack

Create a `Vec<i32>` with values `[1, 2, 3, 4, 5]`. Use `while let` to pop and print each value. Verify the vec is empty afterwards.

---

## Solutions

### Solution 1

```rust
fn main() {
    for i in 1..=50 {
        match (i % 3, i % 5) {
            (0, 0) => println!("FizzBuzz"),
            (0, _) => println!("Fizz"),
            (_, 0) => println!("Buzz"),
            _      => println!("{}", i),
        }
    }
}
```

### Solution 2

```rust
fn grade_letter(score: u32) -> &'static str {
    match score {
        90..=100 => "A",
        80..=89  => "B",
        70..=79  => "C",
        60..=69  => "D",
        0..=59   => "F",
        _        => "Invalid",
    }
}

fn main() {
    for score in [95, 82, 71, 65, 45] {
        println!("{} → {}", score, grade_letter(score));
    }
}
```

### Solution 3

```rust
fn main() {
    let result = loop {
        static mut COUNTER: u32 = 0;
        unsafe { COUNTER += 1; }
        let i = unsafe { COUNTER };
        if i % 7 == 0 && i % 13 == 0 {
            break i;
        }
        if i > 1000 {
            break 0;
        }
    };
    // Cleaner version:
    let result = (1..=1000).find(|&n| n % 7 == 0 && n % 13 == 0);
    match result {
        Some(n) => println!("First divisible by 7 and 13: {}", n),
        None    => println!("None found"),
    }
}
```

### Solution 4

```rust
fn main() {
    let pi = std::f64::consts::PI;

    let shapes: Vec<(&str, Vec<f64>)> = vec![
        ("circle", vec![5.0]),
        ("rect",   vec![4.0, 6.0]),
        ("tri",    vec![3.0, 4.0, 5.0]),
    ];

    for (shape, dims) in &shapes {
        let area = match *shape {
            "circle" => pi * dims[0] * dims[0],
            "rect"   => dims[0] * dims[1],
            "tri"    => {
                let (a, b, c) = (dims[0], dims[1], dims[2]);
                let s = (a + b + c) / 2.0;
                (s * (s - a) * (s - b) * (s - c)).sqrt()
            }
            _ => 0.0,
        };
        println!("{}: area = {:.2}", shape, area);
    }
}
```

### Solution 5

```rust
fn main() {
    let mut stack = vec![1, 2, 3, 4, 5];

    while let Some(top) = stack.pop() {
        println!("popped: {}", top);
    }

    println!("stack is empty: {}", stack.is_empty());
}
```

---

## Quiz

**Q1.** What does the `_` pattern mean in a `match` expression?

a) Match nothing  
b) Match only variables  
c) Match any value (wildcard, catch-all)  
d) Match the unit type  

**Q2.** What is the key difference between `if let` and `match`?

a) `if let` is faster  
b) `if let` handles one pattern; `match` handles many and is exhaustive  
c) `match` can only match integers  
d) `if let` requires no patterns  

**Q3.** How does `loop { break 42; }` differ from `loop { break; }`?

a) No difference  
b) The first returns `42` as the loop's value; the second returns `()`  
c) The first is a syntax error  
d) The second is an infinite loop  

**Q4.** What does `while let Some(x) = stack.pop()` do when the stack is empty?

a) Panics  
b) Loops forever  
c) Exits the loop because `pop()` returns `None` and the pattern doesn't match  
d) Returns 0  

**Q5.** Which loop type is most idiomatic in Rust for iterating a collection?

a) `loop` with a counter  
b) `while` with an index  
c) `for` with direct iteration  
d) `while let` with an iterator  

---

## Quiz Answers

**A1.** c) Match any value (wildcard, catch-all)  
*`_` matches any value without binding it. It's required in most `match` expressions because `match` must be exhaustive.*

**A2.** b) `if let` handles one pattern; `match` handles many and is exhaustive  
*Use `if let` when you care about one case. Use `match` when you care about multiple distinct cases or need the compiler to enforce exhaustiveness.*

**A3.** b) The first returns `42` as the loop's value; the second returns `()`  
*`break value` makes the `loop` expression return that value. This is unique to Rust's `loop` — `while` and `for` cannot return values this way.*

**A4.** c) Exits the loop because `pop()` returns `None` and the pattern doesn't match  
*`while let Some(x) = expr` continues only as long as `expr` returns `Some(...)`. When it returns `None`, the pattern fails to match and the loop exits.*

**A5.** c) `for` with direct iteration  
*`for x in collection` is the idiomatic Rust loop. It uses the iterator protocol, is safe (no bounds errors), and expresses intent clearly.*

---

## Chapter Summary

- `if` is an **expression** in Rust — it can return a value and be used in `let` bindings
- Rust does not have truthy/falsy values — conditions must be explicitly `bool`
- **`loop`** is an infinite loop; `break value` makes it return a value
- **`while`** loops while a condition is true
- **`for`** is the idiomatic loop for iterating collections and ranges
- Loop labels (`'label:`) allow `break` and `continue` to target specific loops
- **`match`** is exhaustive pattern matching — all cases must be handled
- `match` can match integers, ranges, tuples, enums, and more
- Match **guards** (`if condition`) add extra conditions to a pattern
- **`if let`** is concise single-pattern matching — equivalent to a `match` with one arm
- **`while let`** loops while a pattern matches — perfect for processing `Option<T>` values
- **`Option<T>`** replaces null — `Some(value)` or `None`

In Chapter 4, we explore functions: how to declare them, pass parameters, return values, and understand the crucial distinction between expressions and statements.
