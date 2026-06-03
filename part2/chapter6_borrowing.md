# Chapter 6: Borrowing and References

## Learning Objectives

By the end of this chapter, you will:

- Understand what references are and how they work in memory
- Use immutable references (`&T`) to read data without ownership
- Use mutable references (`&mut T`) to modify data without ownership
- Understand the two borrowing rules and why they exist
- Recognize and fix dangling reference errors
- Build a Text Analyzer using borrowing throughout

---

## Theory

### 6.1 References

In Chapter 5, we saw that passing a `String` to a function moves it — the caller loses access. This is inconvenient when you just want to read data:

```rust
fn calculate_length(s: String) -> (String, usize) {
    let len = s.len();
    (s, len)  // must return s to get ownership back!
}
```

Returning a tuple just to hand back ownership is verbose. **References** solve this elegantly.

#### What is a Reference?

A reference is like a pointer: it holds the memory address of some data. But unlike a pointer, Rust guarantees that a reference always points to valid data — it can never be null, and it can never outlive the data it points to.

Creating a reference is called **borrowing**. You "borrow" the value — you can use it, but you don't own it. When you're done, the owner still has their value.

```rust
fn calculate_length(s: &String) -> usize {  // takes a reference
    s.len()
}  // s (the reference) goes out of scope, but the heap data is NOT dropped
   // because this function doesn't own the data

fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);  // pass a reference with &
    println!("The length of '{}' is {}.", s1, len);
    // s1 is STILL VALID! It was borrowed, not moved.
}
```

#### Reference Syntax

```rust
let s = String::from("hello");
let r = &s;   // r is a reference to s, r borrows s

println!("{}", r);   // use r to read through the reference
println!("{}", s);   // s is still valid — only borrowed, not moved
```

```
Memory visualization:

Stack:
s:  ┌─────────────┐      Heap:
    │ ptr ─────────┼─────►│ hello │
    │ len: 5      │      └───────┘
    │ cap: 5      │
    └─────────────┘
          ▲
r:  ┌─────┼─────┐
    │ ptr ─┘    │   ← r points to s (the stack part)
    └───────────┘
```

A reference points to the stack variable `s`, which in turn has a pointer to the heap data. The reference holds no ownership.

#### Automatic Dereferencing

You generally don't need to manually dereference references when calling methods:

```rust
let s = String::from("hello world");
let r = &s;

// Both work — Rust auto-dereferences:
println!("{}", r.len());     // Rust inserts * for you
println!("{}", (*r).len());  // explicit dereference — same thing
```

Rust's **auto-deref** feature automatically applies `*` when needed, so you rarely need to write `*` explicitly when calling methods.

#### Multiple Immutable References

You can have as many immutable references as you want simultaneously:

```rust
fn main() {
    let s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    let r3 = &s;

    println!("{} {} {}", r1, r2, r3);  // all valid
}
```

This is safe because all references are read-only. Multiple readers never conflict.

---

### 6.2 Mutable References

What if you need to modify data through a reference? Use `&mut`:

```rust
fn change(s: &mut String) {
    s.push_str(", world");
}

fn main() {
    let mut s = String::from("hello");  // s must be mut
    change(&mut s);                      // pass mutable reference
    println!("{}", s);                   // hello, world
}
```

Requirements:
1. The variable itself must be declared `mut`
2. The reference must be `&mut`
3. Only **one** mutable reference at a time (the key rule!)

#### The One Mutable Reference Rule

At any given time, you can have **either:**
- Any number of immutable references (`&T`)
- **OR** exactly one mutable reference (`&mut T`)

But **never both** at the same time.

```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;  // ERROR: cannot borrow `s` as mutable more than once

    println!("{} {}", r1, r2);
}
```

And you cannot mix immutable and mutable:

```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &s;       // immutable borrow
    let r2 = &s;       // ok — multiple immutable borrows allowed
    let r3 = &mut s;   // ERROR: cannot borrow as mutable while immutable borrows exist

    println!("{} {} {}", r1, r2, r3);
}
```

#### Why This Rule?

This rule prevents **data races** — two threads simultaneously accessing the same memory where at least one is writing. In Rust, this is enforced at compile time, even for single-threaded code.

Consider a vector:

```rust
let mut v = vec![1, 2, 3];
let first = &v[0];   // reference to first element
v.push(4);           // ERROR: cannot borrow `v` as mutable while borrowed

// WHY?
// push() might cause reallocation — it moves the entire Vec to a new
// memory location. The reference `first` would then point to FREED memory!
// Rust catches this at compile time.
println!("{}", first);
```

This exact pattern causes crashes in C++ iterators — Rust prevents it statically.

#### Non-Lexical Lifetimes (NLL)

Modern Rust uses **Non-Lexical Lifetimes** — a borrow ends when it's last used, not at the end of its enclosing scope:

```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &s;       // immutable borrow begins
    let r2 = &s;       // second immutable borrow
    println!("{} {}", r1, r2);  // r1 and r2 last used HERE — borrows end here

    let r3 = &mut s;   // OK! r1 and r2 are no longer active
    r3.push_str(" world");
    println!("{}", r3);
}
```

This makes the borrow checker much less restrictive in practice — borrows end at their last use, not at the closing brace of the block.

---

### 6.3 Borrowing Rules

The complete borrowing rules:

```
Rule 1: At any given time, you can have EITHER:
          - Any number of immutable references (&T)
          - OR exactly one mutable reference (&mut T)
        But NOT both simultaneously.

Rule 2: References must always be valid (no dangling references).
```

These two rules together guarantee:
- **No data races** (Rule 1)
- **No use-after-free** (Rule 2)

#### The Borrow as a Read/Write Lock

Think of Rust's borrow rules as a compile-time read/write lock:

```
Immutable borrows = read locks
  Multiple threads can read simultaneously
  You cannot write while there are readers

Mutable borrow = write lock
  Only one writer at a time
  No readers allowed while writing
```

Except Rust enforces this at **compile time** rather than runtime — no overhead, no risk of deadlock.

#### Practical Examples

```rust
fn main() {
    let mut data = vec![1, 2, 3, 4, 5];

    // Read pattern — multiple borrows OK
    let sum: i32 = data.iter().sum();
    let max = data.iter().max().unwrap();
    let min = data.iter().min().unwrap();
    println!("sum={}, max={}, min={}", sum, max, min);

    // Write pattern — one mutable borrow
    for x in &mut data {
        *x *= 2;
    }
    println!("{:?}", data);

    // Find then modify — need to be careful
    // This won't compile — can't hold &data while mutating:
    // let first = &data[0];
    // data.push(6);         // ERROR
    // println!("{}", first);

    // Instead: find the value first, then modify
    let first_val = data[0];  // copy the value (i32 is Copy)
    data.push(6);
    println!("first was {}, now data is {:?}", first_val, data);
}
```

---

### 6.4 Dangling References

A dangling reference (or dangling pointer) is a reference that points to memory that has been freed. Rust's compiler prevents these entirely.

#### What is a Dangling Reference?

```c
// In C — this compiles and is undefined behavior:
int* dangle() {
    int x = 5;      // x is on the stack
    return &x;       // return pointer to x
}                    // x is freed here (stack frame popped)
// caller now has a pointer to freed stack memory — DANGER
```

#### Rust Prevents This

```rust
fn dangle() -> &String {         // ERROR: function returns a reference
    let s = String::from("hello");  // s is created
    &s                              // return reference to s
}                                   // s is dropped here! reference would dangle

fn main() {
    let reference_to_nothing = dangle();
}
```

Compiler error:
```
error[E0106]: missing lifetime specifier
  --> src/main.rs:1:16
   |
1  | fn dangle() -> &String {
   |                ^ expected named lifetime parameter
```

Rust rejects this because the reference would outlive the data it points to.

**The fix:** return the owned value instead:

```rust
fn no_dangle() -> String {    // return the String, not a reference
    let s = String::from("hello");
    s   // ownership is moved out — not dropped
}
```

#### More Dangling Reference Scenarios

```rust
fn main() {
    let r;

    {
        let x = 5;
        r = &x;     // ERROR: x does not live long enough
    }               // x dropped here

    println!("{}", r);  // r would point to freed memory — Rust prevents this
}
```

Compiler error:
```
error[E0597]: `x` does not live long enough
  --> src/main.rs:5:9
   |
5  |         r = &x;
   |             ^^ borrowed value does not live long enough
6  |     }
   |     - `x` dropped here while still borrowed
```

Rust compares **lifetimes** — the region of code where a reference is valid must not exceed the lifetime of the data it references. We'll explore lifetimes explicitly in Part 7.

#### The Lifetime Intuition

```
Rule of thumb: A reference cannot outlive the value it refers to.

"Owner" lifetime: the region from declaration to drop
"Borrow" lifetime: the region from reference creation to last use

Rust requires: borrow lifetime ⊆ owner lifetime
```

---

## Code Example

### Practice: String Manipulation with Borrowing

```rust
fn count_words(s: &str) -> usize {
    s.split_whitespace().count()
}

fn count_chars(s: &str) -> usize {
    s.chars().count()  // chars() handles Unicode correctly
}

fn count_bytes(s: &str) -> usize {
    s.len()  // len() returns byte count
}

fn to_title_case(s: &str) -> String {
    s.split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => {
                    let upper: String = first.to_uppercase().collect();
                    upper + chars.as_str()
                }
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}

fn append_exclamation(s: &mut String) {
    s.push('!');
}

fn main() {
    let text = "the quick brown fox jumps over the lazy dog";

    // All of these borrow text — text is never moved
    println!("Words: {}", count_words(text));
    println!("Chars: {}", count_chars(text));
    println!("Bytes: {}", count_bytes(text));
    println!("Title: {}", to_title_case(text));

    let mut greeting = String::from("Hello");
    append_exclamation(&mut greeting);
    println!("{}", greeting);  // Hello!
}
```

### Mini Project: Text Analyzer

```rust
use std::collections::HashMap;
use std::io;
use std::io::Write;

struct TextAnalyzer {
    text: String,
}

impl TextAnalyzer {
    fn new(text: String) -> TextAnalyzer {
        TextAnalyzer { text }
    }

    fn word_count(&self) -> usize {
        self.text.split_whitespace().count()
    }

    fn char_count(&self) -> usize {
        self.text.chars().count()
    }

    fn char_count_no_spaces(&self) -> usize {
        self.text.chars().filter(|c| !c.is_whitespace()).count()
    }

    fn line_count(&self) -> usize {
        if self.text.is_empty() { 0 } else { self.text.lines().count() }
    }

    fn sentence_count(&self) -> usize {
        self.text.chars().filter(|&c| c == '.' || c == '!' || c == '?').count()
    }

    fn average_word_length(&self) -> f64 {
        let words: Vec<&str> = self.text.split_whitespace().collect();
        if words.is_empty() { return 0.0; }
        let total_chars: usize = words.iter().map(|w| w.chars().count()).sum();
        total_chars as f64 / words.len() as f64
    }

    fn word_frequency(&self) -> HashMap<String, usize> {
        let mut freq = HashMap::new();
        for word in self.text.split_whitespace() {
            let clean: String = word.chars()
                .filter(|c| c.is_alphabetic())
                .map(|c| c.to_ascii_lowercase())
                .collect();
            if !clean.is_empty() {
                *freq.entry(clean).or_insert(0) += 1;
            }
        }
        freq
    }

    fn top_words(&self, n: usize) -> Vec<(String, usize)> {
        let mut freq: Vec<(String, usize)> = self.word_frequency().into_iter().collect();
        freq.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
        freq.into_iter().take(n).collect()
    }

    fn longest_word(&self) -> Option<&str> {
        self.text.split_whitespace()
            .max_by_key(|w| w.len())
    }

    fn contains_word(&self, word: &str) -> bool {
        let target = word.to_ascii_lowercase();
        self.text.split_whitespace()
            .any(|w| {
                w.chars()
                 .filter(|c| c.is_alphabetic())
                 .map(|c| c.to_ascii_lowercase())
                 .collect::<String>() == target
            })
    }

    fn append_text(&mut self, additional: &str) {
        if !self.text.is_empty() {
            self.text.push(' ');
        }
        self.text.push_str(additional);
    }

    fn report(&self) {
        println!("\n=== Text Analysis Report ===");
        println!("Characters (total):    {:>8}", self.char_count());
        println!("Characters (no space): {:>8}", self.char_count_no_spaces());
        println!("Words:                 {:>8}", self.word_count());
        println!("Lines:                 {:>8}", self.line_count());
        println!("Sentences (approx):    {:>8}", self.sentence_count());
        println!("Avg word length:       {:>8.2}", self.average_word_length());

        if let Some(longest) = self.longest_word() {
            println!("Longest word:          {:>8} ({})", longest.len(), longest);
        }

        println!("\nTop 5 words:");
        for (i, (word, count)) in self.top_words(5).iter().enumerate() {
            println!("  {}. '{}' — {} time{}", i + 1, word, count, if *count == 1 { "" } else { "s" });
        }
    }
}

fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    println!("=== Text Analyzer ===");
    println!("Enter text to analyze. Type an empty line to finish.\n");

    let mut lines: Vec<String> = Vec::new();
    loop {
        let line = read_input("> ");
        if line.is_empty() {
            break;
        }
        lines.push(line);
    }

    if lines.is_empty() {
        println!("No text entered.");
        return;
    }

    let text = lines.join("\n");
    let mut analyzer = TextAnalyzer::new(text);

    analyzer.report();

    loop {
        println!("\n--- Options ---");
        println!("1. Search for a word");
        println!("2. Append text");
        println!("3. Show report again");
        println!("4. Exit");

        let choice = read_input("Choice: ");

        match choice.as_str() {
            "1" => {
                let word = read_input("Word to search: ");
                if analyzer.contains_word(&word) {
                    println!("'{}' found in the text.", word);
                } else {
                    println!("'{}' not found.", word);
                }
            }
            "2" => {
                let additional = read_input("Text to append: ");
                analyzer.append_text(&additional);
                println!("Text appended. New word count: {}", analyzer.word_count());
            }
            "3" => analyzer.report(),
            "4" | "q" | "quit" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Unknown option."),
        }
    }
}
```

### Line-by-Line Explanation

```rust
fn word_count(&self) -> usize {
```
- `&self` is an immutable reference to the struct itself — read-only access
- The method borrows `self` — it doesn't own or move it

```rust
fn append_text(&mut self, additional: &str) {
    if !self.text.is_empty() {
        self.text.push(' ');
    }
    self.text.push_str(additional);
}
```
- `&mut self` — mutable reference to self; allows modifying fields
- `additional: &str` — borrows the additional text, doesn't take ownership
- `push_str` takes `&str` — it borrows the string slice and appends it to `self.text`

```rust
fn longest_word(&self) -> Option<&str> {
    self.text.split_whitespace()
        .max_by_key(|w| w.len())
}
```
- Returns `Option<&str>` — a reference into the original string
- No allocation! The returned slice points directly into `self.text`
- Lifetime: the returned reference is valid as long as `self` (and thus `self.text`) is valid

```rust
fn top_words(&self, n: usize) -> Vec<(String, usize)> {
    let mut freq: Vec<(String, usize)> = self.word_frequency().into_iter().collect();
    freq.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    freq.into_iter().take(n).collect()
}
```
- Sorts by count descending (`b.1.cmp(&a.1)`), then alphabetically on ties (`.then(a.0.cmp(&b.0))`)
- Returns owned `Vec<(String, usize)>` — caller owns the result

```rust
        if analyzer.contains_word(&word) {
```
- `&word` — borrows the `word` String as `&str`
- `contains_word` takes `&str` — can accept `&String` due to deref coercion

---

## Common Mistakes

### Mistake 1: Holding a reference across a mutation

```rust
let mut v = vec![1, 2, 3];
let first = &v[0];   // immutable borrow of v
v.push(4);           // ERROR: mutable borrow of v while immutable borrow exists
println!("{}", first);

// Fix: end the borrow before mutating
let first_val = v[0];  // copy the value (i32 is Copy)
v.push(4);
println!("{}", first_val);
```

### Mistake 2: Returning reference to local variable

```rust
fn first_word(s: &str) -> &str {
    // This is actually fine — returning reference into the parameter
    let bytes = s.as_bytes();
    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[..i];
        }
    }
    s
}

// But this is WRONG — returning reference to local:
fn get_word() -> &str {
    let s = String::from("hello world");
    let word = s.split_whitespace().next().unwrap();
    word  // ERROR: s is dropped at end of function, word refers to it
}

// Fix: return owned String
fn get_word() -> String {
    let s = String::from("hello world");
    s.split_whitespace().next().unwrap_or("").to_string()
}
```

### Mistake 3: Misunderstanding `&str` vs `&String`

```rust
fn process(s: &String) { ... }  // accepts only &String

fn process(s: &str) { ... }     // accepts &String (auto-deref) AND &str

// Best practice: prefer &str over &String for function parameters
// &str is more general — works with string literals too:
process("a literal");  // works with &str parameter
process("a literal");  // ERROR with &String parameter
```

### Mistake 4: Multiple mutable references

```rust
let mut s = String::from("hello");

let r1 = &mut s;
let r2 = &mut s;  // ERROR: cannot borrow `s` as mutable more than once

println!("{} {}", r1, r2);

// Fix: use r1, drop it, then create r2
let r1 = &mut s;
r1.push_str(" world");
// r1 is last used here — borrow ends

let r2 = &mut s;  // OK! r1 is no longer active
r2.push_str("!");
println!("{}", s);
```

---

## Best Practices

1. **Prefer `&str` over `&String`** for function parameters — more flexible, works with literals
2. **Prefer `&[T]` over `&Vec<T>`** for function parameters — works with both arrays and vectors
3. **End borrows early** — don't hold borrows longer than you need them (NLL helps automatically)
4. **Use `&self`** for read-only methods, `&mut self` for methods that modify state
5. **Design function signatures to take references** unless the function needs to own the data
6. **Return owned values from functions** when the data was created inside the function

---

## Exercises

### Exercise 1: Reference Basics

Write a function `first_char(s: &str) -> Option<char>` that returns the first character of a string without taking ownership.

### Exercise 2: Mutable Reference Practice

Write `double_all(v: &mut Vec<i32>)` that doubles every element in place. Show the vector is modified after the call.

### Exercise 3: Multiple Borrows

Show that you can have multiple immutable borrows simultaneously, then show that you cannot have both immutable and mutable borrows. Demonstrate Non-Lexical Lifetimes.

### Exercise 4: String Analysis

Write `analyze(text: &str)` that prints: character count, word count, whether the text starts with uppercase. The function should borrow, not take ownership.

### Exercise 5: Mutable and Immutable

Given a `Vec<String>`, write:
- `longest(v: &[String]) -> Option<&String>` — find and return a reference to the longest string
- `shorten_all(v: &mut Vec<String>, max_len: usize)` — truncate all strings to max_len characters

---

## Solutions

### Solution 1

```rust
fn first_char(s: &str) -> Option<char> {
    s.chars().next()
}

fn main() {
    println!("{:?}", first_char("hello"));   // Some('h')
    println!("{:?}", first_char(""));         // None
    println!("{:?}", first_char("🦀rust"));  // Some('🦀')
}
```

### Solution 2

```rust
fn double_all(v: &mut Vec<i32>) {
    for x in v.iter_mut() {
        *x *= 2;
    }
}

fn main() {
    let mut numbers = vec![1, 2, 3, 4, 5];
    println!("Before: {:?}", numbers);
    double_all(&mut numbers);
    println!("After:  {:?}", numbers);
}
```

### Solution 3

```rust
fn main() {
    let mut s = String::from("hello");

    // Multiple immutable borrows — OK
    let r1 = &s;
    let r2 = &s;
    println!("r1={}, r2={}", r1, r2);  // r1 and r2 last used here — borrows end

    // Mutable borrow after immutables are done — OK (NLL)
    let r3 = &mut s;
    r3.push_str(" world");
    println!("r3={}", r3);

    // Trying to have both simultaneously — ERROR
    // let r4 = &s;
    // let r5 = &mut s;  // ERROR
    // println!("{} {}", r4, r5);
}
```

### Solution 4

```rust
fn analyze(text: &str) {
    let char_count = text.chars().count();
    let word_count = text.split_whitespace().count();
    let starts_upper = text.chars().next().map_or(false, |c| c.is_uppercase());

    println!("Characters: {}", char_count);
    println!("Words: {}", word_count);
    println!("Starts with uppercase: {}", starts_upper);
}

fn main() {
    let text = String::from("Hello world, how are you?");
    analyze(&text);       // borrow
    println!("{}", text); // still valid!
}
```

### Solution 5

```rust
fn longest(v: &[String]) -> Option<&String> {
    v.iter().max_by_key(|s| s.len())
}

fn shorten_all(v: &mut Vec<String>, max_len: usize) {
    for s in v.iter_mut() {
        if s.chars().count() > max_len {
            *s = s.chars().take(max_len).collect();
        }
    }
}

fn main() {
    let mut words = vec![
        String::from("short"),
        String::from("medium word"),
        String::from("a very long string indeed"),
    ];

    println!("Longest: {:?}", longest(&words));
    shorten_all(&mut words, 8);
    println!("After shortening: {:?}", words);
}
```

---

## Quiz

**Q1.** What is the difference between `&T` and `&mut T`?

a) `&T` is faster  
b) `&T` is read-only; `&mut T` allows modification  
c) `&mut T` creates a copy; `&T` creates a reference  
d) There is no practical difference  

**Q2.** How many mutable references to the same value can exist at the same time?

a) Unlimited  
b) Two  
c) Exactly one  
d) Zero — mutable references don't exist  

**Q3.** Why does the borrow checker prevent you from holding an immutable reference while also having a mutable reference?

a) Performance reasons  
b) To prevent data races — a writer could modify data while a reader is reading it  
c) Syntax limitation  
d) To save memory  

**Q4.** What is a dangling reference?

a) A reference that's too long  
b) A reference that points to memory that has been freed  
c) A reference to a null pointer  
d) A reference inside a loop  

**Q5.** What does Non-Lexical Lifetimes (NLL) change about borrow lifetimes?

a) Borrows last forever  
b) Borrows end at the closing brace of the block, never before  
c) Borrows end at their last use, which may be before the block ends  
d) Borrows are removed from the language  

---

## Quiz Answers

**A1.** b) `&T` is read-only; `&mut T` allows modification  
*`&T` gives shared read access — you cannot call methods that take `&mut self` on it. `&mut T` gives exclusive write access.*

**A2.** c) Exactly one  
*Rust enforces "exclusive mutable access." Only one part of code can modify data at a time — this prevents data races at compile time.*

**A3.** b) To prevent data races — a writer could modify data while a reader is reading it  
*If you could have both simultaneously: the writer moves the data (e.g., realloc), the reader now points to freed memory. Rust prevents this at compile time.*

**A4.** b) A reference that points to memory that has been freed  
*Dangling references cause crashes and security vulnerabilities in C/C++. Rust's lifetime system prevents them at compile time — the compiler rejects code that would create them.*

**A5.** c) Borrows end at their last use, which may be before the block ends  
*Pre-NLL Rust would say a borrow lasts until the end of the block, making the borrow checker unnecessarily strict. NLL makes borrows end at their last actual use, allowing more code to compile.*

---

## Chapter Summary

- **References** let you access data without taking ownership — called **borrowing**
- `&T` is an immutable (shared) reference — read-only access, no ownership transfer
- `&mut T` is a mutable (exclusive) reference — read/write access, no ownership transfer
- The **borrowing rules**: either many `&T` OR exactly one `&mut T`, never both simultaneously
- References must always be **valid** — Rust rejects dangling references at compile time
- **Non-Lexical Lifetimes**: borrows end at their last use, not at the end of the block
- Rust's borrow checker enforces these rules with zero runtime overhead — it's all at compile time
- **Auto-deref**: Rust automatically dereferences when calling methods on references
- Prefer `&str` over `&String` and `&[T]` over `&Vec<T>` for function parameters — they're more general

In Chapter 7, we look at **slices** — the primary way to work with portions of strings and arrays using references.
