# Chapter 7: Slices

## Learning Objectives

By the end of this chapter, you will:

- Understand what slices are and how they work in memory
- Work with string slices (`&str`) confidently
- Work with array and vector slices (`&[T]`)
- Understand the relationship between `String` / `&str` and `Vec<T>` / `&[T]`
- Implement classic slice-based algorithms: first word, split, window iteration
- Understand why slices are the preferred way to write functions that accept strings and sequences

---

## Theory

### 7.1 String Slice

#### The Problem: Referencing Part of a String

Imagine you have a string `"hello world"` and you want to extract just `"hello"`. You could:

1. Copy the characters into a new `String` — wastes memory
2. Return an index (0) and length (5) — fragile, not type-safe
3. Return a **string slice** — safe, zero-copy, type-safe ✓

#### String Slice: `&str`

A string slice is a reference to a part of a `String` (or to a string literal):

```rust
fn main() {
    let s = String::from("hello world");

    let hello = &s[0..5];   // string slice: "hello"
    let world = &s[6..11];  // string slice: "world"

    println!("{} {}", hello, world);
}
```

The `[start..end]` range syntax: `start` is inclusive, `end` is exclusive.

```
s:     h  e  l  l  o     w  o  r  l  d
index: 0  1  2  3  4  5  6  7  8  9  10

&s[0..5]  → "hello"   (bytes 0 to 4, exclusive of 5)
&s[6..11] → "world"   (bytes 6 to 10, exclusive of 11)
&s[0..]   → "hello world"  (from 0 to end)
&s[..5]   → "hello"        (from start to 5)
&s[..]    → "hello world"  (entire string)
```

Memory layout:

```
Stack:
s:     ┌─────────────┐      Heap:
       │ ptr ─────────┼─────►│ h e l l o   w o r l d │
       │ len: 11     │      └────────────────────────┘
       │ cap: 11     │             ▲           ▲
       └─────────────┘             │           │
                                   │           │
hello: ┌─────────────┐             │           │
       │ ptr ─────────┼─────────────┘           │
       │ len: 5      │  (points INTO s's heap)
       └─────────────┘

world: ┌─────────────┐
       │ ptr ─────────┼─────────────────────────┘
       │ len: 5      │  (points INTO s's heap)
       └─────────────┘
```

Key insight: **no new heap allocation**. The slice is just a pointer into existing memory plus a length. It's zero-cost.

#### The Type `&str`

`&str` is the type for string slices. It's:
- A reference (pointer) into some string data
- Plus a length
- Immutable (you can't modify through `&str`)
- Does not own the data

String literals (`"hello"`) are also `&str` — they're references into the program's binary data:

```rust
let s: &str = "hello";  // s points to data in the program binary
                        // lives for the entire duration of the program
```

#### `String` vs `&str`

| Feature | `String` | `&str` |
|---------|----------|--------|
| Ownership | Owns heap data | Borrows data |
| Mutability | Can be mutated | Immutable |
| Resizable | Yes | No |
| Where stored | Heap | Points to heap or binary |
| Created with | `String::from(...)`, `.to_string()` | String literals, `&s[..]` |
| Pass to function | Takes ownership (or pass `&String`) | Always borrow (no ownership) |

**The Golden Rule for Functions:**

```rust
// LESS FLEXIBLE — only accepts &String (must have a String)
fn greet(name: &String) {
    println!("Hello, {}!", name);
}

// MORE FLEXIBLE — accepts &String, &str, and string literals
fn greet(name: &str) {
    println!("Hello, {}!", name);
}

fn main() {
    let s = String::from("Alice");
    greet(&s);         // works with both
    greet("Bob");      // ONLY works if parameter is &str
}
```

When you pass `&String` where `&str` is expected, Rust automatically **dereferences** through a process called **deref coercion**: `&String` → `&str`. So `&str` parameters are strictly more flexible.

#### String Slice Indices Must Be at Character Boundaries

```rust
let s = String::from("Привет");  // Russian "Hello" — each char is 2 bytes

let hello = &s[0..4];   // OK: 0..4 covers first 2 chars (4 bytes)
// let oops = &s[0..1]; // PANIC: byte index is not a char boundary
```

String slices work on **bytes**. For ASCII (1 byte per char) this is no problem. For Unicode, use `.chars()` for character-based access:

```rust
let s = "Привет";
let chars: Vec<char> = s.chars().collect();
println!("{}", chars[0]);  // 'П' — correct character access

// Or to get a sub-string by character count:
let first_two: String = s.chars().take(2).collect();
println!("{}", first_two);  // "Пр"
```

#### Classic Algorithm: First Word

```rust
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[..i];  // return slice up to the space
        }
    }

    s  // no space found — entire string is one word
}

fn main() {
    let sentence = String::from("hello world");
    let word = first_word(&sentence);
    println!("First word: {}", word);  // "hello"

    // sentence.clear();  // ERROR if word is still in use:
    // word borrows from sentence — sentence cannot be cleared
    // while the borrow is active

    let literal = "hello world";
    let word2 = first_word(literal);  // works with &str directly
    println!("First word: {}", word2);
}
```

This is the canonical example from The Rust Book showing why slices are safer than indices:

```rust
// Without slices — fragile
fn first_word_index(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' { return i; }
    }
    s.len()
}

fn main() {
    let mut s = String::from("hello world");
    let word_end = first_word_index(&s);  // word_end = 5
    s.clear();  // s is now ""  — word_end is now INVALID (points past end)
    // word_end is still 5 but the string is empty — stale index!
    // Rust doesn't catch this: word_end is just an integer
}

// With slices — safe, compiler-enforced
fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s);   // word borrows from s
    s.clear();                    // ERROR: s is borrowed, can't mutate it
    println!("{}", word);        // the borrow is still active here
}
```

The slice version is **safer** because the borrow checker prevents clearing the string while the slice still references it.

---

### 7.2 Array Slice

#### Array Slice: `&[T]`

Just as `&str` is a slice of a string, `&[T]` is a slice of any sequence (array, Vec, slice):

```rust
fn main() {
    let arr = [1, 2, 3, 4, 5];

    let slice = &arr[1..4];  // type: &[i32]  → [2, 3, 4]
    println!("{:?}", slice);
    println!("len={}", slice.len());  // 3

    // Iteration over slice:
    for &x in slice {
        print!("{} ", x);
    }
    println!();
}
```

Memory layout mirrors string slices:

```
arr:   [  1  |  2  |  3  |  4  |  5  ]
        ^0    ^1    ^2    ^3    ^4

&arr[1..4]:
  ptr → element at index 1
  len = 3
```

#### `&[T]` Works With Both Arrays and Vectors

The key advantage of `&[T]` in function signatures is that it accepts both:

```rust
fn sum(s: &[i32]) -> i32 {
    s.iter().sum()
}

fn main() {
    let arr = [1, 2, 3, 4, 5];      // array
    let vec = vec![10, 20, 30];      // vector

    println!("{}", sum(&arr));   // pass array slice
    println!("{}", sum(&vec));   // pass vector slice (auto-coercion Vec→slice)
    println!("{}", sum(&arr[1..3]));  // pass a slice of an array
    println!("{}", sum(&vec[..2]));   // pass a slice of a vector
}
```

This is exactly like how `&str` works with both `String` and string literals.

#### Common Slice Operations

```rust
fn main() {
    let data = [10, 20, 30, 40, 50, 60, 70, 80, 90, 100];
    let slice = &data[2..8];  // [30, 40, 50, 60, 70, 80]

    // Length:
    println!("{}", slice.len());         // 6
    println!("{}", slice.is_empty());    // false

    // First and last elements:
    println!("{:?}", slice.first());     // Some(30)
    println!("{:?}", slice.last());      // Some(80)

    // Contains:
    println!("{}", slice.contains(&50)); // true

    // Min and max:
    println!("{:?}", slice.iter().min());  // Some(30)
    println!("{:?}", slice.iter().max());  // Some(80)

    // Split at index:
    let (left, right) = slice.split_at(3);  // ([30,40,50], [60,70,80])
    println!("{:?} {:?}", left, right);

    // Windows (overlapping):
    for window in slice.windows(3) {
        print!("{:?} ", window);  // [30,40,50] [40,50,60] [50,60,70] [60,70,80]
    }
    println!();

    // Chunks (non-overlapping):
    for chunk in slice.chunks(2) {
        print!("{:?} ", chunk);  // [30,40] [50,60] [70,80]
    }
    println!();
}
```

#### Mutable Slices

```rust
fn negate(s: &mut [i32]) {
    for x in s.iter_mut() {
        *x = -*x;
    }
}

fn main() {
    let mut arr = [1, 2, 3, 4, 5];
    negate(&mut arr[1..4]);  // negate elements 1, 2, 3 → [-2, -3, -4]
    println!("{:?}", arr);   // [1, -2, -3, -4, 5]
}
```

---

## Code Example

### Practice: String Utilities

```rust
fn first_word(s: &str) -> &str {
    match s.find(' ') {
        Some(i) => &s[..i],
        None    => s,
    }
}

fn last_word(s: &str) -> &str {
    match s.rfind(' ') {
        Some(i) => &s[i + 1..],
        None    => s,
    }
}

fn nth_word(s: &str, n: usize) -> Option<&str> {
    s.split_whitespace().nth(n)
}

fn words(s: &str) -> Vec<&str> {
    s.split_whitespace().collect()
}

fn trim_to_n_words(s: &str, n: usize) -> &str {
    let mut end = 0;
    let mut count = 0;
    for (i, c) in s.char_indices() {
        if c == ' ' {
            count += 1;
            if count == n {
                end = i;
                break;
            }
        }
        end = i + c.len_utf8();
    }
    if count < n { s } else { &s[..end - (if s.as_bytes()[end - 1] == b' ' { 1 } else { 0 })] }
}

fn contains_substring(haystack: &str, needle: &str) -> bool {
    haystack.contains(needle)
}

fn main() {
    let sentence = "the quick brown fox jumps over the lazy dog";

    println!("First:   '{}'", first_word(sentence));
    println!("Last:    '{}'", last_word(sentence));
    println!("3rd:     '{:?}'", nth_word(sentence, 2));  // 0-indexed
    println!("Words:   {:?}", words(sentence));
    println!("Has 'fox': {}", contains_substring(sentence, "fox"));
    println!("Has 'cat': {}", contains_substring(sentence, "cat"));
}
```

---

### Practice: First Word and Array Utilities

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

fn max_slice(s: &[i32]) -> Option<i32> {
    s.iter().copied().reduce(i32::max)
}

fn sum_slice(s: &[i32]) -> i32 {
    s.iter().sum()
}

fn count_greater_than(s: &[i32], threshold: i32) -> usize {
    s.iter().filter(|&&x| x > threshold).count()
}

fn find_index(s: &[i32], target: i32) -> Option<usize> {
    s.iter().position(|&x| x == target)
}

fn is_sorted(s: &[i32]) -> bool {
    s.windows(2).all(|w| w[0] <= w[1])
}

fn reverse_slice(s: &[i32]) -> Vec<i32> {
    s.iter().copied().rev().collect()
}

fn main() {
    // String slices
    let sentences = [
        "hello world",
        "one",
        "  leading space",
        "",
    ];

    for s in &sentences {
        println!("first_word({:?}) = {:?}", s, first_word(s));
    }

    println!();

    // Array slices
    let nums = [5, 2, 8, 1, 9, 3, 7, 4, 6];

    println!("Data:          {:?}", nums);
    println!("Max:           {:?}", max_slice(&nums));
    println!("Sum:           {}", sum_slice(&nums));
    println!("Count > 5:     {}", count_greater_than(&nums, 5));
    println!("Index of 9:    {:?}", find_index(&nums, 9));
    println!("Is sorted:     {}", is_sorted(&nums));
    println!("Reversed:      {:?}", reverse_slice(&nums));

    let sorted = [1, 2, 3, 4, 5];
    println!("\nIs [1,2,3,4,5] sorted: {}", is_sorted(&sorted));

    // Slice windows
    println!("\nConsecutive pairs:");
    for pair in nums.windows(2) {
        println!("  {:?} → diff = {}", pair, pair[1] - pair[0]);
    }
}
```

### Line-by-Line Explanation

```rust
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &byte) in bytes.iter().enumerate() {
```
- `s: &str` — accepts any string reference (both `&String` and `&str`)
- `s.as_bytes()` — gets the byte slice `&[u8]` of the string (cheap, no allocation)
- `bytes.iter()` — creates an iterator over `&u8` references
- `enumerate()` — wraps each item as `(index, item)`, so `&byte` destructures to the byte value

```rust
        if byte == b' ' {
            return &s[..i];
        }
```
- `b' '` — a byte literal for the space character (ASCII 32)
- `&s[..i]` — slice from start to (not including) the space
- This is a reference INTO `s`, so the returned `&str` borrows from `s`

```rust
fn is_sorted(s: &[i32]) -> bool {
    s.windows(2).all(|w| w[0] <= w[1])
}
```
- `windows(2)` — produces overlapping pairs: `[a,b]`, `[b,c]`, `[c,d]`, etc.
- `.all(|w| w[0] <= w[1])` — returns true if EVERY consecutive pair is non-decreasing
- Elegant: O(n), no indices, reads naturally

```rust
fn reverse_slice(s: &[i32]) -> Vec<i32> {
    s.iter().copied().rev().collect()
}
```
- `iter()` — produces `&i32` references
- `.copied()` — copies each `&i32` to `i32` (since i32 is Copy)
- `.rev()` — reverses the iterator
- `.collect()` — collects into a new `Vec<i32>` (returns owned, new allocation)

---

## Common Mistakes

### Mistake 1: Slicing non-UTF-8 boundaries

```rust
let s = "Héllo";  // 'é' is 2 bytes in UTF-8

let oops = &s[0..2];  // 'H' is 1 byte, 'é' starts at 1 and is 2 bytes
                       // &s[0..2] would slice in the middle of 'é'
                       // PANIC at runtime: byte index 2 is not a char boundary

// Safe approach:
let safe: String = s.chars().take(2).collect();  // "Hé"
```

### Mistake 2: Returning a slice of a local variable

```rust
fn broken() -> &str {
    let s = String::from("hello");
    &s[..5]  // ERROR: s is dropped at end of function
             // the slice would dangle
}

// Fix: return an owned String
fn fixed() -> String {
    let s = String::from("hello");
    s[..5].to_string()  // allocate a new String
}

// Or: if the string comes from a parameter (so it lives long enough)
fn ok<'a>(s: &'a str) -> &'a str {
    &s[..5]  // borrows from s which outlives this function
}
```

### Mistake 3: Treating `&str` as mutable

```rust
let s = "hello";
s.push('!');  // ERROR: `s` is `&str` — immutable

// Fix: convert to String for mutation
let mut s = String::from("hello");
s.push('!');
println!("{}", s);  // hello!
```

### Mistake 4: Using `len()` for character count in Unicode

```rust
let s = "Привет";  // Russian "Hello"
println!("{}", s.len());            // 12 — byte count (each Cyrillic char = 2 bytes)
println!("{}", s.chars().count());  // 6  — character count
```

Always use `.chars().count()` for the number of characters in Unicode strings. `.len()` gives bytes.

---

## Best Practices

1. **Prefer `&str` over `&String` in function parameters** — accepts both `String` and literals
2. **Prefer `&[T]` over `&Vec<T>` in function parameters** — accepts both arrays and vectors
3. **Use `.chars()` for Unicode-safe character iteration** — `.bytes()` is bytes only
4. **Return `&str` from functions when you can** — zero allocation, references into existing data
5. **Return `String` when the data was created in the function** — you must own what you create
6. **Use `windows(n)` for sliding window algorithms** — expressive and safe
7. **Use `chunks(n)` for batching** — clean way to process data in groups

---

## Exercises

### Exercise 1: Count Occurrences

Write `count_occurrences(text: &str, pattern: &str) -> usize` that counts how many times `pattern` appears in `text`.

### Exercise 2: Split by Delimiter

Write `split_by(text: &str, delim: char) -> Vec<&str>` that splits a string on a delimiter and returns a vector of string slices (no allocation of new strings).

### Exercise 3: Array Statistics with Slices

Write a function `stats(data: &[f64]) -> (f64, f64, f64)` returning (min, max, average). Call it with full arrays, partial slices, and vector slices.

### Exercise 4: Sliding Window Maximum

Write `window_max(data: &[i32], window_size: usize) -> Vec<i32>` that returns the maximum of each window.

### Exercise 5: Words in Common

Write `words_in_common<'a>(a: &'a str, b: &'a str) -> Vec<&'a str>` that returns words that appear in both strings (case-insensitive, deduped).

---

## Solutions

### Solution 1

```rust
fn count_occurrences(text: &str, pattern: &str) -> usize {
    if pattern.is_empty() { return 0; }
    let mut count = 0;
    let mut start = 0;
    while let Some(pos) = text[start..].find(pattern) {
        count += 1;
        start += pos + pattern.len();
    }
    count
}

fn main() {
    println!("{}", count_occurrences("hello hello world hello", "hello")); // 3
    println!("{}", count_occurrences("abcabc", "abc")); // 2
    println!("{}", count_occurrences("aaa", "aa")); // 1
}
```

### Solution 2

```rust
fn split_by<'a>(text: &'a str, delim: char) -> Vec<&'a str> {
    text.split(delim).collect()
}

fn main() {
    let csv = "one,two,three,four";
    let parts = split_by(csv, ',');
    println!("{:?}", parts);  // ["one", "two", "three", "four"]

    let path = "/usr/local/bin/rust";
    println!("{:?}", split_by(path, '/'));
}
```

### Solution 3

```rust
fn stats(data: &[f64]) -> (f64, f64, f64) {
    let min = data.iter().cloned().fold(f64::INFINITY, f64::min);
    let max = data.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let avg = data.iter().sum::<f64>() / data.len() as f64;
    (min, max, avg)
}

fn main() {
    let full = [3.0, 1.0, 4.0, 1.0, 5.0, 9.0, 2.0, 6.0];
    let v = vec![10.0, 20.0, 30.0];

    let (min, max, avg) = stats(&full);
    println!("Full: min={}, max={}, avg={:.2}", min, max, avg);

    let (min, max, avg) = stats(&full[2..6]);
    println!("Slice [2..6]: min={}, max={}, avg={:.2}", min, max, avg);

    let (min, max, avg) = stats(&v);
    println!("Vec: min={}, max={}, avg={:.2}", min, max, avg);
}
```

### Solution 4

```rust
fn window_max(data: &[i32], window_size: usize) -> Vec<i32> {
    data.windows(window_size)
        .map(|w| *w.iter().max().unwrap())
        .collect()
}

fn main() {
    let data = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3];
    let maxes = window_max(&data, 3);
    println!("Data:   {:?}", data);
    println!("MaxW3:  {:?}", maxes);  // [4, 4, 5, 9, 9, 9, 6, 6]
}
```

### Solution 5

```rust
fn words_in_common<'a>(a: &'a str, b: &'a str) -> Vec<&'a str> {
    let b_words: Vec<String> = b.split_whitespace()
        .map(|w| w.to_ascii_lowercase())
        .collect();

    let mut result: Vec<&'a str> = Vec::new();
    let mut seen: Vec<String> = Vec::new();

    for word in a.split_whitespace() {
        let lower = word.to_ascii_lowercase();
        if b_words.contains(&lower) && !seen.contains(&lower) {
            result.push(word);
            seen.push(lower);
        }
    }
    result
}

fn main() {
    let a = "the quick brown fox jumps over the lazy dog";
    let b = "the dog ran over the bridge quickly";
    let common = words_in_common(a, b);
    println!("Common: {:?}", common);  // ["the", "over", "dog"]
}
```

---

## Quiz

**Q1.** What does `&str` represent in Rust?

a) An owned, heap-allocated string  
b) A pointer to a mutable character buffer  
c) A reference to a string slice — a pointer and a length into string data  
d) A null-terminated C-style string  

**Q2.** Why is `&str` preferred over `&String` as a function parameter?

a) `&str` is faster to compute  
b) `&str` is more general — it accepts both `String` references and string literals; `&String` only accepts `String` references  
c) `&String` is deprecated  
d) `&str` allows mutation  

**Q3.** What does `s.windows(3)` produce?

a) Exactly 3 elements from the start  
b) The string split into groups of 3  
c) Overlapping sub-slices of length 3: `[0..3]`, `[1..4]`, `[2..5]`, ...  
d) Every 3rd element  

**Q4.** Why does `s.len()` not equal the character count for Unicode strings?

a) `len()` counts words, not chars  
b) `len()` returns the number of bytes, not Unicode scalar values (some chars take 2-4 bytes)  
c) `len()` is broken for non-ASCII  
d) Rust doesn't support Unicode  

**Q5.** What is the type `&[i32]`?

a) A reference to a fixed-size array of i32  
b) A fat pointer (pointer + length) to a contiguous sequence of i32 values  
c) A mutable vector of i32  
d) A null-terminated array  

---

## Quiz Answers

**A1.** c) A reference to a string slice — a pointer and a length into string data  
*`&str` is a fat pointer: a pointer to UTF-8 encoded bytes plus the byte length. It does not own the data.*

**A2.** b) `&str` is more general — it accepts both `String` references and string literals  
*Rust applies deref coercion: `&String` → `&str` automatically. A `&str` parameter accepts anything a `&String` parameter does, plus string literals (`"hello"`). Always prefer `&str`.*

**A3.** c) Overlapping sub-slices of length 3: `[0..3]`, `[1..4]`, `[2..5]`, ...  
*`windows(n)` is a sliding window — each successive window overlaps with the previous by `n-1` elements. It's the idiomatic way to compare or process consecutive elements.*

**A4.** b) `len()` returns the number of bytes, not Unicode scalar values  
*ASCII characters are 1 byte, but Cyrillic, Chinese, emoji etc. use 2-4 bytes in UTF-8. `"café".len()` = 5 bytes but `"café".chars().count()` = 4 characters.*

**A5.** b) A fat pointer (pointer + length) to a contiguous sequence of i32 values  
*`&[i32]` is dynamically sized — it includes both the memory address and the length. It can reference any contiguous sequence of i32 values regardless of how they're stored (stack array, Vec, etc.).*

---

## Chapter Summary

- **String slices** (`&str`) are references into string data — a pointer plus a length, zero-copy
- String literals are `&str` pointing into the program binary
- `&str` is more flexible than `&String` as a function parameter — prefer `&str`
- **Array slices** (`&[T]`) work the same way for sequences — fat pointers into arrays or Vecs
- `&[T]` is more flexible than `&Vec<T>` as a function parameter — prefer `&[T]`
- Slice ranges: `[start..end]` (exclusive end), `[..end]`, `[start..]`, `[..]`
- For Unicode strings: use `.len()` for bytes, `.chars().count()` for character count
- `windows(n)` — overlapping sub-slices for sliding window algorithms
- `chunks(n)` — non-overlapping sub-slices for batch processing
- Slices enforce borrowing rules — you can't clear a String while a slice borrows it

---

**Part 2 Complete!**

You have mastered the core of what makes Rust unique: **ownership**, **borrowing**, and **slices**. These concepts underpin everything else in Rust:

- **Ownership** ensures every value is freed exactly once, automatically
- **Borrowing** lets you use data without taking ownership, with compile-time safety checks
- **Slices** give you efficient, safe views into sequences without copying

With this foundation, you are ready for Part 3: Structs and Enums — where you'll learn to build your own data types that compose naturally with the ownership system.

---

## Part 2 Review: Key Takeaways

| Concept | Rule | Why |
|---------|------|-----|
| Ownership | One owner per value | Prevents double-free |
| Move | Assignment transfers ownership for heap types | Prevents double-free |
| Copy | Stack-only types are copied automatically | Safe because no heap to free twice |
| Clone | Explicit deep copy | Explicit about cost |
| Drop | Automatic when owner leaves scope | No manual free() needed |
| Borrowing | `&T` = shared, `&mut T` = exclusive | Prevents data races |
| Borrow rules | Many `&T` OR one `&mut T`, never both | Prevents data races and use-after-free |
| Dangling refs | Prevented by lifetime checker | Prevents use-after-free |
| String slices | `&str` = pointer + length into string | Zero-copy, UTF-8 safe |
| Array slices | `&[T]` = pointer + length into sequence | Generic over source (array/Vec) |
