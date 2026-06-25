# Chapter 14: Generics

---

## Learning Objectives

By the end of this chapter, you will be able to:

- Explain what generics are and why they exist in Rust
- Write generic functions that work across multiple types
- Define generic structs and enums
- Use type parameters with trait bounds to constrain generics
- Understand how Rust resolves generics at compile time via monomorphization
- Apply generics to eliminate code duplication without sacrificing performance
- Build a complete Generic Inventory Library as a mini project

---

## Theory

### 14.1 Why Generics Exist

Imagine you are writing a function that finds the largest number in a list of integers.

```rust
fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}
```

Now a colleague asks: "Can you make it work for `f64` too?"

You write a second function:

```rust
fn largest_f64(list: &[f64]) -> f64 {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}
```

The logic is identical. Only the type changed. You have just violated the DRY principle — Don't Repeat Yourself.

This is the exact problem generics solve. Generics let you write a single function, struct, or enum that works with any type — and Rust enforces correctness at compile time.

**The core idea:**

> Generics are placeholders for types. You write the logic once, parameterized over an abstract type. The compiler fills in the concrete types when it compiles your program.

This is not a runtime feature. Rust resolves generics entirely at compile time through a process called **monomorphization** — the compiler generates specialized versions of the code for each concrete type that is actually used. This means generics in Rust are truly zero-cost: you get the expressiveness of abstraction with the performance of hand-written concrete code.

---

### 14.2 Code Reusability

Generics are the foundation of code reusability in Rust. Without them, you would need to:

- Write duplicate functions for every type
- Maintain multiple near-identical implementations
- Risk divergence when one version is updated but another is not

With generics, you write the logic once and the compiler handles the rest.

**Comparison with other languages:**

| Language   | Generics Mechanism          | Resolved At       |
|------------|-----------------------------|-------------------|
| C          | Macros / void pointers      | Preprocessor / Runtime |
| C++        | Templates                   | Compile time       |
| Java       | Generics (type erasure)     | Compile time + Runtime erasure |
| Go         | Generics (since 1.18)       | Compile time       |
| Rust       | Generics + Trait bounds     | Compile time (monomorphization) |

Java erases type information at runtime, which introduces overhead and limits certain uses. Rust and C++ generate specialized code per type, which is faster but can increase binary size.

**Monomorphization illustrated:**

```
You write:
    fn largest<T: PartialOrd>(list: &[T]) -> T { ... }

You call it with:
    largest(&[1, 2, 3]);           // T = i32
    largest(&[1.0, 2.0, 3.0]);    // T = f64

Compiler generates:
    fn largest_i32(list: &[i32]) -> i32 { ... }
    fn largest_f64(list: &[f64]) -> f64 { ... }
```

You wrote one function. The compiler produced two. Zero runtime cost for the abstraction.

---

### 14.3 Type Parameters

A **type parameter** is a name (conventionally a single uppercase letter) that stands for a type. It is declared in angle brackets `<T>` after the function or type name.

```
fn foo<T>(x: T) -> T { ... }
        ^
        |
        This is the type parameter declaration
```

You can name it anything, but conventions are:

- `T` — general type
- `U`, `V` — additional types
- `K`, `V` — key and value (common in maps)
- `E` — error type
- `R` — return type

You can have multiple type parameters:

```rust
fn swap<A, B>(pair: (A, B)) -> (B, A) {
    (pair.1, pair.0)
}
```

Type parameters on their own accept *any* type. But often you need to restrict what types are allowed. That is done with **trait bounds**, which we cover in depth in Chapter 15. For now, here is a preview:

```rust
fn largest<T: PartialOrd>(list: &[T]) -> T {
    // T: PartialOrd means T must implement the PartialOrd trait
    // (i.e., T must support the > operator)
    ...
}
```

The syntax `T: SomeTrait` means "T must implement SomeTrait." Without this bound, the compiler cannot allow `>` comparisons on `T` because it does not know whether `T` supports comparison.

---

### 14.4 Generic Functions

A generic function is declared by adding a type parameter list in angle brackets after the function name, before the parameter list.

**Syntax:**

```
fn function_name<TypeParam>(parameter: TypeParam) -> TypeParam {
    ...
}
```

**Example — Identity function (works with any type):**

```rust
fn identity<T>(value: T) -> T {
    value
}

fn main() {
    let x = identity(42);           // T inferred as i32
    let y = identity("hello");      // T inferred as &str
    let z = identity(3.14f64);     // T inferred as f64
    println!("{} {} {}", x, y, z);
}
```

Rust infers the type parameter from the argument — you rarely need to write `identity::<i32>(42)` explicitly (though you can).

**Example — First element:**

```rust
fn first<T>(list: &[T]) -> Option<&T> {
    if list.is_empty() {
        None
    } else {
        Some(&list[0])
    }
}

fn main() {
    let numbers = vec![10, 20, 30];
    let words = vec!["alpha", "beta", "gamma"];

    println!("{:?}", first(&numbers));  // Some(10)
    println!("{:?}", first(&words));    // Some("alpha")
    println!("{:?}", first::<i32>(&[])); // None
}
```

**Example — Largest with trait bound:**

```rust
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let numbers = vec![34, 50, 25, 100, 65];
    println!("Largest number: {}", largest(&numbers));

    let chars = vec!['y', 'm', 'a', 'q'];
    println!("Largest char: {}", largest(&chars));
}
```

Here `T: PartialOrd` is necessary because the `>` operator requires the `PartialOrd` trait. Without it, the compiler rejects the comparison.

**Multiple type parameters:**

```rust
fn pair_display<T: std::fmt::Display, U: std::fmt::Display>(first: T, second: U) {
    println!("First: {}, Second: {}", first, second);
}

fn main() {
    pair_display(42, "hello");
    pair_display(3.14, true);
}
```

---

### 14.5 Generic Structs

Structs can also be parameterized over types. This is how Rust's standard library types like `Vec<T>`, `Option<T>`, and `HashMap<K, V>` are defined.

**Syntax:**

```
struct StructName<T> {
    field: T,
}
```

**Example — A simple Point struct:**

```rust
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 1.5, y: 4.2 };

    println!("Integer point: ({}, {})", integer_point.x, integer_point.y);
    println!("Float point: ({}, {})", float_point.x, float_point.y);
}
```

If you try `Point { x: 5, y: 4.2 }`, the compiler will reject it because `x` and `y` must be the same type `T`.

**Using multiple type parameters in a struct:**

```rust
struct Pair<T, U> {
    first: T,
    second: U,
}

fn main() {
    let p = Pair { first: 42, second: "hello" };
    println!("{} and {}", p.first, p.second);
}
```

**Implementing methods on generic structs:**

When you implement methods on a generic struct, you must re-declare the type parameters in the `impl` block:

```rust
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Point { x, y }
    }

    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &T {
        &self.y
    }
}

fn main() {
    let p = Point::new(3, 7);
    println!("x = {}, y = {}", p.x(), p.y());

    let pf = Point::new(1.5, 2.5);
    println!("x = {}, y = {}", pf.x(), pf.y());
}
```

The `impl<T>` declares that `T` is in scope for the implementation block. Then `Point<T>` names the specific generic struct being implemented.

**Implementing methods only for specific types:**

```rust
impl Point<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

fn main() {
    let p = Point { x: 3.0f64, y: 4.0f64 };
    println!("Distance: {}", p.distance_from_origin()); // 5.0

    let pi = Point { x: 3, y: 4 };
    // pi.distance_from_origin(); // ERROR: method not available for Point<i32>
}
```

This lets you add methods that only make sense for certain concrete types.

**Visual memory layout:**

```
Point<i32> in memory:
+-------+-------+
|   x   |   y   |
| (i32) | (i32) |
+-------+-------+
  4 bytes 4 bytes

Point<f64> in memory:
+----------+----------+
|    x     |    y     |
|  (f64)   |  (f64)   |
+----------+----------+
  8 bytes    8 bytes
```

The compiler generates separate, correctly-sized struct layouts for each concrete type.

---

### 14.6 Generic Enums

Enums can also be generic. You have already used generic enums extensively without realizing it — `Option<T>` and `Result<T, E>` are both generic enums defined in the standard library.

**The standard library definitions:**

```rust
// From the standard library (simplified):
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

`Option<T>` encodes the presence or absence of a value of type `T`. `Result<T, E>` encodes success (`Ok` containing type `T`) or failure (`Err` containing error type `E`).

**Defining your own generic enum:**

```rust
enum Either<L, R> {
    Left(L),
    Right(R),
}

fn main() {
    let value: Either<i32, &str> = Either::Left(42);
    let label: Either<i32, &str> = Either::Right("hello");

    match value {
        Either::Left(n) => println!("Got number: {}", n),
        Either::Right(s) => println!("Got string: {}", s),
    }

    match label {
        Either::Left(n) => println!("Got number: {}", n),
        Either::Right(s) => println!("Got string: {}", s),
    }
}
```

**A generic Tree enum:**

```rust
enum Tree<T> {
    Leaf(T),
    Node(Box<Tree<T>>, Box<Tree<T>>),
}

fn sum_tree(tree: &Tree<i32>) -> i32 {
    match tree {
        Tree::Leaf(value) => *value,
        Tree::Node(left, right) => sum_tree(left) + sum_tree(right),
    }
}

fn main() {
    let tree = Tree::Node(
        Box::new(Tree::Node(
            Box::new(Tree::Leaf(1)),
            Box::new(Tree::Leaf(2)),
        )),
        Box::new(Tree::Leaf(3)),
    );

    println!("Sum: {}", sum_tree(&tree)); // 6
}
```

Notice `Box<Tree<T>>` — the `Box` is required here because an enum variant cannot contain its own type directly (that would be infinitely sized). `Box` puts the value on the heap, giving it a fixed pointer size.

---

### Code Example

The following is a complete, runnable program demonstrating all major concepts from this chapter together: generic functions, generic structs, generic enums, trait bounds, and method implementations.

```rust
use std::fmt::Display;

// ============================================================
// 14.4 Generic Function — find the largest item
// ============================================================

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// ============================================================
// 14.4 Generic Function — identity (no bounds needed)
// ============================================================

fn identity<T>(value: T) -> T {
    value
}

// ============================================================
// 14.4 Generic Function — swap two values
// ============================================================

fn swap<A, B>(pair: (A, B)) -> (B, A) {
    (pair.1, pair.0)
}

// ============================================================
// 14.5 Generic Struct — a 2D Point
// ============================================================

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T: Display> Point<T> {
    fn new(x: T, y: T) -> Self {
        Point { x, y }
    }

    fn display(&self) {
        println!("Point({}, {})", self.x, self.y);
    }
}

impl Point<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

// ============================================================
// 14.5 Generic Struct — a wrapper with a label
// ============================================================

#[derive(Debug)]
struct Labeled<T> {
    label: String,
    value: T,
}

impl<T: Display> Labeled<T> {
    fn new(label: &str, value: T) -> Self {
        Labeled {
            label: label.to_string(),
            value,
        }
    }

    fn describe(&self) {
        println!("[{}]: {}", self.label, self.value);
    }
}

// ============================================================
// 14.6 Generic Enum — a value that can be one of two types
// ============================================================

#[derive(Debug)]
enum Either<L, R> {
    Left(L),
    Right(R),
}

impl<L: Display, R: Display> Either<L, R> {
    fn display(&self) {
        match self {
            Either::Left(val) => println!("Left({})", val),
            Either::Right(val) => println!("Right({})", val),
        }
    }
}

// ============================================================
// 14.6 Generic Enum — a simple binary tree
// ============================================================

#[derive(Debug)]
enum Tree<T> {
    Leaf(T),
    Node(Box<Tree<T>>, Box<Tree<T>>),
}

impl Tree<i32> {
    fn sum(&self) -> i32 {
        match self {
            Tree::Leaf(v) => *v,
            Tree::Node(left, right) => left.sum() + right.sum(),
        }
    }

    fn depth(&self) -> usize {
        match self {
            Tree::Leaf(_) => 1,
            Tree::Node(left, right) => 1 + left.depth().max(right.depth()),
        }
    }
}

// ============================================================
// main — demonstrates all of the above
// ============================================================

fn main() {
    println!("=== Generic Functions ===\n");

    // largest
    let numbers = vec![34, 50, 25, 100, 65];
    println!("Largest integer: {}", largest(&numbers));

    let chars = vec!['y', 'm', 'a', 'q'];
    println!("Largest char: {}", largest(&chars));

    let words = vec!["banana", "apple", "cherry"];
    println!("Largest word: {}", largest(&words));

    // identity
    println!("Identity i32: {}", identity(42));
    println!("Identity &str: {}", identity("Rust"));
    println!("Identity bool: {}", identity(true));

    // swap
    let pair = (10, "hello");
    let swapped = swap(pair);
    println!("Swapped: ({}, {})", swapped.0, swapped.1);

    println!("\n=== Generic Structs ===\n");

    // Point<i32>
    let pi = Point::new(3, 4);
    pi.display();

    // Point<f64>
    let pf = Point::new(3.0f64, 4.0f64);
    pf.display();
    println!("Distance from origin: {:.2}", pf.distance_from_origin());

    // Labeled
    let labeled_int = Labeled::new("score", 99);
    labeled_int.describe();

    let labeled_str = Labeled::new("name", "Alice");
    labeled_str.describe();

    println!("\n=== Generic Enums ===\n");

    // Either
    let left: Either<i32, &str> = Either::Left(42);
    let right: Either<i32, &str> = Either::Right("Rust");
    left.display();
    right.display();

    // Tree
    let tree = Tree::Node(
        Box::new(Tree::Node(
            Box::new(Tree::Leaf(1)),
            Box::new(Tree::Leaf(2)),
        )),
        Box::new(Tree::Node(
            Box::new(Tree::Leaf(3)),
            Box::new(Tree::Leaf(4)),
        )),
    );
    println!("Tree sum: {}", tree.sum());
    println!("Tree depth: {}", tree.depth());
}
```

---

### Line-by-Line Explanation

**Generic function `largest<T: PartialOrd>`**

```rust
fn largest<T: PartialOrd>(list: &[T]) -> &T {
```
- `<T: PartialOrd>` — declares type parameter `T`, constrained to types that implement `PartialOrd` (support `>`, `<`, `>=`, `<=` comparisons)
- `list: &[T]` — a slice of any type `T`; we borrow it, we don't own it
- `-> &T` — returns a reference to an element in the slice (same lifetime as the input)

```rust
    let mut largest = &list[0];
```
- Takes a reference to the first element; does not copy it (important for non-`Copy` types)

```rust
    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
```
- Iterates, comparing each item to the current largest using `>` (enabled by `PartialOrd`)
- Returns the reference to the largest element — no semicolon means this is an expression (return value)

**Generic function `identity<T>`**

```rust
fn identity<T>(value: T) -> T {
    value
}
```
- No trait bound needed — we are not doing anything with `value` except returning it
- Works for any type whatsoever

**Generic function `swap<A, B>`**

```rust
fn swap<A, B>(pair: (A, B)) -> (B, A) {
    (pair.1, pair.0)
}
```
- Two independent type parameters `A` and `B`
- Takes ownership of `pair`, returns a new tuple with types reversed

**Generic struct `Point<T>` and `impl<T: Display>`**

```rust
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}
```
- `T` applies to both `x` and `y` — they must be the same type

```rust
impl<T: Display> Point<T> {
    fn new(x: T, y: T) -> Self { ... }
    fn display(&self) { ... }
}
```
- `impl<T: Display>` — the `Display` bound is needed only because we use `{}` formatting in `display`
- `Self` is shorthand for `Point<T>` — it refers to the implementing type

```rust
impl Point<f64> {
    fn distance_from_origin(&self) -> f64 { ... }
}
```
- No type parameter here — this `impl` only applies when `T` is `f64`
- If you call this on `Point<i32>`, the compiler gives an error: method not found

**Generic struct `Labeled<T>`**

```rust
struct Labeled<T> {
    label: String,
    value: T,
}
```
- `label` is always a `String` (concrete type)
- `value` is generic — can be anything

**Generic enum `Either<L, R>`**

```rust
enum Either<L, R> {
    Left(L),
    Right(R),
}
```
- Two independent type parameters: `L` for the left variant, `R` for the right
- Each variant wraps a value of its respective type

**Generic enum `Tree<T>`**

```rust
enum Tree<T> {
    Leaf(T),
    Node(Box<Tree<T>>, Box<Tree<T>>),
}
```
- `Leaf(T)` — a leaf node holding a value of type `T`
- `Node(Box<Tree<T>>, Box<Tree<T>>)` — `Box` is mandatory because recursive enums need indirection; the compiler cannot know the size of `Tree<T>` if it contains `Tree<T>` directly

**`sum` method on `Tree<i32>`**

```rust
impl Tree<i32> {
    fn sum(&self) -> i32 {
        match self {
            Tree::Leaf(v) => *v,          // dereference to get i32 value
            Tree::Node(left, right) => left.sum() + right.sum(),
        }
    }
}
```
- Recursive: sum of a node = sum of left subtree + sum of right subtree
- `*v` — `v` is `&i32`, we dereference to get `i32`

---

### Common Mistakes

**Mistake 1: Forgetting trait bounds when operations require them**

```rust
// ERROR: cannot use > without PartialOrd bound
fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list.iter() {
        if item > largest { // ERROR: binary operation `>` cannot be applied to type `T`
            largest = item;
        }
    }
    largest
}
```

Fix: add `T: PartialOrd`.

**Mistake 2: Assuming all type parameters are the same**

```rust
struct Point<T> {
    x: T,
    y: T,
}

// This will NOT compile:
let p = Point { x: 5, y: 4.0 }; // ERROR: expected integer, found float
```

Fix: use two type parameters `Point<T, U>` if `x` and `y` can differ.

**Mistake 3: Forgetting to declare type parameter in impl block**

```rust
// ERROR: impl expects T to be declared
impl Point<T> {    // T is undeclared here
    fn new(x: T, y: T) -> Self { ... }
}
```

Fix:

```rust
impl<T> Point<T> {   // T is declared in <T> after impl
    fn new(x: T, y: T) -> Self { ... }
}
```

**Mistake 4: Recursive enum without Box**

```rust
// ERROR: recursive type has infinite size
enum Tree<T> {
    Leaf(T),
    Node(Tree<T>, Tree<T>),  // ERROR: this creates infinite recursion in size
}
```

Fix: use `Box<Tree<T>>` to introduce indirection.

**Mistake 5: Unnecessary bounds that restrict usage**

```rust
// Adding Display bound when it's not needed limits what types can be used
fn first<T: Display>(list: &[T]) -> Option<&T> {
    list.first()
}
```

If you only need to return a reference, there is no reason to require `Display`. The bound unnecessarily restricts callers.

**Mistake 6: Specifying concrete type in impl when generic is needed**

```rust
// This only works for i32 Point, not all Points
impl Point<i32> {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}

// If you call Point::new(1.0, 2.0) it won't work
```

Fix: use `impl<T> Point<T>` when the method should work for all `T`.

---

### Best Practices

1. **Add only necessary bounds.** Start with no bounds and add them when the compiler demands them. Unnecessary bounds restrict who can use your API.

2. **Use descriptive parameter names for complex generics.** Instead of `K, V` when it is not about key-value, use `Input, Output` for clarity.

3. **Prefer returning owned values when possible.** `-> T` is simpler than `-> &T` when ownership is not an issue.

4. **Leverage type inference.** Let the compiler infer type parameters from usage. Avoid spelling them out unless necessary: `identity::<i32>(42)` is rarely needed.

5. **Put bounds in `where` clauses for readability when there are many:**

```rust
// Hard to read inline
fn process<T: Display + Clone + PartialOrd>(items: &[T]) -> T { ... }

// Cleaner with where clause
fn process<T>(items: &[T]) -> T
where
    T: Display + Clone + PartialOrd,
{ ... }
```

6. **Use `Box<T>` for recursive types.** Whenever an enum or struct contains itself, wrap the inner occurrence in `Box`.

7. **Keep generic code well-tested.** Since the compiler generates separate code per type, test with multiple types to catch type-specific edge cases.

8. **Understand when not to use generics.** If a function only ever makes sense for one type, a generic is unnecessary complexity. Use generics only when genuine type-independence adds value.

---

### Exercises

**Exercise 1 — Generic minimum function**

Write a generic function `minimum<T>` that takes a slice `&[T]` and returns a reference to the smallest element. Use an appropriate trait bound. Test it with `i32`, `f64`, and `char`.

**Exercise 2 — Generic Stack**

Implement a generic `Stack<T>` struct backed by a `Vec<T>` with the following methods:
- `new() -> Stack<T>`
- `push(&mut self, item: T)`
- `pop(&mut self) -> Option<T>`
- `peek(&self) -> Option<&T>`
- `is_empty(&self) -> bool`
- `size(&self) -> usize`

Test it with integers and strings.

**Exercise 3 — Generic pair sum**

Write a generic function `sum_pair<T>` that takes two values of the same type `T` and returns their sum. Use an appropriate trait bound. Test with `i32` and `f64`.

**Exercise 4 — Generic Wrapper struct**

Define a generic struct `Wrapper<T>` with a single field `value: T`. Implement:
- `new(value: T) -> Self`
- `get(&self) -> &T`
- `unwrap(self) -> T`
- A `Display` implementation (requires `T: Display`) that prints `Wrapper(value)`

**Exercise 5 — Generic Either enum with map**

Extend the `Either<L, R>` enum from the chapter with:
- `map_left<F, NewL>(self, f: F) -> Either<NewL, R>` where `F: FnOnce(L) -> NewL`
- `map_right<F, NewR>(self, f: F) -> Either<L, NewR>` where `F: FnOnce(R) -> NewR`
- `is_left(&self) -> bool`
- `is_right(&self) -> bool`

Test by transforming values in each variant.

---

### Solutions

**Solution 1 — Generic minimum function**

```rust
fn minimum<T: PartialOrd>(list: &[T]) -> &T {
    let mut min = &list[0];
    for item in list.iter() {
        if item < min {
            min = item;
        }
    }
    min
}

fn main() {
    let ints = vec![5, 3, 8, 1, 9];
    println!("Min int: {}", minimum(&ints));       // 1

    let floats = vec![2.5, 1.1, 3.7, 0.9];
    println!("Min float: {}", minimum(&floats));   // 0.9

    let chars = vec!['z', 'a', 'm', 'b'];
    println!("Min char: {}", minimum(&chars));     // 'a'
}
```

**Solution 2 — Generic Stack**

```rust
#[derive(Debug)]
struct Stack<T> {
    data: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack { data: Vec::new() }
    }

    fn push(&mut self, item: T) {
        self.data.push(item);
    }

    fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }

    fn peek(&self) -> Option<&T> {
        self.data.last()
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    fn size(&self) -> usize {
        self.data.len()
    }
}

fn main() {
    let mut int_stack: Stack<i32> = Stack::new();
    int_stack.push(10);
    int_stack.push(20);
    int_stack.push(30);

    println!("Size: {}", int_stack.size());          // 3
    println!("Peek: {:?}", int_stack.peek());        // Some(30)
    println!("Pop: {:?}", int_stack.pop());          // Some(30)
    println!("Pop: {:?}", int_stack.pop());          // Some(20)
    println!("Is empty: {}", int_stack.is_empty());  // false

    let mut str_stack: Stack<String> = Stack::new();
    str_stack.push(String::from("hello"));
    str_stack.push(String::from("world"));
    println!("String peek: {:?}", str_stack.peek()); // Some("world")
}
```

**Solution 3 — Generic pair sum**

```rust
use std::ops::Add;

fn sum_pair<T: Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

fn main() {
    println!("i32 sum: {}", sum_pair(3i32, 4i32));      // 7
    println!("f64 sum: {}", sum_pair(1.5f64, 2.5f64));  // 4.0
}
```

Note: `Add<Output = T>` means "T can be added to itself and the result is also T."

**Solution 4 — Generic Wrapper struct**

```rust
use std::fmt;

struct Wrapper<T> {
    value: T,
}

impl<T> Wrapper<T> {
    fn new(value: T) -> Self {
        Wrapper { value }
    }

    fn get(&self) -> &T {
        &self.value
    }

    fn unwrap(self) -> T {
        self.value
    }
}

impl<T: fmt::Display> fmt::Display for Wrapper<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Wrapper({})", self.value)
    }
}

fn main() {
    let w = Wrapper::new(42);
    println!("{}", w);               // Wrapper(42)
    println!("Get: {}", w.get());    // Get: 42
    let val = w.unwrap();
    println!("Unwrapped: {}", val);  // Unwrapped: 42

    let ws = Wrapper::new("hello");
    println!("{}", ws);              // Wrapper(hello)
}
```

**Solution 5 — Generic Either with map**

```rust
#[derive(Debug)]
enum Either<L, R> {
    Left(L),
    Right(R),
}

impl<L, R> Either<L, R> {
    fn is_left(&self) -> bool {
        matches!(self, Either::Left(_))
    }

    fn is_right(&self) -> bool {
        matches!(self, Either::Right(_))
    }

    fn map_left<F, NewL>(self, f: F) -> Either<NewL, R>
    where
        F: FnOnce(L) -> NewL,
    {
        match self {
            Either::Left(val) => Either::Left(f(val)),
            Either::Right(val) => Either::Right(val),
        }
    }

    fn map_right<F, NewR>(self, f: F) -> Either<L, NewR>
    where
        F: FnOnce(R) -> NewR,
    {
        match self {
            Either::Left(val) => Either::Left(val),
            Either::Right(val) => Either::Right(f(val)),
        }
    }
}

fn main() {
    let left: Either<i32, &str> = Either::Left(5);
    let right: Either<i32, &str> = Either::Right("hello");

    println!("Is left: {}", left.is_left());   // true
    println!("Is right: {}", right.is_right()); // true

    let doubled = left.map_left(|x| x * 2);
    println!("{:?}", doubled); // Left(10)

    let upper = right.map_right(|s: &str| s.to_uppercase());
    println!("{:?}", upper); // Right("HELLO")
}
```

---

### Quiz

**Question 1**

What does the following code print?

```rust
fn identity<T>(x: T) -> T { x }

fn main() {
    let a = identity(100);
    let b = identity("rust");
    println!("{} {}", a, b);
}
```

A) Compile error — T is ambiguous
B) `100 rust`
C) `identity identity`
D) Panic at runtime

---

**Question 2**

Why does this code fail to compile?

```rust
fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}
```

A) `list` should be `Vec<T>` not `&[T]`
B) `T` does not have a bound that allows `>` comparisons
C) You cannot return a reference from a generic function
D) `largest` needs to be `mut`

---

**Question 3**

What is monomorphization?

A) A runtime polymorphism technique using vtables
B) The process of converting generics to concrete types at compile time
C) A way to make all types identical in memory
D) A garbage collection strategy for generic types

---

**Question 4**

Which of the following is a valid declaration for an `impl` block on a generic struct `Container<T>`?

A) `impl Container { ... }`
B) `impl<T> Container { ... }`
C) `impl<T> Container<T> { ... }`
D) `impl Container<T: Display> { ... }`

---

**Question 5**

Why must recursive enums use `Box` for the recursive variant?

A) `Box` enables runtime polymorphism
B) Without `Box`, the compiler cannot determine the size of the enum at compile time
C) `Box` is required for all enum variants containing other types
D) `Box` prevents stack overflow by automatically using heap allocation at runtime

---

**Question 6**

What is the difference between these two `impl` blocks?

```rust
impl<T> Point<T> { ... }
impl Point<f64> { ... }
```

A) No difference — they both apply to all `Point` instances
B) The first applies to all `Point<T>`, the second only to `Point<f64>`
C) The first is incorrect syntax
D) The second applies to all numeric `Point` types

---

**Question 7**

What additional trait bound is needed to make this compile?

```rust
fn print_value<T>(value: T) {
    println!("{}", value);
}
```

A) `T: Clone`
B) `T: Debug`
C) `T: Display`
D) `T: PartialEq`

---

**Question 8**

Which standard library type is defined as `enum Option<T> { Some(T), None }`?

A) A concrete type — not generic
B) A generic enum with one type parameter
C) A generic enum with two type parameters
D) A generic struct

---

### Quiz Answers

**Answer 1: B) `100 rust`**

The compiler infers `T = i32` for the first call and `T = &str` for the second. `identity` simply returns its argument. Both print fine because `i32` and `&str` implement `Display`.

---

**Answer 2: B) `T` does not have a bound that allows `>` comparisons**

The `>` operator requires the `PartialOrd` trait. Without `T: PartialOrd`, the compiler does not know whether `T` can be compared. Fix: `fn largest<T: PartialOrd>`.

---

**Answer 3: B) The process of converting generics to concrete types at compile time**

Monomorphization is Rust's compile-time strategy for generics. The compiler generates a concrete copy of generic code for each distinct type it is called with. This eliminates runtime overhead at the cost of potentially larger binaries.

---

**Answer 4: C) `impl<T> Container<T> { ... }`**

`impl<T>` declares `T` as a type parameter for the block, and `Container<T>` names the specific generic type being implemented. Option A forgets the generic parameter entirely. Option B applies `T` to `impl` but not to `Container`. Option D has invalid syntax for a trait bound in an `impl` header.

---

**Answer 5: B) Without `Box`, the compiler cannot determine the size of the enum at compile time**

Rust must know the size of every type at compile time. A recursive enum without indirection would be infinitely large. `Box<T>` has a fixed size (one pointer), so the compiler can size the enum correctly. The actual recursive data lives on the heap.

---

**Answer 6: B) The first applies to all `Point<T>`, the second only to `Point<f64>`**

`impl<T> Point<T>` introduces methods available on any `Point` regardless of `T`. `impl Point<f64>` is a specialization — those methods only exist on `Point<f64>`. This allows type-specific functionality without affecting other instantiations.

---

**Answer 7: C) `T: Display`**

The `{}` format specifier requires the `Display` trait. Without the bound, the compiler does not know that `T` can be printed with `{}`. Adding `T: Display` (or `T: std::fmt::Display`) fixes it.

---

**Answer 8: B) A generic enum with one type parameter**

`Option<T>` is a generic enum with a single type parameter `T`. It has two variants: `Some(T)` holding a value, and `None` representing absence. You use it constantly in Rust — for example, `Vec::get()` returns `Option<&T>`.

---

## Mini Project: Generic Inventory Library

### Project Overview

We will build a reusable, generic inventory library in Rust. The library is a crate that can store and manage a collection of any item type. It is generic so it works equally well with products, books, tools, or any user-defined type — without writing the logic twice.

**What we build:**

- `Inventory<T>` — a generic container for items
- `Item<T>` — a wrapper that gives each item an ID and name
- Methods: `add`, `remove`, `search`, `list`, `count`
- A demonstration binary that uses the library with custom product types

**Why this project:**

This project mirrors real-world patterns: a warehouse system, a content management store, an asset registry. The generic core is written once and reused across domains.

---

### Functional Requirements

1. Store items of any type `T`
2. Assign each item an auto-incremented ID
3. Add items by name and value
4. Remove items by ID
5. Search items by name (case-insensitive)
6. List all items
7. Report total count
8. Work correctly for any `T` that satisfies minimal trait bounds

---

### Project Structure

```
inventory_library/
├── Cargo.toml
└── src/
    ├── lib.rs          — library root
    ├── item.rs         — Item<T> struct
    ├── inventory.rs    — Inventory<T> struct and methods
    └── main.rs         — demonstration binary
```

---

### Step-by-Step Development

#### Step 1 — Initialize the project

```bash
cargo new inventory_library
cd inventory_library
```

Because this project has both a library (`lib.rs`) and a binary (`main.rs`), Cargo supports both in the same crate automatically.

#### Step 2 — Define `Item<T>` in `src/item.rs`

The `Item<T>` struct wraps any value of type `T` and adds metadata: a unique ID and a human-readable name.

#### Step 3 — Define `Inventory<T>` in `src/inventory.rs`

The inventory holds a `Vec<Item<T>>` and a counter for auto-generating IDs.

#### Step 4 — Wire up `src/lib.rs`

The library root re-exports the types for external users.

#### Step 5 — Demonstrate in `src/main.rs`

Create custom types and show the inventory working with them.

---

### Complete Source Code

**`Cargo.toml`**

```toml
[package]
name = "inventory_library"
version = "0.1.0"
edition = "2021"

[[bin]]
name = "inventory_library"
path = "src/main.rs"
```

---

**`src/item.rs`**

```rust
use std::fmt;

/// A single item stored in the inventory.
///
/// `T` is the type of the actual item data (e.g., a Product, a Book, etc.).
/// Each item receives a unique integer ID and a name when it is added.
#[derive(Debug, Clone)]
pub struct Item<T> {
    /// Auto-assigned unique identifier.
    pub id: u64,
    /// Human-readable name for the item.
    pub name: String,
    /// The actual item data.
    pub data: T,
}

impl<T> Item<T> {
    /// Creates a new Item with the given id, name, and data.
    pub fn new(id: u64, name: &str, data: T) -> Self {
        Item {
            id,
            name: name.to_string(),
            data,
        }
    }

    /// Returns a reference to the item's data.
    pub fn data(&self) -> &T {
        &self.data
    }

    /// Returns the item's ID.
    pub fn id(&self) -> u64 {
        self.id
    }

    /// Returns the item's name as a string slice.
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl<T: fmt::Display> fmt::Display for Item<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[ID: {}] {} — {}", self.id, self.name, self.data)
    }
}
```

---

**`src/inventory.rs`**

```rust
use crate::item::Item;
use std::fmt;

/// A generic inventory that can store items of any type `T`.
///
/// # Type Parameter
/// - `T`: The type of data stored in each item.
///
/// # Trait Bounds (on methods that need them)
/// - `T: Clone` — required for `add` so we store an owned copy
/// - `T: fmt::Display` — required for `list` and `display` methods
pub struct Inventory<T> {
    /// The list of items stored in the inventory.
    items: Vec<Item<T>>,
    /// Counter used to auto-generate unique item IDs.
    next_id: u64,
}

impl<T: Clone> Inventory<T> {
    /// Creates a new, empty inventory.
    pub fn new() -> Self {
        Inventory {
            items: Vec::new(),
            next_id: 1,
        }
    }

    /// Adds an item to the inventory with the given name and data.
    ///
    /// Returns the assigned ID for the new item.
    ///
    /// # Arguments
    /// - `name`: A human-readable name for the item.
    /// - `data`: The item data of type `T`.
    pub fn add(&mut self, name: &str, data: T) -> u64 {
        let id = self.next_id;
        let item = Item::new(id, name, data);
        self.items.push(item);
        self.next_id += 1;
        id
    }

    /// Removes an item by its ID.
    ///
    /// Returns `Some(Item<T>)` if found and removed, or `None` if no item
    /// with that ID exists.
    pub fn remove(&mut self, id: u64) -> Option<Item<T>> {
        if let Some(pos) = self.items.iter().position(|item| item.id == id) {
            Some(self.items.remove(pos))
        } else {
            None
        }
    }

    /// Searches for items whose names contain the given query string.
    ///
    /// The search is case-insensitive. Returns a `Vec` of references to
    /// matching items.
    ///
    /// # Arguments
    /// - `query`: The substring to search for in item names.
    pub fn search(&self, query: &str) -> Vec<&Item<T>> {
        let query_lower = query.to_lowercase();
        self.items
            .iter()
            .filter(|item| item.name.to_lowercase().contains(&query_lower))
            .collect()
    }

    /// Returns a reference to the item with the given ID, if it exists.
    pub fn find_by_id(&self, id: u64) -> Option<&Item<T>> {
        self.items.iter().find(|item| item.id == id)
    }

    /// Returns references to all items in the inventory.
    pub fn list(&self) -> &[Item<T>] {
        &self.items
    }

    /// Returns the number of items currently in the inventory.
    pub fn count(&self) -> usize {
        self.items.len()
    }

    /// Returns `true` if the inventory contains no items.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Clears all items from the inventory.
    pub fn clear(&mut self) {
        self.items.clear();
    }

    /// Returns an iterator over all item data values.
    pub fn iter_data(&self) -> impl Iterator<Item = &T> {
        self.items.iter().map(|item| &item.data)
    }
}

impl<T: Clone + fmt::Display> Inventory<T> {
    /// Prints all items in the inventory to stdout.
    pub fn display_all(&self) {
        if self.items.is_empty() {
            println!("  (inventory is empty)");
            return;
        }
        for item in &self.items {
            println!("  {}", item);
        }
    }
}

impl<T: Clone> Default for Inventory<T> {
    fn default() -> Self {
        Self::new()
    }
}
```

---

**`src/lib.rs`**

```rust
// Declare modules
pub mod inventory;
pub mod item;

// Re-export the main types for convenient use
pub use inventory::Inventory;
pub use item::Item;
```

---

**`src/main.rs`**

```rust
use inventory_library::{Inventory, Item};
use std::fmt;

// ============================================================
// Define a custom Product type to demonstrate the inventory
// ============================================================

#[derive(Debug, Clone)]
struct Product {
    price: f64,
    quantity: u32,
    category: String,
}

impl Product {
    fn new(price: f64, quantity: u32, category: &str) -> Self {
        Product {
            price,
            quantity,
            category: category.to_string(),
        }
    }

    fn total_value(&self) -> f64 {
        self.price * self.quantity as f64
    }
}

impl fmt::Display for Product {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "${:.2} x {} units [{}]",
            self.price, self.quantity, self.category
        )
    }
}

// ============================================================
// Define a Book type to show the inventory works with any T
// ============================================================

#[derive(Debug, Clone)]
struct Book {
    author: String,
    year: u32,
    isbn: String,
}

impl Book {
    fn new(author: &str, year: u32, isbn: &str) -> Self {
        Book {
            author: author.to_string(),
            year,
            isbn: isbn.to_string(),
        }
    }
}

impl fmt::Display for Book {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "by {} ({}) [ISBN: {}]", self.author, self.year, self.isbn)
    }
}

// ============================================================
// Helper: print a section header
// ============================================================

fn section(title: &str) {
    println!("\n{}", "=".repeat(60));
    println!(" {}", title);
    println!("{}", "=".repeat(60));
}

// ============================================================
// main — demonstrate the generic inventory library
// ============================================================

fn main() {
    // ----------------------------------------------------------
    // Part 1: Product Inventory
    // ----------------------------------------------------------

    section("Product Inventory Demo");

    let mut products: Inventory<Product> = Inventory::new();

    // Add items
    let laptop_id = products.add("Laptop Pro 15", Product::new(1299.99, 10, "Electronics"));
    let phone_id = products.add("Smartphone X", Product::new(799.99, 25, "Electronics"));
    let desk_id = products.add("Standing Desk", Product::new(549.99, 5, "Furniture"));
    let chair_id = products.add("Ergonomic Chair", Product::new(349.99, 8, "Furniture"));
    let _mouse_id = products.add("Wireless Mouse", Product::new(49.99, 50, "Electronics"));

    println!("\n--- All Products ({} total) ---", products.count());
    products.display_all();

    // Search
    println!("\n--- Search: 'electronic' ---");
    let results = products.search("electronic");
    if results.is_empty() {
        println!("  No results.");
    } else {
        for item in &results {
            println!("  {}", item);
        }
    }

    println!("\n--- Search: 'desk' ---");
    let results = products.search("desk");
    for item in &results {
        println!("  {}", item);
    }

    // Find by ID
    println!("\n--- Find by ID ({}) ---", laptop_id);
    match products.find_by_id(laptop_id) {
        Some(item) => println!("  Found: {}", item),
        None => println!("  Not found."),
    }

    // Remove
    println!("\n--- Remove ID {} (Smartphone X) ---", phone_id);
    match products.remove(phone_id) {
        Some(item) => println!("  Removed: {}", item),
        None => println!("  Not found."),
    }

    println!("\n--- Products after removal ({} total) ---", products.count());
    products.display_all();

    // Try to find removed item
    println!("\n--- Find removed ID {} ---", phone_id);
    match products.find_by_id(phone_id) {
        Some(item) => println!("  Found: {}", item),
        None => println!("  Item not found (correctly removed)."),
    }

    // Try to remove non-existent ID
    println!("\n--- Remove non-existent ID 999 ---");
    match products.remove(999) {
        Some(item) => println!("  Removed: {}", item),
        None => println!("  Item not found (ID 999 does not exist)."),
    }

    // Calculate total inventory value using iter_data
    let total_value: f64 = products
        .iter_data()
        .map(|p| p.total_value())
        .sum();
    println!("\n--- Total inventory value: ${:.2} ---", total_value);

    // ----------------------------------------------------------
    // Part 2: Book Inventory — same Inventory<T>, different T
    // ----------------------------------------------------------

    section("Book Inventory Demo");

    let mut books: Inventory<Book> = Inventory::new();

    let rust_id = books.add(
        "The Rust Programming Language",
        Book::new("Steve Klabnik & Carol Nichols", 2023, "978-1718503106"),
    );
    books.add(
        "Programming Rust",
        Book::new("Jim Blandy & Jason Orendorff", 2021, "978-1492052470"),
    );
    books.add(
        "Rust for Rustaceans",
        Book::new("Jon Gjengset", 2021, "978-1718501850"),
    );
    books.add(
        "Zero To Production In Rust",
        Book::new("Luca Palmieri", 2022, "979-8392138228"),
    );

    println!("\n--- All Books ({} total) ---", books.count());
    books.display_all();

    println!("\n--- Search: 'rust' ---");
    let results = books.search("rust");
    for item in &results {
        println!("  {}", item);
    }

    println!("\n--- Search: 'production' ---");
    let results = books.search("production");
    for item in &results {
        println!("  {}", item);
    }

    println!("\n--- Find by ID {} ---", rust_id);
    match books.find_by_id(rust_id) {
        Some(item) => println!("  Found: {}", item),
        None => println!("  Not found."),
    }

    println!("\n--- Remove ID {} ---", rust_id);
    match books.remove(rust_id) {
        Some(item) => println!("  Removed: {}", item.name()),
        None => println!("  Not found."),
    }

    println!("\n--- Books after removal ({} total) ---", books.count());
    books.display_all();

    // ----------------------------------------------------------
    // Part 3: Demonstrate with simple primitive type
    // ----------------------------------------------------------

    section("Integer Inventory Demo");

    let mut numbers: Inventory<i32> = Inventory::new();
    numbers.add("Sensor Reading A", 42);
    numbers.add("Sensor Reading B", 137);
    numbers.add("Sensor Reading C", 99);

    println!("\n--- All Readings ---");
    numbers.display_all();

    println!("\n--- Search: 'sensor' ---");
    for item in numbers.search("sensor") {
        println!("  {}", item);
    }

    let total: i32 = numbers.iter_data().sum();
    println!("\n--- Sum of all readings: {} ---", total);

    section("Done");
    println!("Demonstrated Inventory<Product>, Inventory<Book>, Inventory<i32>");
    println!("Same generic implementation — three different item types.");
}
```

---

### Code Explanation

**`Item<T>` struct (`src/item.rs`)**

```
Item<T> {
    id:   u64       — unique auto-assigned identifier
    name: String    — human-readable label (always a String, not generic)
    data: T         — the actual item, generic over T
}
```

The `id` and `name` are always concrete types because every inventory item, regardless of domain, needs a numeric ID and a text label. Only `data` is generic — that is the part that varies per use case.

`impl<T: fmt::Display> fmt::Display for Item<T>` — we implement `Display` conditionally: only when `T` also implements `Display`. This allows `println!("{}", item)` to work when appropriate, but does not force the constraint on callers who do not need it.

**`Inventory<T>` struct (`src/inventory.rs`)**

The struct itself requires no bounds. Bounds are pushed to individual methods that need them:

- `add` requires `T: Clone` (because the item is moved into the Vec — actually no clone needed at call site, but we derive `Clone` on Item for flexibility in `remove` which returns an owned `Item<T>`)
- `display_all` requires `T: Display` so we can print items
- `iter_data` requires no bounds — returns an iterator of `&T`

**`remove` method — using `position` + `remove`:**

```rust
pub fn remove(&mut self, id: u64) -> Option<Item<T>> {
    if let Some(pos) = self.items.iter().position(|item| item.id == id) {
        Some(self.items.remove(pos))
    } else {
        None
    }
}
```

- `iter().position(...)` finds the index of the matching element
- `self.items.remove(pos)` removes and returns the element at that index
- Wrapped in `Option` — returns `None` if no item matches

**`search` method — case-insensitive:**

```rust
pub fn search(&self, query: &str) -> Vec<&Item<T>> {
    let query_lower = query.to_lowercase();
    self.items
        .iter()
        .filter(|item| item.name.to_lowercase().contains(&query_lower))
        .collect()
}
```

- Converts both the query and the item name to lowercase before comparing
- Returns references — does not clone or move items
- `collect::<Vec<&Item<T>>>()` — collects iterator of references into a Vec

**`iter_data` — generic iterator:**

```rust
pub fn iter_data(&self) -> impl Iterator<Item = &T> {
    self.items.iter().map(|item| &item.data)
}
```

`impl Iterator<Item = &T>` is a return-type notation meaning "I return some type that implements `Iterator` with element type `&T`." The concrete type is inferred by the compiler — the caller does not need to know it.

This enables callers to write:
```rust
let total: f64 = inventory.iter_data().map(|p| p.price).sum();
```

**`main.rs` — three inventories, one implementation:**

The main function creates:
- `Inventory<Product>` — complex custom struct
- `Inventory<Book>` — different custom struct
- `Inventory<i32>` — primitive type

All three use the exact same `Inventory<T>` code. The compiler generates three separate concrete versions via monomorphization.

---

### Refactoring Suggestions

**1. Add pagination to `list`**

For large inventories, returning all items at once is expensive. Add:

```rust
pub fn list_page(&self, page: usize, page_size: usize) -> &[Item<T>] {
    let start = page * page_size;
    let end = (start + page_size).min(self.items.len());
    if start >= self.items.len() {
        &[]
    } else {
        &self.items[start..end]
    }
}
```

**2. Make search return an iterator instead of a Vec**

```rust
pub fn search_iter<'a>(&'a self, query: &'a str) -> impl Iterator<Item = &'a Item<T>> + 'a {
    let query_lower = query.to_lowercase();
    self.items
        .iter()
        .filter(move |item| item.name.to_lowercase().contains(&query_lower))
}
```

This avoids allocating a `Vec` for the results — the caller decides whether to collect.

**3. Add sorting**

```rust
use std::cmp::Ordering;

impl<T: Clone> Inventory<T> {
    pub fn sort_by_name(&mut self) {
        self.items.sort_by(|a, b| a.name.cmp(&b.name));
    }

    pub fn sort_by_id(&mut self) {
        self.items.sort_by_key(|item| item.id);
    }
}
```

**4. Serialize to JSON (add serde)**

In `Cargo.toml`:
```toml
[dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

Then:
```rust
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item<T> {
    pub id: u64,
    pub name: String,
    pub data: T,
}
```

This allows serializing the entire inventory to JSON, enabling persistence.

**5. Add update method**

```rust
pub fn update<F>(&mut self, id: u64, f: F) -> bool
where
    F: FnOnce(&mut T),
{
    if let Some(item) = self.items.iter_mut().find(|i| i.id == id) {
        f(&mut item.data);
        true
    } else {
        false
    }
}
```

Usage:
```rust
inventory.update(laptop_id, |product| {
    product.quantity += 10;
});
```

---

### Challenge Exercises

**Challenge 1: Implement `Inventory<T>` with filtering**

Add a method:
```rust
pub fn filter<F>(&self, predicate: F) -> Vec<&Item<T>>
where
    F: Fn(&T) -> bool,
```

That returns all items where `predicate(item.data)` returns `true`. Test it:
```rust
// Find all products with price > 500.0
let expensive = products.filter(|p| p.price > 500.0);
```

**Challenge 2: Implement bulk add**

```rust
pub fn add_many(&mut self, items: Vec<(&str, T)>) -> Vec<u64>
```

Adds multiple items at once and returns their assigned IDs.

**Challenge 3: Generic inventory with categories**

Extend the inventory so items have an optional category. Add:
- `add_with_category(&mut self, name: &str, data: T, category: &str) -> u64`
- `list_by_category(&self, category: &str) -> Vec<&Item<T>>`

**Challenge 4: Implement `FromIterator`**

Implement the `FromIterator` trait so you can build an inventory from an iterator:

```rust
let inventory: Inventory<i32> = vec![
    ("Item A", 1),
    ("Item B", 2),
    ("Item C", 3),
].into_iter().collect();
```

**Challenge 5: Thread-safe inventory**

Wrap `Inventory<T>` in `Arc<Mutex<Inventory<T>>>` and write a function that adds items from multiple threads concurrently. Verify the final count is correct.

---

## Chapter Summary

In this chapter you learned:

**What generics are:**
Generics are compile-time type placeholders that let you write code once and use it with many types. They are declared with `<T>` or `<T, U>` syntax after function or type names.

**Why Rust uses generics:**
To eliminate code duplication while preserving both type safety and performance. Rust's monomorphization means generics have zero runtime cost — the compiler generates concrete code for each type used.

**Generic functions:**
Functions can accept and return values of parameterized types. Trait bounds (`T: SomeTrait`) restrict which types are allowed and unlock operations like `>` (requires `PartialOrd`) or `{}` formatting (requires `Display`).

**Generic structs:**
Structs can hold fields of generic types. `impl<T>` blocks on generic structs must re-declare the type parameter. You can specialize methods with `impl StructName<ConcreteType>`.

**Generic enums:**
Enums can have generic variants. Standard library types `Option<T>` and `Result<T, E>` are generic enums. Recursive generic enums require `Box` for indirection because types must be sized.

**Mini project takeaway:**
The `Inventory<T>` library demonstrates a real-world use of generics: a single, well-tested implementation that works across product management, library catalogs, sensor data, and any future domain without modification.

**Key rules to remember:**

| Rule | Details |
|------|---------|
| Declare before use | `impl<T>` must declare `T` before `Point<T>` uses it |
| Bounds enable operations | `T: PartialOrd` enables `>`, `T: Display` enables `{}` |
| Minimize bounds | Only add bounds that methods actually require |
| Box recursive types | Recursive enum/struct fields need `Box` for known size |
| Monomorphization | Generics are zero-cost — specialized at compile time |

**Coming up in Chapter 15:**
Traits — the mechanism behind trait bounds. You will learn to define your own traits, implement them on your types, use them as function parameters, and combine them with generics to write highly reusable, type-safe APIs.
