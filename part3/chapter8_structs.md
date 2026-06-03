# Chapter 8: Structs

## Learning Objectives

By the end of this chapter, you will:

- Define and instantiate structs with named fields
- Use struct update syntax to create new instances from existing ones
- Know when to use tuple structs and unit structs
- Understand how ownership interacts with structs
- Build a Student Management System using structs

---

## Theory

### 8.1 Defining Structs

A **struct** (structure) groups related data together under one name. Unlike tuples, struct fields have names — making your code self-documenting.

#### Basic Struct Definition

```rust
struct User {
    username: String,
    email: String,
    age: u32,
    active: bool,
}
```

The `struct` keyword, a name (PascalCase by convention), and named fields with their types.

#### Instantiating a Struct

```rust
fn main() {
    let user1 = User {
        username: String::from("alice"),
        email: String::from("alice@example.com"),
        age: 30,
        active: true,
    };

    // Access fields with dot notation:
    println!("{}", user1.username);   // alice
    println!("{}", user1.email);      // alice@example.com
    println!("{}", user1.age);        // 30
}
```

All fields must be provided — Rust has no default field values unless you implement them yourself (via a constructor or `Default` trait).

#### Mutable Structs

The entire struct instance must be `mut` to modify any field. You cannot mark individual fields as `mut`.

```rust
let mut user1 = User {
    username: String::from("alice"),
    email: String::from("alice@example.com"),
    age: 30,
    active: true,
};

user1.email = String::from("newalice@example.com");
user1.age = 31;
```

#### Struct Constructors (Associated Functions)

It's idiomatic to provide a `new` function (covered fully in Chapter 9):

```rust
impl User {
    fn new(username: &str, email: &str, age: u32) -> User {
        User {
            username: String::from(username),
            email: String::from(email),
            age,           // shorthand when field name == variable name
            active: true,  // default value
        }
    }
}

fn main() {
    let user = User::new("bob", "bob@example.com", 25);
    println!("{} ({})", user.username, user.age);
}
```

#### Field Init Shorthand

When a variable name matches a field name, you can use shorthand:

```rust
fn build_user(username: String, email: String) -> User {
    User {
        username,   // shorthand for username: username
        email,      // shorthand for email: email
        age: 0,
        active: true,
    }
}
```

#### Structs and Ownership

Structs own their data. When a struct is dropped, all its owned fields are dropped too.

```rust
struct Article {
    title: String,    // owned — Article owns this String
    content: String,  // owned — Article owns this String
    views: u64,
}
```

You can also store references in structs, but that requires **lifetime annotations** (Part 7). For now, use owned types (`String` not `&str`) in structs.

```rust
// This won't compile without lifetime annotations:
struct Article<'a> {
    title: &'a str,  // reference — needs lifetime
}
// We'll cover this in Part 7.
```

#### Printing Structs with Debug

Rust doesn't know how to print a struct by default. Add `#[derive(Debug)]` to enable `{:?}` formatting:

```rust
#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

fn main() {
    let p = Point { x: 3.0, y: 4.0 };
    println!("{:?}", p);   // Point { x: 3.0, y: 4.0 }
    println!("{:#?}", p);  // pretty-printed, multi-line
}
```

---

### 8.2 Struct Update Syntax

Create a new instance based on an existing one, changing only some fields:

```rust
let user2 = User {
    email: String::from("user2@example.com"),
    ..user1   // use remaining fields from user1
};
```

The `..user1` must come last. It fills in all fields not explicitly set.

**Important — Ownership with Update Syntax:**

```rust
let user1 = User {
    username: String::from("alice"),
    email: String::from("alice@example.com"),
    age: 30,
    active: true,
};

let user2 = User {
    email: String::from("user2@example.com"),
    ..user1   // moves user1.username into user2
};

// println!("{}", user1.username);  // ERROR: user1.username was moved
println!("{}", user1.age);          // OK: u32 is Copy, not moved
println!("{}", user1.active);       // OK: bool is Copy, not moved
```

If all updated fields were `Copy` types, the original struct remains fully usable. If any `String` (or other non-Copy) field is moved, the original struct is partially moved and cannot be used as a whole.

---

### 8.3 Tuple Structs

Tuple structs have types but no field names. They're useful when:
- You want the type-safety of a named struct
- But the meaning is clear from position
- Or you want distinct types from raw tuples

```rust
struct Color(u8, u8, u8);       // RGB
struct Point3D(f64, f64, f64);  // x, y, z

fn main() {
    let red = Color(255, 0, 0);
    let origin = Point3D(0.0, 0.0, 0.0);

    // Access by index:
    println!("Red channel: {}", red.0);
    println!("X: {}", origin.0);

    // Destructuring:
    let Color(r, g, b) = red;
    println!("r={} g={} b={}", r, g, b);
}
```

**Why not just use plain tuples?**

```rust
// Without tuple structs:
fn mix(c1: (u8, u8, u8), c2: (u8, u8, u8)) -> (u8, u8, u8) { ... }
// You could accidentally pass a Point where a Color is expected

// With tuple structs — type safety:
fn mix(c1: Color, c2: Color) -> Color { ... }
// Point3D cannot be passed where Color is expected — compiler catches it
```

---

### 8.4 Unit Structs

Unit structs have no fields. They're used when you need a type but don't need to store data — typically as marker types or to implement traits on.

```rust
struct AlwaysTrue;
struct Meters;
struct Kilograms;

// They're useful as type-level markers:
struct JsonFormatter;
struct HtmlFormatter;

trait Format {
    fn format(&self, text: &str) -> String;
}

impl Format for JsonFormatter {
    fn format(&self, text: &str) -> String {
        format!("{{\"text\": \"{}\"}}", text)
    }
}

impl Format for HtmlFormatter {
    fn format(&self, text: &str) -> String {
        format!("<p>{}</p>", text)
    }
}

fn main() {
    let formatters: Vec<Box<dyn Format>> = vec![
        Box::new(JsonFormatter),
        Box::new(HtmlFormatter),
    ];

    for f in &formatters {
        println!("{}", f.format("hello world"));
    }
}
```

---

## Code Example

### Mini Project: Student Management System

#### Project Overview

A CLI-based student management system demonstrating struct definition, instantiation, update syntax, and operations on collections of structs.

#### Functional Requirements

1. Add a student with name, ID, and subject scores
2. Update a student's data
3. List all students with their averages and grades
4. Search for a student by name or ID
5. Remove a student
6. Show class statistics (highest/lowest average, class average)

#### Project Structure

```
student_manager/
└── src/
    └── main.rs   ← all code in one file for this project
```

#### Complete Source Code

```rust
use std::io;
use std::io::Write;

#[derive(Debug, Clone)]
struct Student {
    id: u32,
    name: String,
    scores: Vec<f64>,
}

impl Student {
    fn new(id: u32, name: &str, scores: Vec<f64>) -> Student {
        Student {
            id,
            name: String::from(name),
            scores,
        }
    }

    fn average(&self) -> f64 {
        if self.scores.is_empty() {
            return 0.0;
        }
        self.scores.iter().sum::<f64>() / self.scores.len() as f64
    }

    fn grade(&self) -> &'static str {
        match self.average() as u32 {
            90..=100 => "A",
            80..=89  => "B",
            70..=79  => "C",
            60..=69  => "D",
            _        => "F",
        }
    }

    fn status(&self) -> &'static str {
        if self.average() >= 60.0 { "Pass" } else { "Fail" }
    }

    fn display(&self) {
        println!(
            "  [{:04}] {:<20} avg: {:>6.2}  grade: {}  {}",
            self.id, self.name, self.average(), self.grade(), self.status()
        );
    }
}

struct StudentManager {
    students: Vec<Student>,
    next_id: u32,
}

impl StudentManager {
    fn new() -> StudentManager {
        StudentManager {
            students: Vec::new(),
            next_id: 1,
        }
    }

    fn add(&mut self, name: &str, scores: Vec<f64>) -> u32 {
        let id = self.next_id;
        self.students.push(Student::new(id, name, scores));
        self.next_id += 1;
        id
    }

    fn find_by_id(&self, id: u32) -> Option<&Student> {
        self.students.iter().find(|s| s.id == id)
    }

    fn find_by_name(&self, name: &str) -> Vec<&Student> {
        let lower = name.to_ascii_lowercase();
        self.students.iter()
            .filter(|s| s.name.to_ascii_lowercase().contains(&lower))
            .collect()
    }

    fn update_scores(&mut self, id: u32, scores: Vec<f64>) -> bool {
        if let Some(student) = self.students.iter_mut().find(|s| s.id == id) {
            student.scores = scores;
            true
        } else {
            false
        }
    }

    fn remove(&mut self, id: u32) -> Option<Student> {
        if let Some(pos) = self.students.iter().position(|s| s.id == id) {
            Some(self.students.remove(pos))
        } else {
            None
        }
    }

    fn list_all(&self) {
        if self.students.is_empty() {
            println!("  No students registered.");
            return;
        }
        println!("  {:<6} {:<20} {:>8}  {:>7}  {}", "ID", "Name", "Average", "Grade", "Status");
        println!("  {}", "-".repeat(55));
        for s in &self.students {
            s.display();
        }
    }

    fn class_stats(&self) {
        if self.students.is_empty() {
            println!("  No data.");
            return;
        }

        let avgs: Vec<f64> = self.students.iter().map(|s| s.average()).collect();
        let class_avg = avgs.iter().sum::<f64>() / avgs.len() as f64;
        let best = self.students.iter().max_by(|a, b| a.average().partial_cmp(&b.average()).unwrap()).unwrap();
        let worst = self.students.iter().min_by(|a, b| a.average().partial_cmp(&b.average()).unwrap()).unwrap();
        let passing = self.students.iter().filter(|s| s.average() >= 60.0).count();

        println!("  Total students:  {}", self.students.len());
        println!("  Class average:   {:.2}", class_avg);
        println!("  Highest avg:     {} — {:.2}", best.name, best.average());
        println!("  Lowest avg:      {} — {:.2}", worst.name, worst.average());
        println!("  Passing:         {} / {}", passing, self.students.len());
    }
}

fn prompt(msg: &str) -> String {
    print!("{}", msg);
    io::stdout().flush().unwrap();
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s.trim().to_string()
}

fn read_scores() -> Vec<f64> {
    loop {
        let input = prompt("  Scores (comma-separated, e.g. 85,90,78): ");
        let scores: Vec<f64> = input.split(',')
            .filter_map(|s| s.trim().parse().ok())
            .collect();
        if !scores.is_empty() {
            return scores;
        }
        println!("  Invalid scores. Try again.");
    }
}

fn main() {
    let mut manager = StudentManager::new();

    // Seed some data
    manager.add("Alice Johnson", vec![92.0, 88.0, 95.0, 90.0]);
    manager.add("Bob Smith", vec![70.0, 75.0, 68.0, 72.0]);
    manager.add("Carol White", vec![55.0, 60.0, 58.0, 62.0]);
    manager.add("David Brown", vec![98.0, 95.0, 99.0, 97.0]);

    println!("=== Student Management System ===\n");

    loop {
        println!("\n--- Menu ---");
        println!("1. List all students");
        println!("2. Add student");
        println!("3. Search student");
        println!("4. Update scores");
        println!("5. Remove student");
        println!("6. Class statistics");
        println!("7. Exit");

        match prompt("Choice: ").as_str() {
            "1" => {
                println!("\n--- Student List ---");
                manager.list_all();
            }
            "2" => {
                let name = prompt("  Name: ");
                let scores = read_scores();
                let id = manager.add(&name, scores);
                println!("  Added with ID {}", id);
            }
            "3" => {
                let query = prompt("  Search (name or ID): ");
                if let Ok(id) = query.parse::<u32>() {
                    match manager.find_by_id(id) {
                        Some(s) => s.display(),
                        None    => println!("  Not found."),
                    }
                } else {
                    let results = manager.find_by_name(&query);
                    if results.is_empty() {
                        println!("  No matches found.");
                    } else {
                        for s in results { s.display(); }
                    }
                }
            }
            "4" => {
                let id: u32 = match prompt("  Student ID: ").parse() {
                    Ok(n) => n,
                    Err(_) => { println!("  Invalid ID."); continue; }
                };
                let scores = read_scores();
                if manager.update_scores(id, scores) {
                    println!("  Updated.");
                } else {
                    println!("  Student not found.");
                }
            }
            "5" => {
                let id: u32 = match prompt("  Student ID to remove: ").parse() {
                    Ok(n) => n,
                    Err(_) => { println!("  Invalid ID."); continue; }
                };
                match manager.remove(id) {
                    Some(s) => println!("  Removed: {}", s.name),
                    None    => println!("  Student not found."),
                }
            }
            "6" => {
                println!("\n--- Class Statistics ---");
                manager.class_stats();
            }
            "7" | "q" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("  Unknown option."),
        }
    }
}
```

#### Code Explanation

```rust
#[derive(Debug, Clone)]
struct Student {
```
- `Debug` → enables `{:?}` printing
- `Clone` → enables `.clone()` for making copies of a Student

```rust
fn average(&self) -> f64 {
    self.scores.iter().sum::<f64>() / self.scores.len() as f64
}
```
- `&self` → borrows the struct, read-only, no ownership transfer
- `.sum::<f64>()` → turbofish specifies the accumulator type
- `as f64` → cast `usize` (len) to `f64` for division

```rust
fn find_by_name(&self, name: &str) -> Vec<&Student> {
    let lower = name.to_ascii_lowercase();
    self.students.iter()
        .filter(|s| s.name.to_ascii_lowercase().contains(&lower))
        .collect()
}
```
- Returns `Vec<&Student>` — references into the manager's students, no ownership transfer
- Case-insensitive search using `.to_ascii_lowercase()`
- `.contains()` checks for substring match

```rust
fn update_scores(&mut self, id: u32, scores: Vec<f64>) -> bool {
    if let Some(student) = self.students.iter_mut().find(|s| s.id == id) {
        student.scores = scores;
        true
    } else {
        false
    }
}
```
- `iter_mut()` → produces `&mut Student` references so we can modify
- `if let Some(student) = ...` → binds the mutable reference if found
- Returns `bool` to signal success/failure

#### Refactoring Suggestions

1. **Separate modules**: move `Student` and `StudentManager` into their own modules (`student.rs`, `manager.rs`)
2. **Error types**: replace `bool` return with `Result<(), StudentError>` for richer error info
3. **Persistence**: serialize to JSON with `serde_json` and save/load from a file
4. **Sorting**: add a `sort_by` method (by name, average, ID)
5. **Validation**: validate score range (0.0–100.0), non-empty name

#### Challenge Exercises

1. Add a `subject_name: Vec<String>` field to track which score belongs to which subject
2. Implement a `rank()` method that returns the student's rank among all students in the manager
3. Add grade distribution statistics (how many A, B, C, D, F)
4. Add an `import_csv(filename: &str)` function that reads students from a CSV file

#### Real World Extensions

- Add database persistence with SQLite via the `rusqlite` crate
- Build a REST API over this with `axum`
- Add authentication so each teacher can only see their own students
- Export reports as PDF or HTML

---

## Common Mistakes

### Mistake 1: Trying to use a struct after partial move

```rust
let s = Student::new(1, "Alice", vec![90.0, 85.0]);
let name = s.name;  // String is moved out of s

println!("{}", s.id);    // OK: u32 is Copy
println!("{}", s.name);  // ERROR: name was moved

// Fix: borrow instead of move
let name = &s.name;      // borrow
println!("{}", s.name);  // OK — still owned by s
```

### Mistake 2: Forgetting `mut` on the struct instance

```rust
let student = Student::new(1, "Alice", vec![90.0]);
student.scores.push(85.0);  // ERROR: cannot borrow as mutable

let mut student = Student::new(1, "Alice", vec![90.0]);
student.scores.push(85.0);  // OK
```

### Mistake 3: Using `&str` fields without lifetimes

```rust
struct User {
    name: &str,  // ERROR: missing lifetime specifier
}

// Fix for now: use owned String
struct User {
    name: String,
}
// Lifetime annotations (Part 7) enable &str fields
```

### Mistake 4: Struct update syntax moves data

```rust
let u1 = User { name: String::from("Alice"), age: 30, active: true };
let u2 = User { age: 25, ..u1 };
println!("{}", u1.name);  // ERROR: u1.name was moved into u2
```

---

## Best Practices

1. **Use `String` not `&str` in struct fields** until you learn lifetimes
2. **Always derive `Debug`** on structs — invaluable for debugging
3. **Provide a `new()` constructor** for structs with more than 2-3 fields
4. **Keep structs focused** — one struct, one responsibility
5. **Derive `Clone`** when you'll need to copy the struct
6. **Use tuple structs for newtypes** — wrapping a primitive in a named type adds type safety

---

## Exercises

### Exercise 1: Rectangle Struct

Define a `Rectangle` struct with `width: f64` and `height: f64`. Write functions (not methods yet) to calculate area and perimeter. Instantiate two rectangles and print their dimensions and measurements.

### Exercise 2: Struct Update

Create a `Config` struct with fields: `host: String`, `port: u16`, `debug: bool`, `max_connections: u32`. Create a `default_config()` function returning a default. Use struct update syntax to create a production config that changes only `debug` and `max_connections`.

### Exercise 3: Tuple Struct Newtype

Create tuple structs `Celsius(f64)` and `Fahrenheit(f64)`. Write a `to_fahrenheit(c: Celsius) -> Fahrenheit` conversion function. Show that passing `Celsius` where `Fahrenheit` is expected is a compile error.

### Exercise 4: Vec of Structs

Create a `Book` struct with `title: String`, `author: String`, `pages: u32`, `rating: f64`. Build a `Vec<Book>` with 5 books. Find and print the highest-rated book and the longest book.

### Exercise 5: Nested Structs

Create an `Address` struct (`street`, `city`, `country` as `String`s). Create a `Person` struct that has a `name: String`, `age: u32`, and `address: Address`. Instantiate a person and access nested fields.

---

## Solutions

### Solution 1

```rust
#[derive(Debug)]
struct Rectangle {
    width: f64,
    height: f64,
}

fn area(r: &Rectangle) -> f64 {
    r.width * r.height
}

fn perimeter(r: &Rectangle) -> f64 {
    2.0 * (r.width + r.height)
}

fn main() {
    let r1 = Rectangle { width: 5.0, height: 3.0 };
    let r2 = Rectangle { width: 10.0, height: 7.5 };

    for r in [&r1, &r2] {
        println!("{:?} → area={}, perimeter={}", r, area(r), perimeter(r));
    }
}
```

### Solution 2

```rust
#[derive(Debug)]
struct Config {
    host: String,
    port: u16,
    debug: bool,
    max_connections: u32,
}

fn default_config() -> Config {
    Config {
        host: String::from("localhost"),
        port: 8080,
        debug: true,
        max_connections: 10,
    }
}

fn main() {
    let dev = default_config();
    let prod = Config {
        debug: false,
        max_connections: 100,
        ..default_config()
    };
    println!("Dev:  {:?}", dev);
    println!("Prod: {:?}", prod);
}
```

### Solution 3

```rust
struct Celsius(f64);
struct Fahrenheit(f64);

fn to_fahrenheit(c: Celsius) -> Fahrenheit {
    Fahrenheit(c.0 * 9.0 / 5.0 + 32.0)
}

fn main() {
    let boiling = Celsius(100.0);
    let f = to_fahrenheit(boiling);
    println!("{}°F", f.0);

    // to_fahrenheit(Fahrenheit(100.0)); // ERROR: type mismatch
}
```

### Solution 4

```rust
#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    pages: u32,
    rating: f64,
}

fn main() {
    let books = vec![
        Book { title: String::from("The Rust Book"), author: String::from("Steve Klabnik"), pages: 526, rating: 4.9 },
        Book { title: String::from("Programming Rust"), author: String::from("Jim Blandy"), pages: 622, rating: 4.8 },
        Book { title: String::from("Rust in Action"), author: String::from("Tim McNamara"), pages: 456, rating: 4.6 },
        Book { title: String::from("Zero to Production"), author: String::from("Luca Palmieri"), pages: 485, rating: 4.9 },
        Book { title: String::from("Rust for Rustaceans"), author: String::from("Jon Gjengset"), pages: 280, rating: 4.7 },
    ];

    let best = books.iter().max_by(|a, b| a.rating.partial_cmp(&b.rating).unwrap()).unwrap();
    let longest = books.iter().max_by_key(|b| b.pages).unwrap();

    println!("Best rated: {} ({:.1}★)", best.title, best.rating);
    println!("Longest:    {} ({} pages)", longest.title, longest.pages);
}
```

### Solution 5

```rust
#[derive(Debug)]
struct Address {
    street: String,
    city: String,
    country: String,
}

#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
    address: Address,
}

fn main() {
    let p = Person {
        name: String::from("Alice"),
        age: 30,
        address: Address {
            street: String::from("123 Main St"),
            city: String::from("Jakarta"),
            country: String::from("Indonesia"),
        },
    };

    println!("Name: {}", p.name);
    println!("City: {}", p.address.city);
    println!("{:#?}", p);
}
```

---

## Quiz

**Q1.** What is the difference between a struct and a tuple?

a) Structs are faster  
b) Structs have named fields; tuples access fields by index  
c) Tuples can hold more data  
d) There is no difference  

**Q2.** When using struct update syntax (`..other`), what determines whether the original struct is still usable?

a) The number of fields updated  
b) Whether any non-Copy fields (like String) were moved from the original  
c) The size of the struct  
d) Whether the struct derives Clone  

**Q3.** What is a tuple struct used for?

a) Storing unnamed data only  
b) Adding a named type to data without field names, often for type safety (newtypes)  
c) Replacing regular structs  
d) Storing more than one type  

**Q4.** Why can't you mark individual struct fields as `mut`?

a) It's a compiler limitation  
b) Mutability in Rust is a property of the binding (variable), not the data itself  
c) Fields are always mutable  
d) You need to use `RefCell` for mutable fields  

**Q5.** What does `#[derive(Debug)]` do to a struct?

a) Makes the struct printable with `{}` format  
b) Automatically generates a `fmt::Debug` implementation enabling `{:?}` formatting  
c) Adds debug breakpoints  
d) Makes the struct Copy  

---

## Quiz Answers

**A1.** b) Structs have named fields; tuples access fields by index  
*`point.x` vs `point.0` — named fields are self-documenting, easier to maintain as the struct grows.*

**A2.** b) Whether any non-Copy fields (like String) were moved from the original  
*Copy types (u32, bool, f64, etc.) are bitwise-copied. Non-Copy types (String, Vec) are moved. If a non-Copy field is moved into the new struct via `..original`, `original` can no longer be used as a whole.*

**A3.** b) Adding a named type to data without field names, often for type safety (newtypes)  
*A `struct Meters(f64)` and `struct Seconds(f64)` are distinct types even though both wrap f64 — preventing you from accidentally adding meters to seconds.*

**A4.** b) Mutability in Rust is a property of the binding, not the data  
*`let mut s = Struct {...}` makes the entire binding mutable. This is consistent with Rust's ownership model — you either own something mutably or you don't.*

**A5.** b) Automatically generates a `fmt::Debug` implementation enabling `{:?}` formatting  
*`derive(Debug)` is a macro that writes the Debug trait implementation for you, showing field names and values. `{}` uses the Display trait, which you must implement yourself.*

---

## Chapter Summary

- **Structs** group related named fields into a single type — use PascalCase for names
- **Field init shorthand**: `username` instead of `username: username` when the variable matches the field name
- **Struct update syntax** (`..other`) fills unspecified fields from an existing instance; moves non-Copy fields
- **Tuple structs** have types but no field names — useful for newtypes and distinct type safety
- **Unit structs** have no fields — used as marker types or to implement traits
- Struct fields should use **owned types** (`String`, not `&str`) until you learn lifetimes (Part 7)
- `#[derive(Debug)]` enables `{:?}` printing — always add it during development
- Mutability applies to the **entire binding**, not individual fields

In Chapter 9, we add behavior to structs through **methods** — functions defined in `impl` blocks.
