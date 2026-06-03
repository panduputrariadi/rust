# Chapter 5: Ownership

## Learning Objectives

By the end of this chapter, you will:

- Understand why ownership exists and what problem it solves
- Know the three rules of ownership
- Understand how scope determines when memory is freed
- Understand move semantics and why they exist
- Use `.clone()` when you need to copy heap data
- Know which types implement `Copy` and why
- Build a working Inventory System demonstrating ownership concepts

---

## Theory

### 5.1 What is Ownership?

Ownership is Rust's most distinctive feature. It is a set of rules that the **compiler** uses to manage memory. There is no runtime overhead — no garbage collector scanning memory, no reference counting happening at runtime. All checking happens at compile time.

#### The Memory Problem

To understand why ownership exists, you need to understand how memory works.

Every program uses two kinds of memory:

**Stack memory:**
```
Fast. Organized like a stack of plates.
Push (allocate) at the top, pop (free) from the top.
Size must be known at compile time.
Automatically cleaned up when a function returns.
Stores: integers, floats, booleans, chars, fixed-size arrays, tuples.
```

**Heap memory:**
```
Flexible but slower. Like a parking lot — find a spot, use it, leave.
Size can be determined at runtime.
Must be explicitly allocated AND deallocated.
Stores: Strings, Vecs, HashMaps, Box<T> — anything that can grow.
```

The heap is where the memory problems live:

```
Problem 1: Forget to free
    allocate("hello")
    ... use it ...
    // forgot to free → memory leak
    // program slowly consumes more and more RAM

Problem 2: Free too early (dangling pointer)
    let ptr = allocate("hello")
    free(ptr)
    println!("{}", *ptr)  // reading freed memory → crash / security exploit

Problem 3: Free twice (double-free)
    let ptr = allocate("hello")
    free(ptr)
    free(ptr)  // undefined behavior → crash

Problem 4: Data race
    thread 1 reads ptr
    thread 2 frees ptr simultaneously → thread 1 is now reading freed memory
```

C and C++ leave these problems entirely to the programmer. Garbage-collected languages solve these by tracking all references at runtime and freeing memory when nothing references it — but this introduces GC pauses and overhead.

**Rust's solution:** the ownership system proves at compile time that none of these problems can occur, without any runtime overhead.

---

### 5.2 Ownership Rules

There are exactly three rules:

```
Rule 1: Every value in Rust has an owner.
Rule 2: There can only be one owner at a time.
Rule 3: When the owner goes out of scope, the value is dropped (freed).
```

These three simple rules, enforced by the compiler, eliminate all the memory problems described above.

Let's walk through each one.

#### Rule 1: Every value has an owner

```rust
let x = 5;       // x owns the integer 5
let s = String::from("hello");  // s owns the String "hello"
```

When we say "x owns 5", it means x is responsible for that value. If x ceases to exist, the value is freed.

#### Rule 2: Only one owner at a time

```rust
let s1 = String::from("hello");
let s2 = s1;  // ownership MOVES from s1 to s2
              // s1 is no longer the owner

// s1 is no longer valid — the compiler will reject any use of s1
println!("{}", s1);  // ERROR: value borrowed here after move
```

This prevents double-free: if both s1 and s2 tried to free the same memory, that's a double-free bug. Rust prevents it by making only ONE of them the owner.

#### Rule 3: When the owner goes out of scope, the value is dropped

```rust
{
    let s = String::from("hello");  // s is allocated on the heap
    // s is valid here
    // ... use s ...
}   // s goes out of scope → Rust calls `drop(s)` → heap memory freed
    // AUTOMATIC, DETERMINISTIC, NO GARBAGE COLLECTOR
```

Rust calls the `drop` function automatically when a value goes out of scope. This is similar to C++'s RAII (Resource Acquisition Is Initialization) pattern.

---

### 5.3 Scope

Scope is the region of code where a variable is valid.

```rust
fn main() {
    // s is not valid here — hasn't been declared yet

    let s = String::from("hello");   // s comes into scope
                                     // heap memory allocated

    // s is valid from here...
    println!("{}", s);

    // ... until this closing brace
}   // s goes out of scope — Rust calls drop(s) — heap memory freed
```

#### Nested Scopes

```rust
fn main() {
    let outer = String::from("outer");

    {
        let inner = String::from("inner");
        println!("{} and {}", outer, inner);  // both valid here
    }  // inner dropped here

    println!("{}", outer);  // outer still valid
    // println!("{}", inner);  // ERROR: inner is out of scope
}  // outer dropped here
```

#### Why This Matters

In C, you have to remember to call `free()` at exactly the right time. In Rust, you don't. The compiler knows when every variable goes out of scope and inserts the deallocation automatically. It's as deterministic as manual memory management but requires no manual effort.

---

### 5.4 Move Semantics

When you assign a value to a new variable, or pass it to a function, the behavior depends on whether the type is **stack-only (Copy)** or **heap-allocated (Move)**.

#### Stack-only types: Copy

```rust
let x = 5;      // x is an i32, stored on the stack
let y = x;      // y gets a COPY of the value 5

println!("{}", x);  // x is still valid — it was copied, not moved
println!("{}", y);  // y is also valid
```

For stack-only types (integers, floats, booleans, chars), copying is cheap — just copy the bytes. Both `x` and `y` are independent values. Dropping one doesn't affect the other.

#### Heap types: Move

```rust
let s1 = String::from("hello");
let s2 = s1;  // s1 is MOVED into s2

println!("{}", s1);  // ERROR: s1 was moved — it's no longer valid
println!("{}", s2);  // s2 is valid
```

Why can't Rust just copy `s1`? Let's look at what a `String` actually is in memory:

```
Stack:                  Heap:
s1: ┌─────────────┐    ┌─────────────────────┐
    │ ptr ─────────┼───►│  h  e  l  l  o      │
    │ len: 5      │    └─────────────────────┘
    │ cap: 5      │
    └─────────────┘
```

A `String` on the stack contains three fields: a pointer to heap data, a length, and a capacity.

If Rust did a shallow copy (copied just the stack part):

```
Stack:                  Heap:
s1: ┌─────────────┐    ┌─────────────────────┐
    │ ptr ─────────┼───►│  h  e  l  l  o      │◄─── two pointers to same data!
    │ len: 5      │    └─────────────────────┘
    └─────────────┘                               ▲
s2: ┌─────────────┐                               │
    │ ptr ─────────┼───────────────────────────────┘
    │ len: 5      │
    └─────────────┘
```

Now both `s1` and `s2` point to the same heap memory. When one drops, it frees that memory. When the other drops, it tries to free the same memory again → **double-free bug**.

Rust solves this by making `let s2 = s1;` a **move**: s1 is invalidated, s2 becomes the sole owner:

```
Stack:                  Heap:
s1: (invalidated)
                        ┌─────────────────────┐
s2: ┌─────────────┐    │  h  e  l  l  o      │
    │ ptr ─────────┼───►│                     │
    │ len: 5      │    └─────────────────────┘
    └─────────────┘
```

Only one owner → only one free. No double-free possible.

#### Move into Functions

When you pass a heap value to a function, it's moved into that function:

```rust
fn print_and_drop(s: String) {
    println!("{}", s);
}  // s is dropped here — heap memory freed

fn main() {
    let s = String::from("hello");
    print_and_drop(s);          // s is MOVED into the function
    println!("{}", s);          // ERROR: s was moved
}
```

The function receives ownership. When the function returns, the owned value is dropped. If you want to use the value after calling a function, you have two options:
1. Return ownership back (tedious)
2. Use borrowing — which is covered in Chapter 6

#### Move from Functions

Functions can give ownership back by returning the value:

```rust
fn create_greeting(name: &str) -> String {
    format!("Hello, {}!", name)
}

fn take_and_give_back(s: String) -> String {
    s  // return ownership
}

fn main() {
    let greeting = create_greeting("Alice");  // gets ownership
    println!("{}", greeting);

    let s1 = String::from("world");
    let s2 = take_and_give_back(s1);  // s1 moved in, comes back as s2
    println!("{}", s2);
}
```

This is valid but verbose. Rust provides **borrowing** (Chapter 6) to avoid transferring ownership just to read a value.

---

### 5.5 Clone

If you need an independent copy of heap data (not a move), use `.clone()`:

```rust
let s1 = String::from("hello");
let s2 = s1.clone();  // deep copy — heap data is copied

println!("s1={}, s2={}", s1, s2);  // both valid!
```

After `.clone()`:

```
Stack:                  Heap:
s1: ┌─────────────┐    ┌─────────────────────┐
    │ ptr ─────────┼───►│  h  e  l  l  o      │
    │ len: 5      │    └─────────────────────┘
    └─────────────┘
s2: ┌─────────────┐    ┌─────────────────────┐
    │ ptr ─────────┼───►│  h  e  l  l  o      │  ← separate copy!
    │ len: 5      │    └─────────────────────┘
    └─────────────┘
```

Each has its own heap allocation. Dropping one doesn't affect the other.

**When to use clone:**
- When you genuinely need two independent copies of the data
- Not as a workaround for borrow checker errors — that's usually a design smell

**Performance note:** `.clone()` allocates new heap memory and copies all the data. For a small string this is fine, but cloning a large Vec<T> or complex struct can be expensive. Prefer borrowing (Chapter 6) when you only need to read data.

```rust
fn main() {
    let original = String::from("original text");

    let uppercase = original.clone().to_uppercase();  // clone to modify
    println!("Original: {}", original);    // original unchanged
    println!("Uppercase: {}", uppercase);

    let long_list: Vec<i32> = (1..=1000).collect();
    let copy = long_list.clone();  // expensive — allocates 1000 i32s on heap
    // prefer &long_list if you only need to read
}
```

---

### 5.6 Copy Trait

The **`Copy` trait** marks types that can be safely duplicated by copying their bytes. For these types, assignment and function calls create copies automatically (no move).

#### Types that implement Copy

```rust
// All these are Copy:
let a: i8    = 1;    let b = a;  // copied
let a: i16   = 2;    let b = a;
let a: i32   = 3;    let b = a;
let a: i64   = 4;    let b = a;
let a: i128  = 5;    let b = a;
let a: u8    = 6;    let b = a;
let a: u16   = 7;    let b = a;
let a: u32   = 8;    let b = a;
let a: u64   = 9;    let b = a;
let a: u128  = 10;   let b = a;
let a: f32   = 1.0;  let b = a;
let a: f64   = 2.0;  let b = a;
let a: bool  = true; let b = a;
let a: char  = 'x';  let b = a;
let a: ()    = ();   let b = a;

// Tuples are Copy IF all their elements are Copy:
let a: (i32, bool) = (5, true);  let b = a;  // Copy
let a: (i32, String) = (5, String::from("x")); // NOT Copy (String is not Copy)

// Arrays are Copy IF the element type is Copy:
let a: [i32; 3] = [1, 2, 3];  let b = a;  // Copy
```

#### Types that are NOT Copy

```rust
// Not Copy — these are MOVED:
let s = String::from("hello");        // heap-allocated, not Copy
let v = vec![1, 2, 3];               // heap-allocated, not Copy
let b = Box::new(5);                  // heap-allocated, not Copy
let m: HashMap<i32, i32> = HashMap::new(); // heap-allocated, not Copy
```

#### Why the distinction?

Copy types are stack-only — their entire data is on the stack, copying is trivially cheap (just copy bytes). Moving is unnecessary since there's no heap memory to worry about freeing twice.

Non-Copy types have heap data. Copying the stack part but not the heap part would create a double-free bug. So Rust **moves** them instead (one owner at a time).

#### Implementing Copy for your own types

A struct can implement `Copy` if ALL its fields are `Copy`:

```rust
#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,  // f64 is Copy
    y: f64,  // f64 is Copy
}

fn print_point(p: Point) {
    println!("({}, {})", p.x, p.y);
}

fn main() {
    let p = Point { x: 1.0, y: 2.0 };
    print_point(p);   // p is copied into the function
    print_point(p);   // still works! p was not moved
    println!("{:?}", p);  // still valid
}
```

If any field is not `Copy` (like `String`), the whole struct cannot be `Copy`.

---

## Code Example

### Practice: Ownership Exercises

```rust
fn main() {
    // Exercise 1: Observe move semantics
    let s1 = String::from("hello");
    let s2 = s1;  // s1 moved to s2
    // println!("{}", s1);  // would error
    println!("s2 = {}", s2);

    // Exercise 2: Clone for independent copies
    let original = String::from("world");
    let cloned = original.clone();
    println!("original={}, cloned={}", original, cloned);

    // Exercise 3: Copy types — no move
    let x = 42;
    let y = x;  // copied
    println!("x={}, y={}", x, y);  // both valid

    // Exercise 4: Function ownership transfer
    let s = String::from("ownership");
    let s = takes_and_returns(s);  // moved in, returned back
    println!("got back: {}", s);

    // Exercise 5: Multiple return values
    let s = String::from("hello world");
    let (s, length) = calculate_length(s);
    println!("'{}' has length {}", s, length);
}

fn takes_and_returns(s: String) -> String {
    println!("inside function: {}", s);
    s  // return ownership
}

fn calculate_length(s: String) -> (String, usize) {
    let len = s.len();
    (s, len)  // return both the String and its length
}
```

---

### Mini Project: Inventory System

```rust
#[derive(Debug, Clone)]
struct Product {
    name: String,
    price: f64,
    quantity: u32,
}

impl Product {
    fn new(name: &str, price: f64, quantity: u32) -> Product {
        Product {
            name: String::from(name),
            price,
            quantity,
        }
    }

    fn total_value(&self) -> f64 {
        self.price * self.quantity as f64
    }

    fn display(&self) {
        println!(
            "  {:<20} ${:>8.2}  qty: {:>4}  total: ${:>10.2}",
            self.name, self.price, self.quantity, self.total_value()
        );
    }
}

struct Inventory {
    products: Vec<Product>,
    name: String,
}

impl Inventory {
    fn new(name: &str) -> Inventory {
        Inventory {
            products: Vec::new(),
            name: String::from(name),
        }
    }

    fn add_product(&mut self, product: Product) {
        println!("Adding '{}' to inventory.", product.name);
        self.products.push(product);  // product is MOVED into the Vec
    }

    fn total_inventory_value(&self) -> f64 {
        self.products.iter().map(|p| p.total_value()).sum()
    }

    fn find_product(&self, name: &str) -> Option<&Product> {
        self.products.iter().find(|p| p.name == name)
    }

    fn remove_product(&mut self, name: &str) -> Option<Product> {
        if let Some(pos) = self.products.iter().position(|p| p.name == name) {
            Some(self.products.remove(pos))  // Product MOVED out of Vec
        } else {
            None
        }
    }

    fn display(&self) {
        println!("\n=== {} ===", self.name);
        println!("  {:<20} {:>9}  {:>8}  {:>14}", "Product", "Price", "Qty", "Total Value");
        println!("  {}", "-".repeat(60));
        for product in &self.products {
            product.display();
        }
        println!("  {}", "-".repeat(60));
        println!("  {:<20} {:>9}  {:>8}  ${:>10.2}", "TOTAL", "", "", self.total_inventory_value());
    }
}

fn demonstrate_clone(inventory: &Inventory) {
    println!("\n--- Clone Demonstration ---");
    if let Some(product) = inventory.find_product("Laptop") {
        let my_copy = product.clone();  // clone the product
        println!("I have a copy: {} at ${}", my_copy.name, my_copy.price);
        println!("Original still in inventory: {}", product.name);
    }
}

fn demonstrate_ownership_transfer(mut inventory: Inventory) -> Inventory {
    println!("\n--- Ownership Transfer ---");
    println!("Inventory '{}' moved into function.", inventory.name);
    inventory.name = format!("{} (processed)", inventory.name);
    inventory  // return ownership
}

fn main() {
    let mut inventory = Inventory::new("Main Warehouse");

    // Add products — each Product is MOVED into the inventory
    let laptop = Product::new("Laptop", 999.99, 15);
    let mouse = Product::new("Wireless Mouse", 29.99, 100);
    let keyboard = Product::new("Mechanical Keyboard", 149.99, 50);
    let monitor = Product::new("4K Monitor", 399.99, 25);

    inventory.add_product(laptop);
    inventory.add_product(mouse);
    inventory.add_product(keyboard);
    inventory.add_product(monitor);

    // laptop, mouse, etc. are now INVALID — moved into inventory
    // println!("{}", laptop.name);  // ERROR: value moved

    inventory.display();

    // Find a product — returns a reference (borrow), not ownership
    match inventory.find_product("Wireless Mouse") {
        Some(p) => println!("\nFound: {} at ${:.2}", p.name, p.price),
        None    => println!("\nProduct not found"),
    }

    // Clone demonstration
    demonstrate_clone(&inventory);

    // Remove a product — Product MOVED out of inventory
    match inventory.remove_product("Mechanical Keyboard") {
        Some(p) => println!("\nRemoved: {} (was worth ${:.2})", p.name, p.total_value()),
        None    => println!("\nProduct not found for removal"),
    }

    inventory.display();

    // Ownership transfer to function and back
    let inventory = demonstrate_ownership_transfer(inventory);
    println!("\nFinal inventory name: '{}'", inventory.name);
}
```

### Line-by-Line Explanation

```rust
#[derive(Debug, Clone)]
struct Product {
```
- `#[derive(Debug)]` automatically generates `{:?}` formatting for debug printing
- `#[derive(Clone)]` automatically generates a `.clone()` method
- Product does NOT derive `Copy` because it contains `String` (which is not `Copy`)

```rust
    fn add_product(&mut self, product: Product) {
        self.products.push(product);  // product is MOVED into the Vec
    }
```
- `product: Product` — takes ownership of the product
- `.push(product)` — moves the product into the Vec
- After this function returns, `product` no longer exists as a standalone variable (it lives inside the Vec)

```rust
    fn find_product(&self, name: &str) -> Option<&Product> {
        self.products.iter().find(|p| p.name == name)
    }
```
- Returns `Option<&Product>` — an optional **reference** to a product
- Does not give ownership to the caller — the product stays in the inventory
- `&self` means this function only borrows the inventory (read-only)

```rust
    fn remove_product(&mut self, name: &str) -> Option<Product> {
        if let Some(pos) = self.products.iter().position(|p| p.name == name) {
            Some(self.products.remove(pos))  // Product MOVED out of Vec
        } else {
            None
        }
    }
```
- `Vec::remove(pos)` removes the element at position `pos` and **returns it** (moves it out)
- Now the caller owns the removed product
- `&mut self` — mutably borrows the inventory to modify it

```rust
fn demonstrate_ownership_transfer(mut inventory: Inventory) -> Inventory {
    inventory.name = format!("{} (processed)", inventory.name);
    inventory  // return ownership
}
```
- Takes full ownership of the inventory (it's moved in)
- Modifies it and returns it — the caller gets ownership back
- This pattern is sometimes called "builder pattern" or "consuming transform"

---

## Common Mistakes

### Mistake 1: Using a value after it's been moved

```rust
let s = String::from("hello");
let t = s;                // s moved to t
println!("{}", s);        // ERROR: value borrowed here after move

// Fix 1: use t instead
println!("{}", t);

// Fix 2: clone if you need both
let s = String::from("hello");
let t = s.clone();
println!("{} {}", s, t);  // both valid

// Fix 3: borrow instead (Chapter 6)
let s = String::from("hello");
let t = &s;               // borrow
println!("{} {}", s, t);
```

### Mistake 2: Moving in a loop

```rust
let s = String::from("hello");

for _ in 0..3 {
    println!("{}", s);  // ERROR on second iteration: s moved in first iteration
}

// Fix: borrow instead
for _ in 0..3 {
    println!("{}", &s);  // borrow s each time
}
// Or: for _ in 0..3 { println!("{}", s); } — actually works for &str/Copy types
// For String, use the borrow &s in the println
```

Actually `println!` automatically borrows, so `println!("{}", s)` in a loop would work. The issue arises when you explicitly pass the value:

```rust
let s = String::from("hello");
let items = vec![s, s, s];  // ERROR: s moved in first element, can't use for second
```

### Mistake 3: Expecting clone to be free

```rust
let huge_vec: Vec<String> = (0..1_000_000).map(|i| i.to_string()).collect();

// This is EXPENSIVE: copies 1 million strings
let copy = huge_vec.clone();

// Better: borrow if you only need to read
process(&huge_vec);  // pass reference, no clone needed
```

### Mistake 4: Fighting the borrow checker with unnecessary clones

```rust
// WRONG approach: cloning to silence the borrow checker
fn process(data: Vec<i32>) {
    // ...
}

let v = vec![1, 2, 3];
process(v.clone());  // unnecessary clone
process(v.clone());  // unnecessary clone

// CORRECT: redesign to use references
fn process(data: &[i32]) {  // takes a reference instead
    // ...
}

let v = vec![1, 2, 3];
process(&v);  // borrow
process(&v);  // borrow again — no clone needed
```

---

## Best Practices

1. **Prefer borrowing over cloning** — only clone when you genuinely need an independent copy
2. **Design functions to take references** (`&T` or `&mut T`) unless they need to own the data
3. **Return owned values from constructors** — `fn new() -> T` not `fn new() -> &T`
4. **Use `#[derive(Clone)]`** on structs you'll need to copy — don't implement it manually unless necessary
5. **Understand move semantics before using `clone`** — many "borrow checker fights" are solved by redesigning to use references
6. **Take ownership when you need to store** — if a function stores data for longer than the call, it should own it

---

## Exercises

### Exercise 1: Move Detection

For each of the following, predict whether it will compile or error, then verify:

```rust
// A
let x = 5;
let y = x;
println!("{}", x);

// B
let s = String::from("hello");
let t = s;
println!("{}", s);

// C
let s = String::from("hello");
let t = s.clone();
println!("{} {}", s, t);

// D
let arr = [1, 2, 3];
let arr2 = arr;
println!("{:?}", arr);
```

### Exercise 2: Ownership through Functions

Write two functions:
- `consume(s: String)` — takes ownership and prints the string
- `borrow(s: &String)` — borrows and prints the string

Show that after calling `consume`, the value is gone, but after calling `borrow`, it's still available.

### Exercise 3: Clone Practice

Create a `Vec<String>` with 5 names. Clone it. Modify the clone (add a name, change the first element). Show the original is unchanged.

### Exercise 4: Copy Types

Define a struct `Color { r: u8, g: u8, b: u8 }`. Add `#[derive(Copy, Clone, Debug)]`. Show that it behaves as a Copy type (can be used after "assignment").

### Exercise 5: Return Ownership

Write a function `make_uppercase(s: String) -> String` that takes ownership, converts to uppercase, and returns the new string.

---

## Solutions

### Solution 1

```
A: Compiles — x is i32 (Copy), y gets a copy, x still valid
B: Errors — String is not Copy, s is moved to t, s no longer valid
C: Compiles — clone() makes an independent copy, both valid
D: Compiles — [i32; 3] is Copy (all elements are Copy), arr2 gets a copy
```

### Solution 2

```rust
fn consume(s: String) {
    println!("Consuming: {}", s);
}  // s dropped here

fn borrow(s: &String) {
    println!("Borrowing: {}", s);
}  // s returned to caller

fn main() {
    let s1 = String::from("owned value");
    borrow(&s1);         // s1 still valid
    println!("s1 still here: {}", s1);

    let s2 = String::from("will be consumed");
    consume(s2);         // s2 moved into consume
    // consume(s2);      // ERROR: s2 was moved
    // println!("{}", s2); // ERROR: s2 was moved
}
```

### Solution 3

```rust
fn main() {
    let original = vec![
        String::from("Alice"),
        String::from("Bob"),
        String::from("Charlie"),
        String::from("Diana"),
        String::from("Eve"),
    ];

    let mut modified = original.clone();
    modified[0] = String::from("Alex");
    modified.push(String::from("Frank"));

    println!("Original: {:?}", original);   // unchanged
    println!("Modified: {:?}", modified);
}
```

### Solution 4

```rust
#[derive(Copy, Clone, Debug)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

fn main() {
    let red = Color { r: 255, g: 0, b: 0 };
    let also_red = red;  // copied, not moved
    println!("{:?}", red);       // still valid!
    println!("{:?}", also_red);
}
```

### Solution 5

```rust
fn make_uppercase(s: String) -> String {
    s.to_uppercase()  // consumes s, returns new String
}

fn main() {
    let original = String::from("hello world");
    let upper = make_uppercase(original);
    // println!("{}", original);  // ERROR: moved into make_uppercase
    println!("{}", upper);        // HELLO WORLD
}
```

---

## Quiz

**Q1.** What happens to heap memory when its owner goes out of scope?

a) Nothing — it stays until the garbage collector runs  
b) It's freed immediately and deterministically  
c) It leaks  
d) It's moved to another owner  

**Q2.** Why does `let s2 = s1;` (where s1 is a String) invalidate s1?

a) To save memory  
b) To prevent double-free when both s1 and s2 would try to free the same heap memory  
c) Because Strings are always immutable  
d) It's a language limitation that will be removed  

**Q3.** What is the difference between a Move and a Clone?

a) Move copies the data; Clone moves it  
b) Move transfers ownership (no heap copy); Clone allocates new heap memory and copies data  
c) There is no difference  
d) Clone moves the data to a new location  

**Q4.** Which of these types implements Copy?

a) `String`  
b) `Vec<i32>`  
c) `(i32, bool)`  
d) `Box<i32>`  

**Q5.** When should you use `.clone()`?

a) Whenever the borrow checker complains  
b) Only when you genuinely need two independent copies of the data  
c) Always — it's safer than moves  
d) Never — Rust handles this automatically  

---

## Quiz Answers

**A1.** b) It's freed immediately and deterministically  
*Rust calls `drop()` at the closing brace of the owner's scope. No GC, no delay — exactly when the scope ends.*

**A2.** b) To prevent double-free when both would try to free the same heap memory  
*A String's heap allocation is owned by one variable. If both s1 and s2 tried to free it, that's a double-free bug. Rust prevents this by making only s2 the owner after the move.*

**A3.** b) Move transfers ownership (no heap copy); Clone allocates new heap memory  
*A move just updates the stack metadata (pointer/len/cap). A clone allocates new heap space and copies all the data into it — more expensive but independent.*

**A4.** c) `(i32, bool)`  
*Tuples are Copy only if ALL elements are Copy. Both i32 and bool are Copy, so `(i32, bool)` is Copy. String, Vec, and Box are heap-allocated, not Copy.*

**A5.** b) Only when you genuinely need two independent copies of the data  
*Cloning as a reflexive response to borrow checker errors is a code smell. Usually the right fix is to redesign to use references. Clone when you actually need an independent copy.*

---

## Chapter Summary

- **Ownership** is Rust's compile-time memory management system — no garbage collector, no runtime overhead
- **Stack** memory is fast, fixed-size, automatic. **Heap** memory is flexible but must be explicitly managed.
- The **three ownership rules**: every value has an owner, only one owner at a time, when the owner goes out of scope the value is dropped
- **Move semantics**: assigning or passing a heap value transfers ownership; the original is invalidated
- **Copy trait**: stack-only types (integers, floats, bool, char, fixed arrays) are copied automatically — no move
- **`.clone()`**: explicitly copies heap data; more expensive — use when you genuinely need two independent copies
- `drop()` is called automatically when an owner goes out of scope — no manual `free()` needed
- Functions **consume** values when they take ownership; use references (Chapter 6) to avoid this

In Chapter 6, we learn **borrowing** — how to let functions read or modify data without taking ownership. This unlocks the full power of Rust's memory model.
