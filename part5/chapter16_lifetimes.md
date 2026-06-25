# Chapter 16: Lifetimes

## Learning Objectives

By the end of this chapter, you will:

- Understand the memory safety problem that lifetimes solve (dangling references, use-after-free bugs)
- Understand what lifetime annotations are and what they communicate to the Rust compiler
- Read and write lifetime annotations in function signatures, struct definitions, and impl blocks
- Understand the three lifetime elision rules and know when explicit annotations are required
- Understand `'static` and when it applies
- Use lifetime bounds on generic type parameters
- Confidently interpret borrow checker errors related to lifetimes
- Build a complete Document Parser that borrows text without unnecessary allocations

---

## Theory

### 16.1 Why Lifetimes Exist

#### The Memory Safety Problem

To understand why Rust has lifetimes, you first need to understand a class of bugs that plagues systems languages like C and C++: **dangling references** and **use-after-free** errors.

A dangling reference is a pointer or reference that points to memory that has already been freed or gone out of scope. When you access that memory, you are reading garbage — or worse, memory that has been reused for something else entirely. This is one of the most common sources of security vulnerabilities and crashes in C and C++ codebases.

Let's look at how this happens in C:

```c
// C code — THIS IS DANGEROUS
#include <stdio.h>

const char* get_greeting() {
    char greeting[50] = "Hello, world!"; // allocated on the stack
    return greeting;                      // return pointer to stack memory
}                                         // greeting is destroyed here!

int main() {
    const char* message = get_greeting(); // message now points to freed memory
    printf("%s\n", message);              // UNDEFINED BEHAVIOR: dangling pointer
    return 0;
}
```

What happens here:

1. `greeting` is a local array on the stack inside `get_greeting`.
2. The function returns a pointer to that array.
3. When the function returns, its stack frame is cleaned up — `greeting` no longer exists.
4. `message` now points to freed memory.
5. `printf` reads from that freed memory — this is undefined behavior.

Sometimes this appears to work (the memory hasn't been overwritten yet). Sometimes it crashes. Sometimes it reads sensitive data from a previous stack frame. This unpredictability is exactly what makes use-after-free bugs so dangerous.

Here is the same problem in C++ with a reference:

```cpp
// C++ code — THIS IS DANGEROUS
#include <string>

const std::string& get_name() {
    std::string name = "Alice";  // local string on the heap, owned by this scope
    return name;                  // return reference to local variable
}                                 // name is destroyed here!

int main() {
    const std::string& n = get_name(); // dangling reference!
    // Using n here is undefined behavior
    return 0;
}
```

C++ compilers may warn about this, but the code compiles. The bug only manifests at runtime.

#### How Rust Solves This

Rust prevents dangling references **at compile time** using the **borrow checker**. The borrow checker tracks how long every value lives (its **lifetime**) and ensures that no reference ever outlives the value it refers to.

Let's try to write the same dangerous function in Rust:

```rust
fn get_greeting() -> &str {
    let greeting = String::from("Hello, world!");
    &greeting  // ERROR: cannot return reference to local variable
}              // greeting is dropped here
```

The Rust compiler refuses to compile this. It will give you an error like:

```
error[E0106]: missing lifetime specifier
 --> src/main.rs:1:20
  |
1 | fn get_greeting() -> &str {
  |                      ^ expected named lifetime parameter
```

The compiler is telling you: this function returns a reference, but there is nothing for that reference to be borrowed from. The reference would dangle.

#### Lifetimes as a Tracking Mechanism

A **lifetime** is the span of time during program execution when a value is valid in memory. Every value in Rust has a lifetime that begins when the value is created and ends when the value is dropped.

When you take a reference to a value, that reference must not outlive the value it refers to. This is the fundamental rule.

The borrow checker enforces this by tracking lifetimes. For values in simple, local code, the borrow checker can figure out the lifetimes automatically by looking at the scopes. But when references cross function boundaries, the compiler cannot always figure out the relationships between input and output lifetimes on its own — so you annotate them explicitly.

Let's visualize lifetimes with ASCII diagrams:

```
fn main() {
    let x = 5;           // <-- x's lifetime begins here ('x)
    {
        let r = &x;      // <-- r's lifetime begins here ('r)
        println!("{}", r);
    }                    // <-- r's lifetime ends here ('r ends)
    println!("{}", x);
}                        // <-- x's lifetime ends here ('x ends)

Timeline:
+-----------------------------+
| 'x (x is alive)             |
|  +-----------------------+  |
|  | 'r (r borrows x)      |  |
|  |   r is valid: OK      |  |
|  +-----------------------+  |
+-----------------------------+

r's lifetime is SHORTER than x's lifetime. OK.
```

Now let's see what a dangling reference would look like:

```
fn main() {
    let r;                    // r declared but uninitialized
    {
        let x = 5;            // <-- x's lifetime begins ('x)
        r = &x;               // r borrows x
    }                         // <-- x's lifetime ENDS here ('x ends)
    println!("{}", r);        // ERROR: r outlives x!
}

Timeline:
         +---------------+
         | 'x (x alive)  |
         |  r = &x       |
         +---------------+   x is DROPPED
    +----|----------------------------+
    | 'r (r's scope)                 |  r used here — x is GONE
    +---------------------------------+

r's lifetime is LONGER than x's lifetime. COMPILER ERROR.
```

The borrow checker catches this at compile time. No runtime crash, no undefined behavior.

#### What Lifetime Annotations Are (and Are Not)

This is a critical point that confuses many beginners:

**Lifetime annotations do NOT change how long a value lives.** They do not extend or shorten the lifetime of any value. They only describe relationships between reference lifetimes to the compiler.

Think of lifetime annotations as **contracts** or **constraints**: "This output reference will live at least as long as this input reference." The compiler uses these contracts to verify that all references are valid at every point in the code.

---

### 16.2 Lifetime Annotations

#### Syntax

Lifetime annotations use a tick (`'`) followed by a short name, by convention single lowercase letters like `'a`, `'b`, `'c`. They appear after the `&` in reference types:

```
&i32         — a reference (no explicit lifetime, often elided)
&'a i32      — a reference with an explicit lifetime 'a
&'a mut i32  — a mutable reference with an explicit lifetime 'a
```

Lifetime parameters are declared in angle brackets, similar to generic type parameters:

```rust
fn example<'a>(x: &'a str) -> &'a str { x }
//         ^^   ^^               ^^
//  declare 'a  use 'a in input  use 'a in output
```

#### Why We Annotate: The Compiler's Perspective

Consider this function:

```rust
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

The Rust compiler rejects this with an error:

```
error[E0106]: missing lifetime specifier
 --> src/main.rs:1:33
  |
1 | fn longest(x: &str, y: &str) -> &str {
  |               ----     ----     ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but the
    signature does not say whether it is borrowed from `x` or `y`
```

The error message is very clear: the compiler does not know whether the returned reference comes from `x` or `y`. Without knowing this, it cannot check that the returned reference is valid at the call site. The reference could come from either input, depending on which is longer — and that is determined at runtime, not at compile time.

By annotating with `'a`, you tell the compiler: "the return value will live at least as long as the shorter of `x` and `y`":

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

This does not mean `x` and `y` have the same lifetime — it means that the lifetime `'a` is constrained to be the **overlap** (the shorter) of the two input lifetimes. The return reference is valid for at most as long as the shorter-lived input.

Let's visualize this:

```
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str

Call site:
    let string1 = String::from("long string");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
        // 'a is constrained to overlap of string1 and string2 lifetimes
        // which is the lifetime of string2 (shorter)
        println!("{}", result); // OK: result used within string2's lifetime
    }
    // string2 is dropped here
    // println!("{}", result); // ERROR: result could point to dropped string2

Timeline:
+----------------------------------+
| 'string1 alive                   |
|  +---------------------+         |
|  | 'string2 alive      |         |
|  | 'a = overlap        |         |
|  | result valid here   |         |
|  +---------------------+         |
|  result INVALID here             |
+----------------------------------+
```

#### The `'static` Lifetime

The `'static` lifetime means the reference is valid for the **entire duration of the program**. There are two main sources of `'static` references:

1. **String literals** — they are embedded in the program binary and never freed:

```rust
let s: &'static str = "hello, world"; // valid for the entire program
```

2. **Values explicitly made `'static`** — for example, values leaked with `Box::leak` or defined as `static` constants:

```rust
static GREETING: &str = "Hello!"; // 'static
```

You will often see `'static` in error messages or in trait bounds for multi-threaded code. For example, `std::thread::spawn` requires `'static` because the closure may outlive the current scope.

#### Multiple Lifetime Parameters

Sometimes you need multiple independent lifetime parameters:

```rust
// x and the return value share lifetime 'a
// y has its own independent lifetime 'b
fn first_word_from<'a, 'b>(sentence: &'a str, _ignored: &'b str) -> &'a str {
    let bytes = sentence.as_bytes();
    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &sentence[0..i];
        }
    }
    sentence
}
```

Here `'b` is declared but the output only depends on `'a`. The compiler knows the return value does not borrow from `_ignored`, so it doesn't need to constrain the output lifetime to `'b`.

#### Lifetime Bounds on Generic Types

You can combine lifetime annotations with generic type bounds:

```rust
// T must implement Display, and T must live at least as long as 'a
use std::fmt::Display;

fn print_ref<'a, T>(x: &'a T)
where
    T: Display + 'a,
{
    println!("{}", x);
}
```

The `T: 'a` bound means: any references inside `T` must live at least as long as `'a`. This is important when you hold references inside generic types.

---

### 16.3 Lifetime Elision

In early Rust, every reference in a function signature required explicit lifetime annotations. This was verbose. Rust's designers identified common patterns and introduced **lifetime elision rules** — situations where the compiler can infer lifetimes without you writing them.

Lifetime elision does not change the rules — it just lets the compiler fill in the annotations when they are unambiguous. When elision rules do not uniquely determine the lifetimes, the compiler asks you to annotate explicitly.

#### The Three Elision Rules

The rules are applied in order to function signatures (not struct definitions). The compiler assigns a distinct lifetime to each reference parameter, then applies the rules:

**Rule 1 — Each input reference gets its own lifetime parameter.**

```rust
// Written:
fn foo(x: &str) -> &str { ... }
fn bar(x: &str, y: &str) -> &str { ... }

// Compiler expands to:
fn foo<'a>(x: &'a str) -> &str { ... }
fn bar<'a, 'b>(x: &'a str, y: &'b str) -> &str { ... }
```

After Rule 1, we have distinct lifetimes for all input references, but output lifetimes may still be unresolved.

**Rule 2 — If there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters.**

```rust
// Written:
fn foo(x: &str) -> &str { ... }

// After Rule 1:
fn foo<'a>(x: &'a str) -> &str { ... }

// After Rule 2 (one input ref, so output gets 'a):
fn foo<'a>(x: &'a str) -> &'a str { ... }
```

This is why you can write:

```rust
fn first_word(s: &str) -> &str {
    // ... no annotation needed
}
```

The compiler automatically infers that the output has the same lifetime as `s`.

**Rule 3 — If one of the input parameters is `&self` or `&mut self`, the lifetime of `self` is assigned to all output lifetime parameters.**

This rule makes methods ergonomic. When a method returns a reference, it almost always returns something borrowed from `self`, so the compiler assumes this.

```rust
struct Parser<'a> {
    text: &'a str,
}

impl<'a> Parser<'a> {
    // Written:
    fn current_text(&self) -> &str { self.text }

    // Compiler expands (Rule 3 — &self's lifetime assigned to output):
    fn current_text<'b>(&'b self) -> &'b str { self.text }
}
```

#### When Elision Is Not Enough

If after applying all three rules there are still output lifetimes that are not determined, the compiler requires explicit annotations:

```rust
// Two input references, output reference — ambiguous which input the output borrows from
fn longest(x: &str, y: &str) -> &str { ... }
// ERROR: cannot determine lifetime of output from elision rules
// Must write: fn longest<'a>(x: &'a str, y: &'a str) -> &'a str { ... }
```

Rule 1 gives `x` lifetime `'a` and `y` lifetime `'b`. Rule 2 does not apply (two input refs). Rule 3 does not apply (no `self`). So the output lifetime is undetermined — compiler requires explicit annotation.

#### Elision Examples

```rust
// No annotation needed (one input ref, Rule 2 applies):
fn trim(s: &str) -> &str {
    s.trim()
}

// No annotation needed (one input ref, Rule 2 applies):
fn get_bytes(data: &[u8]) -> &[u8] {
    &data[1..]
}

// No annotation needed (method, Rule 3 applies):
struct Config {
    value: String,
}
impl Config {
    fn get_value(&self) -> &str {
        &self.value
    }
}

// Annotation REQUIRED (two input refs, output ambiguous):
fn pick<'a>(a: &'a str, b: &'a str, use_a: bool) -> &'a str {
    if use_a { a } else { b }
}
```

---

### 16.4 Struct Lifetimes

#### Why Structs Need Lifetime Annotations

When a struct holds a reference, the struct cannot outlive the data that reference points to. The compiler needs to know this relationship. Lifetime annotations on structs express this.

Without a lifetime annotation, the compiler would not know how long the struct can live relative to the data it borrows. The annotation ties the struct's validity to the borrowed data's validity.

```rust
// This DOES NOT compile — missing lifetime annotation
struct Excerpt {
    text: &str,  // ERROR: expected named lifetime parameter
}
```

```rust
// Correct: struct borrows a &str that must live at least as long as the struct
struct Excerpt<'a> {
    text: &'a str,
}
```

The `'a` on the struct says: "An `Excerpt` cannot outlive the `&str` it holds in `text`."

#### Using Struct Lifetimes

```rust
struct Excerpt<'a> {
    text: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence;
    {
        let i = novel.find('.').unwrap_or(novel.len());
        first_sentence = &novel[..i];
        // Excerpt borrows from novel
        let excerpt = Excerpt { text: first_sentence };
        println!("Excerpt: {}", excerpt.text);
    }
    // novel is still alive here, so first_sentence is still valid
    println!("First sentence: {}", first_sentence);
}
```

Lifetime diagram for the struct example:

```
+--------------------------------------------------+
| 'novel (novel String is alive)                   |
|  +----------------------------------------------+|
|  | 'first_sentence (borrows from novel)          ||
|  |  +------------------------------------------+||
|  |  | Excerpt { text: first_sentence }          |||
|  |  | struct lifetime ⊆ 'first_sentence         |||
|  |  +------------------------------------------+||
|  +----------------------------------------------+|
+--------------------------------------------------+

All lifetimes nest correctly: struct ⊆ 'first_sentence ⊆ 'novel
```

#### impl Blocks on Structs with Lifetimes

When you write `impl` blocks for structs with lifetime parameters, you must declare the lifetime on the `impl` keyword and use it on the struct name:

```rust
struct Excerpt<'a> {
    text: &'a str,
}

impl<'a> Excerpt<'a> {
    // Rule 3 applies: &self lifetime assigned to output
    fn level(&self) -> i32 {
        3
    }

    // Rule 3 applies: returns a &str borrowed from self
    fn announce(&self, announcement: &str) -> &str {
        println!("Attention: {}", announcement);
        self.text
    }
}
```

In `announce`, there are two input references (`&self` with lifetime `'a` and `announcement` with its own elided lifetime). The output `&str` gets `self`'s lifetime via Rule 3, which is correct because we return `self.text`.

#### Combining Generic Types, Trait Bounds, and Lifetimes

```rust
use std::fmt::Display;

fn longest_with_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    announcement: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement: {}", announcement);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

Here `'a` is a lifetime parameter and `T` is a generic type parameter with a trait bound. Both are declared in the same angle brackets.

---

### 16.5 Lifetime in Functions

#### Single Reference Parameter

When a function takes one reference and returns a reference derived from it, elision handles it automatically:

```rust
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[..i];
        }
    }
    s
}
```

Elision expands this to `fn first_word<'a>(s: &'a str) -> &'a str`. The output borrows from `s`.

#### Multiple Reference Parameters — Same Lifetime

When you want to express that the output could borrow from either of two inputs, use the same lifetime for both and the output:

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

The lifetime `'a` here is the **intersection** (overlap) of the lifetimes of `x` and `y`. The returned reference is valid for the shorter of the two lifetimes.

```
Call: longest(string1.as_str(), string2.as_str())

+------------------------------------+
| 'string1 alive                     |
|  +--------------------------+       |
|  | 'string2 alive           |       |
|  | 'a = shorter of the two  |       |
|  | returned ref valid here  |       |
|  +--------------------------+       |
|  returned ref INVALID here         |
+------------------------------------+
```

#### Multiple Reference Parameters — Different Lifetimes

When inputs have independent lifetimes and the output only depends on one:

```rust
// Output borrows from 'a (from `text`), not from 'b (from `delimiter`)
fn before_delimiter<'a, 'b>(text: &'a str, delimiter: &'b str) -> &'a str {
    match text.find(delimiter) {
        Some(pos) => &text[..pos],
        None => text,
    }
}
```

Here `'b` is the lifetime of `delimiter`. It does not appear in the return type, meaning the returned reference does not borrow from `delimiter`. The compiler knows the output only depends on `'a`.

#### Lifetime Bounds on Generics in Functions

```rust
// T must implement Display and must not contain references shorter than 'a
use std::fmt::Display;

fn print_longest_with_type<'a, T>(x: &'a str, y: &'a str, extra: T) -> &'a str
where
    T: Display + 'a,
{
    println!("Extra: {}", extra);
    if x.len() > y.len() { x } else { y }
}
```

The `T: 'a` bound means T itself (or any references inside T) must be valid for at least `'a`. Without this bound, T could contain short-lived references that become invalid.

#### Functions That Never Return References to Their Inputs

If a function only returns owned values, no lifetime annotations are needed:

```rust
fn count_words(text: &str) -> usize {
    text.split_whitespace().count()
}

fn to_uppercase_owned(text: &str) -> String {
    text.to_uppercase()
}
```

These functions consume references but return owned values, so no output lifetimes are needed.

---

### Code Example

Below is a complete, self-contained program demonstrating all the lifetime concepts covered in this chapter:

```rust
// chapter16_lifetimes.rs
// Complete demonstration of Rust lifetimes

use std::fmt;

// ─────────────────────────────────────────────
// SECTION 1: Basic lifetime in functions
// ─────────────────────────────────────────────

/// Returns the longest of two string slices.
/// 'a is the overlap of the lifetimes of x and y.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

/// Returns the first word in a string slice.
/// Elision applies: output borrows from s (one input ref → Rule 2).
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[..i];
        }
    }
    s
}

/// Returns the substring before the delimiter.
/// Two lifetime params: output borrows from 'a (text), not 'b (delimiter).
fn before_delimiter<'a, 'b>(text: &'a str, delimiter: &'b str) -> &'a str {
    match text.find(delimiter) {
        Some(pos) => &text[..pos],
        None => text,
    }
}

// ─────────────────────────────────────────────
// SECTION 2: Struct with lifetime
// ─────────────────────────────────────────────

/// A paragraph borrowed from a larger text.
/// The struct cannot outlive the text it borrows.
struct Paragraph<'a> {
    content: &'a str,
    line_number: usize,
}

impl<'a> Paragraph<'a> {
    /// Create a new Paragraph borrowing from text.
    fn new(content: &'a str, line_number: usize) -> Self {
        Paragraph { content, line_number }
    }

    /// Return the content — Rule 3 applies (returns borrow from self).
    fn content(&self) -> &str {
        self.content
    }

    /// Return the word count of this paragraph — no ref in output.
    fn word_count(&self) -> usize {
        self.content.split_whitespace().count()
    }

    /// Return a slice of the content up to a given length.
    /// Output borrows from 'a (self.content), via Rule 3 on &self.
    fn preview(&self, max_chars: usize) -> &str {
        if self.content.len() <= max_chars {
            self.content
        } else {
            // Find the last space within max_chars to avoid splitting words
            let boundary = self.content[..max_chars]
                .rfind(' ')
                .unwrap_or(max_chars);
            &self.content[..boundary]
        }
    }
}

impl<'a> fmt::Display for Paragraph<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[Line {}] {}", self.line_number, self.content)
    }
}

// ─────────────────────────────────────────────
// SECTION 3: Combined generics, trait bounds, lifetimes
// ─────────────────────────────────────────────

/// Print the longest string and also a generic announcement.
/// Combines lifetime 'a with generic T bounded by Display + 'a.
fn longest_with_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: fmt::Display + 'a,
{
    println!("  Announcement: {}", ann);
    if x.len() > y.len() { x } else { y }
}

// ─────────────────────────────────────────────
// SECTION 4: 'static lifetime
// ─────────────────────────────────────────────

/// Returns a &'static str — lives for the whole program.
fn get_error_message(code: u32) -> &'static str {
    match code {
        404 => "Not found",
        500 => "Internal server error",
        _ => "Unknown error",
    }
}

// ─────────────────────────────────────────────
// SECTION 5: Multiple structs with lifetimes
// ─────────────────────────────────────────────

/// A highlighted excerpt: borrows from the source text.
struct Highlight<'a> {
    text: &'a str,
    tag: &'a str,
}

impl<'a> Highlight<'a> {
    fn new(text: &'a str, tag: &'a str) -> Self {
        Highlight { text, tag }
    }

    fn display(&self) {
        println!("  [{}] {}", self.tag, self.text);
    }
}

// ─────────────────────────────────────────────
// MAIN
// ─────────────────────────────────────────────

fn main() {
    println!("=== Chapter 16: Lifetimes Demo ===\n");

    // --- longest() ---
    println!("--- longest() ---");
    let s1 = String::from("long string is long");
    let result;
    {
        let s2 = String::from("xyz");
        result = longest(s1.as_str(), s2.as_str());
        println!("  Longest: {}", result);
        // result is only used inside this block, within s2's lifetime
    }

    // --- first_word() ---
    println!("\n--- first_word() ---");
    let sentence = String::from("hello world from Rust");
    let word = first_word(&sentence);
    println!("  First word: {}", word);

    // --- before_delimiter() ---
    println!("\n--- before_delimiter() ---");
    let full_text = String::from("name: Alice; age: 30");
    let delimiter = String::from(":");
    let before = before_delimiter(&full_text, &delimiter);
    println!("  Before ':': '{}'", before);

    // --- Paragraph struct ---
    println!("\n--- Paragraph struct ---");
    let document = String::from(
        "Rust is a systems programming language. It runs blazingly fast. \
         Memory safety is guaranteed without garbage collection.",
    );
    let para = Paragraph::new(&document, 1);
    println!("  Content: {}", para.content());
    println!("  Words: {}", para.word_count());
    println!("  Preview (30 chars): '{}'", para.preview(30));
    println!("  Display: {}", para);

    // --- longest_with_announcement() ---
    println!("\n--- longest_with_announcement() ---");
    let s3 = String::from("abcdef");
    let s4 = String::from("xy");
    let longer = longest_with_announcement(
        s3.as_str(),
        s4.as_str(),
        "Comparing string lengths",
    );
    println!("  Result: {}", longer);

    // --- 'static lifetime ---
    println!("\n--- 'static lifetime ---");
    let msg = get_error_message(404);
    println!("  Error 404: {}", msg);

    // --- Highlight struct ---
    println!("\n--- Highlight struct ---");
    let source = String::from("fearless concurrency in Rust");
    let tag_name = String::from("IMPORTANT");
    let highlight = Highlight::new(&source, &tag_name);
    highlight.display();

    println!("\n=== Demo complete ===");
}
```

---

### Line-by-Line Explanation

Let's walk through the key parts of the code:

**`fn longest<'a>(x: &'a str, y: &'a str) -> &'a str`**

- `<'a>` declares the lifetime parameter `'a` for this function.
- `x: &'a str` — `x` is a reference to a `str` that is valid for at least `'a`.
- `y: &'a str` — `y` is also valid for at least `'a`.
- `-> &'a str` — the return value is a reference valid for `'a`.
- Together: the return value is valid for the overlap (shorter) of x's and y's lifetimes.

**`fn first_word(s: &str) -> &str`**

- No explicit lifetimes needed. Elision Rule 2 applies: one input reference, so the output gets the same lifetime.
- The body iterates bytes looking for a space, returns a slice of `s`.
- `&s[..i]` and `s` both borrow from `s`, which matches the elided annotation.

**`fn before_delimiter<'a, 'b>(text: &'a str, delimiter: &'b str) -> &'a str`**

- Two lifetime parameters: `'a` for `text`, `'b` for `delimiter`.
- The return type uses `'a` only, telling the compiler the returned reference borrows from `text`, not from `delimiter`.
- `text.find(delimiter)` — finds the position where delimiter starts.
- `&text[..pos]` — returns a slice of `text` (lifetime `'a`).

**`struct Paragraph<'a> { content: &'a str, line_number: usize }`**

- The struct holds a `&'a str`. The struct itself cannot outlive `'a`.
- `line_number: usize` is an owned type — no lifetime annotation needed for it.

**`impl<'a> Paragraph<'a>`**

- We re-declare `'a` on `impl` and use it on `Paragraph<'a>`.
- Inside the impl block, methods can use `'a` to refer to the struct's lifetime parameter.

**`fn content(&self) -> &str`**

- Rule 3 applies: `&self` is the only input reference relevant here, so the output borrows from `self`.
- The compiler expands this to `fn content<'b>(&'b self) -> &'b str` where `'b` is constrained to be within `'a`.

**`fn preview(&self, max_chars: usize) -> &str`**

- `max_chars` is `usize` — not a reference, no lifetime.
- Output gets `self`'s lifetime via Rule 3.
- `self.content[..max_chars].rfind(' ')` — searches for the last space for a clean word break.

**`fn get_error_message(code: u32) -> &'static str`**

- Returns a `&'static str` — a string literal embedded in the binary.
- All `match` arms return string literals, which are `'static`. No reference to local memory.

**In `main()`:**

- `result` is declared outside the block, then assigned inside. The `println!` using `result` is inside the block — within `s2`'s lifetime. This is valid.
- If we moved the `println!` after the closing brace (after `s2` is dropped), the compiler would reject it.

---

### Common Mistakes

#### Mistake 1: Thinking Lifetime Annotations Change How Long Values Live

```rust
// WRONG mental model:
fn foo<'a>(x: &'a str) -> &'a str {
    // "I'm giving x a long lifetime by annotating it" — NO
    // 'a is just a label for an existing lifetime, not a command
    x
}
```

Lifetime annotations are descriptive, not prescriptive. They describe relationships. They cannot make a value live longer.

#### Mistake 2: Over-Annotating When Elision Handles It

```rust
// VERBOSE and unnecessary:
fn first_word<'a>(s: &'a str) -> &'a str {
    // ...
}

// IDIOMATIC — elision handles it:
fn first_word(s: &str) -> &str {
    // ...
}
```

If elision can determine the lifetimes unambiguously, omitting the annotations is preferred. Over-annotating is not wrong, but it clutters the code.

#### Mistake 3: Missing Annotations When Needed

```rust
// COMPILE ERROR: missing lifetime specifier
fn pick(a: &str, b: &str, flag: bool) -> &str {
    if flag { a } else { b }
}

// FIX: annotate with shared lifetime
fn pick<'a>(a: &'a str, b: &'a str, flag: bool) -> &'a str {
    if flag { a } else { b }
}
```

When a function has multiple input references and the output could come from any of them, you must annotate.

#### Mistake 4: Confusing Lifetime Scope with Variable Scope

```rust
let r;
{
    let x = 5;
    r = &x; // borrow x
}
// x is dropped here — its scope ends
// but r's SCOPE continues
println!("{}", r); // ERROR: x does not live long enough

// Scope of 'r' extends beyond scope of x
// Lifetime of x ends at the closing brace
// Therefore r's lifetime outlives x's lifetime — rejected
```

Variable scope and reference lifetime are related but not identical. A variable's scope is where it is syntactically visible. A reference's lifetime is how long the referent is valid. The borrow checker tracks both.

#### Mistake 5: Returning References to Local Variables

```rust
fn make_string<'a>() -> &'a str {
    let s = String::from("hello"); // s is local
    &s // ERROR: s is dropped at end of function
}
```

You cannot return a reference to a local variable. The local is dropped when the function returns, making any reference dangle. The solution is to return an owned `String`:

```rust
fn make_string() -> String {
    String::from("hello")
}
```

#### Mistake 6: Confusing `'static` with "forever available"

```rust
fn process(data: &'static str) { ... }

let owned = String::from("hello");
process(&owned); // ERROR: &owned is not 'static — it will be dropped
process("hello"); // OK: string literal is 'static
```

`'static` does not mean "long-lived" in a relative sense — it means the value lives for the entire program. Heap-allocated Strings are not `'static`.

#### Mistake 7: Multiple Mutable Borrows or Mixed Borrows

Lifetimes interact with borrow rules:

```rust
let mut v = vec![1, 2, 3];
let first = &v[0];   // immutable borrow
v.push(4);           // ERROR: cannot mutate v while borrowed
println!("{}", first);
```

The `first` reference has a lifetime that extends to `println!`. The `push` tries to mutate `v` while `first` is still live. This is a borrow checker error, and lifetimes are central to why it is caught.

---

### Best Practices

1. **Let elision do its work.** Only add lifetime annotations when the compiler requires them. Do not add them for functions with one input reference returning a reference from it.

2. **Keep lifetime parameters minimal.** Use the fewest lifetime parameters that correctly express the relationships. Adding unnecessary parameters makes code harder to read.

3. **Use owned types when in doubt.** If you are not sure whether you need a reference, start with an owned type (`String` instead of `&str`). Optimize with references later when you understand the lifetimes involved.

4. **Read compiler errors carefully.** Rust's lifetime errors are detailed and usually point exactly to what is wrong. The error messages often suggest the fix.

5. **Use `'static` sparingly.** Requiring `'static` bounds unnecessarily limits the callers of your function. Only require it when truly needed (e.g., for sending across threads).

6. **Document lifetime semantics in complex APIs.** When a struct or function has non-obvious lifetime relationships, add a doc comment explaining what borrows what.

7. **Prefer borrowing over cloning for performance.** Use `&str` instead of `String` where possible. Use slices instead of owning collections. This is why lifetimes exist — to make safe borrowing possible without runtime overhead.

8. **Understand that lifetimes are a compile-time feature.** There is zero runtime cost to lifetime annotations. They exist only for the compiler to verify safety.

---

### Exercises

**Exercise 1: Annotate the function**

The following function does not compile. Add the correct lifetime annotation to fix it:

```rust
fn longer_greeting(name: &str, greeting: &str) -> &str {
    if greeting.len() > 5 {
        greeting
    } else {
        name
    }
}
```

Write the corrected function signature and explain why the annotation is needed.

---

**Exercise 2: What's the lifetime?**

Does the following code compile? If not, explain why. If yes, explain why it is safe:

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = String::from("world");
    let result = longest(s1.as_str(), s2.as_str());
    println!("{}", result);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

---

**Exercise 3: Find the error**

The following code does not compile. Identify the lifetime error and fix it:

```rust
fn main() {
    let result;
    {
        let s = String::from("temporary");
        result = first_word(&s);
    }
    println!("{}", result);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[..i];
        }
    }
    s
}
```

---

**Exercise 4: Struct lifetimes**

Define a struct `Summary<'a>` that holds:
- A `title: &'a str`
- A `body: &'a str`

Implement a method `short_body(&self) -> &str` that returns the first 50 characters of `body` (or all of it if shorter than 50 characters). Make sure the method compiles without explicit lifetime annotations on the method (let elision handle it).

---

**Exercise 5: Two lifetime parameters**

Write a function `merge_start<'a, 'b>(prefix: &'a str, _suffix: &'b str) -> &'a str` that returns the first five characters of `prefix` (or all of `prefix` if shorter than 5 characters). Why does this function need two lifetime parameters instead of one?

---

**Exercise 6: Does this compile?**

For each snippet below, state whether it compiles, and if not, explain why:

**Snippet A:**
```rust
fn identity(x: &str) -> &str { x }
```

**Snippet B:**
```rust
fn get_static() -> &'static str { "hello" }
```

**Snippet C:**
```rust
fn dangling() -> &str {
    let s = String::from("hello");
    &s
}
```

**Snippet D:**
```rust
struct Wrapper<'a> {
    value: &'a i32,
}

fn main() {
    let w;
    {
        let x = 42;
        w = Wrapper { value: &x };
    }
    println!("{}", w.value);
}
```

---

### Solutions

**Solution 1:**

```rust
fn longer_greeting<'a>(name: &'a str, greeting: &'a str) -> &'a str {
    if greeting.len() > 5 {
        greeting
    } else {
        name
    }
}
```

The annotation is needed because the function has two input references, and the output could be either one. The compiler cannot determine from elision rules alone which input the output borrows from. By using `'a` for both inputs and the output, we tell the compiler the return value borrows from whichever input has the shorter lifetime, ensuring the return value is always valid.

---

**Solution 2:**

Yes, this compiles. Both `s1` and `s2` are defined in `main` before `result` is assigned, and `result` is used within the same scope where both `s1` and `s2` are alive. The lifetime `'a` is the overlap of `s1`'s and `s2`'s lifetimes, which is the entire `main` function — long enough to cover the `println!`.

---

**Solution 3:**

The error: `result` is declared outside the block, but the value it borrows (`s`) is dropped at the end of the inner block. The `println!` uses `result` after `s` is gone.

Fix options:

Option A — Use `result` inside the block:
```rust
fn main() {
    {
        let s = String::from("temporary");
        let result = first_word(&s);
        println!("{}", result); // OK: s is alive here
    }
}
```

Option B — Move `s` outside the block:
```rust
fn main() {
    let s = String::from("temporary");
    let result = first_word(&s);
    println!("{}", result); // OK: s is alive for all of main
}
```

Option C — Return an owned `String` from the function (if you need `result` to outlive `s`):
```rust
fn first_word_owned(s: &str) -> String {
    let bytes = s.as_bytes();
    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return s[..i].to_string();
        }
    }
    s.to_string()
}

fn main() {
    let result;
    {
        let s = String::from("temporary");
        result = first_word_owned(&s); // owned String, not a reference
    }
    println!("{}", result); // OK: result owns its data
}
```

---

**Solution 4:**

```rust
struct Summary<'a> {
    title: &'a str,
    body: &'a str,
}

impl<'a> Summary<'a> {
    fn new(title: &'a str, body: &'a str) -> Self {
        Summary { title, body }
    }

    // Elision Rule 3: &self's lifetime assigned to output
    fn short_body(&self) -> &str {
        let max = 50;
        if self.body.len() <= max {
            self.body
        } else {
            // Find a clean break at a space boundary
            let boundary = self.body[..max].rfind(' ').unwrap_or(max);
            &self.body[..boundary]
        }
    }
}

fn main() {
    let title = String::from("Rust Lifetimes");
    let body = String::from(
        "Lifetimes are a compile-time mechanism that ensures references are always valid."
    );
    let summary = Summary::new(&title, &body);
    println!("Title: {}", summary.title);
    println!("Short body: {}", summary.short_body());
}
```

---

**Solution 5:**

```rust
fn merge_start<'a, 'b>(prefix: &'a str, _suffix: &'b str) -> &'a str {
    if prefix.len() <= 5 {
        prefix
    } else {
        &prefix[..5]
    }
}
```

Two lifetime parameters are needed because the output only depends on `prefix`, not on `_suffix`. With a single lifetime `'a`, both inputs would have the same lifetime, and the output lifetime would be constrained to the shorter of the two — even though `_suffix` is not used in the output at all. By separating them into `'a` and `'b`, we correctly express that the output borrows only from `prefix` (lifetime `'a`), and `_suffix` can have any independent lifetime `'b`.

---

**Solution 6:**

**Snippet A:** Compiles. Elision Rule 2 applies: one input reference, output gets the same lifetime.

**Snippet B:** Compiles. The function returns a string literal, which is `'static`. No borrows from input.

**Snippet C:** Does not compile. `s` is a local `String` that is dropped at the end of `dangling`. Returning `&s` would create a dangling reference. The compiler gives error E0106/E0515.

**Snippet D:** Does not compile. `x` is defined inside the inner block and dropped when the block ends. `w` is defined outside and used after the block ends with `println!`. Since `w.value` points to `x`, and `x` is dropped, this would dangle. The compiler gives a "does not live long enough" error.

---

## Mini Project: Document Parser

### Project Overview

You will build a **Document Parser** that takes a raw text document and extracts structured information from it — sections, headers, and paragraphs — using **borrowed slices** rather than owned `String`s. The parser will work entirely with `&str` slices into the original document, demonstrating zero-copy parsing: no allocation beyond the original document storage.

This project shows why lifetimes matter in real systems code: parsers, configuration loaders, and network protocol handlers often need to slice and reference into a large buffer without copying data.

### Functional Requirements

1. Parse a document string into a list of `Section`s.
2. Each `Section` has a header (a `&str` slice) and a body (a `&str` slice).
3. Sections are delimited by lines starting with `##` (a simplified Markdown-style format).
4. The parser must **borrow from the original document** — no `String::from`, no `.to_string()`, no `.to_owned()` on the parsed slices.
5. Support querying sections by header (case-insensitive).
6. Support extracting the first paragraph of a section.
7. Track character counts and word counts without allocating.

### Project Structure

```
document_parser/
├── Cargo.toml
└── src/
    ├── main.rs
    ├── parser.rs
    └── section.rs
```

### Step-by-Step Development

**Step 1: Define the `Section` struct with lifetimes**

The `Section` struct must hold borrowed slices. The lifetime parameter `'doc` (we use a descriptive name instead of just `'a`) expresses that the section borrows from the original document.

**Step 2: Implement methods on `Section`**

Methods that return string slices will use lifetime elision (Rule 3 on `&self`).

**Step 3: Build the `DocumentParser` struct**

The parser holds a reference to the full document text and produces sections on demand.

**Step 4: Implement parsing logic**

Parse sections by scanning lines and identifying headers (lines starting with `##`).

**Step 5: Implement query methods**

Search sections by header, count words, etc.

**Step 6: Wire everything together in `main.rs`**

### Complete Source Code

**`Cargo.toml`:**

```toml
[package]
name = "document_parser"
version = "0.1.0"
edition = "2021"

[[bin]]
name = "document_parser"
path = "src/main.rs"
```

---

**`src/section.rs`:**

```rust
// src/section.rs
//
// Defines the Section type. Every field is a borrowed &str slice,
// meaning Section<'doc> cannot outlive the document it borrows from.

use std::fmt;

/// A parsed section of a document.
///
/// The lifetime `'doc` ties this section to the document it was parsed from.
/// No data is copied — header and body are slices into the original text.
#[derive(Debug, Clone)]
pub struct Section<'doc> {
    /// The header text (without the leading ## and surrounding whitespace).
    pub header: &'doc str,

    /// The full body of the section, including all lines after the header
    /// up to (but not including) the next section header.
    pub body: &'doc str,
}

impl<'doc> Section<'doc> {
    /// Create a new Section from borrowed slices.
    ///
    /// Both `header` and `body` must come from the same document with lifetime `'doc`.
    pub fn new(header: &'doc str, body: &'doc str) -> Self {
        Section { header, body }
    }

    /// Return the header text. Elision (Rule 3) handles the lifetime.
    pub fn header(&self) -> &str {
        self.header
    }

    /// Return the full body text. Elision (Rule 3) handles the lifetime.
    pub fn body(&self) -> &str {
        self.body
    }

    /// Return the first paragraph of the body.
    ///
    /// A paragraph is text up to the first blank line. If no blank line
    /// exists, the entire body is returned.
    /// Elision (Rule 3): output borrows from self (which borrows from 'doc).
    pub fn first_paragraph(&self) -> &str {
        // Split on double newlines to find paragraph boundaries
        match self.body.find("\n\n") {
            Some(pos) => self.body[..pos].trim(),
            None => self.body.trim(),
        }
    }

    /// Return a preview of the body up to `max_chars` characters.
    ///
    /// Tries to break at a word boundary for clean output.
    /// Elision (Rule 3): output borrows from self.
    pub fn preview(&self, max_chars: usize) -> &str {
        let trimmed = self.body.trim();
        if trimmed.len() <= max_chars {
            return trimmed;
        }
        // Find the last space within max_chars to avoid cutting a word
        match trimmed[..max_chars].rfind(' ') {
            Some(pos) => &trimmed[..pos],
            None => &trimmed[..max_chars],
        }
    }

    /// Count the words in the body.
    ///
    /// Returns a usize — no reference in the output, no lifetime needed.
    pub fn word_count(&self) -> usize {
        self.body.split_whitespace().count()
    }

    /// Count the characters in the body (Unicode scalar values).
    pub fn char_count(&self) -> usize {
        self.body.chars().count()
    }

    /// Check if the body contains the given keyword (case-insensitive).
    ///
    /// `keyword` has its own independent lifetime — it does not affect
    /// the section's lifetime.
    pub fn contains_keyword(&self, keyword: &str) -> bool {
        let body_lower = self.body.to_lowercase();
        let keyword_lower = keyword.to_lowercase();
        body_lower.contains(&keyword_lower)
    }

    /// Return all lines in the body as an iterator of &str slices.
    ///
    /// Each yielded &str borrows from the original document (via self.body).
    /// The iterator itself is tied to the lifetime of self (Rule 3 applies
    /// to methods returning iterators containing refs).
    pub fn lines(&self) -> impl Iterator<Item = &str> {
        self.body.lines()
    }
}

impl<'doc> fmt::Display for Section<'doc> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "## {}\n{}", self.header, self.body)
    }
}
```

---

**`src/parser.rs`:**

```rust
// src/parser.rs
//
// DocumentParser: borrows the document text and produces Section<'doc> values.
// No allocations are made for parsed text — everything is sliced from
// the original document.

use crate::section::Section;

/// A zero-copy document parser.
///
/// The parser holds a reference to the document text. All sections it
/// produces are slices into that same text. The parser cannot outlive
/// the document it parses.
pub struct DocumentParser<'doc> {
    text: &'doc str,
}

impl<'doc> DocumentParser<'doc> {
    /// Create a parser that borrows the given document text.
    pub fn new(text: &'doc str) -> Self {
        DocumentParser { text }
    }

    /// Return the raw text this parser is working with.
    /// Elision (Rule 3): returns a borrow from self (which holds 'doc).
    pub fn raw_text(&self) -> &str {
        self.text
    }

    /// Parse the document into a Vec of Section<'doc>.
    ///
    /// Sections are identified by lines that start with "## ".
    /// The text before the first section header is treated as a preamble
    /// and is not included in any section.
    ///
    /// All returned sections borrow from `self.text` (lifetime 'doc).
    pub fn parse_sections(&self) -> Vec<Section<'doc>> {
        let mut sections: Vec<Section<'doc>> = Vec::new();

        // We track where each section header starts in the text.
        // header_start: byte offset of the "## " line
        // header_end: byte offset just past the end of the header line
        // body_start: byte offset where the body begins (after the header line)

        let text = self.text;
        let mut section_starts: Vec<usize> = Vec::new();

        // Find all byte offsets where a section header begins.
        // We iterate over lines with their byte offsets.
        let mut offset = 0;
        for line in text.lines() {
            if line.starts_with("## ") {
                section_starts.push(offset);
            }
            // Advance offset: line length + 1 for the newline character.
            // We use the actual byte length of the line in the original text.
            offset += line.len() + 1; // +1 for '\n'
        }

        // For each section header, extract header and body slices.
        for (i, &start) in section_starts.iter().enumerate() {
            // Find the end of the header line (the next newline after start).
            let header_line_end = text[start..]
                .find('\n')
                .map(|pos| start + pos)
                .unwrap_or(text.len());

            // The header text is the line minus the "## " prefix.
            // We slice into the original text — no copy.
            let header_prefix = "## ";
            let header = &text[start + header_prefix.len()..header_line_end].trim();

            // Body starts on the line after the header.
            let body_start = if header_line_end + 1 <= text.len() {
                header_line_end + 1
            } else {
                text.len()
            };

            // Body ends at the start of the next section (or end of document).
            let body_end = if i + 1 < section_starts.len() {
                section_starts[i + 1]
            } else {
                text.len()
            };

            // Slice the body from the original text — no copy.
            let body = &text[body_start..body_end];

            sections.push(Section::new(header, body));
        }

        sections
    }

    /// Find a section by header text (case-insensitive).
    ///
    /// Returns the first matching section, or None.
    /// The returned Section<'doc> borrows from the same document.
    pub fn find_section<'q>(&self, query: &'q str) -> Option<Section<'doc>> {
        let query_lower = query.to_lowercase();
        self.parse_sections()
            .into_iter()
            .find(|s| s.header.to_lowercase() == query_lower)
    }

    /// Return the total number of sections in the document.
    pub fn section_count(&self) -> usize {
        self.parse_sections().len()
    }

    /// Count total words in the entire document.
    pub fn total_word_count(&self) -> usize {
        self.text.split_whitespace().count()
    }

    /// Count total characters in the document.
    pub fn total_char_count(&self) -> usize {
        self.text.chars().count()
    }

    /// Return a slice of the document's preamble (text before the first section).
    ///
    /// If no sections exist, returns the entire document.
    /// If the document starts immediately with a section, returns an empty slice.
    pub fn preamble(&self) -> &str {
        let text = self.text;
        // Find the first "## " header
        let mut offset = 0;
        for line in text.lines() {
            if line.starts_with("## ") {
                // Preamble is everything before this line
                return &text[..offset];
            }
            offset += line.len() + 1;
        }
        // No sections found — entire text is preamble
        text
    }

    /// Extract a summary: first section header + its preview (up to 100 chars).
    ///
    /// Returns (header, preview) as slices into the document.
    /// Both returned slices have lifetime 'doc.
    pub fn summary(&self) -> Option<(&str, &str)> {
        self.parse_sections()
            .into_iter()
            .next()
            .map(|section| {
                // We cannot return section directly because it is a local variable
                // in the closure. We need to find the slices from self.text.
                // This is why we re-parse here — to get 'doc-lifetime slices.
                // (See Refactoring Suggestions for a cleaner approach.)
                let _ = section; // intentional: see note above
                // Re-find via find_section to get correct 'doc lifetime
                // This is a known limitation we address in refactoring.
                (self.text, self.text) // placeholder — see full version below
            })
    }

    /// Properly typed summary using lifetime-correct extraction.
    ///
    /// Returns (header_slice, body_preview_slice) from the first section.
    pub fn first_section_preview(&self) -> Option<(&str, &str)> {
        let text = self.text;
        let mut offset = 0;
        let mut found_header_start: Option<usize> = None;
        let mut found_header_end: Option<usize> = None;

        for line in text.lines() {
            if line.starts_with("## ") && found_header_start.is_none() {
                found_header_start = Some(offset);
                found_header_end = Some(offset + line.len());
            }
            offset += line.len() + 1;
        }

        match (found_header_start, found_header_end) {
            (Some(hs), Some(he)) => {
                let prefix_len = "## ".len();
                let header = &text[hs + prefix_len..he].trim();
                let body_start = if he + 1 <= text.len() { he + 1 } else { text.len() };
                let body = &text[body_start..];
                // Preview: up to 100 chars, break at word boundary
                let preview = if body.len() <= 100 {
                    body
                } else {
                    match body[..100].rfind(' ') {
                        Some(p) => &body[..p],
                        None => &body[..100],
                    }
                };
                Some((header, preview))
            }
            _ => None,
        }
    }
}
```

---

**`src/main.rs`:**

```rust
// src/main.rs
//
// Document Parser: demonstrates zero-copy text parsing with lifetimes.
// All parsed sections are &str slices into the original document String.

mod parser;
mod section;

use parser::DocumentParser;

fn main() {
    println!("=== Document Parser — Chapter 16 Mini Project ===\n");

    // The document is an owned String. It lives for the duration of main().
    // All parsed slices will borrow from this String.
    let document = String::from(
        "Rust Programming Language Guide\n\
         A comprehensive reference for Rust developers.\n\
         \n\
         ## Introduction\n\
         Rust is a systems programming language focused on three goals:\n\
         safety, speed, and concurrency. It accomplishes these goals without\n\
         a garbage collector, making it useful for a number of use cases\n\
         other languages are not good at: embedding in other languages,\n\
         programs with specific space and time requirements, and writing\n\
         low-level code, like device drivers and operating systems.\n\
         \n\
         Rust improves on current languages targeting this space by having a\n\
         number of compile-time safety checks with no runtime overhead.\n\
         \n\
         ## Ownership and Borrowing\n\
         Ownership is Rust's most unique feature, and it enables Rust to make\n\
         memory safety guarantees without needing a garbage collector.\n\
         \n\
         All programs have to manage the way they use a computer's memory\n\
         while running. Some languages have garbage collection that constantly\n\
         looks for no longer used memory as the program runs.\n\
         \n\
         In Rust, memory is managed through a system of ownership with a set\n\
         of rules that the compiler checks at compile time. None of the\n\
         ownership features slow down your program while it is running.\n\
         \n\
         ## Lifetimes\n\
         Every reference in Rust has a lifetime, which is the scope for which\n\
         that reference is valid. Most of the time, lifetimes are implicit and\n\
         inferred, just like most of the time, types are inferred.\n\
         \n\
         We must annotate lifetimes when the lifetimes of references could be\n\
         related in a few different ways. Rust requires us to annotate the\n\
         relationships using generic lifetime parameters to ensure the actual\n\
         references used at runtime will definitely be valid.\n\
         \n\
         ## Error Handling\n\
         Rust groups errors into two major categories: recoverable and\n\
         unrecoverable errors. For a recoverable error, such as a file not\n\
         found error, it is reasonable to report the problem to the user and\n\
         retry the operation. Unrecoverable errors are always symptoms of bugs.\n\
         \n\
         Rust has the type Result<T, E> for recoverable errors and the panic!\n\
         macro that stops execution when the program encounters an unrecoverable\n\
         error.\n",
    );

    // Create the parser. It borrows `document`.
    // All Section<'_> values it produces will have a lifetime tied to `document`.
    let parser = DocumentParser::new(&document);

    // ─── Basic stats ───
    println!("--- Document Statistics ---");
    println!("Total sections: {}", parser.section_count());
    println!("Total words:    {}", parser.total_word_count());
    println!("Total chars:    {}", parser.total_char_count());

    // ─── Preamble ───
    println!("\n--- Preamble (before first section) ---");
    let preamble = parser.preamble(); // &str slice into document
    println!("{}", preamble.trim());

    // ─── Parse all sections ───
    println!("\n--- All Sections ---");
    let sections = parser.parse_sections();
    // sections: Vec<Section<'_>> where '_ borrows from `document`
    for (i, section) in sections.iter().enumerate() {
        println!(
            "\n[Section {}] Header: '{}' | Words: {} | Chars: {}",
            i + 1,
            section.header(),
            section.word_count(),
            section.char_count(),
        );
        println!("  Preview: '{}'", section.preview(80));
    }

    // ─── Find section by name ───
    println!("\n--- Find Section: 'lifetimes' ---");
    match parser.find_section("Lifetimes") {
        Some(section) => {
            // section.header and section.body are &str slices into `document`
            // No allocation has been made for these strings
            println!("Found: '{}'", section.header());
            println!("First paragraph:\n  {}", section.first_paragraph());
            println!("Contains 'annotate': {}", section.contains_keyword("annotate"));
            println!("Word count: {}", section.word_count());
        }
        None => println!("Section not found."),
    }

    // ─── First section preview (direct lifetime-correct extraction) ───
    println!("\n--- First Section Preview ---");
    match parser.first_section_preview() {
        Some((header, preview)) => {
            // header and preview are &str slices with 'doc lifetime
            // They borrow from `document` which is still alive
            println!("Header:  '{}'", header);
            println!("Preview: '{}'", preview);
        }
        None => println!("No sections found."),
    }

    // ─── Keyword search across all sections ───
    println!("\n--- Keyword Search: 'memory' ---");
    let sections2 = parser.parse_sections();
    let matching: Vec<&section::Section<'_>> = sections2
        .iter()
        .filter(|s| s.contains_keyword("memory"))
        .collect();
    println!("Sections containing 'memory': {}", matching.len());
    for s in &matching {
        println!("  - '{}'", s.header());
    }

    // ─── Line iteration without allocation ───
    println!("\n--- Lines in 'Error Handling' section ---");
    if let Some(err_section) = parser.find_section("Error Handling") {
        // .lines() returns an iterator of &str borrows from the section body
        for (i, line) in err_section.lines().enumerate() {
            if !line.trim().is_empty() {
                println!("  Line {}: {}", i + 1, line.trim());
            }
        }
    }

    // ─── Demonstrate lifetime constraint ───
    // The following would NOT compile (uncomment to see the error):
    //
    // let dangling_section;
    // {
    //     let temp_doc = String::from("## Temp\nContent here.");
    //     let temp_parser = DocumentParser::new(&temp_doc);
    //     let temp_sections = temp_parser.parse_sections();
    //     dangling_section = temp_sections.into_iter().next(); // ERROR
    // } // temp_doc dropped here
    // println!("{:?}", dangling_section); // dangling_section would dangle

    println!("\n=== Mini Project Complete ===");
    println!("All parsed slices borrowed from the original document.");
    println!("No unnecessary String allocations were made for parsed content.");
}
```

---

### Code Explanation

**`Section<'doc>`**

The `'doc` lifetime parameter makes explicit that every `Section` borrows its data from a "document" source. Using a descriptive name (`'doc` instead of `'a`) improves readability in complex code.

`header: &'doc str` — the header is a slice of the original document text. No string copy.
`body: &'doc str` — the body is also a slice. Rust's borrow checker ensures the `Section` value cannot outlive `'doc`.

**`first_paragraph(&self) -> &str`**

Elision applies via Rule 3: `&self` provides the lifetime, so the output borrows from `self` (and transitively from `'doc`). `self.body.find("\n\n")` searches for a blank line without allocating.

**`DocumentParser<'doc>`**

The parser struct itself holds `text: &'doc str`. All methods that return `&str` slices return slices with lifetime `'doc` (the document's lifetime), not `'self` (the parser's lifetime). This is important: even if the `DocumentParser` is dropped, the slices it produced remain valid as long as `'doc` is alive.

**`parse_sections(&self) -> Vec<Section<'doc>>`**

The return type is `Vec<Section<'doc>>` — not `Vec<Section<'_>>` (which would be `Section<'self>`). By returning `'doc` lifetime sections, we decouple the sections' validity from the parser's validity. The caller can drop the parser and still use the sections.

This is a subtle but important lifetime distinction in real parser code.

**Offset tracking in `parse_sections`**

The parser accumulates byte offsets while iterating lines. `offset += line.len() + 1` accounts for the `\n` after each line. It uses these offsets to create slices like `&text[start..end]` directly into the document — zero copy.

**`find_section<'q>(&self, query: &'q str) -> Option<Section<'doc>>`**

Two lifetime parameters: `'doc` for the parser's document, `'q` for the query string. The query is only used for comparison — it does not appear in the output. The output `Section<'doc>` borrows from the document, not from `query`.

**`first_section_preview(&self) -> Option<(&str, &str)>`**

Returns a tuple of two `&str` slices, both with the lifetime of `self` (via elision Rule 3). But because `self` contains `'doc` slices, the returned slices effectively have `'doc` lifetime. The function navigates byte offsets to slice header and body directly from the document.

**In `main()`**

`document` is a `String` that lives for all of `main`. All parser operations borrow from it. The comment showing the dangling reference demonstrates what the borrow checker prevents.

---

### Refactoring Suggestions

1. **Iterate lines with offset more robustly**: The current offset calculation (`line.len() + 1`) assumes `\n` line endings. For cross-platform support, use `str::split_inclusive('\n')` or byte indexing with `memchr` to handle `\r\n` endings.

2. **Pre-compute section positions**: Instead of calling `parse_sections()` multiple times (which re-parses on every call), cache the positions or sections inside the parser:

```rust
pub struct DocumentParser<'doc> {
    text: &'doc str,
    // Cache section byte ranges on first parse
    section_ranges: std::cell::OnceCell<Vec<(usize, usize, usize, usize)>>,
}
```

3. **Cleaner `summary()` method**: The current `summary()` has a placeholder. Refactor to use `first_section_preview()` directly and remove the placeholder.

4. **Return iterators instead of `Vec`**: `parse_sections` allocates a `Vec<Section<'doc>>`. For streaming use cases, return an iterator:

```rust
pub fn sections_iter(&'doc self) -> impl Iterator<Item = Section<'doc>> + 'doc {
    // ...
}
```

5. **Support triple-level headers**: Extend the parser to recognize `#`, `##`, and `###` as different section levels, storing the level in the `Section` struct.

6. **Add error handling**: Currently the parser silently handles malformed input. Add a `ParseError` type and return `Result<Vec<Section<'doc>>, ParseError>` from `parse_sections`.

---

### Challenge Exercises

1. **Add a `find_all_sections` method** that returns all sections whose headers contain a given substring (case-insensitive). Return `Vec<Section<'doc>>`.

2. **Add a `word_frequency` method** to `Section` that counts word frequencies without allocating a `HashMap<String, usize>`. Hint: this is hard without allocation — consider when owned types are genuinely necessary.

3. **Add a `subsection` method** to `Section` that extracts a sub-slice of the body between two line numbers. Return `Option<&str>`.

4. **Write a function** `longest_section<'a>(sections: &[Section<'a>]) -> Option<&Section<'a>>` that returns a reference to the section with the most words. Practice working with references to structs that themselves contain references.

5. **Extend `DocumentParser`** to support documents with a `# Title` header (single `#`) as a document title, separate from `##` section headers. Extract the title as a `&str` slice with `'doc` lifetime.

---

### Real World Extensions

- **Configuration file parser**: Use the same zero-copy approach to parse `.ini` or `.toml`-style files. Return `HashMap<&str, &str>` mapping keys to values — all slices into the original config text.

- **Log file analyzer**: Parse structured log lines (timestamp, level, message) as borrowed slices from a memory-mapped log file. `memmap2` crate maps files into memory, and your parser can work with `&str` slices into the mapping.

- **HTTP header parser**: HTTP/1.1 headers are text. A zero-copy parser that returns `HashMap<&str, &str>` (header name → value) without allocation is extremely performance-sensitive and exactly the pattern this mini project demonstrates.

- **JSON or CSV tokenizer**: The first stage of many parsers is tokenizing — splitting the input into tokens that are slices of the original. This is safe and efficient with Rust's lifetime system.

- **Streaming parser with `BufReader`**: For files too large to load into memory at once, extend the parser to work with buffered reads, returning owned sections only for the currently buffered window.

---

## Quiz

**Question 1**

What does a lifetime annotation tell the Rust compiler?

A. How long a value should live in memory.
B. When to allocate and deallocate memory.
C. The relationship between reference lifetimes in a function or struct.
D. The scope of a variable.

---

**Question 2**

Which of the following functions requires an explicit lifetime annotation? Assume no additional context.

A. `fn double(x: i32) -> i32 { x * 2 }`
B. `fn trim(s: &str) -> &str { s.trim() }`
C. `fn pick(a: &str, b: &str, flag: bool) -> &str { if flag { a } else { b } }`
D. `fn len(s: &str) -> usize { s.len() }`

---

**Question 3**

Consider this code. Does it compile? If not, why?

```rust
fn main() {
    let result;
    {
        let text = String::from("hello world");
        result = first_word(&text);
    }
    println!("{}", result);
}

fn first_word(s: &str) -> &str {
    match s.find(' ') {
        Some(i) => &s[..i],
        None => s,
    }
}
```

A. Yes, it compiles because `result` is declared before the block.
B. No, because `text` does not live long enough — `result` is used after `text` is dropped.
C. No, because `first_word` is missing a lifetime annotation.
D. Yes, because `result` is a `&str` which is always `'static`.

---

**Question 4**

Which lifetime elision rule allows the following to compile without annotations?

```rust
fn get_label(&self) -> &str {
    &self.name
}
```

A. Rule 1 — each input reference gets its own lifetime.
B. Rule 2 — one input reference, so output gets its lifetime.
C. Rule 3 — `&self` lifetime is assigned to output.
D. No rule — this requires explicit annotations.

---

**Question 5**

What is the `'static` lifetime?

A. A lifetime that applies to all local variables.
B. A lifetime for references that are valid for the entire program's execution.
C. A lifetime used only for global variables.
D. A lifetime that prevents values from being dropped.

---

**Question 6**

What does the following function signature mean?

```rust
fn process<'a, 'b>(input: &'a str, config: &'b str) -> &'a str
```

A. The output borrows from either `input` or `config`.
B. The output borrows from `input` only; `config` is independent.
C. `'a` and `'b` must be the same lifetime.
D. The function returns a `'static` string.

---

**Question 7**

Consider this struct. Is it correct?

```rust
struct Cache {
    data: &str,
}
```

A. Yes, it compiles because `&str` is a primitive type.
B. No, it is missing a lifetime annotation on the `&str`.
C. Yes, elision handles it automatically for structs.
D. No, structs cannot hold references.

---

**Question 8**

What is wrong with the following code?

```rust
fn main() {
    let r;
    {
        let x = 5;
        r = &x;
    }
    println!("{}", r);
}
```

A. `r` is uninitialized.
B. `x` does not implement `Display`.
C. `r` outlives `x` — `x` is dropped before `r` is used, creating a dangling reference.
D. `r` cannot be a reference to an integer.

---

**Question 9**

In what situation does a function need multiple lifetime parameters?

A. Always, for every function with references.
B. When the function has more than two parameters.
C. When the output lifetime depends on some, but not all, input references, or when inputs have independent lifetimes.
D. Never — a single lifetime parameter always suffices.

---

**Question 10**

What happens at runtime when you add a lifetime annotation to a function?

A. The compiler inserts reference counting logic.
B. The runtime tracks reference validity.
C. Nothing — lifetime annotations are compile-time only and have zero runtime cost.
D. Memory is allocated for the lifetime bookkeeping.

---

## Quiz Answers

**Answer 1: C**

Lifetime annotations describe the **relationship** between reference lifetimes — for example, "the output reference lives at least as long as this input reference." They do not control when memory is allocated or freed, and they do not extend or shorten any value's lifetime.

---

**Answer 2: C**

Option C (`pick`) requires an explicit annotation because it has two input references and an output reference. Elision cannot determine whether the output borrows from `a` or `b` — that depends on the runtime value of `flag`. Options B and D have one input reference (elision Rule 2 applies). Option A has no references at all.

---

**Answer 3: B**

The code does not compile. `text` is dropped at the end of the inner block. `result` borrows from `text` (via `first_word`). The `println!` uses `result` after `text` is gone — `result` would dangle. The borrow checker catches this with a "does not live long enough" error. Note: `first_word` does not need an explicit annotation because elision handles it, so C is incorrect.

---

**Answer 4: C**

Rule 3: if one of the input parameters is `&self` or `&mut self`, the lifetime of `self` is assigned to all output lifetime parameters. This is exactly the situation here — `&self` is the only input reference, so the output `&str` gets `self`'s lifetime.

---

**Answer 5: B**

`'static` means the reference is valid for the entire program's execution. String literals have `'static` lifetime because they are embedded in the program binary and never freed. Note: `'static` values are not forbidden from being dropped earlier — a `String` can be `'static` if leaked, but the key property is "valid for at least the duration of the program."

---

**Answer 6: B**

The return type is `&'a str`, which uses `'a` (the lifetime of `input`). The `'b` (lifetime of `config`) does not appear in the return type. This tells the compiler the output borrows from `input` only. `config` can have a completely independent, shorter or longer lifetime.

---

**Answer 7: B**

Structs that hold references must have lifetime annotations. `data: &str` is missing a lifetime parameter. The compiler requires `struct Cache<'a> { data: &'a str }`. Elision does not apply to struct field definitions.

---

**Answer 8: C**

`x` is defined inside the inner block and dropped when the block ends. `r` is assigned `&x` inside the block, making `r` a reference to `x`. After the block, `x` is gone, but `r` is still in scope and used in `println!`. This is a dangling reference — exactly the problem Rust's borrow checker prevents. The compiler gives a "does not live long enough" error.

---

**Answer 9: C**

Multiple lifetime parameters are needed when input references have independent lifetimes and the output only depends on a subset of them, or when you need to express that the output could borrow from one specific input among several. If the output could borrow from any input, a single shared lifetime often suffices. Multiple lifetimes make the independence explicit.

---

**Answer 10: C**

Lifetimes are a **compile-time only** feature. The compiler uses them to verify reference validity, then discards all lifetime information. At runtime, there is no reference counting, no bookkeeping, and no overhead whatsoever. This is one of Rust's zero-cost abstractions.

---

## Chapter Summary

This chapter covered one of Rust's most distinctive and powerful features: **lifetimes**.

### The Problem Lifetimes Solve

Lifetimes exist to prevent **dangling references** — the use of references to memory that has already been freed. In C and C++, dangling pointers and use-after-free bugs are among the most common causes of crashes and security vulnerabilities. Rust's borrow checker eliminates this entire class of bugs at **compile time**, with zero runtime cost.

### What Lifetimes Are

A lifetime is the span of time during program execution when a value is valid. Every reference in Rust has a lifetime. The borrow checker tracks these lifetimes and ensures no reference outlives the value it refers to.

**Critically**: lifetime annotations do not change how long values live. They are descriptive labels that communicate relationships between reference lifetimes to the compiler.

### Lifetime Annotations

Lifetime annotations use the syntax `'a`, `'b`, etc. They appear after `&` in reference types and are declared in angle brackets alongside generic type parameters:

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str { ... }
```

When a function has multiple input references and an output reference, and the output could borrow from any input, explicit annotations are required. The annotation constrains the output's lifetime to be within the overlap (intersection) of the annotated input lifetimes.

### Lifetime Elision

Three rules let the compiler infer lifetimes without explicit annotation:

1. Each input reference gets its own distinct lifetime.
2. If there is exactly one input reference, its lifetime is assigned to all output references.
3. If one input is `&self` or `&mut self`, `self`'s lifetime is assigned to all output references.

When all output lifetimes are determinable from these rules, no annotation is needed. When they are not, the compiler requires explicit annotations.

### Struct Lifetimes

Structs that hold references must declare lifetime parameters. This expresses that the struct cannot outlive the data it borrows:

```rust
struct Parser<'doc> {
    text: &'doc str,
}
```

`impl` blocks for such structs must also declare the lifetime parameter: `impl<'a> Parser<'a>`.

### Lifetime in Functions

- One input reference → elision handles it.
- Multiple input references, output depends on all → use one shared lifetime.
- Multiple input references, output depends on one → use multiple lifetime parameters, apply relevant one to output.
- Combining with generics: use `T: Display + 'a` bounds when generic types may contain references.

### `'static` Lifetime

`'static` means valid for the entire program. String literals are `'static`. It is not a catch-all solution — requiring `'static` restricts callers and should be used only when genuinely needed (e.g., sending data across threads).

### The Document Parser Mini Project

The mini project demonstrated **zero-copy parsing**: all parsed slices (`Section<'doc>`) are `&str` slices into the original document `String`. No allocation is made for parsed text. This pattern is common in high-performance systems code — parsers, protocol handlers, and configuration loaders frequently use borrowed slices to avoid unnecessary copies.

### Key Takeaways

| Concept | Key Point |
|---|---|
| Why lifetimes exist | Prevent dangling references at compile time |
| What annotations do | Describe relationships, not control lifetimes |
| Elision | Compiler infers when rules apply unambiguously |
| Structs with refs | Must have lifetime parameters |
| `'static` | Valid for entire program; mainly string literals |
| Runtime cost | Zero — lifetimes are compile-time only |
| Common mistakes | Returning refs to locals, outliving borrowed data |

Mastering lifetimes takes practice. The borrow checker is your ally — its error messages are detailed and usually point to the exact issue. With experience, reading and writing lifetimes becomes natural, and the safety guarantees they provide become second nature.

In the next chapter, we move into **Error Handling** — how Rust handles recoverable and unrecoverable errors using `Result<T, E>`, `panic!`, the `?` operator, and custom error types.
