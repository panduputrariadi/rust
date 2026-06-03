# Chapter 13: HashMap

## Learning Objectives

By the end of this chapter, you will:

- Create and populate `HashMap<K, V>`
- Insert, access, update, and remove entries
- Use the Entry API for efficient conditional inserts
- Iterate over keys, values, and key-value pairs
- Know when to use HashMap vs other collections
- Build a Contact Manager using HashMap

---

## Theory

### 13.1 Insert

`HashMap<K, V>` maps keys of type `K` to values of type `V` using a hash function. Lookup, insert, and delete are O(1) on average.

#### Creating a HashMap

```rust
use std::collections::HashMap;

fn main() {
    // Empty map:
    let mut scores: HashMap<String, i32> = HashMap::new();

    // Insert:
    scores.insert(String::from("Alice"), 92);
    scores.insert(String::from("Bob"),   85);
    scores.insert(String::from("Carol"), 78);

    println!("{:?}", scores);

    // From iterators (zip two lists into a map):
    let names = vec!["Alice", "Bob", "Carol"];
    let grades = vec![92, 85, 78];
    let map: HashMap<&str, i32> = names.iter().copied().zip(grades.iter().copied()).collect();
    println!("{:?}", map);

    // From array of tuples:
    let config: HashMap<&str, &str> = [
        ("host", "localhost"),
        ("port", "8080"),
        ("mode", "debug"),
    ].iter().cloned().collect();
    println!("{:?}", config);
}
```

#### HashMap and Ownership

```rust
use std::collections::HashMap;

fn main() {
    let key = String::from("color");
    let val = String::from("blue");

    let mut map = HashMap::new();
    map.insert(key, val);  // key and val are MOVED into the map

    // println!("{}", key);  // ERROR: key was moved
    // println!("{}", val);  // ERROR: val was moved

    // For Copy types (i32, bool, etc.), they are copied — not moved
    let score = 42;
    map.insert(String::from("score"), score.to_string());
    println!("{}", score);  // OK — i32 is Copy (but we passed to_string() here, a String)

    // To avoid moving, use references (but the map can't outlive the referenced data):
    let key = String::from("theme");
    map.insert(key.clone(), String::from("dark")); // clone key to keep both
    println!("{}", key);  // still valid
}
```

#### Accessing Values

```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert("Alice", 92);
    scores.insert("Bob",   85);

    // get() returns Option<&V>:
    match scores.get("Alice") {
        Some(score) => println!("Alice: {}", score),
        None        => println!("Not found"),
    }

    // get() with unwrap_or:
    let carol = scores.get("Carol").unwrap_or(&0);
    println!("Carol: {}", carol);  // 0

    // Index with [] (panics if key missing):
    println!("Bob: {}", scores["Bob"]);  // 85
    // println!("{}", scores["Carol"]);  // PANIC: key not found

    // Check existence:
    println!("{}", scores.contains_key("Alice"));  // true
    println!("{}", scores.contains_key("Carol"));  // false
}
```

---

### 13.2 Remove

```rust
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert("a", 1);
    map.insert("b", 2);
    map.insert("c", 3);

    // Remove and get the value:
    let removed = map.remove("b");
    println!("{:?}", removed);  // Some(2)
    println!("{:?}", map);      // {"a": 1, "c": 3}

    let missing = map.remove("z");
    println!("{:?}", missing);  // None

    // Remove only if condition met (Entry API):
    map.retain(|k, v| *v > 1);  // keep entries where value > 1
    println!("{:?}", map);       // {"c": 3}

    // Clear:
    map.clear();
    println!("len: {}", map.len());  // 0
}
```

---

### 13.3 Iterate

```rust
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert("one", 1);
    map.insert("two", 2);
    map.insert("three", 3);

    // Iterate over key-value pairs (arbitrary order):
    for (key, value) in &map {
        println!("{}: {}", key, value);
    }

    // Keys only:
    let mut keys: Vec<&&str> = map.keys().collect();
    keys.sort();
    println!("keys: {:?}", keys);

    // Values only:
    let mut values: Vec<&i32> = map.values().collect();
    values.sort();
    println!("values: {:?}", values);

    // Mutable values:
    for value in map.values_mut() {
        *value *= 10;
    }
    println!("{:?}", map);

    // into_iter() consumes the map:
    let owned_map = map;
    for (k, v) in owned_map {  // consumed
        println!("{} → {}", k, v);
    }
    // owned_map is no longer usable
}
```

#### Sorted Iteration

HashMap does not guarantee order. To iterate in sorted order:

```rust
use std::collections::HashMap;

fn main() {
    let mut map: HashMap<&str, i32> = HashMap::new();
    map.insert("banana", 3);
    map.insert("apple", 1);
    map.insert("cherry", 2);

    // Sort by key:
    let mut entries: Vec<(&&str, &i32)> = map.iter().collect();
    entries.sort_by_key(|&(k, _)| *k);
    for (k, v) in entries {
        println!("{}: {}", k, v);
    }

    // Sort by value descending:
    let mut entries: Vec<(&&str, &i32)> = map.iter().collect();
    entries.sort_by(|a, b| b.1.cmp(a.1));
    for (k, v) in entries {
        println!("{}: {}", k, v);
    }
}
```

For a map that maintains insertion order, use `IndexMap` from the `indexmap` crate.

#### The Entry API

The Entry API provides efficient, conditional modification:

```rust
use std::collections::HashMap;

fn main() {
    let mut scores: HashMap<String, i32> = HashMap::new();

    // Insert only if key doesn't exist:
    scores.entry(String::from("Alice")).or_insert(0);
    scores.entry(String::from("Alice")).or_insert(100);  // ignored — Alice already exists
    println!("{}", scores["Alice"]);  // 0

    // Insert using a function (lazily evaluated):
    scores.entry(String::from("Bob")).or_insert_with(|| 50 + 25);
    println!("{}", scores["Bob"]);  // 75

    // Modify existing or insert default then modify:
    let text = "hello world hello rust hello";
    let mut word_count: HashMap<&str, i32> = HashMap::new();
    for word in text.split_whitespace() {
        let count = word_count.entry(word).or_insert(0);
        *count += 1;  // dereference to modify the value in place
    }
    println!("{:?}", word_count);
    // {"hello": 3, "world": 1, "rust": 1}

    // and_modify + or_insert:
    scores
        .entry(String::from("Carol"))
        .and_modify(|s| *s += 10)  // if exists: add 10
        .or_insert(50);             // if not: insert 50
    println!("{}", scores["Carol"]);  // 50 (was not present)

    scores
        .entry(String::from("Alice"))
        .and_modify(|s| *s += 10)
        .or_insert(50);
    println!("{}", scores["Alice"]);  // 10 (was 0, modified by +10)
}
```

`entry().or_insert()` is the idiomatic pattern for counting and grouping — much more efficient than `get` + `insert` because it only hashes the key once.

#### Grouping with HashMap

```rust
use std::collections::HashMap;

fn group_by_length(words: &[&str]) -> HashMap<usize, Vec<&str>> {
    let mut groups: HashMap<usize, Vec<&str>> = HashMap::new();
    for &word in words {
        groups.entry(word.len()).or_insert_with(Vec::new).push(word);
    }
    groups
}

fn main() {
    let words = ["one", "two", "three", "four", "five", "six", "seven"];
    let groups = group_by_length(&words);

    let mut keys: Vec<usize> = groups.keys().cloned().collect();
    keys.sort();
    for k in keys {
        println!("length {}: {:?}", k, groups[&k]);
    }
}
```

#### HashMap vs BTreeMap

| Feature | `HashMap` | `BTreeMap` |
|---------|-----------|------------|
| Order | No guaranteed order | Sorted by key |
| Lookup | O(1) average | O(log n) |
| Memory | Higher (hash table) | Lower (B-tree) |
| Iteration | Arbitrary | Sorted |
| Use when | Fast lookup, order doesn't matter | Need sorted iteration |

```rust
use std::collections::BTreeMap;

fn main() {
    let mut map = BTreeMap::new();
    map.insert("c", 3);
    map.insert("a", 1);
    map.insert("b", 2);

    for (k, v) in &map {
        println!("{}: {}", k, v);  // always prints a, b, c in order
    }
}
```

---

## Code Example

### Mini Project: Contact Manager

#### Project Overview

A CLI contact book demonstrating HashMap for O(1) lookups by ID, search, grouping, and CRUD operations.

#### Functional Requirements

1. Add contacts (name, phone, email, category)
2. Search by name or phone
3. List all contacts, optionally grouped by category
4. Update a contact's details
5. Delete a contact
6. Show statistics (contacts per category, etc.)

#### Complete Source Code

```rust
use std::collections::HashMap;
use std::io;
use std::io::Write;
use std::fmt;

#[derive(Debug, Clone)]
struct Contact {
    id: u32,
    name: String,
    phone: String,
    email: String,
    category: String,
}

impl Contact {
    fn new(id: u32, name: &str, phone: &str, email: &str, category: &str) -> Contact {
        Contact {
            id,
            name: String::from(name),
            phone: String::from(phone),
            email: String::from(email),
            category: String::from(category),
        }
    }

    fn display(&self) {
        println!(
            "  [{:04}] {:<20} {:>15}  {:<25}  [{}]",
            self.id, self.name, self.phone, self.email, self.category
        );
    }
}

impl fmt::Display for Contact {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.name, self.phone)
    }
}

struct ContactBook {
    contacts: HashMap<u32, Contact>,
    next_id: u32,
}

impl ContactBook {
    fn new() -> ContactBook {
        ContactBook {
            contacts: HashMap::new(),
            next_id: 1,
        }
    }

    fn add(&mut self, name: &str, phone: &str, email: &str, category: &str) -> u32 {
        let id = self.next_id;
        self.contacts.insert(id, Contact::new(id, name, phone, email, category));
        self.next_id += 1;
        id
    }

    fn get(&self, id: u32) -> Option<&Contact> {
        self.contacts.get(&id)
    }

    fn update(&mut self, id: u32, phone: Option<&str>, email: Option<&str>, category: Option<&str>) -> bool {
        if let Some(contact) = self.contacts.get_mut(&id) {
            if let Some(p) = phone    { contact.phone    = String::from(p); }
            if let Some(e) = email    { contact.email    = String::from(e); }
            if let Some(c) = category { contact.category = String::from(c); }
            true
        } else {
            false
        }
    }

    fn delete(&mut self, id: u32) -> Option<Contact> {
        self.contacts.remove(&id)
    }

    fn search_by_name(&self, query: &str) -> Vec<&Contact> {
        let q = query.to_ascii_lowercase();
        let mut results: Vec<&Contact> = self.contacts.values()
            .filter(|c| c.name.to_ascii_lowercase().contains(&q))
            .collect();
        results.sort_by_key(|c| &c.name);
        results
    }

    fn search_by_phone(&self, query: &str) -> Vec<&Contact> {
        self.contacts.values()
            .filter(|c| c.phone.contains(query))
            .collect()
    }

    fn list_all(&self) {
        if self.contacts.is_empty() {
            println!("  (no contacts)");
            return;
        }
        let mut contacts: Vec<&Contact> = self.contacts.values().collect();
        contacts.sort_by_key(|c| &c.name);

        println!("  {:<6} {:<20} {:>15}  {:<25}  Category", "ID", "Name", "Phone", "Email");
        println!("  {}", "-".repeat(78));
        for c in contacts {
            c.display();
        }
        println!("  Total: {} contacts", self.contacts.len());
    }

    fn list_by_category(&self) {
        if self.contacts.is_empty() {
            println!("  (no contacts)");
            return;
        }

        let mut groups: HashMap<String, Vec<&Contact>> = HashMap::new();
        for contact in self.contacts.values() {
            groups.entry(contact.category.clone())
                .or_insert_with(Vec::new)
                .push(contact);
        }

        let mut categories: Vec<&String> = groups.keys().collect();
        categories.sort();

        for category in categories {
            let mut members = groups[category].clone();
            members.sort_by_key(|c| &c.name);
            println!("\n  === {} ({}) ===", category, members.len());
            for c in members {
                c.display();
            }
        }
    }

    fn stats(&self) {
        println!("  Total contacts: {}", self.contacts.len());

        let mut category_counts: HashMap<&str, usize> = HashMap::new();
        for c in self.contacts.values() {
            *category_counts.entry(c.category.as_str()).or_insert(0) += 1;
        }

        let mut sorted: Vec<(&&str, &usize)> = category_counts.iter().collect();
        sorted.sort_by(|a, b| b.1.cmp(a.1));

        println!("  By category:");
        for (cat, count) in sorted {
            println!("    {:<20} {}", cat, count);
        }
    }
}

fn prompt(msg: &str) -> String {
    print!("{}", msg);
    io::stdout().flush().unwrap();
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s.trim().to_string()
}

fn main() {
    let mut book = ContactBook::new();

    // Seed data
    book.add("Alice Johnson",   "+62-812-3456-7890", "alice@email.com",   "Work");
    book.add("Bob Smith",       "+62-813-2345-6789", "bob@gmail.com",     "Friends");
    book.add("Carol Williams",  "+62-814-3456-7891", "carol@work.com",    "Work");
    book.add("David Brown",     "+62-815-4567-8901", "david@gmail.com",   "Friends");
    book.add("Eve Davis",       "+62-816-5678-9012", "eve@company.com",   "Work");
    book.add("Frank Miller",    "+62-817-6789-0123", "frank@home.com",    "Family");
    book.add("Grace Wilson",    "+62-818-7890-1234", "grace@family.net",  "Family");

    println!("=== Contact Manager ===\n");

    loop {
        println!("\n1.List  2.Group  3.Add  4.Search  5.Update  6.Delete  7.Stats  q.Quit");
        match prompt("Choice: ").as_str() {
            "1" => { println!(); book.list_all(); }
            "2" => { println!(); book.list_by_category(); }
            "3" => {
                let name     = prompt("  Name: ");
                let phone    = prompt("  Phone: ");
                let email    = prompt("  Email: ");
                let category = prompt("  Category (Work/Friends/Family/Other): ");
                let id = book.add(&name, &phone, &email, &category);
                println!("  Added contact #{}", id);
            }
            "4" => {
                let query = prompt("  Search (name or phone): ");
                let by_name  = book.search_by_name(&query);
                let by_phone = book.search_by_phone(&query);

                let mut results: Vec<&Contact> = by_name;
                for c in by_phone {
                    if !results.iter().any(|r| r.id == c.id) {
                        results.push(c);
                    }
                }
                results.sort_by_key(|c| &c.name);

                if results.is_empty() {
                    println!("  No matches found.");
                } else {
                    println!("  Found {} result(s):", results.len());
                    for c in results { c.display(); }
                }
            }
            "5" => {
                let id: u32 = match prompt("  Contact ID to update: ").parse() {
                    Ok(n) => n,
                    Err(_) => { println!("  Invalid ID."); continue; }
                };
                if book.get(id).is_none() {
                    println!("  Contact not found.");
                    continue;
                }
                let phone_input    = prompt("  New phone (enter to skip): ");
                let email_input    = prompt("  New email (enter to skip): ");
                let category_input = prompt("  New category (enter to skip): ");

                let phone    = if phone_input.is_empty()    { None } else { Some(phone_input.as_str()) };
                let email    = if email_input.is_empty()    { None } else { Some(email_input.as_str()) };
                let category = if category_input.is_empty() { None } else { Some(category_input.as_str()) };

                if book.update(id, phone, email, category) {
                    println!("  Updated.");
                }
            }
            "6" => {
                let id: u32 = match prompt("  Contact ID to delete: ").parse() {
                    Ok(n) => n,
                    Err(_) => { println!("  Invalid ID."); continue; }
                };
                match book.delete(id) {
                    Some(c) => println!("  Deleted: {}", c),
                    None    => println!("  Contact not found."),
                }
            }
            "7" => { println!(); book.stats(); }
            "q" | "quit" => { println!("Goodbye!"); break; }
            _ => println!("  Unknown option."),
        }
    }
}
```

#### Code Explanation

```rust
struct ContactBook {
    contacts: HashMap<u32, Contact>,
    next_id: u32,
}
```
- `HashMap<u32, Contact>` — integer ID maps to Contact struct
- O(1) lookup by ID — faster than scanning a `Vec` when the book grows large

```rust
fn add(&mut self, ...) -> u32 {
    let id = self.next_id;
    self.contacts.insert(id, Contact::new(id, ...));
    self.next_id += 1;
    id
}
```
- Returns the assigned ID so the caller can reference the new contact
- `insert` on HashMap: if the key existed, it returns `Some(old_value)` and replaces it; here IDs are always new so no collision

```rust
fn update(&mut self, id: u32, phone: Option<&str>, ...) -> bool {
    if let Some(contact) = self.contacts.get_mut(&id) {
        if let Some(p) = phone { contact.phone = String::from(p); }
        true
    } else {
        false
    }
}
```
- `get_mut` returns `Option<&mut Contact>` — a mutable reference to the value
- Uses `Option` parameters to allow partial updates (only update fields that were provided)

```rust
fn list_by_category(&self) {
    let mut groups: HashMap<String, Vec<&Contact>> = HashMap::new();
    for contact in self.contacts.values() {
        groups.entry(contact.category.clone())
            .or_insert_with(Vec::new)
            .push(contact);
    }
```
- **Entry API**: `or_insert_with(Vec::new)` inserts an empty Vec only if the category key doesn't exist yet, then returns a `&mut Vec<&Contact>` — `.push(contact)` adds to it
- `Vec<&Contact>` — borrowed references, no cloning of Contact data

#### Refactoring Suggestions

1. **Persistent storage**: serialize to JSON with `serde_json`, load on startup
2. **Multiple indexes**: maintain `HashMap<String, Vec<u32>>` for name→IDs and phone→IDs
3. **Fuzzy search**: use the `fuzzy-matcher` crate for forgiving search
4. **Import/Export**: support CSV import/export
5. **Validation**: validate phone format, email format

#### Challenge Exercises

1. Add a `tags: Vec<String>` field to Contact and implement tag-based filtering
2. Build a reverse phone book: given a phone number, return all contacts with that phone
3. Implement undo/redo for the last 10 operations using a `Vec<Operation>` history
4. Add birthday field and a `upcoming_birthdays(days: u32)` method

#### Real World Extensions

- Store contacts in SQLite using `rusqlite`
- Sync with Google Contacts via their API
- Build a REST API around the ContactBook using `axum`
- Add WebSocket notifications when a contact is updated

---

## Common Mistakes

### Mistake 1: Using `[]` operator on a missing key

```rust
let map: HashMap<&str, i32> = HashMap::new();
let val = map["missing"];  // PANIC: key not found

// Fix: use get()
let val = map.get("missing").unwrap_or(&0);
// Or: map.get("missing").copied().unwrap_or(0)
```

### Mistake 2: Inserting then getting in two steps

```rust
// Inefficient — hashes the key twice
if !map.contains_key("key") {
    map.insert("key", 0);
}
let val = map.get_mut("key").unwrap();
*val += 1;

// Efficient — hashes once
let val = map.entry("key").or_insert(0);
*val += 1;
```

### Mistake 3: Expecting sorted output from HashMap

```rust
let mut map = HashMap::new();
map.insert("b", 2);
map.insert("a", 1);
map.insert("c", 3);

for (k, v) in &map {
    print!("{} ", k);  // output order is NOT guaranteed — could be b,a,c or any order
}
```

### Mistake 4: Moving keys into the map and then trying to use them

```rust
let key = String::from("name");
map.insert(key, String::from("Alice"));  // key moved into map
println!("{}", key);  // ERROR: key was moved

// Fix: use a clone or use &str keys
map.insert(key.clone(), String::from("Alice"));
println!("{}", key);  // OK

// Or use string literals as keys (&str):
let mut map: HashMap<&str, String> = HashMap::new();
map.insert("name", String::from("Alice"));
let key_literal = "name";  // still valid
println!("{}", map[key_literal]);  // OK
```

---

## Best Practices

1. **Use the Entry API** for insert-or-update patterns — single hash lookup
2. **Use `&str` keys when possible** — avoids allocating `String` for every key
3. **Pre-allocate with `HashMap::with_capacity`** for known-size maps
4. **Use `BTreeMap` when sorted order matters** — pays O(log n) for sorted iteration
5. **Return `Option<&V>` from lookup methods** — let callers handle missing keys
6. **Collect with `.iter().collect()`** for a sorted snapshot when you need ordered output
7. **Use `retain`** for efficient conditional removal without creating a new map

---

## Exercises

### Exercise 1: Word Frequency Counter

Write `word_frequency(text: &str) -> HashMap<String, usize>` that counts how often each word appears (case-insensitive, stripping punctuation). Print the top 5 most frequent words.

### Exercise 2: Invert a Map

Write `invert<K: Clone + Eq + std::hash::Hash, V: Clone + Eq + std::hash::Hash>(map: &HashMap<K, V>) -> HashMap<V, Vec<K>>` that swaps keys and values (handling duplicate values by grouping keys into a Vec).

### Exercise 3: Student Grade Book

Use a `HashMap<String, Vec<f64>>` mapping student names to score lists. Write: `add_score(name, score)`, `average(name) -> Option<f64>`, `top_student() -> Option<(&String, f64)>`, `class_average() -> f64`.

### Exercise 4: Anagram Groups

Given a list of words, group them by their sorted-character signature (anagrams have the same signature). Return a `HashMap<String, Vec<String>>` where each value is a group of anagrams.

### Exercise 5: Caching Fibonacci

Write a recursive Fibonacci with memoization using `HashMap<u64, u64>` as a cache. The cache should be passed as a mutable reference to avoid recreating it on each call.

---

## Solutions

### Solution 1

```rust
use std::collections::HashMap;

fn word_frequency(text: &str) -> HashMap<String, usize> {
    let mut freq = HashMap::new();
    for word in text.split_whitespace() {
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

fn main() {
    let text = "To be or not to be that is the question whether tis nobler in the mind";
    let freq = word_frequency(text);
    let mut sorted: Vec<(&String, &usize)> = freq.iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));
    for (word, count) in sorted.iter().take(5) {
        println!("  '{}': {}", word, count);
    }
}
```

### Solution 2

```rust
use std::collections::HashMap;

fn invert<K, V>(map: &HashMap<K, V>) -> HashMap<V, Vec<K>>
where
    K: Clone + Eq + std::hash::Hash,
    V: Clone + Eq + std::hash::Hash,
{
    let mut inverted: HashMap<V, Vec<K>> = HashMap::new();
    for (k, v) in map {
        inverted.entry(v.clone()).or_insert_with(Vec::new).push(k.clone());
    }
    inverted
}

fn main() {
    let mut map = HashMap::new();
    map.insert("alice", "eng");
    map.insert("bob",   "eng");
    map.insert("carol", "sales");
    map.insert("dave",  "sales");

    let inv = invert(&map);
    let mut keys: Vec<&&str> = inv.keys().collect();
    keys.sort();
    for k in keys {
        let mut names = inv[k].clone();
        names.sort();
        println!("{}: {:?}", k, names);
    }
}
```

### Solution 3

```rust
use std::collections::HashMap;

struct GradeBook {
    scores: HashMap<String, Vec<f64>>,
}

impl GradeBook {
    fn new() -> Self { Self { scores: HashMap::new() } }

    fn add_score(&mut self, name: &str, score: f64) {
        self.scores.entry(String::from(name)).or_insert_with(Vec::new).push(score);
    }

    fn average(&self, name: &str) -> Option<f64> {
        self.scores.get(name).map(|scores| {
            scores.iter().sum::<f64>() / scores.len() as f64
        })
    }

    fn top_student(&self) -> Option<(&String, f64)> {
        self.scores.iter()
            .filter_map(|(name, scores)| {
                if scores.is_empty() { None }
                else { Some((name, scores.iter().sum::<f64>() / scores.len() as f64)) }
            })
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
    }

    fn class_average(&self) -> f64 {
        let all_scores: Vec<f64> = self.scores.values().flatten().cloned().collect();
        if all_scores.is_empty() { return 0.0; }
        all_scores.iter().sum::<f64>() / all_scores.len() as f64
    }
}

fn main() {
    let mut gb = GradeBook::new();
    gb.add_score("Alice", 92.0); gb.add_score("Alice", 88.0); gb.add_score("Alice", 95.0);
    gb.add_score("Bob",   78.0); gb.add_score("Bob",   82.0);
    gb.add_score("Carol", 95.0); gb.add_score("Carol", 97.0); gb.add_score("Carol", 99.0);

    println!("Alice avg: {:.2}", gb.average("Alice").unwrap_or(0.0));
    if let Some((name, avg)) = gb.top_student() {
        println!("Top student: {} ({:.2})", name, avg);
    }
    println!("Class avg: {:.2}", gb.class_average());
}
```

### Solution 4

```rust
use std::collections::HashMap;

fn anagram_groups(words: &[&str]) -> HashMap<String, Vec<String>> {
    let mut groups: HashMap<String, Vec<String>> = HashMap::new();
    for &word in words {
        let mut key: Vec<char> = word.to_ascii_lowercase().chars().collect();
        key.sort_unstable();
        let key: String = key.into_iter().collect();
        groups.entry(key).or_insert_with(Vec::new).push(String::from(word));
    }
    groups
}

fn main() {
    let words = ["eat", "tea", "tan", "ate", "nat", "bat", "listen", "silent", "enlist"];
    let groups = anagram_groups(&words);
    let mut group_list: Vec<Vec<String>> = groups.into_values()
        .filter(|g| g.len() > 1)
        .collect();
    group_list.sort_by_key(|g| g[0].clone());
    for mut group in group_list {
        group.sort();
        println!("{:?}", group);
    }
}
```

### Solution 5

```rust
use std::collections::HashMap;

fn fib(n: u64, cache: &mut HashMap<u64, u64>) -> u64 {
    if n <= 1 { return n; }
    if let Some(&cached) = cache.get(&n) {
        return cached;
    }
    let result = fib(n - 1, cache) + fib(n - 2, cache);
    cache.insert(n, result);
    result
}

fn main() {
    let mut cache = HashMap::new();
    for i in 0..=20 {
        print!("{} ", fib(i, &mut cache));
    }
    println!();
    println!("fib(50) = {}", fib(50, &mut cache));
    println!("cache size: {}", cache.len());
}
```

---

## Quiz

**Q1.** What does `map.entry(key).or_insert(default)` do?

a) Always inserts `default`  
b) Inserts `default` if the key doesn't exist; either way returns a `&mut V` to the value  
c) Returns `None` if the key is missing  
d) Panics if the key is missing  

**Q2.** Why doesn't HashMap guarantee iteration order?

a) A design oversight  
b) Hash tables store data in buckets based on hash values — insertion order is not preserved  
c) For thread safety  
d) HashMap always iterates in reverse insertion order  

**Q3.** What is the time complexity of `HashMap::get`?

a) O(n)  
b) O(log n)  
c) O(1) average (O(n) worst case with hash collisions)  
d) O(n²)  

**Q4.** When should you use `BTreeMap` instead of `HashMap`?

a) Always — BTreeMap is faster  
b) When you need O(1) lookups  
c) When you need keys to be iterated in sorted order  
d) When keys are strings  

**Q5.** What does `map.get_mut(&key)` return?

a) `V` — the value directly  
b) `Option<&mut V>` — a mutable reference to the value, or None  
c) `&V` — an immutable reference  
d) `Option<V>` — removes and returns the value  

---

## Quiz Answers

**A1.** b) Inserts `default` only if absent; returns `&mut V` to the value  
*The Entry API is designed for efficient insert-or-update. It hashes the key once, finds or creates the entry, and returns a mutable reference so you can modify the value immediately.*

**A2.** b) Hash tables store data in hash-based buckets — insertion order not preserved  
*Hash maps use a hash function to decide where to store each entry. The order depends on hash values and collision resolution, not insertion order. Use `IndexMap` (from `indexmap` crate) for insertion-order preservation.*

**A3.** c) O(1) average  
*A good hash function distributes keys uniformly across buckets. Each lookup computes the hash (O(1)) and accesses the bucket (O(1)). With many hash collisions it degrades to O(n), but Rust's `HashMap` uses a cryptographically strong hash (SipHash) to resist collision attacks.*

**A4.** c) When you need keys iterated in sorted order  
*`BTreeMap` stores keys in a B-tree — iteration is always sorted, and range queries are efficient. Use it when you need `map.range(lo..hi)` or want stable/predictable iteration order.*

**A5.** b) `Option<&mut V>` — a mutable reference to the value  
*`get_mut` lets you modify a value in-place without removing it. The `Option` handles the missing-key case. Use it when the Entry API is overkill — i.e., when you only need to modify an existing entry.*

---

## Chapter Summary

- `HashMap<K, V>` provides O(1) average lookups, insertions, and deletions by key
- Use `HashMap::new()` or `HashMap::with_capacity(n)` to create maps
- **Ownership**: non-Copy keys and values are moved into the map
- **Access**: `map.get(&key)` → `Option<&V>` (safe); `map[&key]` → `&V` (panics if missing)
- **Entry API**: `map.entry(key).or_insert(default)` — the idiomatic insert-or-update, hashes once
- **Iteration order is not guaranteed** — use `BTreeMap` if sorted order matters
- For sorted iteration, collect to Vec, sort, then iterate
- **`retain`** removes entries matching a condition in-place
- **`values_mut()`** gives mutable references to all values for in-place modification
- Use `HashMap` as a **grouping** and **counting** tool (word frequency, anagram groups, etc.)

---

**Part 4 Complete!**

You can now work with all three core Rust collections:
- `String` — owned, growable UTF-8 text
- `Vec<T>` — ordered, growable sequences
- `HashMap<K, V>` — key-value lookups

Combined with Parts 1–3, you have the full Beginner Rust toolkit. You are ready to tackle Part 5: **Error Handling** — where you'll learn to write robust, production-grade code using `panic!`, `Result<T, E>`, and custom error types.
