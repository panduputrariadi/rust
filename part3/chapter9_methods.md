# Chapter 9: Methods

## Learning Objectives

By the end of this chapter, you will:

- Define methods on structs using `impl` blocks
- Understand `&self`, `&mut self`, and `self` method receivers
- Write associated functions (like constructors) with no `self` parameter
- Chain methods together
- Build a Rectangle Calculator with a rich method interface

---

## Theory

### 9.1 impl Blocks

Methods are functions defined **inside an `impl` block** for a struct. They differ from regular functions in that their first parameter is always `self` in some form — representing the instance the method is called on.

```rust
#[derive(Debug)]
struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }
}

fn main() {
    let rect = Rectangle { width: 5.0, height: 3.0 };
    println!("Area: {}", rect.area());
    println!("Perimeter: {}", rect.perimeter());
}
```

Call syntax: `instance.method(args)` — Rust automatically passes the instance as the first argument.

#### Multiple impl Blocks

A type can have multiple `impl` blocks. This is the same as having one combined block:

```rust
impl Rectangle {
    fn area(&self) -> f64 { self.width * self.height }
}

impl Rectangle {
    fn perimeter(&self) -> f64 { 2.0 * (self.width + self.height) }
}
```

This is useful when implementing traits — each trait gets its own `impl` block.

---

### 9.2 Associated Functions

Associated functions are defined in `impl` but do NOT take `self` — they belong to the type, not an instance. They're called with `::` syntax.

The most common use is **constructors**:

```rust
impl Rectangle {
    fn new(width: f64, height: f64) -> Rectangle {
        Rectangle { width, height }
    }

    fn square(size: f64) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn main() {
    let r = Rectangle::new(10.0, 5.0);
    let s = Rectangle::square(4.0);
    println!("{:?}", r);
    println!("{:?}", s);
}
```

`String::from(...)`, `Vec::new()`, `HashMap::new()` — these are all associated functions.

---

### 9.3 Self Keyword

The `self` parameter determines how the method accesses the instance:

| Receiver | Meaning | When to use |
|----------|---------|-------------|
| `&self` | Immutable borrow of the instance | Read data, no modification |
| `&mut self` | Mutable borrow of the instance | Modify the instance |
| `self` | Takes ownership of the instance | Consume the instance (rare) |

```rust
#[derive(Debug)]
struct Counter {
    value: u32,
    max: u32,
}

impl Counter {
    fn new(max: u32) -> Counter {
        Counter { value: 0, max }
    }

    fn current(&self) -> u32 {      // &self — read only
        self.value
    }

    fn increment(&mut self) {       // &mut self — modify
        if self.value < self.max {
            self.value += 1;
        }
    }

    fn reset(&mut self) {           // &mut self — modify
        self.value = 0;
    }

    fn is_done(&self) -> bool {
        self.value >= self.max
    }

    fn into_value(self) -> u32 {    // self — consume instance
        self.value                  // after this, counter is dropped
    }
}

fn main() {
    let mut counter = Counter::new(3);

    while !counter.is_done() {
        println!("count: {}", counter.current());
        counter.increment();
    }
    println!("done! final: {}", counter.current());

    let val = counter.into_value(); // counter is consumed here
    println!("extracted: {}", val);
    // counter is no longer available
}
```

#### Method Chaining

Methods that return `Self` (or `&mut Self`) enable chaining:

```rust
#[derive(Debug)]
struct QueryBuilder {
    table: String,
    conditions: Vec<String>,
    limit: Option<u32>,
    order_by: Option<String>,
}

impl QueryBuilder {
    fn new(table: &str) -> QueryBuilder {
        QueryBuilder {
            table: String::from(table),
            conditions: Vec::new(),
            limit: None,
            order_by: None,
        }
    }

    fn where_clause(mut self, condition: &str) -> QueryBuilder {
        self.conditions.push(String::from(condition));
        self  // return self to allow chaining
    }

    fn limit(mut self, n: u32) -> QueryBuilder {
        self.limit = Some(n);
        self
    }

    fn order_by(mut self, field: &str) -> QueryBuilder {
        self.order_by = Some(String::from(field));
        self
    }

    fn build(&self) -> String {
        let mut query = format!("SELECT * FROM {}", self.table);
        if !self.conditions.is_empty() {
            query.push_str(" WHERE ");
            query.push_str(&self.conditions.join(" AND "));
        }
        if let Some(field) = &self.order_by {
            query.push_str(&format!(" ORDER BY {}", field));
        }
        if let Some(n) = self.limit {
            query.push_str(&format!(" LIMIT {}", n));
        }
        query
    }
}

fn main() {
    let query = QueryBuilder::new("users")
        .where_clause("age > 18")
        .where_clause("active = true")
        .order_by("name")
        .limit(10)
        .build();

    println!("{}", query);
    // SELECT * FROM users WHERE age > 18 AND active = true ORDER BY name LIMIT 10
}
```

#### `Self` as a Type Alias

Inside an `impl` block, `Self` (capital S) refers to the type being implemented:

```rust
impl Rectangle {
    fn new(width: f64, height: f64) -> Self {  // Self = Rectangle
        Self { width, height }                  // Self = Rectangle
    }

    fn clone_scaled(&self, factor: f64) -> Self {
        Self {
            width: self.width * factor,
            height: self.height * factor,
        }
    }
}
```

Using `Self` instead of the type name means you only update one place if the type is renamed.

#### Automatic Referencing

Rust automatically adds `&`, `&mut`, or `*` when calling methods, matching the method's receiver:

```rust
let rect = Rectangle::new(5.0, 3.0);

// These are identical — Rust inserts & automatically:
rect.area();          // Rust sees: &self → calls (&rect).area()
(&rect).area();       // explicit — same thing
```

This is why you don't need to write `(&mut counter).increment()` — Rust handles it.

---

## Code Example

### Practice: Rectangle Calculator

```rust
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    fn new(width: f64, height: f64) -> Self {
        assert!(width > 0.0 && height > 0.0, "Dimensions must be positive");
        Self { width, height }
    }

    fn square(size: f64) -> Self {
        Self::new(size, size)
    }

    fn from_area_and_ratio(area: f64, ratio: f64) -> Self {
        // ratio = width / height, area = width * height
        // height = sqrt(area / ratio), width = ratio * height
        let height = (area / ratio).sqrt();
        let width = ratio * height;
        Self::new(width, height)
    }

    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }

    fn diagonal(&self) -> f64 {
        (self.width * self.width + self.height * self.height).sqrt()
    }

    fn is_square(&self) -> bool {
        (self.width - self.height).abs() < f64::EPSILON
    }

    fn aspect_ratio(&self) -> f64 {
        self.width / self.height
    }

    fn can_contain(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn scale(&mut self, factor: f64) {
        self.width *= factor;
        self.height *= factor;
    }

    fn scaled(&self, factor: f64) -> Self {
        Self::new(self.width * factor, self.height * factor)
    }

    fn rotate(&self) -> Self {
        Self::new(self.height, self.width)
    }
}

impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Rectangle({:.2} x {:.2})", self.width, self.height)
    }
}

fn main() {
    let r1 = Rectangle::new(10.0, 5.0);
    let r2 = Rectangle::square(4.0);
    let r3 = Rectangle::from_area_and_ratio(100.0, 2.0);

    println!("{}", r1);
    println!("  area      = {:.2}", r1.area());
    println!("  perimeter = {:.2}", r1.perimeter());
    println!("  diagonal  = {:.4}", r1.diagonal());
    println!("  is_square = {}", r1.is_square());
    println!("  ratio     = {:.2}", r1.aspect_ratio());
    println!("  contains r2? {}", r1.can_contain(&r2));

    println!("\n{} (is square: {})", r2, r2.is_square());

    println!("\nfrom area=100, ratio=2: {}", r3);
    println!("  area check: {:.4}", r3.area());

    let mut r4 = r1.clone();
    r4.scale(2.0);
    println!("\nOriginal: {} → Scaled 2x: {}", r1, r4);

    let rotated = r1.rotate();
    println!("Rotated: {}", rotated);
}
```

### Line-by-Line Explanation

```rust
fn from_area_and_ratio(area: f64, ratio: f64) -> Self {
    let height = (area / ratio).sqrt();
    let width = ratio * height;
    Self::new(width, height)
}
```
- An alternative constructor — multiple constructors are idiomatic in Rust
- Mathematical derivation: if `w/h = ratio` and `w*h = area`, then `h = sqrt(area/ratio)`
- Delegates to `Self::new()` to reuse the validation assert

```rust
fn can_contain(&self, other: &Rectangle) -> bool {
    self.width > other.width && self.height > other.height
}
```
- Takes two references — neither instance is moved or modified
- Returns a simple `bool` derived from reading both instances

```rust
fn scale(&mut self, factor: f64) {       // mutates in place
fn scaled(&self, factor: f64) -> Self {  // returns a new scaled copy
```
- Two variants of scaling — a common Rust pattern
- In-place mutation (`scale`) vs immutable transformation returning a new value (`scaled`)
- This mirrors the standard library: `sort` vs `sort_unstable`, `drain` vs `iter`

```rust
impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Rectangle({:.2} x {:.2})", self.width, self.height)
    }
}
```
- Implementing the `Display` trait enables `{}` formatting (not just `{:?}`)
- `fmt::Formatter` controls the output stream
- `write!` writes formatted text into the formatter

---

## Common Mistakes

### Mistake 1: Calling a method that needs `&mut self` on an immutable binding

```rust
let counter = Counter::new(10);
counter.increment();  // ERROR: cannot borrow as mutable

let mut counter = Counter::new(10);
counter.increment();  // OK
```

### Mistake 2: Confusing associated function call vs method call

```rust
// Associated function — called with ::
let r = Rectangle::new(5.0, 3.0);

// Method — called with . on an instance
let area = r.area();

// Wrong:
let r = Rectangle.new(5.0, 3.0);  // ERROR: expected value, found struct
let area = Rectangle::area();      // ERROR: no `self` argument
```

### Mistake 3: Returning `self` by value when `&mut self` would work

```rust
// Less efficient — moves the whole struct to enable chaining
fn set_width(mut self, w: f64) -> Self {
    self.width = w;
    self
}

// More efficient for the simple setter case
fn set_width(&mut self, w: f64) {
    self.width = w;
}
// (chaining works differently — use the builder pattern for that)
```

### Mistake 4: Forgetting `&` in `can_contain(&self, other: &Rectangle)`

```rust
// Wrong — this takes ownership of `other`
fn can_contain(&self, other: Rectangle) -> bool { ... }

let big = Rectangle::new(10.0, 10.0);
let small = Rectangle::new(3.0, 3.0);
big.can_contain(small);   // small is moved! can't use it again

// Correct — borrows other
fn can_contain(&self, other: &Rectangle) -> bool { ... }
big.can_contain(&small);  // small is borrowed, still usable
```

---

## Best Practices

1. **Use `&self` by default** — only use `&mut self` when mutation is needed, `self` when consuming
2. **Provide `new()` constructors** with descriptive names (`new`, `from_*`, `with_*`)
3. **Implement `Display`** for types users will print often
4. **Implement `Debug`** (via `derive`) for all types
5. **Keep methods focused** — if a method does too much, split it
6. **Use `Self`** instead of the type name inside `impl` blocks for maintainability
7. **Group related methods** — constructors first, then getters, then mutating methods

---

## Exercises

### Exercise 1: Circle

Define a `Circle` struct with `radius: f64`. Add methods: `new(radius)`, `area()`, `circumference()`, `diameter()`, `is_unit()` (radius == 1), `scale(factor: f64)` (mutates), `scaled(factor: f64) -> Circle` (returns new). Implement `Display`.

### Exercise 2: Stack

Define a `Stack<i32>` struct backed by a `Vec<i32>`. Add methods: `new()`, `push(val)`, `pop() -> Option<i32>`, `peek() -> Option<&i32>`, `is_empty() -> bool`, `size() -> usize`. Demonstrate push, pop, and peek.

### Exercise 3: BankAccount

Define a `BankAccount` with `owner: String`, `balance: f64`. Add methods: `new(owner, initial_balance)`, `deposit(amount) -> Result<f64, String>` (error if amount <= 0), `withdraw(amount) -> Result<f64, String>` (error if insufficient funds), `balance() -> f64`. Demonstrate a series of transactions.

### Exercise 4: Method Chaining Builder

Define a `Pizza` struct with `size: &'static str`, `crust: &'static str`, `toppings: Vec<String>`. Add a builder pattern using consuming methods (`fn size(mut self, s: &'static str) -> Self`, etc.) and a `build() -> String` that describes the pizza.

### Exercise 5: impl for Tuple Struct

Define `Meters(f64)` and implement: `new(val) -> Meters`, `to_km() -> f64`, `to_cm() -> f64`, `add(&self, other: &Meters) -> Meters`. Implement `Display` showing "X m".

---

## Solutions

### Solution 1

```rust
use std::fmt;

#[derive(Debug, Clone)]
struct Circle {
    radius: f64,
}

impl Circle {
    fn new(radius: f64) -> Self {
        assert!(radius > 0.0, "Radius must be positive");
        Self { radius }
    }

    fn area(&self) -> f64 { std::f64::consts::PI * self.radius * self.radius }
    fn circumference(&self) -> f64 { 2.0 * std::f64::consts::PI * self.radius }
    fn diameter(&self) -> f64 { 2.0 * self.radius }
    fn is_unit(&self) -> bool { (self.radius - 1.0).abs() < f64::EPSILON }

    fn scale(&mut self, factor: f64) { self.radius *= factor; }
    fn scaled(&self, factor: f64) -> Self { Self::new(self.radius * factor) }
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Circle(r={:.2})", self.radius)
    }
}

fn main() {
    let mut c = Circle::new(5.0);
    println!("{}: area={:.4}, circ={:.4}", c, c.area(), c.circumference());
    c.scale(2.0);
    println!("After scale: {}", c);
    println!("Unit circle? {}", Circle::new(1.0).is_unit());
}
```

### Solution 2

```rust
struct Stack {
    data: Vec<i32>,
}

impl Stack {
    fn new() -> Self { Self { data: Vec::new() } }
    fn push(&mut self, val: i32) { self.data.push(val); }
    fn pop(&mut self) -> Option<i32> { self.data.pop() }
    fn peek(&self) -> Option<&i32> { self.data.last() }
    fn is_empty(&self) -> bool { self.data.is_empty() }
    fn size(&self) -> usize { self.data.len() }
}

fn main() {
    let mut s = Stack::new();
    s.push(1); s.push(2); s.push(3);
    println!("peek: {:?}, size: {}", s.peek(), s.size());
    while let Some(v) = s.pop() { println!("popped: {}", v); }
    println!("empty: {}", s.is_empty());
}
```

### Solution 3

```rust
struct BankAccount {
    owner: String,
    balance: f64,
}

impl BankAccount {
    fn new(owner: &str, initial_balance: f64) -> Self {
        Self { owner: String::from(owner), balance: initial_balance.max(0.0) }
    }

    fn deposit(&mut self, amount: f64) -> Result<f64, String> {
        if amount <= 0.0 { return Err(String::from("Deposit amount must be positive")); }
        self.balance += amount;
        Ok(self.balance)
    }

    fn withdraw(&mut self, amount: f64) -> Result<f64, String> {
        if amount <= 0.0 { return Err(String::from("Withdrawal must be positive")); }
        if amount > self.balance { return Err(format!("Insufficient funds (balance: {:.2})", self.balance)); }
        self.balance -= amount;
        Ok(self.balance)
    }

    fn balance(&self) -> f64 { self.balance }
}

fn main() {
    let mut acc = BankAccount::new("Alice", 1000.0);
    println!("Balance: {:.2}", acc.balance());
    println!("{:?}", acc.deposit(500.0));
    println!("{:?}", acc.withdraw(200.0));
    println!("{:?}", acc.withdraw(2000.0));
    println!("Final: {:.2}", acc.balance());
}
```

### Solution 4

```rust
struct Pizza {
    size: &'static str,
    crust: &'static str,
    toppings: Vec<String>,
}

impl Pizza {
    fn new() -> Self {
        Self { size: "medium", crust: "thin", toppings: Vec::new() }
    }

    fn size(mut self, s: &'static str) -> Self { self.size = s; self }
    fn crust(mut self, c: &'static str) -> Self { self.crust = c; self }
    fn topping(mut self, t: &str) -> Self { self.toppings.push(String::from(t)); self }

    fn build(&self) -> String {
        format!("{} {} pizza with: {}", self.size, self.crust, self.toppings.join(", "))
    }
}

fn main() {
    let pizza = Pizza::new()
        .size("large")
        .crust("thick")
        .topping("cheese")
        .topping("pepperoni")
        .topping("mushrooms");
    println!("{}", pizza.build());
}
```

### Solution 5

```rust
use std::fmt;

struct Meters(f64);

impl Meters {
    fn new(val: f64) -> Self { Self(val) }
    fn to_km(&self) -> f64 { self.0 / 1000.0 }
    fn to_cm(&self) -> f64 { self.0 * 100.0 }
    fn add(&self, other: &Meters) -> Meters { Meters(self.0 + other.0) }
}

impl fmt::Display for Meters {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.2} m", self.0)
    }
}

fn main() {
    let a = Meters::new(1500.0);
    let b = Meters::new(750.0);
    let c = a.add(&b);
    println!("{} = {} km = {} cm", c, c.to_km(), c.to_cm());
}
```

---

## Quiz

**Q1.** What is the difference between `&self` and `&mut self`?

a) `&self` is faster  
b) `&self` borrows immutably (read-only); `&mut self` borrows mutably (read/write)  
c) `&mut self` creates a new instance  
d) There is no practical difference  

**Q2.** How do you call an associated function vs a method?

a) Both use dot notation  
b) Associated functions use `Type::function()`; methods use `instance.method()`  
c) Associated functions use `instance.function()`; methods use `Type::method()`  
d) Both use `::` notation  

**Q3.** What does `self` (without `&`) as a receiver do?

a) Borrows the instance  
b) Takes ownership of the instance — the instance is consumed and dropped after the method  
c) Creates a copy of the instance  
d) Makes the instance immutable  

**Q4.** What does `Self` (capital S) refer to inside an `impl` block?

a) The parent module  
b) The `self` variable  
c) The type being implemented — a type alias for the struct  
d) The superclass  

**Q5.** When is method chaining possible?

a) Always — all Rust methods support chaining  
b) When a method returns `Self`, `&mut Self`, or another type that has the next method  
c) Only with `&mut self` methods  
d) Only with builder pattern structs  

---

## Quiz Answers

**A1.** b) `&self` borrows immutably; `&mut self` borrows mutably  
*`&self` methods can be called on any Rectangle (mutable or not). `&mut self` requires the instance to be declared `mut`.*

**A2.** b) Associated functions: `Type::function()`; methods: `instance.method()`  
*`Rectangle::new(5.0, 3.0)` creates an instance. `rect.area()` calls a method on it. The `::` vs `.` distinction signals whether you have an instance or not.*

**A3.** b) Takes ownership — the instance is consumed  
*After calling a `self` method, the original variable is invalid. Used for consuming builders or conversions (like `into_vec()`, `into_string()`).*

**A4.** c) A type alias for the struct being implemented  
*`Self` in `impl Rectangle { fn new() -> Self }` means `Rectangle`. If you rename the struct, you only update the `impl Rectangle` line, not every `Self`.*

**A5.** b) When a method returns `Self` or another type that has the next method  
*Chaining works because each method returns something you can call the next method on. Builder patterns use consuming `Self` returns; streaming APIs often use `&mut Self`.*

---

## Chapter Summary

- **`impl` blocks** define methods on structs — group related behavior with the data it operates on
- **`&self`** — immutable borrow (read-only), most common; **`&mut self`** — mutable borrow (modify fields); **`self`** — consumes the instance (rare)
- **Associated functions** have no `self` parameter; called with `Type::function()` — used for constructors
- **`Self`** (capital S) is a type alias for the current type inside `impl` blocks
- Rust **automatically applies `&` / `&mut`** when calling methods — no manual dereferencing needed
- **Method chaining** is enabled by returning `Self` — the builder pattern is the classic example
- Implementing `Display` (for `{}`) and `Debug` (for `{:?}`) makes types printable
- Multiple `impl` blocks on the same type are valid — each trait gets its own block

In Chapter 10, we explore **enums** — types that can be one of several variants, each potentially holding different data. Combined with `match`, they form one of Rust's most expressive tools.
