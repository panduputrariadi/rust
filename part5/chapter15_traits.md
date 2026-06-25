# Chapter 15: Traits

---

## Learning Objectives

By the end of this chapter, you will be able to:

- Explain what a trait is and why Rust uses traits instead of inheritance
- Define your own traits and implement them on custom types
- Use default implementations to provide shared behavior without duplication
- Apply trait bounds to constrain generic functions and structs
- Combine multiple trait bounds using `+` syntax and `where` clauses
- Understand how traits compare to Java interfaces and Python duck typing
- Build a complete Plugin System mini project using trait objects

---

## Theory

### 15.1 What is a Trait

When you write a program, you often want to say:

> "I do not care what exact type this is — I just care that it can do X."

In Python, you simply call the method and hope the type has it. This is called **duck typing**:

```python
# Python — no contract, just hope
def make_sound(animal):
    animal.speak()   # Will crash at runtime if `speak` doesn't exist
```

In Java, you define an interface — a named contract that classes must explicitly sign:

```java
// Java
interface Animal {
    String speak();
}
class Dog implements Animal {
    public String speak() { return "Woof"; }
}
```

Rust takes the Java approach but goes further. In Rust, the contract is called a **trait**.

A trait is a named collection of method signatures (and optionally default implementations) that types can implement. When a type implements a trait, it promises: "I can do everything this trait requires."

**Why does Rust use traits instead of inheritance?**

Many languages use class inheritance to share behavior. A `Dog` extends `Animal`, which means `Dog` gets all of `Animal`'s methods automatically. But inheritance creates tight coupling — the subclass is forever tied to the parent's internal structure.

Rust deliberately has no class inheritance. Instead:

- Types are defined with `struct` or `enum`.
- Behavior is added via `impl` blocks.
- Shared behavior is expressed via traits.

This means a single type can implement many traits from many sources without being locked into a rigid hierarchy. This is called **composition over inheritance**, and it makes code more flexible, testable, and maintainable.

**The core idea:**

> A trait defines *what a type can do*. A struct defines *what a type is*. They are kept separate deliberately.

---

### 15.2 Defining Traits

A trait is defined with the `trait` keyword followed by a name and a block of method signatures.

**Syntax:**

```rust
trait TraitName {
    fn method_name(&self) -> ReturnType;
    fn another_method(&self, param: Type) -> ReturnType;
}
```

The methods inside a trait definition are called **required methods**. Any type that wants to implement this trait must provide a concrete implementation of every required method.

**Example — defining an `Animal` trait:**

```rust
trait Animal {
    fn name(&self) -> &str;
    fn speak(&self) -> String;
    fn legs(&self) -> u32;
}
```

This trait says: "Any type that claims to be an `Animal` must be able to tell me its name, make a sound, and report how many legs it has."

Notice:

- `&self` — the method borrows the value immutably (reads but does not modify).
- `-> &str` — returns a string slice (borrowed text).
- `-> String` — returns an owned String (heap-allocated).
- `-> u32` — returns an unsigned 32-bit integer.

The trait definition contains no implementation — only contracts.

---

### 15.3 Implementing Traits

To make a type satisfy a trait, you use `impl TraitName for TypeName`.

**Full example:**

```rust
trait Animal {
    fn name(&self) -> &str;
    fn speak(&self) -> String;
    fn legs(&self) -> u32;
}

struct Dog {
    name: String,
}

struct Bird {
    name: String,
    can_fly: bool,
}

impl Animal for Dog {
    fn name(&self) -> &str {
        &self.name
    }

    fn speak(&self) -> String {
        String::from("Woof!")
    }

    fn legs(&self) -> u32 {
        4
    }
}

impl Animal for Bird {
    fn name(&self) -> &str {
        &self.name
    }

    fn speak(&self) -> String {
        String::from("Tweet!")
    }

    fn legs(&self) -> u32 {
        2
    }
}

fn main() {
    let dog = Dog { name: String::from("Rex") };
    let bird = Bird { name: String::from("Tweety"), can_fly: true };

    println!("{} says: {}", dog.name(), dog.speak());
    println!("{} says: {}", bird.name(), bird.speak());
    println!("Dog has {} legs.", dog.legs());
    println!("Bird has {} legs.", bird.legs());
}
```

**Output:**
```
Rex says: Woof!
Tweety says: Tweet!
Dog has 4 legs.
Bird has 2 legs.
```

**Key rules for implementing traits:**

1. You must implement every required method — the compiler enforces this.
2. You can implement a trait for any type — your own types or even standard library types (with restrictions).
3. You cannot implement a foreign trait on a foreign type. This is called the **orphan rule**. You can implement `MyTrait` on `Vec<T>`, or implement `Display` on `MyStruct`, but you cannot implement `Display` on `Vec<T>` because both are defined elsewhere. This prevents conflicting implementations.

---

### 15.4 Default Implementations

Sometimes you want a trait to provide a sensible default behavior that types can use without overriding.

You do this by providing a method body inside the trait definition itself.

**Example:**

```rust
trait Animal {
    fn name(&self) -> &str;
    fn speak(&self) -> String;
    fn legs(&self) -> u32;

    // Default implementation — types may override this
    fn description(&self) -> String {
        format!(
            "{} has {} legs and says: {}",
            self.name(),
            self.legs(),
            self.speak()
        )
    }
}

struct Cat {
    name: String,
}

impl Animal for Cat {
    fn name(&self) -> &str {
        &self.name
    }

    fn speak(&self) -> String {
        String::from("Meow!")
    }

    fn legs(&self) -> u32 {
        4
    }

    // We do NOT override `description` — we get the default for free
}

fn main() {
    let cat = Cat { name: String::from("Whiskers") };
    println!("{}", cat.description());
}
```

**Output:**
```
Whiskers has 4 legs and says: Meow!
```

Notice that `description` calls `self.name()`, `self.legs()`, and `self.speak()` — default implementations can call other methods on the same trait, even if those methods are required (not defaulted). This is powerful because it lets you build rich behavior on top of a small required surface area.

**Overriding a default:**

A type can always override a default implementation:

```rust
struct Snake {
    name: String,
}

impl Animal for Snake {
    fn name(&self) -> &str { &self.name }
    fn speak(&self) -> String { String::from("Hiss!") }
    fn legs(&self) -> u32 { 0 }

    // Custom override — snakes deserve special treatment
    fn description(&self) -> String {
        format!("{} is a legless wonder that says: {}", self.name(), self.speak())
    }
}
```

---

### 15.5 Trait Bounds

Now that types can implement traits, you can write functions that accept *any* type — as long as it implements a specific trait.

This is called a **trait bound**.

**Syntax using `impl Trait` (simple, readable):**

```rust
fn print_animal_info(animal: &impl Animal) {
    println!("Animal: {}", animal.name());
    println!("Sound: {}", animal.speak());
    println!("Legs: {}", animal.legs());
}
```

The `impl Animal` syntax means: "Accept any type that implements the `Animal` trait."

**Syntax using generic type parameter with bound (explicit, more flexible):**

```rust
fn print_animal_info<A: Animal>(animal: &A) {
    println!("Animal: {}", animal.name());
    println!("Sound: {}", animal.speak());
    println!("Legs: {}", animal.legs());
}
```

Both are equivalent for simple cases. The generic parameter form (`<A: Animal>`) is needed when you reference the same type multiple times in a signature.

**Example — function that accepts any Animal:**

```rust
trait Animal {
    fn name(&self) -> &str;
    fn speak(&self) -> String;
    fn legs(&self) -> u32;
}

struct Dog { name: String }
struct Cat { name: String }

impl Animal for Dog {
    fn name(&self) -> &str { &self.name }
    fn speak(&self) -> String { String::from("Woof!") }
    fn legs(&self) -> u32 { 4 }
}

impl Animal for Cat {
    fn name(&self) -> &str { &self.name }
    fn speak(&self) -> String { String::from("Meow!") }
    fn legs(&self) -> u32 { 4 }
}

fn introduce(animal: &impl Animal) {
    println!("Hi, I am {} and I say '{}'", animal.name(), animal.speak());
}

fn main() {
    let dog = Dog { name: String::from("Rex") };
    let cat = Cat { name: String::from("Mochi") };

    introduce(&dog);
    introduce(&cat);
}
```

**Output:**
```
Hi, I am Rex and I say 'Woof!'
Hi, I am Mochi and I say 'Meow!'
```

**Trait bounds on return types:**

You can also use `impl Trait` in return position — but with a restriction: you must always return the same concrete type.

```rust
fn make_animal() -> impl Animal {
    Dog { name: String::from("Buddy") }
}
```

This is useful when you do not want to expose the concrete return type in your public API.

---

### 15.6 Multiple Trait Bounds

Sometimes a function needs a type to satisfy more than one trait. Use `+` to combine bounds.

**Example:**

```rust
use std::fmt;

trait Animal {
    fn name(&self) -> &str;
    fn speak(&self) -> String;
    fn legs(&self) -> u32;
}

// Require the type to implement both Animal AND Display
fn describe_and_display<A: Animal + fmt::Display>(animal: &A) {
    println!("Description: {}", animal);
    println!("Sound: {}", animal.speak());
}
```

You can also write this with `impl Trait`:

```rust
fn describe_and_display(animal: &(impl Animal + fmt::Display)) {
    println!("Description: {}", animal);
    println!("Sound: {}", animal.speak());
}
```

**Practical example with Display:**

```rust
use std::fmt;

trait Animal {
    fn name(&self) -> &str;
    fn speak(&self) -> String;
    fn legs(&self) -> u32;
}

struct Dog {
    name: String,
    breed: String,
}

impl Animal for Dog {
    fn name(&self) -> &str { &self.name }
    fn speak(&self) -> String { String::from("Woof!") }
    fn legs(&self) -> u32 { 4 }
}

impl fmt::Display for Dog {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Dog({}, breed: {})", self.name, self.breed)
    }
}

fn describe_and_display<A: Animal + fmt::Display>(animal: &A) {
    println!("Display:  {}", animal);
    println!("Sound:    {}", animal.speak());
    println!("Legs:     {}", animal.legs());
}

fn main() {
    let dog = Dog {
        name: String::from("Rex"),
        breed: String::from("German Shepherd"),
    };
    describe_and_display(&dog);
}
```

**Output:**
```
Display:  Dog(Rex, breed: German Shepherd)
Sound:    Woof!
Legs:     4
```

---

### 15.7 where Clauses

When you have multiple generic parameters with multiple trait bounds each, the function signature becomes unreadable:

```rust
// Hard to read
fn complex_fn<A: Animal + fmt::Display + Clone, B: Animal + fmt::Debug>(
    a: &A,
    b: &B,
) -> String {
    todo!()
}
```

Rust provides `where` clauses to move the bounds below the function signature, making it much cleaner:

```rust
// Clean and readable
fn complex_fn<A, B>(a: &A, b: &B) -> String
where
    A: Animal + fmt::Display + Clone,
    B: Animal + fmt::Debug,
{
    todo!()
}
```

The behavior is identical — `where` is purely for readability.

**When to use `where`:**

- Multiple generic parameters
- Each parameter has two or more bounds
- The signature is longer than ~60 characters with the bounds inline

**Example with where clause in practice:**

```rust
use std::fmt;

trait Describable {
    fn describe(&self) -> String;
}

trait Serializable {
    fn serialize(&self) -> String;
}

struct Report<T>
where
    T: Describable + Serializable + fmt::Debug,
{
    title: String,
    data: T,
}

impl<T> Report<T>
where
    T: Describable + Serializable + fmt::Debug,
{
    fn new(title: &str, data: T) -> Self {
        Report {
            title: String::from(title),
            data,
        }
    }

    fn print(&self) {
        println!("=== {} ===", self.title);
        println!("Description: {}", self.data.describe());
        println!("Serialized:  {}", self.data.serialize());
        println!("Debug:       {:?}", self.data);
    }
}
```

---

### Code Example

The following is a complete, runnable program demonstrating all concepts from this chapter together: trait definition, implementation, default methods, trait bounds, multiple bounds, and `where` clauses.

```rust
use std::fmt;

// ============================================================
// TRAIT DEFINITIONS
// ============================================================

/// Core Animal trait — all animals must implement these
trait Animal {
    fn name(&self) -> &str;
    fn speak(&self) -> String;
    fn legs(&self) -> u32;

    /// Default implementation — calls other trait methods
    fn describe(&self) -> String {
        format!(
            "{} has {} leg(s) and says '{}'",
            self.name(),
            self.legs(),
            self.speak()
        )
    }
}

/// Trait for animals that can perform tricks
trait Trainable {
    fn learn_trick(&mut self, trick: &str);
    fn perform(&self) -> String;
}

// ============================================================
// STRUCT DEFINITIONS
// ============================================================

#[derive(Debug)]
struct Dog {
    name: String,
    breed: String,
    tricks: Vec<String>,
}

#[derive(Debug)]
struct Parrot {
    name: String,
    vocabulary: Vec<String>,
}

#[derive(Debug)]
struct Snake {
    name: String,
    length_cm: u32,
}

// ============================================================
// TRAIT IMPLEMENTATIONS
// ============================================================

impl Animal for Dog {
    fn name(&self) -> &str {
        &self.name
    }

    fn speak(&self) -> String {
        String::from("Woof!")
    }

    fn legs(&self) -> u32 {
        4
    }
    // `describe` uses the default implementation
}

impl Animal for Parrot {
    fn name(&self) -> &str {
        &self.name
    }

    fn speak(&self) -> String {
        if self.vocabulary.is_empty() {
            String::from("Squawk!")
        } else {
            // Parrot repeats the last word it learned
            self.vocabulary.last().unwrap().clone()
        }
    }

    fn legs(&self) -> u32 {
        2
    }

    // Override the default `describe` for parrots
    fn describe(&self) -> String {
        format!(
            "{} is a parrot with a vocabulary of {} word(s)",
            self.name(),
            self.vocabulary.len()
        )
    }
}

impl Animal for Snake {
    fn name(&self) -> &str {
        &self.name
    }

    fn speak(&self) -> String {
        String::from("Hiss!")
    }

    fn legs(&self) -> u32 {
        0
    }

    fn describe(&self) -> String {
        format!(
            "{} is a {}-cm snake with no legs",
            self.name(),
            self.length_cm
        )
    }
}

// ============================================================
// TRAINABLE IMPLEMENTATIONS
// ============================================================

impl Trainable for Dog {
    fn learn_trick(&mut self, trick: &str) {
        self.tricks.push(String::from(trick));
        println!("{} learned '{}'!", self.name, trick);
    }

    fn perform(&self) -> String {
        if self.tricks.is_empty() {
            format!("{} doesn't know any tricks yet.", self.name)
        } else {
            format!("{} performs: {}", self.name, self.tricks.join(", "))
        }
    }
}

impl Trainable for Parrot {
    fn learn_trick(&mut self, trick: &str) {
        self.vocabulary.push(String::from(trick));
        println!("{} learned to say '{}'!", self.name, trick);
    }

    fn perform(&self) -> String {
        if self.vocabulary.is_empty() {
            format!("{} has nothing to say.", self.name)
        } else {
            format!("{} says: {}", self.name, self.vocabulary.join(", "))
        }
    }
}

// ============================================================
// Display implementations
// ============================================================

impl fmt::Display for Dog {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Dog '{}' ({})", self.name, self.breed)
    }
}

impl fmt::Display for Parrot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Parrot '{}'", self.name)
    }
}

// ============================================================
// GENERIC FUNCTIONS WITH TRAIT BOUNDS
// ============================================================

/// Accepts any Animal — single bound
fn print_animal(animal: &impl Animal) {
    println!("  {}", animal.describe());
}

/// Accepts any Animal — generic parameter form
fn greet_animal<A: Animal>(animal: &A) {
    println!("  Meet {}: {}", animal.name(), animal.speak());
}

/// Multiple bounds: must implement both Animal and Display
fn show_animal<A>(animal: &A)
where
    A: Animal + fmt::Display,
{
    println!("  Display: {}", animal);
    println!("  Describe: {}", animal.describe());
}

/// Multiple bounds: must implement both Animal and Trainable
fn train_and_show<A>(animal: &mut A, tricks: &[&str])
where
    A: Animal + Trainable,
{
    println!("  Training {}...", animal.name());
    for trick in tricks {
        animal.learn_trick(trick);
    }
    println!("  {}", animal.perform());
}

// ============================================================
// MAIN
// ============================================================

fn main() {
    println!("=== Animal Trait Demo ===\n");

    let mut dog = Dog {
        name: String::from("Rex"),
        breed: String::from("German Shepherd"),
        tricks: Vec::new(),
    };

    let mut parrot = Parrot {
        name: String::from("Polly"),
        vocabulary: Vec::new(),
    };

    let snake = Snake {
        name: String::from("Slinky"),
        length_cm: 120,
    };

    println!("--- Basic Animal Info ---");
    print_animal(&dog);
    print_animal(&parrot);
    print_animal(&snake);

    println!("\n--- Greeting Animals ---");
    greet_animal(&dog);
    greet_animal(&parrot);
    greet_animal(&snake);

    println!("\n--- Animals with Display ---");
    show_animal(&dog);
    show_animal(&parrot);

    println!("\n--- Training Session ---");
    train_and_show(&mut dog, &["sit", "shake", "roll over"]);
    train_and_show(&mut parrot, &["Hello!", "Pretty bird!", "Polly wants a cracker"]);
}
```

**Output:**
```
=== Animal Trait Demo ===

--- Basic Animal Info ---
  Rex has 4 leg(s) and says 'Woof!'
  Polly is a parrot with a vocabulary of 0 word(s)
  Slinky is a 120-cm snake with no legs

--- Greeting Animals ---
  Meet Rex: Woof!
  Meet Polly: Squawk!
  Meet Slinky: Hiss!

--- Animals with Display ---
  Display: Dog 'Rex' (German Shepherd)
  Describe: Rex has 4 leg(s) and says 'Woof!'
  Display: Parrot 'Polly'
  Describe: Polly is a parrot with a vocabulary of 0 word(s)

--- Training Session ---
  Training Rex...
Rex learned 'sit'!
Rex learned 'shake'!
Rex learned 'roll over'!
  Rex performs: sit, shake, roll over
  Training Polly...
Polly learned to say 'Hello!'!
Polly learned to say 'Pretty bird!'!
Polly learned to say 'Polly wants a cracker'!
  Polly says: Hello!, Pretty bird!, Polly wants a cracker
```

---

### Line-by-Line Explanation

```rust
trait Animal {
```
Defines a new trait named `Animal`. Everything inside the braces is the trait's contract.

```rust
    fn name(&self) -> &str;
```
A required method. `&self` borrows the implementing type immutably. `&str` returns a string slice — the caller borrows text from the struct's own `String` field. No body means implementing types must provide one.

```rust
    fn describe(&self) -> String {
        format!("{} has {} leg(s)...", self.name(), self.legs(), self.speak())
    }
```
A default method. Notice it calls `self.name()`, `self.legs()`, and `self.speak()` — other methods on the same trait. This is legal because any type using this default must also implement those methods.

```rust
impl Animal for Dog {
```
Starts the implementation block. This tells the compiler: "Dog satisfies the Animal contract."

```rust
    fn name(&self) -> &str {
        &self.name
    }
```
Returns a reference (`&str`) into `self.name` (a `String`). This works because `String` implements `Deref<Target = str>`, so `&self.name` coerces to `&str`. The borrowed slice lives as long as the `Dog` lives.

```rust
impl Trainable for Dog {
    fn learn_trick(&mut self, trick: &str) {
        self.tricks.push(String::from(trick));
```
`&mut self` borrows the `Dog` mutably, allowing us to modify `self.tricks`. `String::from(trick)` converts the borrowed `&str` into an owned `String` before pushing it into the `Vec`.

```rust
fn print_animal(animal: &impl Animal) {
```
`impl Animal` in parameter position means "accept a reference to any type that implements `Animal`." This is syntactic sugar for a hidden generic type parameter with a trait bound.

```rust
fn show_animal<A>(animal: &A)
where
    A: Animal + fmt::Display,
{
```
`where` clause form. `A` must implement both `Animal` and `fmt::Display`. The `+` combines bounds. Inside the function, we can call both `animal.describe()` (from `Animal`) and `println!("{}", animal)` (from `Display`).

```rust
fn train_and_show<A>(animal: &mut A, tricks: &[&str])
where
    A: Animal + Trainable,
{
```
`&mut A` — mutable reference to the generic type. Required because `learn_trick` takes `&mut self`. `&[&str]` — a slice of string slices, borrowed and read-only.

---

### Common Mistakes

**Mistake 1 — Forgetting to implement all required methods**

```rust
trait Animal {
    fn name(&self) -> &str;
    fn speak(&self) -> String;
    fn legs(&self) -> u32;
}

struct Fish { name: String }

impl Animal for Fish {
    fn name(&self) -> &str { &self.name }
    // ERROR: missing `speak` and `legs`
}
```

The compiler will refuse with:
```
error[E0046]: not all trait items implemented, missing: `speak`, `legs`
```

Fix: implement every required method.

---

**Mistake 2 — Trying to implement a foreign trait on a foreign type**

```rust
use std::fmt;

// ERROR: both Display and Vec are from the standard library
impl fmt::Display for Vec<i32> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
```

Error:
```
error[E0117]: only traits defined in the current crate can be implemented for types defined outside of the crate
```

Fix: either define your own trait, or wrap the foreign type in a newtype.

---

**Mistake 3 — Using `impl Trait` when you need the same type in multiple positions**

```rust
// This compiles, but a and b can be DIFFERENT types
fn combine(a: impl Animal, b: impl Animal) { ... }

// This enforces a and b are the SAME type
fn combine<A: Animal>(a: A, b: A) { ... }
```

If you want two parameters to be the same concrete type, use the generic parameter form.

---

**Mistake 4 — Calling a trait method without the trait in scope**

```rust
mod animals {
    pub trait Animal {
        fn speak(&self) -> String;
    }
    pub struct Dog;
    impl Animal for Dog {
        fn speak(&self) -> String { String::from("Woof!") }
    }
}

fn main() {
    let dog = animals::Dog;
    // ERROR: method `speak` not found
    println!("{}", dog.speak());
}
```

Fix: bring the trait into scope with `use`:

```rust
use animals::Animal;

fn main() {
    let dog = animals::Dog;
    println!("{}", dog.speak()); // now works
}
```

---

**Mistake 5 — Confusing `impl Trait` in parameter vs return position**

In parameter position, `impl Trait` accepts any type that implements the trait.

In return position, `impl Trait` means you return *one specific concrete type*, chosen by you, but you hide that type from the caller. You cannot return different concrete types from different branches:

```rust
// ERROR: `if` and `else` return different types
fn make_animal(is_dog: bool) -> impl Animal {
    if is_dog {
        Dog { name: String::from("Rex") }
    } else {
        Cat { name: String::from("Mochi") } // different type!
    }
}
```

Fix: use a trait object (`Box<dyn Animal>`) for runtime polymorphism when you need to return different types.

---

### Best Practices

1. **Keep traits small and focused.** A trait should represent one coherent capability. Prefer five single-method traits over one five-method trait. This is the Interface Segregation Principle.

2. **Provide default implementations for derived behavior.** If a method's behavior can be fully expressed in terms of other trait methods, make it a default. Implementing types get it for free.

3. **Use `where` clauses when bounds grow past one line.** Readability matters more than brevity.

4. **Prefer `impl Trait` for simple cases, generic parameters when you need to refer to the type more than once.**

5. **Name traits after capabilities, not categories.** `Serializable`, `Printable`, `Drawable` are better names than `Serializer`, `Printer`, `Drawer` — because the trait describes what the *type* can do, not what it is.

6. **Understand when to use trait objects.** `&dyn Trait` and `Box<dyn Trait>` enable runtime polymorphism at the cost of a small indirection. Use them when you need to store a collection of mixed types or when the type is unknown at compile time.

---

## Practice: Animal Trait

This practice section builds a complete animal system from scratch, progressively adding traits and implementations.

```rust
use std::fmt;

// ============================================================
// TRAITS
// ============================================================

trait Animal {
    fn name(&self) -> &str;
    fn speak(&self) -> String;
    fn legs(&self) -> u32;
    fn is_domestic(&self) -> bool;

    fn summary(&self) -> String {
        format!(
            "[{}] legs={}, domestic={}, sound='{}'",
            self.name(),
            self.legs(),
            self.is_domestic(),
            self.speak()
        )
    }
}

trait Movable {
    fn move_type(&self) -> &str;
    fn speed_kmh(&self) -> f64;

    fn speed_description(&self) -> String {
        format!("{} moves by {} at {:.1} km/h", "", self.move_type(), self.speed_kmh())
    }
}

trait Feedable {
    fn diet(&self) -> &str;
    fn feed(&self) -> String {
        format!("Feeding {} some {}", "animal", self.diet())
    }
}

// ============================================================
// TYPES
// ============================================================

#[derive(Debug)]
struct Dog {
    name: String,
    breed: String,
}

#[derive(Debug)]
struct Eagle {
    name: String,
    wingspan_cm: u32,
}

#[derive(Debug)]
struct Dolphin {
    name: String,
    pod_size: u32,
}

// ============================================================
// ANIMAL IMPLEMENTATIONS
// ============================================================

impl Animal for Dog {
    fn name(&self) -> &str { &self.name }
    fn speak(&self) -> String { String::from("Woof!") }
    fn legs(&self) -> u32 { 4 }
    fn is_domestic(&self) -> bool { true }
}

impl Animal for Eagle {
    fn name(&self) -> &str { &self.name }
    fn speak(&self) -> String { String::from("Screech!") }
    fn legs(&self) -> u32 { 2 }
    fn is_domestic(&self) -> bool { false }

    fn summary(&self) -> String {
        format!(
            "[Eagle: {}] wingspan={}cm, sound='{}'",
            self.name(), self.wingspan_cm, self.speak()
        )
    }
}

impl Animal for Dolphin {
    fn name(&self) -> &str { &self.name }
    fn speak(&self) -> String { String::from("Click!") }
    fn legs(&self) -> u32 { 0 }
    fn is_domestic(&self) -> bool { false }
}

// ============================================================
// MOVABLE IMPLEMENTATIONS
// ============================================================

impl Movable for Dog {
    fn move_type(&self) -> &str { "running" }
    fn speed_kmh(&self) -> f64 { 48.0 }
}

impl Movable for Eagle {
    fn move_type(&self) -> &str { "flying" }
    fn speed_kmh(&self) -> f64 { 120.0 }
}

impl Movable for Dolphin {
    fn move_type(&self) -> &str { "swimming" }
    fn speed_kmh(&self) -> f64 { 40.0 }
}

// ============================================================
// FEEDABLE IMPLEMENTATIONS
// ============================================================

impl Feedable for Dog {
    fn diet(&self) -> &str { "kibble" }
}

impl Feedable for Eagle {
    fn diet(&self) -> &str { "fish and small mammals" }
}

impl Feedable for Dolphin {
    fn diet(&self) -> &str { "fish and squid" }
    fn feed(&self) -> String {
        format!("{} catches {} in the wild", self.name, self.diet())
    }
}

// ============================================================
// Display
// ============================================================

impl fmt::Display for Dog {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Dog({}, {})", self.name, self.breed)
    }
}

impl fmt::Display for Eagle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Eagle({})", self.name)
    }
}

impl fmt::Display for Dolphin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Dolphin({}, pod of {})", self.name, self.pod_size)
    }
}

// ============================================================
// GENERIC FUNCTIONS
// ============================================================

fn print_full_profile<A>(animal: &A)
where
    A: Animal + Movable + Feedable + fmt::Display,
{
    println!("  Display:  {}", animal);
    println!("  Summary:  {}", animal.summary());
    println!("  Movement: {} at {:.1} km/h", animal.move_type(), animal.speed_kmh());
    println!("  Diet:     {}", animal.diet());
    println!("  Feed:     {}", animal.feed());
}

// ============================================================
// MAIN
// ============================================================

fn main() {
    let dog = Dog { name: String::from("Max"), breed: String::from("Labrador") };
    let eagle = Eagle { name: String::from("Aria"), wingspan_cm: 210 };
    let dolphin = Dolphin { name: String::from("Finn"), pod_size: 8 };

    println!("=== Animal Profiles ===\n");

    println!("--- Dog ---");
    print_full_profile(&dog);

    println!("\n--- Eagle ---");
    print_full_profile(&eagle);

    println!("\n--- Dolphin ---");
    print_full_profile(&dolphin);
}
```

**Output:**
```
=== Animal Profiles ===

--- Dog ---
  Display:  Dog(Max, Labrador)
  Summary:  [Max] legs=4, domestic=true, sound='Woof!'
  Movement: running at 48.0 km/h
  Diet:     kibble
  Feed:     Feeding animal some kibble

--- Eagle ---
  Display:  Eagle(Aria)
  Summary:  [Eagle: Aria] wingspan=210cm, sound='Screech!'
  Movement: flying at 120.0 km/h
  Diet:     fish and small mammals
  Feed:     Feeding animal some fish and small mammals

--- Dolphin ---
  Display:  Dolphin(Finn, pod of 8)
  Summary:  [Finn] legs=0, domestic=false, sound='Click!'
  Movement: swimming at 40.0 km/h
  Diet:     fish and squid
  Feed:     Finn catches fish and squid in the wild
```

---

## Practice: Vehicle Trait

This practice builds a vehicle hierarchy with multiple traits, demonstrating how traits compose cleanly.

```rust
use std::fmt;

// ============================================================
// TRAITS
// ============================================================

trait Vehicle {
    fn make(&self) -> &str;
    fn model(&self) -> &str;
    fn year(&self) -> u32;
    fn fuel_type(&self) -> &str;

    fn full_name(&self) -> String {
        format!("{} {} {} ({})", self.year(), self.make(), self.model(), self.fuel_type())
    }
}

trait Drivable {
    fn start_engine(&self) -> String;
    fn stop_engine(&self) -> String;
    fn current_speed_kmh(&self) -> f64;

    fn status(&self) -> String {
        format!(
            "Engine: {} | Speed: {:.1} km/h",
            self.start_engine(),
            self.current_speed_kmh()
        )
    }
}

trait Chargeable {
    fn battery_percent(&self) -> u32;
    fn range_km(&self) -> u32;

    fn charge_status(&self) -> String {
        format!(
            "Battery: {}% | Range: {} km",
            self.battery_percent(),
            self.range_km()
        )
    }
}

// ============================================================
// TYPES
// ============================================================

#[derive(Debug)]
struct GasCar {
    make: String,
    model: String,
    year: u32,
    speed_kmh: f64,
}

#[derive(Debug)]
struct ElectricCar {
    make: String,
    model: String,
    year: u32,
    battery_percent: u32,
    range_km: u32,
    speed_kmh: f64,
}

#[derive(Debug)]
struct Bicycle {
    brand: String,
    model: String,
    year: u32,
    speed_kmh: f64,
}

// ============================================================
// VEHICLE IMPLEMENTATIONS
// ============================================================

impl Vehicle for GasCar {
    fn make(&self) -> &str { &self.make }
    fn model(&self) -> &str { &self.model }
    fn year(&self) -> u32 { self.year }
    fn fuel_type(&self) -> &str { "Gasoline" }
}

impl Vehicle for ElectricCar {
    fn make(&self) -> &str { &self.make }
    fn model(&self) -> &str { &self.model }
    fn year(&self) -> u32 { self.year }
    fn fuel_type(&self) -> &str { "Electric" }
}

impl Vehicle for Bicycle {
    fn make(&self) -> &str { &self.brand }
    fn model(&self) -> &str { &self.model }
    fn year(&self) -> u32 { self.year }
    fn fuel_type(&self) -> &str { "Human-powered" }
}

// ============================================================
// DRIVABLE IMPLEMENTATIONS
// ============================================================

impl Drivable for GasCar {
    fn start_engine(&self) -> String { String::from("Vroom!") }
    fn stop_engine(&self) -> String { String::from("Engine off.") }
    fn current_speed_kmh(&self) -> f64 { self.speed_kmh }
}

impl Drivable for ElectricCar {
    fn start_engine(&self) -> String { String::from("Whirr... (silent start)") }
    fn stop_engine(&self) -> String { String::from("Power off.") }
    fn current_speed_kmh(&self) -> f64 { self.speed_kmh }
}

impl Drivable for Bicycle {
    fn start_engine(&self) -> String { String::from("Pedaling...") }
    fn stop_engine(&self) -> String { String::from("Coasting to stop.") }
    fn current_speed_kmh(&self) -> f64 { self.speed_kmh }
}

// ============================================================
// CHARGEABLE (only for ElectricCar)
// ============================================================

impl Chargeable for ElectricCar {
    fn battery_percent(&self) -> u32 { self.battery_percent }
    fn range_km(&self) -> u32 { self.range_km }
}

// ============================================================
// Display
// ============================================================

impl fmt::Display for GasCar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "GasCar({})", self.full_name())
    }
}

impl fmt::Display for ElectricCar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "EV({})", self.full_name())
    }
}

impl fmt::Display for Bicycle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Bike({})", self.full_name())
    }
}

// ============================================================
// GENERIC FUNCTIONS
// ============================================================

/// Print info for any vehicle that can be driven
fn print_vehicle_status<V>(vehicle: &V)
where
    V: Vehicle + Drivable + fmt::Display,
{
    println!("  Vehicle:  {}", vehicle);
    println!("  Name:     {}", vehicle.full_name());
    println!("  Status:   {}", vehicle.status());
}

/// Print charge status only for chargeable vehicles
fn print_charge_info<V>(vehicle: &V)
where
    V: Vehicle + Chargeable,
{
    println!("  Charge:   {}", vehicle.charge_status());
}

// ============================================================
// MAIN
// ============================================================

fn main() {
    let gas_car = GasCar {
        make: String::from("Toyota"),
        model: String::from("Camry"),
        year: 2022,
        speed_kmh: 60.0,
    };

    let ev = ElectricCar {
        make: String::from("Tesla"),
        model: String::from("Model 3"),
        year: 2023,
        battery_percent: 78,
        range_km: 390,
        speed_kmh: 100.0,
    };

    let bike = Bicycle {
        brand: String::from("Trek"),
        model: String::from("Domane"),
        year: 2021,
        speed_kmh: 25.0,
    };

    println!("=== Vehicle Status Report ===\n");

    println!("--- Gas Car ---");
    print_vehicle_status(&gas_car);

    println!("\n--- Electric Car ---");
    print_vehicle_status(&ev);
    print_charge_info(&ev);

    println!("\n--- Bicycle ---");
    print_vehicle_status(&bike);
}
```

**Output:**
```
=== Vehicle Status Report ===

--- Gas Car ---
  Vehicle:  GasCar(2022 Toyota Camry (Gasoline))
  Name:     2022 Toyota Camry (Gasoline)
  Status:   Engine: Vroom! | Speed: 60.0 km/h

--- Electric Car ---
  Vehicle:  EV(2023 Tesla Model 3 (Electric))
  Name:     2023 Tesla Model 3 (Electric)
  Status:   Engine: Whirr... (silent start) | Speed: 100.0 km/h
  Charge:   Battery: 78% | Range: 390 km

--- Bicycle ---
  Vehicle:  Bike(2021 Trek Domane (Human-powered))
  Name:     2021 Trek Domane (Human-powered)
  Status:   Engine: Pedaling... | Speed: 25.0 km/h
```

---

## Mini Project: Plugin System

### Project Overview

A **plugin system** allows an application to load and execute behavior at runtime without hardcoding every possible implementation. Think of browser extensions, VSCode plugins, or Webpack loaders.

In Rust, traits are the perfect mechanism for defining a plugin contract: every plugin must satisfy the trait, but each plugin decides how to implement the behavior. The host application only knows about the trait — never about the concrete types.

In this project, we build:

- A `Plugin` trait that all plugins must implement
- A `LoggerPlugin` that records events to stdout
- A `MetricsPlugin` that counts and reports execution statistics
- A `PluginRunner` that manages and executes a collection of plugins at runtime using trait objects

### Functional Requirements

1. The `Plugin` trait defines a contract with `name()`, `version()`, `on_start()`, `on_stop()`, and `execute(event: &str)`.
2. `LoggerPlugin` writes timestamped log entries for every event.
3. `MetricsPlugin` counts events per category and reports totals.
4. The `PluginRunner` stores a `Vec<Box<dyn Plugin>>` — a heterogeneous collection of trait objects.
5. The runner fires lifecycle events: start all plugins, run events, stop all plugins.
6. The system is extensible — adding a new plugin requires zero changes to existing code.

### Why `Box<dyn Plugin>`?

When you store plugins in a `Vec`, Rust needs to know the size of each element at compile time. But different plugins (`LoggerPlugin`, `MetricsPlugin`) have different sizes. 

`Box<dyn Plugin>` solves this:

- `Box<T>` stores the data on the heap and gives you a fixed-size pointer (8 bytes on 64-bit systems).
- `dyn Plugin` means "some type that implements `Plugin`" — resolved at runtime via a **vtable** (a table of function pointers).

This is called **dynamic dispatch** (runtime polymorphism), as opposed to **static dispatch** (compile-time monomorphization from generic bounds).

```
Memory layout:

Vec<Box<dyn Plugin>>
┌──────────────────────────────────────────────────────────┐
│  Box ptr ──► [LoggerPlugin data on heap]                 │
│  Box ptr ──► [MetricsPlugin data on heap]                │
│  Box ptr ──► [AuditPlugin data on heap]  (future plugin) │
└──────────────────────────────────────────────────────────┘
         ▲
         Each Box is a (data_ptr, vtable_ptr) fat pointer
```

### Project Structure

```
plugin_system/
├── Cargo.toml
└── src/
    ├── main.rs
    ├── plugin.rs          ← Plugin trait definition
    ├── logger_plugin.rs   ← LoggerPlugin implementation
    ├── metrics_plugin.rs  ← MetricsPlugin implementation
    └── runner.rs          ← PluginRunner
```

### Step-by-Step Development

**Step 1 — Create the project**

```bash
cargo new plugin_system
cd plugin_system
```

**Step 2 — Define the Plugin trait**

**`src/plugin.rs`**

```rust
/// The Plugin trait defines the contract that every plugin must satisfy.
///
/// Lifecycle:
///   on_start() → execute() × N → on_stop()
pub trait Plugin {
    /// Returns the plugin's name (used for logging and identification)
    fn name(&self) -> &str;

    /// Returns the plugin's version string
    fn version(&self) -> &str;

    /// Called once when the plugin system starts up.
    /// Use for initialization: opening files, allocating resources, etc.
    fn on_start(&mut self);

    /// Called once when the plugin system shuts down.
    /// Use for cleanup: flushing buffers, closing connections, etc.
    fn on_stop(&mut self);

    /// Called for each event that the plugin should process.
    /// `event` is a string describing what happened (e.g., "user.login").
    fn execute(&mut self, event: &str);

    /// Returns a human-readable status report for this plugin.
    /// Default implementation — plugins may override.
    fn report(&self) -> String {
        format!("Plugin '{}' v{} — no report available", self.name(), self.version())
    }
}
```

**Step 3 — Implement LoggerPlugin**

**`src/logger_plugin.rs`**

```rust
use crate::plugin::Plugin;

/// LoggerPlugin records every event to stdout with a counter.
///
/// This simulates a real logger that would write to a file or logging service.
pub struct LoggerPlugin {
    name: String,
    version: String,
    event_count: u64,
    running: bool,
}

impl LoggerPlugin {
    /// Constructor — creates a new LoggerPlugin with the given name
    pub fn new(name: &str) -> Self {
        LoggerPlugin {
            name: String::from(name),
            version: String::from("1.0.0"),
            event_count: 0,
            running: false,
        }
    }

    /// Internal helper — formats a log line with prefix
    fn log(&self, message: &str) {
        println!("[LOG][{}] {}", self.name, message);
    }
}

impl Plugin for LoggerPlugin {
    fn name(&self) -> &str {
        &self.name
    }

    fn version(&self) -> &str {
        &self.version
    }

    fn on_start(&mut self) {
        self.running = true;
        self.log("Logger started. Ready to capture events.");
    }

    fn on_stop(&mut self) {
        self.running = false;
        self.log(&format!(
            "Logger stopped. Total events captured: {}",
            self.event_count
        ));
    }

    fn execute(&mut self, event: &str) {
        if !self.running {
            self.log("WARNING: execute called before on_start");
            return;
        }
        self.event_count += 1;
        self.log(&format!("Event #{}: '{}'", self.event_count, event));
    }

    fn report(&self) -> String {
        format!(
            "LoggerPlugin '{}' v{} — captured {} event(s), running={}",
            self.name, self.version, self.event_count, self.running
        )
    }
}
```

**Step 4 — Implement MetricsPlugin**

**`src/metrics_plugin.rs`**

```rust
use crate::plugin::Plugin;
use std::collections::HashMap;

/// MetricsPlugin counts events grouped by their category prefix.
///
/// Event format: "category.action" (e.g., "user.login", "user.logout", "order.placed")
/// The plugin extracts the category (before the first dot) and counts occurrences.
pub struct MetricsPlugin {
    name: String,
    version: String,
    counters: HashMap<String, u64>,
    total_events: u64,
    running: bool,
}

impl MetricsPlugin {
    /// Constructor
    pub fn new(name: &str) -> Self {
        MetricsPlugin {
            name: String::from(name),
            version: String::from("2.1.0"),
            counters: HashMap::new(),
            total_events: 0,
            running: false,
        }
    }

    /// Extracts the category prefix from an event string.
    /// "user.login" → "user"
    /// "order.placed" → "order"
    /// "ping" → "ping" (no dot → entire string is the category)
    fn extract_category(event: &str) -> &str {
        match event.find('.') {
            Some(index) => &event[..index],
            None => event,
        }
    }
}

impl Plugin for MetricsPlugin {
    fn name(&self) -> &str {
        &self.name
    }

    fn version(&self) -> &str {
        &self.version
    }

    fn on_start(&mut self) {
        self.running = true;
        println!("[METRICS][{}] Metrics collection started.", self.name);
    }

    fn on_stop(&mut self) {
        self.running = false;
        println!(
            "[METRICS][{}] Metrics collection stopped. Total events: {}",
            self.name, self.total_events
        );
    }

    fn execute(&mut self, event: &str) {
        if !self.running {
            println!("[METRICS][{}] WARNING: Not running.", self.name);
            return;
        }

        self.total_events += 1;

        let category = Self::extract_category(event);
        // `entry` API: get the counter for this category, insert 0 if missing, then add 1
        let count = self.counters.entry(String::from(category)).or_insert(0);
        *count += 1;

        println!(
            "[METRICS][{}] Counted event '{}' (category: '{}', count: {})",
            self.name, event, category, count
        );
    }

    fn report(&self) -> String {
        let mut lines = Vec::new();
        lines.push(format!(
            "MetricsPlugin '{}' v{} — {} total event(s):",
            self.name, self.version, self.total_events
        ));

        // Sort categories alphabetically for consistent output
        let mut categories: Vec<(&String, &u64)> = self.counters.iter().collect();
        categories.sort_by_key(|(k, _)| k.as_str());

        for (category, count) in &categories {
            lines.push(format!("  {} → {} event(s)", category, count));
        }

        if categories.is_empty() {
            lines.push(String::from("  (no events recorded)"));
        }

        lines.join("\n")
    }
}
```

**Step 5 — Implement PluginRunner**

**`src/runner.rs`**

```rust
use crate::plugin::Plugin;

/// PluginRunner manages a collection of plugins and drives their lifecycle.
///
/// It stores plugins as `Box<dyn Plugin>` — boxed trait objects — so it can
/// hold plugins of different concrete types in the same Vec.
pub struct PluginRunner {
    plugins: Vec<Box<dyn Plugin>>,
    started: bool,
}

impl PluginRunner {
    /// Creates a new, empty PluginRunner
    pub fn new() -> Self {
        PluginRunner {
            plugins: Vec::new(),
            started: false,
        }
    }

    /// Registers a plugin with the runner.
    ///
    /// The `Box<dyn Plugin>` takes ownership of the plugin.
    /// This is how we store different plugin types in the same Vec.
    pub fn register(&mut self, plugin: Box<dyn Plugin>) {
        println!(
            "[RUNNER] Registered plugin: '{}' v{}",
            plugin.name(),
            plugin.version()
        );
        self.plugins.push(plugin);
    }

    /// Starts all registered plugins by calling `on_start` on each.
    pub fn start_all(&mut self) {
        println!("\n[RUNNER] === Starting {} plugin(s) ===", self.plugins.len());
        for plugin in self.plugins.iter_mut() {
            plugin.on_start();
        }
        self.started = true;
        println!("[RUNNER] All plugins started.\n");
    }

    /// Dispatches an event to all registered plugins.
    /// Each plugin receives the same event string and processes it independently.
    pub fn dispatch(&mut self, event: &str) {
        println!("[RUNNER] Dispatching event: '{}'", event);
        for plugin in self.plugins.iter_mut() {
            plugin.execute(event);
        }
        println!();
    }

    /// Stops all registered plugins by calling `on_stop` on each.
    pub fn stop_all(&mut self) {
        println!("[RUNNER] === Stopping {} plugin(s) ===", self.plugins.len());
        for plugin in self.plugins.iter_mut() {
            plugin.on_stop();
        }
        self.started = false;
        println!("[RUNNER] All plugins stopped.\n");
    }

    /// Prints the report from every plugin.
    pub fn print_reports(&self) {
        println!("[RUNNER] === Plugin Reports ===");
        for plugin in self.plugins.iter() {
            println!("---");
            println!("{}", plugin.report());
        }
        println!("---");
    }

    /// Returns how many plugins are registered
    pub fn plugin_count(&self) -> usize {
        self.plugins.len()
    }

    /// Returns whether the runner has been started
    pub fn is_running(&self) -> bool {
        self.started
    }
}

impl Default for PluginRunner {
    fn default() -> Self {
        Self::new()
    }
}
```

**Step 6 — Wire everything together in main**

**`src/main.rs`**

```rust
mod plugin;
mod logger_plugin;
mod metrics_plugin;
mod runner;

use logger_plugin::LoggerPlugin;
use metrics_plugin::MetricsPlugin;
use runner::PluginRunner;

fn main() {
    println!("╔══════════════════════════════════════╗");
    println!("║       Rust Plugin System Demo        ║");
    println!("╚══════════════════════════════════════╝\n");

    // Create the runner
    let mut runner = PluginRunner::new();

    // Register plugins — each is boxed to create a trait object
    runner.register(Box::new(LoggerPlugin::new("AppLogger")));
    runner.register(Box::new(MetricsPlugin::new("AppMetrics")));

    println!("[MAIN] {} plugin(s) registered.\n", runner.plugin_count());

    // Start all plugins
    runner.start_all();

    // Simulate application events
    let events = [
        "user.login",
        "user.view_dashboard",
        "order.placed",
        "user.add_to_cart",
        "order.placed",
        "payment.processed",
        "user.logout",
        "order.placed",
        "user.login",
        "payment.failed",
    ];

    println!("[MAIN] === Dispatching {} event(s) ===\n", events.len());
    for event in &events {
        runner.dispatch(event);
    }

    // Stop all plugins
    runner.stop_all();

    // Print final reports
    runner.print_reports();

    println!("\n[MAIN] Plugin system shutdown complete.");
}
```

**`Cargo.toml`**

```toml
[package]
name = "plugin_system"
version = "0.1.0"
edition = "2021"

[dependencies]
```

### Complete Output

```
╔══════════════════════════════════════╗
║       Rust Plugin System Demo        ║
╚══════════════════════════════════════╝

[RUNNER] Registered plugin: 'AppLogger' v1.0.0
[RUNNER] Registered plugin: 'AppMetrics' v2.1.0
[MAIN] 2 plugin(s) registered.

[RUNNER] === Starting 2 plugin(s) ===
[LOG][AppLogger] Logger started. Ready to capture events.
[METRICS][AppMetrics] Metrics collection started.
[RUNNER] All plugins started.

[MAIN] === Dispatching 10 event(s) ===

[RUNNER] Dispatching event: 'user.login'
[LOG][AppLogger] Event #1: 'user.login'
[METRICS][AppMetrics] Counted event 'user.login' (category: 'user', count: 1)

[RUNNER] Dispatching event: 'user.view_dashboard'
[LOG][AppLogger] Event #2: 'user.view_dashboard'
[METRICS][AppMetrics] Counted event 'user.view_dashboard' (category: 'user', count: 2)

[RUNNER] Dispatching event: 'order.placed'
[LOG][AppLogger] Event #3: 'order.placed'
[METRICS][AppMetrics] Counted event 'order.placed' (category: 'order', count: 1)

[RUNNER] Dispatching event: 'user.add_to_cart'
[LOG][AppLogger] Event #4: 'user.add_to_cart'
[METRICS][AppMetrics] Counted event 'user.add_to_cart' (category: 'user', count: 3)

[RUNNER] Dispatching event: 'order.placed'
[LOG][AppLogger] Event #5: 'order.placed'
[METRICS][AppMetrics] Counted event 'order.placed' (category: 'order', count: 2)

[RUNNER] Dispatching event: 'payment.processed'
[LOG][AppLogger] Event #6: 'payment.processed'
[METRICS][AppMetrics] Counted event 'payment.processed' (category: 'payment', count: 1)

[RUNNER] Dispatching event: 'user.logout'
[LOG][AppLogger] Event #7: 'user.logout'
[METRICS][AppMetrics] Counted event 'user.logout' (category: 'user', count: 4)

[RUNNER] Dispatching event: 'order.placed'
[LOG][AppLogger] Event #8: 'order.placed'
[METRICS][AppMetrics] Counted event 'order.placed' (category: 'order', count: 3)

[RUNNER] Dispatching event: 'user.login'
[LOG][AppLogger] Event #9: 'user.login'
[METRICS][AppMetrics] Counted event 'user.login' (category: 'user', count: 5)

[RUNNER] Dispatching event: 'payment.failed'
[LOG][AppLogger] Event #10: 'payment.failed'
[METRICS][AppMetrics] Counted event 'payment.failed' (category: 'payment', count: 2)

[RUNNER] === Stopping 2 plugin(s) ===
[LOG][AppLogger] Logger stopped. Total events captured: 10
[METRICS][AppMetrics] Metrics collection stopped. Total events: 10
[RUNNER] All plugins stopped.

[RUNNER] === Plugin Reports ===
---
LoggerPlugin 'AppLogger' v1.0.0 — captured 10 event(s), running=false
---
MetricsPlugin 'AppMetrics' v2.1.0 — 10 total event(s):
  order → 3 event(s)
  payment → 2 event(s)
  user → 5 event(s)
---

[MAIN] Plugin system shutdown complete.
```

### Code Explanation

**Why `Box<dyn Plugin>` instead of generics?**

Consider the alternative — a generic runner:

```rust
// This would only work for ONE plugin type
struct PluginRunner<P: Plugin> {
    plugins: Vec<P>,
}
```

This forces all plugins to be the same concrete type. You cannot mix `LoggerPlugin` and `MetricsPlugin` in the same `Vec`.

`Box<dyn Plugin>` uses **dynamic dispatch**: at runtime, Rust looks up the correct method through a vtable pointer stored alongside the data pointer inside the `Box`. Each `Box<dyn Plugin>` is a **fat pointer** — two machine words wide: one pointing to the data, one pointing to the vtable.

```
Fat pointer layout:
┌─────────────────┬─────────────────┐
│  data_ptr       │  vtable_ptr     │
│  (→ heap data)  │  (→ fn table)   │
└─────────────────┴─────────────────┘

vtable for LoggerPlugin:
┌──────────────────────────────────┐
│  drop glue                       │
│  fn name() → &str                │
│  fn version() → &str             │
│  fn on_start(&mut self)          │
│  fn on_stop(&mut self)           │
│  fn execute(&mut self, event)    │
│  fn report(&self) → String       │
└──────────────────────────────────┘
```

**The `entry` API in MetricsPlugin:**

```rust
let count = self.counters.entry(String::from(category)).or_insert(0);
*count += 1;
```

`entry()` returns an `Entry` — either the existing entry for the key or a vacant slot. `or_insert(0)` inserts `0` if the key is absent, then returns a mutable reference to the value. We dereference it with `*count` to increment in place. This avoids a double-lookup (one to check, one to insert).

**Why `iter_mut()` in the runner?**

```rust
for plugin in self.plugins.iter_mut() {
    plugin.on_start();
}
```

`on_start`, `on_stop`, and `execute` all take `&mut self` — they need mutable access to the plugin. `iter_mut()` gives us mutable references to each element. If we used `iter()` (immutable), the compiler would reject the call.

**Why `iter()` for `print_reports`?**

```rust
for plugin in self.plugins.iter() {
    println!("{}", plugin.report());
}
```

`report` takes `&self` — it only reads. So `iter()` (immutable references) is sufficient and is the safer choice.

---

### Refactoring Suggestions

**1 — Add an `AuditPlugin` without touching existing code**

The Open/Closed Principle: the system is open for extension, closed for modification. To add audit logging:

```rust
// src/audit_plugin.rs
use crate::plugin::Plugin;

pub struct AuditPlugin {
    name: String,
    version: String,
    audit_log: Vec<String>,
}

impl AuditPlugin {
    pub fn new(name: &str) -> Self {
        AuditPlugin {
            name: String::from(name),
            version: String::from("1.0.0"),
            audit_log: Vec::new(),
        }
    }
}

impl Plugin for AuditPlugin {
    fn name(&self) -> &str { &self.name }
    fn version(&self) -> &str { &self.version }
    fn on_start(&mut self) { self.audit_log.push(String::from("SYSTEM_START")); }
    fn on_stop(&mut self) { self.audit_log.push(String::from("SYSTEM_STOP")); }
    fn execute(&mut self, event: &str) {
        self.audit_log.push(format!("EVENT: {}", event));
    }
    fn report(&self) -> String {
        format!("AuditPlugin: {} log entries", self.audit_log.len())
    }
}
```

Then in `main.rs`, add one line:
```rust
runner.register(Box::new(AuditPlugin::new("AuditLog")));
```

Zero changes to `runner.rs`, `logger_plugin.rs`, or `metrics_plugin.rs`.

---

**2 — Add event filtering to the runner**

Allow plugins to specify which events they care about:

```rust
// Extend Plugin trait with optional filter
trait Plugin {
    // ... existing methods ...
    fn accepts_event(&self, event: &str) -> bool {
        let _ = event;
        true // default: accept all events
    }
}

// In runner.dispatch:
pub fn dispatch(&mut self, event: &str) {
    for plugin in self.plugins.iter_mut() {
        if plugin.accepts_event(event) {
            plugin.execute(event);
        }
    }
}
```

---

**3 — Add a `priority` method to control dispatch order**

```rust
trait Plugin {
    // ... existing ...
    fn priority(&self) -> i32 { 0 }  // higher = runs first
}

// In PluginRunner::register, sort by priority after insertion:
pub fn register(&mut self, plugin: Box<dyn Plugin>) {
    self.plugins.push(plugin);
    self.plugins.sort_by(|a, b| b.priority().cmp(&a.priority()));
}
```

---

### Challenge Exercises

1. **Add an `AlertPlugin`** that monitors events matching a specific pattern (e.g., any event containing "failed" or "error") and prints an alert message when such an event is dispatched. It should report the total number of alerts fired.

2. **Add `plugin_count_by_status`** to `PluginRunner` that returns a tuple `(u32, u32)` — the number of plugins currently running vs stopped. You will need to add an `is_running(&self) -> bool` method to the `Plugin` trait with a default returning `true`.

3. **Make `PluginRunner` generic over a filter function.** Add a method `dispatch_filtered<F: Fn(&str) -> bool>(&mut self, event: &str, filter: F)` that only dispatches the event if `filter(event)` returns `true`.

4. **Add event batching.** Create a method `dispatch_batch(&mut self, events: &[&str])` on `PluginRunner` that processes a slice of events, collecting each plugin's results in a `Vec<String>`.

5. **Implement `Clone` for the plugins** by deriving it and updating the `Plugin` trait to require `Clone`. Then add a `PluginRunner::clone_plugins(&self) -> Vec<Box<dyn Plugin>>` method. What problem do you encounter? Research `dyn Clone` limitations and the `dyn-clone` crate.

### Real World Extensions

- **Dynamic loading**: In production systems, plugins are compiled as shared libraries (`.so` on Linux, `.dll` on Windows) and loaded at runtime with `libloading`. The `Plugin` trait becomes the stable ABI boundary.
- **Async plugins**: Replace `fn execute(&mut self, event: &str)` with `async fn execute(...)`. This requires `async_trait` crate because async methods in traits are not natively supported in stable Rust (as of 2025, they are being stabilized with Return Position Impl Trait in Traits).
- **Plugin configuration**: Add `fn configure(&mut self, config: &HashMap<String, String>)` to the `Plugin` trait so plugins can be configured at startup from a config file.
- **Error propagation**: Change `fn execute(&mut self, event: &str)` to `fn execute(&mut self, event: &str) -> Result<(), PluginError>` and have the runner collect and report errors rather than panicking.

---

### Exercises

**Exercise 1 — Shape Trait**

Define a `Shape` trait with methods `area(&self) -> f64`, `perimeter(&self) -> f64`, and a default method `describe(&self) -> String`. Implement it for `Circle` (with `radius: f64`), `Rectangle` (with `width: f64, height: f64`), and `Triangle` (with `a: f64, b: f64, c: f64`). Write a function `largest_area(shapes: &[&dyn Shape]) -> f64` that returns the area of the largest shape.

**Exercise 2 — Greetable Trait**

Define a `Greetable` trait with `fn greet(&self) -> String` and a default `fn formal_greet(&self) -> String` that returns `"Dear " + &self.greet()`. Implement it for `Person { name: String, title: String }` and `Robot { id: u32 }`. Write a generic function that accepts any `Greetable` and prints both greet variants.

**Exercise 3 — Summable Trait**

Define a `Summable` trait with `fn values(&self) -> Vec<f64>` and a default `fn sum(&self) -> f64` that sums the returned values. Implement for `Expenses { items: Vec<f64> }` and `Revenue { amounts: Vec<f64> }`. Write a function `net_profit(revenue: &impl Summable, expenses: &impl Summable) -> f64`.

**Exercise 4 — Multiple Bounds**

Given these two traits:
```rust
trait Printable { fn print(&self); }
trait Saveable { fn save(&self) -> String; }
```

Write a function `print_and_save<T: Printable + Saveable>(item: &T) -> String` that calls both methods and returns the saved string. Implement both traits on a `Document { title: String, content: String }` struct.

**Exercise 5 — Trait Objects**

Create a trait `Notification { fn send(&self, message: &str); fn channel(&self) -> &str; }`. Implement it for `EmailNotification { email: String }` and `SlackNotification { webhook: String }`. Write a `NotificationService` struct that holds a `Vec<Box<dyn Notification>>` and has a method `broadcast(&self, message: &str)` that sends to all channels.

---

### Solutions

**Solution 1 — Shape Trait**

```rust
use std::f64::consts::PI;

trait Shape {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;

    fn describe(&self) -> String {
        format!(
            "area={:.2}, perimeter={:.2}",
            self.area(),
            self.perimeter()
        )
    }
}

struct Circle { radius: f64 }
struct Rectangle { width: f64, height: f64 }
struct Triangle { a: f64, b: f64, c: f64 }

impl Shape for Circle {
    fn area(&self) -> f64 { PI * self.radius * self.radius }
    fn perimeter(&self) -> f64 { 2.0 * PI * self.radius }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 { self.width * self.height }
    fn perimeter(&self) -> f64 { 2.0 * (self.width + self.height) }
}

impl Shape for Triangle {
    fn area(&self) -> f64 {
        // Heron's formula
        let s = (self.a + self.b + self.c) / 2.0;
        (s * (s - self.a) * (s - self.b) * (s - self.c)).sqrt()
    }
    fn perimeter(&self) -> f64 { self.a + self.b + self.c }
}

fn largest_area(shapes: &[&dyn Shape]) -> f64 {
    shapes.iter().map(|s| s.area()).fold(0.0_f64, f64::max)
}

fn main() {
    let circle = Circle { radius: 5.0 };
    let rect = Rectangle { width: 4.0, height: 6.0 };
    let tri = Triangle { a: 3.0, b: 4.0, c: 5.0 };

    println!("Circle: {}", circle.describe());
    println!("Rectangle: {}", rect.describe());
    println!("Triangle: {}", tri.describe());

    let shapes: Vec<&dyn Shape> = vec![&circle, &rect, &tri];
    println!("Largest area: {:.2}", largest_area(&shapes));
}
```

---

**Solution 2 — Greetable Trait**

```rust
trait Greetable {
    fn greet(&self) -> String;

    fn formal_greet(&self) -> String {
        format!("Dear {}", self.greet())
    }
}

struct Person { name: String, title: String }
struct Robot { id: u32 }

impl Greetable for Person {
    fn greet(&self) -> String {
        format!("{} {}", self.title, self.name)
    }
}

impl Greetable for Robot {
    fn greet(&self) -> String {
        format!("Unit-{:04}", self.id)
    }
}

fn print_greetings(g: &impl Greetable) {
    println!("Casual:  {}", g.greet());
    println!("Formal:  {}", g.formal_greet());
}

fn main() {
    let person = Person { name: String::from("Smith"), title: String::from("Dr.") };
    let robot = Robot { id: 42 };

    println!("--- Person ---");
    print_greetings(&person);

    println!("--- Robot ---");
    print_greetings(&robot);
}
```

---

**Solution 3 — Summable Trait**

```rust
trait Summable {
    fn values(&self) -> Vec<f64>;

    fn sum(&self) -> f64 {
        self.values().iter().sum()
    }
}

struct Expenses { items: Vec<f64> }
struct Revenue { amounts: Vec<f64> }

impl Summable for Expenses {
    fn values(&self) -> Vec<f64> { self.items.clone() }
}

impl Summable for Revenue {
    fn values(&self) -> Vec<f64> { self.amounts.clone() }
}

fn net_profit(revenue: &impl Summable, expenses: &impl Summable) -> f64 {
    revenue.sum() - expenses.sum()
}

fn main() {
    let revenue = Revenue { amounts: vec![10_000.0, 25_000.0, 8_000.0] };
    let expenses = Expenses { items: vec![5_000.0, 3_200.0, 1_500.0] };

    println!("Revenue:  ${:.2}", revenue.sum());
    println!("Expenses: ${:.2}", expenses.sum());
    println!("Profit:   ${:.2}", net_profit(&revenue, &expenses));
}
```

---

**Solution 4 — Multiple Bounds**

```rust
trait Printable {
    fn print(&self);
}

trait Saveable {
    fn save(&self) -> String;
}

struct Document {
    title: String,
    content: String,
}

impl Printable for Document {
    fn print(&self) {
        println!("=== {} ===\n{}", self.title, self.content);
    }
}

impl Saveable for Document {
    fn save(&self) -> String {
        format!("{{\"title\":\"{}\",\"content\":\"{}\"}}", self.title, self.content)
    }
}

fn print_and_save<T: Printable + Saveable>(item: &T) -> String {
    item.print();
    let saved = item.save();
    println!("Saved as: {}", saved);
    saved
}

fn main() {
    let doc = Document {
        title: String::from("Rust Traits"),
        content: String::from("Traits define shared behavior in Rust."),
    };
    let json = print_and_save(&doc);
    println!("\nReturned JSON length: {} bytes", json.len());
}
```

---

**Solution 5 — Trait Objects**

```rust
trait Notification {
    fn send(&self, message: &str);
    fn channel(&self) -> &str;
}

struct EmailNotification { email: String }
struct SlackNotification { webhook: String }

impl Notification for EmailNotification {
    fn send(&self, message: &str) {
        println!("[EMAIL → {}] {}", self.email, message);
    }
    fn channel(&self) -> &str { "email" }
}

impl Notification for SlackNotification {
    fn send(&self, message: &str) {
        println!("[SLACK → {}] {}", self.webhook, message);
    }
    fn channel(&self) -> &str { "slack" }
}

struct NotificationService {
    channels: Vec<Box<dyn Notification>>,
}

impl NotificationService {
    fn new() -> Self {
        NotificationService { channels: Vec::new() }
    }

    fn add_channel(&mut self, channel: Box<dyn Notification>) {
        println!("Added channel: {}", channel.channel());
        self.channels.push(channel);
    }

    fn broadcast(&self, message: &str) {
        println!("\nBroadcasting to {} channel(s):", self.channels.len());
        for channel in &self.channels {
            channel.send(message);
        }
    }
}

fn main() {
    let mut service = NotificationService::new();

    service.add_channel(Box::new(EmailNotification {
        email: String::from("admin@company.com"),
    }));
    service.add_channel(Box::new(SlackNotification {
        webhook: String::from("https://hooks.slack.com/xyz"),
    }));
    service.add_channel(Box::new(EmailNotification {
        email: String::from("ops@company.com"),
    }));

    service.broadcast("System alert: High memory usage detected.");
}
```

---

### Quiz

**Q1.** What is the key difference between Rust traits and Python duck typing?

A) Traits require more code  
B) Traits are checked at compile time; duck typing is checked at runtime  
C) Traits only work with structs  
D) Duck typing is faster  

---

**Q2.** What does the orphan rule prevent?

A) Implementing the same trait twice on the same type  
B) Implementing a foreign trait on a foreign type  
C) Using default implementations  
D) Defining traits with no methods  

---

**Q3.** Which syntax is used to combine multiple trait bounds on a single generic parameter?

A) `&&`  
B) `,`  
C) `+`  
D) `|`  

---

**Q4.** When should you use `where` clauses instead of inline bounds?

A) Never — they are deprecated  
B) When you have multiple generic parameters with multiple bounds each  
C) Only when using `Box<dyn Trait>`  
D) When the trait has default implementations  

---

**Q5.** What is the difference between `&impl Animal` and `&dyn Animal`?

A) They are identical  
B) `impl Animal` uses static dispatch (monomorphization); `dyn Animal` uses dynamic dispatch (vtable)  
C) `dyn Animal` only works in struct fields  
D) `impl Animal` requires the `Clone` trait  

---

**Q6.** Consider this code. What happens?

```rust
trait Greet {
    fn hello(&self) -> String;
    fn goodbye(&self) -> String {
        format!("Goodbye from {}", self.hello())
    }
}

struct User { name: String }

impl Greet for User {
    fn hello(&self) -> String { self.name.clone() }
}
```

A) Compile error — `goodbye` must be implemented  
B) Runtime error — `goodbye` is not defined  
C) Compiles and works — `User` uses the default `goodbye`  
D) Compile error — default methods cannot call required methods  

---

**Q7.** Why do we use `Box<dyn Plugin>` instead of a generic `<P: Plugin>` in the `PluginRunner`?

A) `Box` is faster than generics  
B) Generics would force all plugins to be the same concrete type; `Box<dyn Plugin>` allows mixing different plugin types  
C) `dyn` is required for all trait usage  
D) `Box` is needed to satisfy the borrow checker  

---

**Q8.** What does `iter_mut()` give you that `iter()` does not?

A) Faster iteration  
B) Mutable references to elements, enabling methods that take `&mut self`  
C) The ability to remove elements during iteration  
D) Access to the index of each element  

---

### Quiz Answers

**A1. B** — Traits are contracts verified by the Rust compiler at compile time. If a type does not implement a required trait method, the code will not compile. Python's duck typing only fails at runtime when the method is actually called on a value that lacks it.

**A2. B** — The orphan rule says you can only implement a trait if either the trait or the type is defined in your crate. You cannot implement `std::fmt::Display` (foreign trait) on `std::vec::Vec` (foreign type) because both live outside your crate. This prevents conflicting implementations across crates.

**A3. C** — The `+` operator combines bounds: `A: Animal + fmt::Display + Clone`. Each bound is an additional requirement the type must satisfy.

**A4. B** — When signatures become long and unreadable due to multiple generic parameters each with multiple bounds, move the bounds into a `where` clause below the signature for clarity. They are functionally identical.

**A5. B** — `impl Trait` in function parameters uses **static dispatch**: the compiler generates a separate version of the function for each concrete type used (monomorphization), just like generics. `dyn Trait` uses **dynamic dispatch**: a fat pointer (data + vtable) is used at runtime to look up the correct method. Static dispatch is zero-cost; dynamic dispatch has a small indirection cost but enables heterogeneous collections and runtime flexibility.

**A6. C** — Default implementations are fully valid when they build on required methods. `User` implements `hello`, and the default `goodbye` calls `self.hello()` — which will resolve to `User`'s implementation at runtime. This compiles and runs correctly.

**A7. B** — A generic `struct PluginRunner<P: Plugin> { plugins: Vec<P> }` forces all elements of `plugins` to be the same type `P`. If you register a `LoggerPlugin`, you cannot also register a `MetricsPlugin`. `Box<dyn Plugin>` allocates each plugin on the heap and stores a fat pointer, allowing the `Vec` to hold plugins of completely different concrete types.

**A8. B** — `iter()` gives you immutable references (`&T`), which only allows calling methods that take `&self`. `iter_mut()` gives mutable references (`&mut T`), which allows calling methods that take `&mut self` — like `on_start`, `on_stop`, and `execute` in the plugin system, all of which need to modify the plugin's internal state.

---

## Chapter Summary

In this chapter, you learned one of Rust's most important abstraction mechanisms: **traits**.

**Key concepts covered:**

- **Traits define shared behavior**, not shared data or inheritance. A trait is a named contract — a collection of method signatures a type agrees to implement.

- **Rust uses traits instead of inheritance.** This promotes composition over inheritance, avoids tight coupling, and enables more flexible, testable code.

- **Traits vs alternatives:**
  - Python duck typing: implicit, checked at runtime.
  - Java interfaces: explicit contract, but no multiple inheritance.
  - Rust traits: explicit contract, multiple trait implementations per type, checked entirely at compile time.

- **Defining a trait** uses the `trait` keyword and lists method signatures. Methods with bodies are **default implementations** that types inherit unless they override them.

- **Implementing a trait** uses `impl TraitName for TypeName`. Every required method must be implemented — the compiler enforces this.

- **The orphan rule** prevents implementing a foreign trait on a foreign type, avoiding conflicting implementations across crates.

- **Trait bounds** let you write generic functions and structs that require their type parameters to implement specific traits. Use `impl Trait` for simple single-use bounds; use `<T: Trait>` generic parameter form when the same type appears multiple times.

- **Multiple bounds** are combined with `+`: `A: Animal + Display + Clone`.

- **`where` clauses** move complex bounds below the function signature for readability. They are semantically identical to inline bounds.

- **Trait objects (`Box<dyn Trait>`)** enable runtime polymorphism — storing a heterogeneous collection of different types that all implement the same trait. They use dynamic dispatch via vtables at a small runtime cost.

- **Static dispatch vs dynamic dispatch:**
  - `impl Trait` / `<T: Trait>` → static dispatch, monomorphization, zero runtime overhead.
  - `&dyn Trait` / `Box<dyn Trait>` → dynamic dispatch, vtable lookup, small overhead but maximum flexibility.

**What you built:**

- **Animal Trait system** — multiple animals implementing multiple traits (`Animal`, `Movable`, `Feedable`) with `where` clauses and multi-bound generic functions.
- **Vehicle Trait system** — demonstrating trait composition across domain types including optional traits (`Chargeable` only for electric vehicles).
- **Plugin System** — a production-architecture plugin framework using `Box<dyn Plugin>` for runtime extensibility, the `entry` HashMap API, mutable iteration, and the Open/Closed Principle.

**What comes next:**

Chapter 16 covers **Lifetimes** — Rust's mechanism for tracking how long references remain valid. Understanding lifetimes is the final piece needed to fully master Rust's ownership and borrowing system, and it completes the trio of generic features: generics (types), traits (behavior), and lifetimes (validity).
