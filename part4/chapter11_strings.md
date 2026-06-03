# Chapter 11: Strings

## Learning Objectives

By the end of this chapter, you will:

- Understand the difference between `String` and `&str` deeply
- Know how UTF-8 encoding works and why it matters
- Perform common string operations: concatenation, slicing, searching, replacing
- Understand why you can't index a string with `s[0]`
- Work confidently with Unicode text

---

## Theory

### 11.1 String

`String` is Rust's heap-allocated, owned, growable string type. Under the hood it is `Vec<u8>` where the bytes are guaranteed to be valid UTF-8.

#### Creating Strings

```rust
fn main() {
    // From a string literal:
    let s1 = String::from("hello");
    let s2 = "hello".to_string();
    let s3 = "hello".to_owned();   // same as to_string() for &str

    // Empty string:
    let mut s = String::new();

    // With capacity pre-allocated (avoids reallocation):
    let mut s = String::with_capacity(50);
    println!("len={}, capacity={}", s.len(), s.capacity());

    // format! — like println! but returns a String:
    let name = "Alice";
    let greeting = format!("Hello, {}!", name);
    println!("{}", greeting);
}
```

#### Modifying Strings

```rust
fn main() {
    let mut s = String::from("hello");

    // Append a char:
    s.push(' ');

    // Append a string slice:
    s.push_str("world");

    println!("{}", s);  // hello world

    // Concatenation with + (moves left operand):
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;   // s1 is moved into s3; s2 is borrowed
    // println!("{}", s1); // ERROR: s1 was moved
    println!("{}", s3);   // Hello, world!

    // format! for multiple concatenations (no moves):
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);  // s1, s2, s3 still valid
    println!("{}", s);  // tic-tac-toe
}
```

The `+` operator uses the `Add` trait: `fn add(self, s: &str) -> String`. It takes `self` by value (moves it) and `s` by reference — hence why `s1` is consumed and `&s2` is borrowed.

#### Common String Methods

```rust
fn main() {
    let s = String::from("  Hello, World!  ");

    // Trim whitespace:
    println!("{}", s.trim());        // "Hello, World!"
    println!("{}", s.trim_start()); // "Hello, World!  "
    println!("{}", s.trim_end());   // "  Hello, World!"

    let s = String::from("Hello, World!");

    // Case:
    println!("{}", s.to_uppercase());  // HELLO, WORLD!
    println!("{}", s.to_lowercase());  // hello, world!

    // Contains / starts / ends:
    println!("{}", s.contains("World"));    // true
    println!("{}", s.starts_with("Hello")); // true
    println!("{}", s.ends_with("!"));       // true

    // Find (byte index):
    println!("{:?}", s.find('W'));          // Some(7)
    println!("{:?}", s.find("World"));      // Some(7)

    // Replace:
    let replaced = s.replace("World", "Rust");
    println!("{}", replaced);  // Hello, Rust!

    let replaced = "aabbcc".replacen("a", "x", 1);
    println!("{}", replaced);  // xabbcc (replace first occurrence only)

    // Split:
    let csv = "one,two,three,four";
    let parts: Vec<&str> = csv.split(',').collect();
    println!("{:?}", parts);  // ["one", "two", "three", "four"]

    let words: Vec<&str> = "  hello   world  ".split_whitespace().collect();
    println!("{:?}", words);  // ["hello", "world"]

    // Length (bytes) and char count:
    let s = "Hello 🦀";
    println!("bytes={}, chars={}", s.len(), s.chars().count());  // bytes=10, chars=7

    // Repeat:
    println!("{}", "ha".repeat(3));  // hahaha

    // Chars:
    for c in "hello".chars() {
        print!("{} ", c);  // h e l l o
    }
    println!();
}
```

---

### 11.2 &str

`&str` is a string slice — a reference into string data (in the heap, stack, or program binary). It's immutable and doesn't own the data.

We covered the mechanics of `&str` in Chapter 7. Here we focus on the practical conversion between the two types.

#### Conversions

```rust
fn main() {
    // &str → String:
    let literal: &str = "hello";
    let owned: String = literal.to_string();
    let owned: String = String::from(literal);
    let owned: String = literal.to_owned();

    // String → &str:
    let s: String = String::from("hello");
    let slice: &str = &s;          // deref coercion
    let slice: &str = &s[..];      // explicit full slice
    let slice: &str = s.as_str();  // explicit method

    // String → owned slice of part:
    let part: String = s[0..3].to_string();  // "hel"

    // Function accepting &str works with both:
    fn greet(name: &str) { println!("Hello, {}!", name); }
    greet("literal");     // &str directly
    greet(&owned);        // &String auto-deref'd to &str
}
```

#### When to Use Which

| Situation | Use |
|-----------|-----|
| Store a string in a struct | `String` |
| Function parameter (read-only) | `&str` |
| Return an owned string from a function | `String` |
| Reference into existing string data | `&str` |
| Build a string incrementally | `String` |
| Config file / compile-time constant | `&'static str` |

---

### 11.3 UTF-8

Rust strings are always **valid UTF-8**. This has important consequences for how you work with them.

#### What is UTF-8?

UTF-8 is a variable-length encoding for Unicode. Each character (Unicode scalar value) is encoded as 1 to 4 bytes:

| Characters | Example | Bytes |
|------------|---------|-------|
| ASCII (U+0000–U+007F) | `A`, `5`, `!` | 1 byte |
| Extended Latin, Greek, Arabic (U+0080–U+07FF) | `é`, `ñ`, `ψ` | 2 bytes |
| CJK, symbols (U+0800–U+FFFF) | `猫`, `€`, `→` | 3 bytes |
| Emoji, historic (U+10000–U+10FFFF) | `🦀`, `😀` | 4 bytes |

#### Why You Can't Index Strings

```rust
let s = String::from("hello");
let h = s[0];  // ERROR: cannot index into a `String`
```

Why not? In most languages, `s[0]` returns a character. But if `s` is UTF-8:

```
"café" bytes:  c(1) a(1) f(1) é(2)
Total bytes: 5
                                     ↑ é spans bytes 3 and 4
```

What should `s[3]` return? Half of `é`? That would be invalid UTF-8. Rust prevents this by simply not allowing integer indexing on strings.

#### Correct Ways to Access Characters

```rust
fn main() {
    let s = "café";

    // Byte access (use for ASCII-only data):
    let bytes = s.as_bytes();
    println!("bytes: {:?}", bytes);  // [99, 97, 102, 195, 169]
    println!("byte count: {}", s.len());  // 5

    // Character access (Unicode-correct):
    let chars: Vec<char> = s.chars().collect();
    println!("chars: {:?}", chars);  // ['c', 'a', 'f', 'é']
    println!("char count: {}", s.chars().count());  // 4

    // Nth character:
    let third = s.chars().nth(2);  // Some('f')
    println!("3rd char: {:?}", third);

    // Slice by BYTE index (you must know the boundaries!):
    let slice = &s[0..3];  // "caf" (first 3 bytes = first 3 ASCII chars)
    println!("slice: {}", slice);

    // Safe slicing by character position:
    let take_two: String = s.chars().take(2).collect();  // "ca"
    let skip_two: String = s.chars().skip(2).collect();  // "fé"
    println!("{} {}", take_two, skip_two);
}
```

#### Grapheme Clusters (Advanced)

Sometimes even `chars()` isn't enough. Some visual characters are made of multiple Unicode code points:

```
"é"  can be represented as:
  1. Single code point: U+00E9 (1 char, 2 bytes)
  2. Two code points:   U+0065 'e' + U+0301 '◌́' (combining accent) (2 chars)

Both look identical on screen but are different byte sequences.
```

For user-facing text where visual characters matter, use the `unicode-segmentation` crate:

```toml
# Cargo.toml
[dependencies]
unicode-segmentation = "1.10"
```

```rust
use unicode_segmentation::UnicodeSegmentation;

fn main() {
    let s = "नमस्ते";  // Hindi word (devanagari script)
    println!("bytes:     {}", s.len());                    // 18
    println!("chars:     {}", s.chars().count());           // 6
    println!("graphemes: {}", s.graphemes(true).count());   // 4 (visual letters)
}
```

For most applications (English text, identifiers, ASCII data), `chars()` is sufficient. Only reach for graphemes when building text editors or user-facing character counting.

---

## Code Example

### Practice: String Manipulation

```rust
fn count_vowels(s: &str) -> usize {
    s.chars().filter(|c| "aeiouAEIOU".contains(*c)).count()
}

fn reverse(s: &str) -> String {
    s.chars().rev().collect()
}

fn is_palindrome(s: &str) -> bool {
    let clean: String = s.chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect();
    clean == clean.chars().rev().collect::<String>()
}

fn word_count(s: &str) -> usize {
    s.split_whitespace().count()
}

fn capitalize_words(s: &str) -> String {
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
        .collect::<Vec<_>>()
        .join(" ")
}

fn truncate(s: &str, max_chars: usize) -> String {
    if s.chars().count() <= max_chars {
        s.to_string()
    } else {
        let truncated: String = s.chars().take(max_chars).collect();
        format!("{}…", truncated)
    }
}

fn count_words_map(s: &str) -> std::collections::HashMap<String, usize> {
    let mut map = std::collections::HashMap::new();
    for word in s.split_whitespace() {
        let clean: String = word.chars()
            .filter(|c| c.is_alphabetic())
            .map(|c| c.to_ascii_lowercase())
            .collect();
        if !clean.is_empty() {
            *map.entry(clean).or_insert(0) += 1;
        }
    }
    map
}

fn main() {
    let text = "A man a plan a canal Panama";
    println!("Text:           '{}'", text);
    println!("Vowels:         {}", count_vowels(text));
    println!("Reversed:       {}", reverse(text));
    println!("Is palindrome:  {}", is_palindrome(text));
    println!("Word count:     {}", word_count(text));
    println!("Capitalized:    {}", capitalize_words(&text.to_lowercase()));

    let long = "the quick brown fox jumps over the lazy dog";
    println!("\nTruncate 15:    '{}'", truncate(long, 15));

    println!("\nWord frequency:");
    let mut freq: Vec<(String, usize)> = count_words_map(text).into_iter().collect();
    freq.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    for (word, count) in freq.iter().take(5) {
        println!("  '{}': {}", word, count);
    }

    // UTF-8 examples
    println!("\nUTF-8 examples:");
    let emoji = "Hello 🦀 Rust!";
    println!("bytes={}, chars={}", emoji.len(), emoji.chars().count());

    let mixed = "Привет мир";
    println!("'{}' — {} bytes, {} chars", mixed, mixed.len(), mixed.chars().count());
}
```

### Line-by-Line Explanation

```rust
fn reverse(s: &str) -> String {
    s.chars().rev().collect()
}
```
- `.chars()` iterates Unicode scalar values (not bytes!)
- `.rev()` reverses the iterator
- `.collect()` collects chars into a `String` — type inferred from return type
- This correctly reverses multi-byte characters

```rust
fn capitalize_words(s: &str) -> String {
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
```
- `first.to_uppercase()` returns an iterator (because some chars uppercase to multiple chars, e.g., `ß` → `SS`)
- `.collect::<String>()` collects the uppercase iterator into a String
- `chars.as_str()` returns the remaining (non-consumed) part of the char iterator as a `&str`
- `upper + chars.as_str()` concatenates the uppercase first char with the rest

```rust
fn truncate(s: &str, max_chars: usize) -> String {
    let truncated: String = s.chars().take(max_chars).collect();
    format!("{}…", truncated)
}
```
- `chars().take(max_chars)` takes exactly `max_chars` Unicode characters (not bytes!)
- This correctly handles multi-byte characters — `truncate("café", 3)` → `"caf…"` not a broken byte slice

---

## Common Mistakes

### Mistake 1: Assuming `len()` gives character count

```rust
let s = "café";
println!("{}", s.len());           // 5 (bytes)
println!("{}", s.chars().count()); // 4 (characters)
// WRONG if you meant "how many characters"
```

### Mistake 2: Slicing in the middle of a multi-byte character

```rust
let s = "café";
let bad = &s[0..4];   // PANIC: byte index 4 is not a char boundary
                       // 'é' spans bytes 3-4 (2 bytes)
let good = &s[0..3];  // "caf" (first 3 bytes = 3 ASCII chars)
```

Use `chars()` operations when you need character-safe slicing.

### Mistake 3: Using `+` when format! is clearer

```rust
// Confusing — s1 is moved!
let result = s1 + ", " + &s2 + " and " + &s3;

// Clearer:
let result = format!("{}, {} and {}", s1, s2, s3);
```

### Mistake 4: Collecting chars to count when you just need `len` for ASCII

```rust
let s = "hello world";  // ASCII only
let count = s.chars().count();  // unnecessary — allocates iterator

// For ASCII-only strings, len() is fine:
let count = s.len();  // O(1), no allocation
```

---

## Best Practices

1. **Use `&str` in function parameters** — works with both `String` and literals
2. **Use `format!` for concatenation** — clearer than `+` chains, no moves
3. **Use `.chars()` for iteration** — not `.bytes()` unless you specifically need bytes
4. **Be careful with byte slicing** on strings that might contain non-ASCII
5. **Pre-allocate with `String::with_capacity`** when you know the approximate size
6. **Use `.trim()` on user input** — always removes the `\n` from `read_line`
7. **Prefer `chars().count()` over `.len()`** when measuring user-visible length

---

## Exercises

### Exercise 1: Pangram Checker

Write `is_pangram(s: &str) -> bool` — returns true if the string contains every letter of the alphabet at least once (case-insensitive).

### Exercise 2: Caesar Cipher

Write `caesar(text: &str, shift: u8) -> String` that shifts each ASCII letter by `shift` positions (wrapping: z+1=a). Non-letter characters pass through unchanged.

### Exercise 3: Run-Length Encoding

Write `rle_encode(s: &str) -> String` that encodes consecutive repeated characters: `"aaabbc"` → `"3a2b1c"`. Write `rle_decode(s: &str) -> String` that reverses it.

### Exercise 4: String Statistics

Write a function that takes a `&str` and returns a struct `StringStats { char_count: usize, word_count: usize, sentence_count: usize, avg_word_len: f64, longest_word: String }`.

### Exercise 5: Anagram Check

Write `are_anagrams(a: &str, b: &str) -> bool` — returns true if two strings are anagrams (same characters in different order, case-insensitive, ignoring spaces).

---

## Solutions

### Solution 1

```rust
fn is_pangram(s: &str) -> bool {
    let lower = s.to_ascii_lowercase();
    ('a'..='z').all(|c| lower.contains(c))
}

fn main() {
    println!("{}", is_pangram("the quick brown fox jumps over the lazy dog")); // true
    println!("{}", is_pangram("hello world")); // false
}
```

### Solution 2

```rust
fn caesar(text: &str, shift: u8) -> String {
    text.chars().map(|c| {
        if c.is_ascii_alphabetic() {
            let base = if c.is_uppercase() { b'A' } else { b'a' };
            let shifted = (c as u8 - base + shift) % 26 + base;
            shifted as char
        } else {
            c
        }
    }).collect()
}

fn main() {
    let msg = "Hello, World!";
    let encoded = caesar(msg, 13);
    let decoded = caesar(&encoded, 13);
    println!("Original: {}", msg);
    println!("Encoded:  {}", encoded);
    println!("Decoded:  {}", decoded);
}
```

### Solution 3

```rust
fn rle_encode(s: &str) -> String {
    if s.is_empty() { return String::new(); }
    let mut result = String::new();
    let mut chars = s.chars();
    let mut current = chars.next().unwrap();
    let mut count = 1usize;

    for c in chars {
        if c == current {
            count += 1;
        } else {
            result.push_str(&format!("{}{}", count, current));
            current = c;
            count = 1;
        }
    }
    result.push_str(&format!("{}{}", count, current));
    result
}

fn rle_decode(s: &str) -> String {
    let mut result = String::new();
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if let Some(n) = c.to_digit(10) {
            if let Some(ch) = chars.next() {
                for _ in 0..n { result.push(ch); }
            }
        }
    }
    result
}

fn main() {
    let encoded = rle_encode("aaabbc");
    println!("Encoded: {}", encoded);       // 3a2b1c
    println!("Decoded: {}", rle_decode(&encoded)); // aaabbc
}
```

### Solution 4

```rust
struct StringStats {
    char_count: usize,
    word_count: usize,
    sentence_count: usize,
    avg_word_len: f64,
    longest_word: String,
}

fn analyze(s: &str) -> StringStats {
    let words: Vec<&str> = s.split_whitespace().collect();
    let word_count = words.len();
    let avg_word_len = if word_count == 0 { 0.0 }
        else { words.iter().map(|w| w.chars().count()).sum::<usize>() as f64 / word_count as f64 };
    let longest = words.iter().max_by_key(|w| w.len()).map(|s| s.to_string()).unwrap_or_default();

    StringStats {
        char_count: s.chars().count(),
        word_count,
        sentence_count: s.chars().filter(|&c| c == '.' || c == '!' || c == '?').count(),
        avg_word_len,
        longest_word: longest,
    }
}

fn main() {
    let s = "The quick brown fox jumps over the lazy dog. It was a great day!";
    let stats = analyze(s);
    println!("chars={}, words={}, sentences={}, avg_len={:.2}, longest='{}'",
        stats.char_count, stats.word_count, stats.sentence_count,
        stats.avg_word_len, stats.longest_word);
}
```

### Solution 5

```rust
fn are_anagrams(a: &str, b: &str) -> bool {
    let mut ca: Vec<char> = a.chars().filter(|c| !c.is_whitespace())
        .map(|c| c.to_ascii_lowercase()).collect();
    let mut cb: Vec<char> = b.chars().filter(|c| !c.is_whitespace())
        .map(|c| c.to_ascii_lowercase()).collect();
    ca.sort_unstable();
    cb.sort_unstable();
    ca == cb
}

fn main() {
    println!("{}", are_anagrams("listen", "silent"));      // true
    println!("{}", are_anagrams("Astronomer", "Moon starer")); // true
    println!("{}", are_anagrams("hello", "world"));         // false
}
```

---

## Quiz

**Q1.** Why can't you index a Rust String with `s[0]`?

a) It's a language limitation  
b) Strings are immutable  
c) UTF-8 characters have variable byte lengths — indexing by integer would return a byte, potentially in the middle of a multi-byte character  
d) Strings are not arrays  

**Q2.** What does `String::from("hello").len()` return?

a) The number of characters  
b) The number of bytes  
c) The number of words  
d) The capacity  

**Q3.** What is the difference between `.chars()` and `.bytes()`?

a) No difference — both iterate the same data  
b) `.chars()` iterates Unicode scalar values (char); `.bytes()` iterates raw bytes (u8)  
c) `.bytes()` is deprecated  
d) `.chars()` only works on ASCII  

**Q4.** Why does `s1 + &s2` consume `s1` but not `s2`?

a) A bug in Rust  
b) The `Add` trait for String is `fn add(self, s: &str) -> String` — takes ownership of `self`  
c) `s2` is automatically cloned  
d) Strings are always copied  

**Q5.** What is the safest way to concatenate multiple strings without ownership issues?

a) Use `+` in a chain  
b) Clone everything  
c) Use `format!("{}{}{}", s1, s2, s3)` — borrows all arguments  
d) Use `push_str` in a loop  

---

## Quiz Answers

**A1.** c) UTF-8 characters have variable byte lengths  
*`s[3]` would return the byte at position 3. If that byte is in the middle of a multi-byte character (like `é`), returning it as a `char` would be invalid. Rust prevents this at the type level.*

**A2.** b) The number of bytes  
*`len()` on `String` and `&str` always returns byte count. For ASCII strings this equals the character count. For Unicode strings (containing non-ASCII), use `.chars().count()`.*

**A3.** b) `.chars()` iterates Unicode scalar values; `.bytes()` iterates raw bytes  
*For `"café"`: `.chars()` yields `'c','a','f','é'` (4 items). `.bytes()` yields `99,97,102,195,169` (5 items, because `é` is 2 bytes). Use `.chars()` for text processing, `.bytes()` for binary/ASCII data.*

**A4.** b) The `Add` trait for String takes `self` by value  
*`fn add(self, s: &str) -> String` — `self` (s1) is consumed into the new String. `s` (&s2) is borrowed. This is an optimization: the left String's buffer is reused rather than allocating new memory.*

**A5.** c) Use `format!`  
*`format!` takes all arguments by reference — no ownership is transferred, no clones needed. It's the idiomatic way to build strings from multiple parts.*

---

## Chapter Summary

- `String` is an owned, heap-allocated, growable UTF-8 string
- `&str` is a borrowed string slice — a pointer + length into existing string data
- Rust strings are **always valid UTF-8** — this affects how you access characters
- You **cannot index strings** with integers — use `.chars()`, `.bytes()`, or byte-range slicing
- `s.len()` → byte count; `s.chars().count()` → Unicode character count
- `.chars()` is the correct way to iterate over characters in Unicode text
- **Conversions**: `&str` → `String` via `.to_string()` / `String::from()`, `String` → `&str` via `&s` / `.as_str()`
- `format!` is the idiomatic multi-string concatenation — no ownership transfers

In Chapter 12, we explore `Vec<T>` — Rust's growable, heap-allocated sequence type.
