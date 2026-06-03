# Chapter 10: Enums

## Learning Objectives

By the end of this chapter, you will:

- Define enums with simple and data-carrying variants
- Use `match` exhaustively with enum variants
- Understand `Option<T>` and replace null-checks with it
- Implement methods on enums with `impl`
- Build a Traffic Light Simulator using enums and state machines

---

## Theory

### 10.1 Enum Basics

An **enum** (enumeration) defines a type that can be one of several named **variants**. Unlike structs (which have ALL their fields), an enum value is exactly ONE of its variants.

```rust
enum Direction {
    North,
    South,
    East,
    West,
}

fn main() {
    let dir = Direction::North;

    match dir {
        Direction::North => println!("Going north"),
        Direction::South => println!("Going south"),
        Direction::East  => println!("Going east"),
        Direction::West  => println!("Going west"),
    }
}
```

Variants are namespaced under the enum: `Direction::North`. The `match` is exhaustive — all four variants must be handled.

#### Enums Are Types

You can use enums in function parameters, return types, and struct fields:

```rust
struct Delivery {
    destination: String,
    direction: Direction,  // enum as struct field
    distance_km: f64,
}

fn opposite(dir: Direction) -> Direction {  // enum in function signature
    match dir {
        Direction::North => Direction::South,
        Direction::South => Direction::North,
        Direction::East  => Direction::West,
        Direction::West  => Direction::East,
    }
}
```

---

### 10.2 Enum Variants with Data

Rust enums are far more powerful than C/Java enums — each variant can hold different data:

```rust
enum Message {
    Quit,                          // no data
    Move { x: i32, y: i32 },      // named fields (like a struct)
    Write(String),                 // single value (like a tuple struct)
    ChangeColor(u8, u8, u8),      // multiple values (like a tuple struct)
}
```

This is equivalent to defining four different structs, but grouped under one type:

```rust
// What Rust enums replace:
struct QuitMessage;
struct MoveMessage { x: i32, y: i32 }
struct WriteMessage(String);
struct ChangeColorMessage(u8, u8, u8);
// But now a function can't accept "any of these" — no common type
// Enums solve this: Message is the single type that unifies all variants
```

#### Matching Data-Carrying Variants

```rust
fn process(msg: Message) {
    match msg {
        Message::Quit => println!("Quit"),
        Message::Move { x, y } => println!("Move to ({}, {})", x, y),
        Message::Write(text) => println!("Write: {}", text),
        Message::ChangeColor(r, g, b) => println!("Color: ({}, {}, {})", r, g, b),
    }
}

fn main() {
    process(Message::Quit);
    process(Message::Move { x: 10, y: 20 });
    process(Message::Write(String::from("hello")));
    process(Message::ChangeColor(255, 0, 128));
}
```

#### Enums vs Structs

| Feature | Struct | Enum |
|---------|--------|------|
| Represents | A thing with ALL of its properties | ONE of several possible states |
| Fields | All fields present in every instance | Only one variant's data present |
| Memory | Fixed: all fields | Variable: size of largest variant |
| Pattern matching | Access fields directly | Use `match` to destructure |

**Real-world analogy:**
- A `User` struct has a name, email, and age — all present simultaneously (struct)
- An HTTP `Status` can be `Ok`, `NotFound`, `Error(message)` — only one at a time (enum)

#### impl on Enums

You can define methods on enums too:

```rust
impl Message {
    fn is_quit(&self) -> bool {
        matches!(self, Message::Quit)
    }

    fn describe(&self) -> String {
        match self {
            Message::Quit             => String::from("quit command"),
            Message::Move { x, y }   => format!("move to ({}, {})", x, y),
            Message::Write(s)         => format!("write '{}'", s),
            Message::ChangeColor(r, g, b) => format!("color ({}, {}, {})", r, g, b),
        }
    }
}

fn main() {
    let msg = Message::Move { x: 5, y: 10 };
    println!("{}", msg.describe());
    println!("is quit: {}", msg.is_quit());
}
```

---

### 10.3 Option\<T\>

`Option<T>` is Rust's answer to `null`. It's a standard library enum defined as:

```rust
enum Option<T> {
    Some(T),  // a value exists
    None,     // no value
}
```

#### Why No Null?

In languages with `null`, any reference can be null — you can never be sure. Calling a method on a null reference crashes at runtime. Tony Hoare (who invented null) called it his "billion-dollar mistake."

In Rust, if a value can be absent, the type **must** be `Option<T>`. The compiler forces you to handle both cases before using the value.

```rust
// In Rust:
let x: i32 = 5;              // always has a value — NEVER null
let y: Option<i32> = Some(5); // might have a value
let z: Option<i32> = None;    // no value

// You cannot use Option<i32> as i32 directly:
let sum = x + y;  // ERROR: cannot add `Option<i32>` to `i32`
```

The type system separates "value that must exist" from "value that might not exist."

#### Using Option Values

```rust
fn divide(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 { None } else { Some(a / b) }
}

fn main() {
    // Method 1: match
    match divide(10.0, 2.0) {
        Some(result) => println!("Result: {}", result),
        None         => println!("Cannot divide by zero"),
    }

    // Method 2: if let
    if let Some(result) = divide(10.0, 3.0) {
        println!("Result: {:.4}", result);
    }

    // Method 3: unwrap_or (safe fallback)
    let result = divide(10.0, 0.0).unwrap_or(0.0);
    println!("With fallback: {}", result);

    // Method 4: map (transform the inner value if Some)
    let doubled = divide(10.0, 2.0).map(|x| x * 2.0);
    println!("Doubled: {:?}", doubled);  // Some(10.0)

    // Method 5: and_then (chain operations that return Option)
    let result = divide(10.0, 2.0).and_then(|x| divide(x, 2.0));
    println!("Chained: {:?}", result);   // Some(2.5)

    // Method 6: unwrap (panics if None — use only when sure)
    let r = divide(10.0, 2.0).unwrap();  // safe here
    println!("Unwrapped: {}", r);

    // Method 7: ? operator (propagate None)
    // (covered in Part 5 — Error Handling)
}
```

#### Common Option Methods

```rust
let x: Option<i32> = Some(5);
let none: Option<i32> = None;

x.is_some()                  // true
x.is_none()                  // false
x.unwrap()                   // 5 (panics if None)
x.unwrap_or(0)               // 5 (returns 0 if None)
x.unwrap_or_else(|| 0)       // 5 (lazy default)
x.map(|v| v * 2)             // Some(10)
x.filter(|v| *v > 3)         // Some(5)
x.filter(|v| *v > 10)        // None
x.or(Some(99))               // Some(5) (returns self if Some, else other)
none.or(Some(99))            // Some(99)
x.and(Some("hello"))         // Some("hello") (if self is Some, return other)
x.and_then(|v| if v > 3 { Some(v) } else { None })  // Some(5)
```

---

### 10.4 Match with Enums

Pattern matching with enums is one of Rust's most powerful features. Let's go deep.

#### Binding in Patterns

```rust
#[derive(Debug)]
enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
    Triangle { base: f64, height: f64 },
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle { radius }               => std::f64::consts::PI * radius * radius,
            Shape::Rectangle { width, height }     => width * height,
            Shape::Triangle { base, height }       => 0.5 * base * height,
        }
    }

    fn name(&self) -> &'static str {
        match self {
            Shape::Circle { .. }    => "circle",     // .. ignores remaining fields
            Shape::Rectangle { .. } => "rectangle",
            Shape::Triangle { .. }  => "triangle",
        }
    }

    fn is_circle(&self) -> bool {
        matches!(self, Shape::Circle { .. })  // matches! macro — returns bool
    }
}

fn main() {
    let shapes = vec![
        Shape::Circle { radius: 5.0 },
        Shape::Rectangle { width: 4.0, height: 6.0 },
        Shape::Triangle { base: 3.0, height: 8.0 },
    ];

    for shape in &shapes {
        println!("{}: area = {:.4}", shape.name(), shape.area());
    }
}
```

#### Nested Enums

```rust
#[derive(Debug)]
enum Color {
    Rgb(u8, u8, u8),
    Named(&'static str),
    Transparent,
}

#[derive(Debug)]
enum Background {
    Solid(Color),
    Gradient(Color, Color),
    None,
}

fn describe_bg(bg: &Background) -> String {
    match bg {
        Background::None => String::from("no background"),
        Background::Solid(Color::Transparent) => String::from("transparent"),
        Background::Solid(Color::Named(name)) => format!("solid {}", name),
        Background::Solid(Color::Rgb(r, g, b)) => format!("solid rgb({},{},{})", r, g, b),
        Background::Gradient(c1, c2) => format!("gradient {:?} → {:?}", c1, c2),
    }
}
```

---

## Code Example

### Mini Project: Traffic Light Simulator

#### Project Overview

A traffic light simulator demonstrating enums as state machines, with state transitions, durations, and pedestrian crossing integration.

#### Functional Requirements

1. Model traffic light states: Red, Yellow, Green
2. Implement correct state transitions (Green → Yellow → Red → Green)
3. Track duration each state should last
4. Model pedestrian light states
5. Simulate a sequence of transitions
6. Demonstrate enums with impl blocks

#### Complete Source Code

```rust
use std::fmt;
use std::thread;
use std::time::Duration;

#[derive(Debug, Clone, PartialEq)]
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

impl TrafficLight {
    fn next(&self) -> TrafficLight {
        match self {
            TrafficLight::Green  => TrafficLight::Yellow,
            TrafficLight::Yellow => TrafficLight::Red,
            TrafficLight::Red    => TrafficLight::Green,
        }
    }

    fn duration_secs(&self) -> u64 {
        match self {
            TrafficLight::Red    => 30,
            TrafficLight::Yellow => 5,
            TrafficLight::Green  => 25,
        }
    }

    fn can_go(&self) -> bool {
        matches!(self, TrafficLight::Green)
    }

    fn must_stop(&self) -> bool {
        matches!(self, TrafficLight::Red)
    }

    fn action(&self) -> &'static str {
        match self {
            TrafficLight::Red    => "STOP",
            TrafficLight::Yellow => "PREPARE",
            TrafficLight::Green  => "GO",
        }
    }
}

impl fmt::Display for TrafficLight {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol = match self {
            TrafficLight::Red    => "🔴 RED",
            TrafficLight::Yellow => "🟡 YELLOW",
            TrafficLight::Green  => "🟢 GREEN",
        };
        write!(f, "{}", symbol)
    }
}

#[derive(Debug, Clone, PartialEq)]
enum PedestrianLight {
    Walk,
    DontWalk,
    Flashing,
}

impl PedestrianLight {
    fn from_traffic(light: &TrafficLight) -> PedestrianLight {
        match light {
            TrafficLight::Red    => PedestrianLight::Walk,
            TrafficLight::Yellow => PedestrianLight::Flashing,
            TrafficLight::Green  => PedestrianLight::DontWalk,
        }
    }

    fn can_cross(&self) -> bool {
        matches!(self, PedestrianLight::Walk)
    }
}

impl fmt::Display for PedestrianLight {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PedestrianLight::Walk      => write!(f, "🚶 WALK"),
            PedestrianLight::DontWalk  => write!(f, "🛑 DON'T WALK"),
            PedestrianLight::Flashing  => write!(f, "⚠️  FLASHING — finish crossing"),
        }
    }
}

#[derive(Debug)]
enum IntersectionEvent {
    LightChanged(TrafficLight),
    PedestrianRequest,
    EmergencyVehicle,
    PowerOutage,
}

struct Intersection {
    name: String,
    light: TrafficLight,
    cycle_count: u32,
    emergency_mode: bool,
}

impl Intersection {
    fn new(name: &str) -> Intersection {
        Intersection {
            name: String::from(name),
            light: TrafficLight::Red,
            cycle_count: 0,
            emergency_mode: false,
        }
    }

    fn handle_event(&mut self, event: IntersectionEvent) {
        match event {
            IntersectionEvent::LightChanged(new_light) => {
                self.light = new_light;
                if self.light == TrafficLight::Red {
                    self.cycle_count += 1;
                }
            }
            IntersectionEvent::PedestrianRequest => {
                match &self.light {
                    TrafficLight::Green => {
                        println!("  [PEDESTRIAN] Request noted — will activate on next red");
                    }
                    TrafficLight::Red => {
                        println!("  [PEDESTRIAN] Walk signal is active now");
                    }
                    TrafficLight::Yellow => {
                        println!("  [PEDESTRIAN] Finishing current cycle");
                    }
                }
            }
            IntersectionEvent::EmergencyVehicle => {
                self.emergency_mode = true;
                println!("  [EMERGENCY] All lights set to flashing yellow!");
            }
            IntersectionEvent::PowerOutage => {
                self.emergency_mode = true;
                println!("  [POWER] Intersection offline — treat as 4-way stop");
            }
        }
    }

    fn advance(&mut self) {
        let next = self.light.next();
        self.handle_event(IntersectionEvent::LightChanged(next));
    }

    fn status(&self) {
        let pedestrian = PedestrianLight::from_traffic(&self.light);
        println!(
            "  [{}] Traffic: {}  ({} sec) → {}  |  Pedestrian: {}  |  Cycle #{}",
            self.name,
            self.light,
            self.light.duration_secs(),
            self.light.action(),
            pedestrian,
            self.cycle_count,
        );
    }
}

fn simulate(intersection: &mut Intersection, steps: u32, delay_ms: u64) {
    println!("\n=== Simulation Start: {} ===\n", intersection.name);

    for step in 1..=steps {
        println!("Step {}:", step);
        intersection.status();

        if step % 4 == 0 {
            intersection.handle_event(IntersectionEvent::PedestrianRequest);
        }

        if delay_ms > 0 {
            thread::sleep(Duration::from_millis(delay_ms));
        }

        intersection.advance();
        println!();
    }

    println!("=== Simulation End ===");
    println!("Total cycles completed: {}", intersection.cycle_count);
}

fn main() {
    let mut intersection = Intersection::new("Main St & 1st Ave");
    simulate(&mut intersection, 9, 0); // 9 steps = 3 full cycles, 0ms delay

    println!("\n--- Emergency Event ---");
    intersection.handle_event(IntersectionEvent::EmergencyVehicle);

    println!("\n--- Direct Enum Usage ---");
    let states = [TrafficLight::Red, TrafficLight::Yellow, TrafficLight::Green];
    for state in &states {
        let ped = PedestrianLight::from_traffic(state);
        println!(
            "  {:<20} can_go={:<5} must_stop={:<5} pedestrian can_cross={}",
            state.to_string(),
            state.can_go(),
            state.must_stop(),
            ped.can_cross()
        );
    }
}
```

#### Code Explanation

```rust
fn next(&self) -> TrafficLight {
    match self {
        TrafficLight::Green  => TrafficLight::Yellow,
        TrafficLight::Yellow => TrafficLight::Red,
        TrafficLight::Red    => TrafficLight::Green,
    }
}
```
- Returns a new `TrafficLight` — the next state in the cycle
- `match` on `self` is exhaustive — all three variants handled
- This is a pure state transition function with no side effects

```rust
fn from_traffic(light: &TrafficLight) -> PedestrianLight {
    match light {
        TrafficLight::Red    => PedestrianLight::Walk,
        TrafficLight::Yellow => PedestrianLight::Flashing,
        TrafficLight::Green  => PedestrianLight::DontWalk,
    }
}
```
- An associated function (no `self`) — constructs a `PedestrianLight` from a `TrafficLight`
- Maps between two related enums — shows how enums compose

```rust
fn handle_event(&mut self, event: IntersectionEvent) {
    match event {
        IntersectionEvent::LightChanged(new_light) => { ... }
        IntersectionEvent::PedestrianRequest => { ... }
        IntersectionEvent::EmergencyVehicle => { ... }
        IntersectionEvent::PowerOutage => { ... }
    }
}
```
- Each variant of `IntersectionEvent` carries different data — `LightChanged` carries a `TrafficLight`
- `match` destructures the data from each variant
- Exhaustive — the compiler would error if a new event variant were added without handling it

#### Refactoring Suggestions

1. **Separate concerns**: put each enum in its own file/module
2. **Configuration**: make durations configurable via `Config` struct instead of hardcoded
3. **Logging**: replace `println!` with a proper logging crate (`tracing`, `log`)
4. **Real timing**: use `tokio::time::sleep` for async simulation with real delays
5. **Tests**: add unit tests for `next()`, `from_traffic()`, and `handle_event()`

#### Challenge Exercises

1. Add a `FlashingYellow` variant for construction zones and handle it in all matches
2. Implement a countdown timer that counts down the seconds for each state
3. Add a `history: Vec<TrafficLight>` field to `Intersection` and log all state changes
4. Implement multiple intersections that coordinate (e.g., green wave)

#### Real World Extensions

- Connect to a real hardware GPIO pin with the `rppal` crate (Raspberry Pi)
- Build a web dashboard that shows the current state via WebSocket
- Simulate traffic flow with multiple cars using threads

---

## Common Mistakes

### Mistake 1: Forgetting to handle all variants

```rust
let msg = Message::Write(String::from("hi"));

match msg {
    Message::Quit      => println!("quit"),
    Message::Write(s)  => println!("{}", s),
    // ERROR: non-exhaustive patterns — Move and ChangeColor not covered
}

// Fix: add remaining variants or wildcard
match msg {
    Message::Quit          => println!("quit"),
    Message::Write(s)      => println!("{}", s),
    Message::Move { x, y } => println!("move to {},{}", x, y),
    Message::ChangeColor(r, g, b) => println!("color {},{},{}", r, g, b),
}
```

### Mistake 2: Using `==` directly on complex enums

```rust
let light = TrafficLight::Red;
if light == TrafficLight::Red { ... }
// ERROR: binary operation `==` cannot be applied unless PartialEq is derived
```

```rust
#[derive(PartialEq)]
enum TrafficLight { Red, Yellow, Green }
// Now == works

// OR use match/matches!
let is_red = matches!(light, TrafficLight::Red);
```

### Mistake 3: Trying to use `Option<T>` as `T`

```rust
let maybe: Option<i32> = Some(5);
let doubled = maybe * 2;  // ERROR: Option<i32> is not i32

// Fix: extract the value first
let doubled = maybe.map(|x| x * 2);    // Option<i32>
let doubled = maybe.unwrap() * 2;      // i32 (panics if None)
let doubled = maybe.unwrap_or(0) * 2;  // i32 (safe)
```

### Mistake 4: Redundant Option wrapping

```rust
// Redundant — match already unwraps
if let Some(x) = some_option {
    return Some(x);  // just wrapping back up
}
return None;

// Better:
return some_option;  // just return it directly
```

---

## Best Practices

1. **Use enums for state machines** — each state is a variant, transitions are methods
2. **Derive `Debug` and `PartialEq`** on enums for testing and comparison
3. **Prefer `matches!` macro** over single-variant `if let` for boolean checks
4. **Use `Option` instead of sentinel values** (`-1`, `""`, `false`) to represent absence
5. **Prefer `unwrap_or` over `unwrap`** in application code — handle the None case
6. **Use `and_then` and `map`** to chain Option operations without nested matches
7. **Group related data** into enum variants — an enum is often better than a struct + bool flag

---

## Exercises

### Exercise 1: Coin Enum

Define `enum Coin { Penny, Nickel, Dime, Quarter }`. Implement a `value_cents(&self) -> u32` method. Write a function `total_cents(coins: &[Coin]) -> u32`.

### Exercise 2: Result-like Enum

Define `enum MathResult { Value(f64), DivByZero, Overflow, Underflow }`. Write `safe_divide(a: f64, b: f64) -> MathResult` and `safe_sqrt(x: f64) -> MathResult` (Underflow for negative). Use match to print results.

### Exercise 3: Option Chaining

Write a function `find_and_double(numbers: &[i32], target: i32) -> Option<i32>` that finds the first occurrence of `target` and returns double its value (using `find`, `map`). Test with present and missing targets.

### Exercise 4: Nested Enum

Define `enum Json { Null, Bool(bool), Number(f64), Str(String), Array(Vec<Json>), Object(Vec<(String, Json)>) }`. Write a `to_string(&self) -> String` method that serializes it. Test with a nested structure.

### Exercise 5: State Machine

Model a door lock: `enum LockState { Locked, Unlocked, Jammed }`. Implement `try_unlock(pin: u32) -> LockState` and `try_lock() -> LockState` and `is_passable() -> bool`. Simulate a sequence: lock → fail unlock → correct unlock → lock again.

---

## Solutions

### Solution 1

```rust
#[derive(Debug)]
enum Coin { Penny, Nickel, Dime, Quarter }

impl Coin {
    fn value_cents(&self) -> u32 {
        match self {
            Coin::Penny   => 1,
            Coin::Nickel  => 5,
            Coin::Dime    => 10,
            Coin::Quarter => 25,
        }
    }
}

fn total_cents(coins: &[Coin]) -> u32 {
    coins.iter().map(|c| c.value_cents()).sum()
}

fn main() {
    let coins = [Coin::Quarter, Coin::Dime, Coin::Nickel, Coin::Penny, Coin::Penny];
    println!("Total: {} cents", total_cents(&coins));
}
```

### Solution 2

```rust
#[derive(Debug)]
enum MathResult { Value(f64), DivByZero, Overflow, Underflow }

fn safe_divide(a: f64, b: f64) -> MathResult {
    if b == 0.0 { MathResult::DivByZero }
    else if (a / b).is_infinite() { MathResult::Overflow }
    else { MathResult::Value(a / b) }
}

fn safe_sqrt(x: f64) -> MathResult {
    if x < 0.0 { MathResult::Underflow }
    else { MathResult::Value(x.sqrt()) }
}

fn main() {
    for result in [safe_divide(10.0, 2.0), safe_divide(5.0, 0.0), safe_sqrt(16.0), safe_sqrt(-4.0)] {
        match result {
            MathResult::Value(v)  => println!("= {}", v),
            MathResult::DivByZero => println!("Error: division by zero"),
            MathResult::Overflow  => println!("Error: overflow"),
            MathResult::Underflow => println!("Error: underflow (negative sqrt)"),
        }
    }
}
```

### Solution 3

```rust
fn find_and_double(numbers: &[i32], target: i32) -> Option<i32> {
    numbers.iter()
        .find(|&&x| x == target)
        .map(|&x| x * 2)
}

fn main() {
    let nums = [3, 7, 2, 9, 4];
    println!("{:?}", find_and_double(&nums, 7));   // Some(14)
    println!("{:?}", find_and_double(&nums, 99));  // None
}
```

### Solution 4

```rust
#[derive(Debug)]
enum Json {
    Null,
    Bool(bool),
    Number(f64),
    Str(String),
    Array(Vec<Json>),
    Object(Vec<(String, Json)>),
}

impl Json {
    fn to_string(&self) -> String {
        match self {
            Json::Null          => String::from("null"),
            Json::Bool(b)       => b.to_string(),
            Json::Number(n)     => n.to_string(),
            Json::Str(s)        => format!("\"{}\"", s),
            Json::Array(items)  => format!("[{}]", items.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(",")),
            Json::Object(pairs) => {
                let body = pairs.iter()
                    .map(|(k, v)| format!("\"{}\":{}", k, v.to_string()))
                    .collect::<Vec<_>>().join(",");
                format!("{{{}}}", body)
            }
        }
    }
}

fn main() {
    let data = Json::Object(vec![
        (String::from("name"), Json::Str(String::from("Alice"))),
        (String::from("age"),  Json::Number(30.0)),
        (String::from("active"), Json::Bool(true)),
        (String::from("scores"), Json::Array(vec![Json::Number(95.0), Json::Number(88.0)])),
    ]);
    println!("{}", data.to_string());
}
```

### Solution 5

```rust
const CORRECT_PIN: u32 = 1234;

#[derive(Debug, PartialEq)]
enum LockState { Locked, Unlocked, Jammed }

impl LockState {
    fn try_unlock(&self, pin: u32) -> LockState {
        match self {
            LockState::Locked => {
                if pin == CORRECT_PIN { LockState::Unlocked }
                else { LockState::Locked }
            }
            LockState::Jammed => { println!("Door is jammed!"); LockState::Jammed }
            LockState::Unlocked => LockState::Unlocked,
        }
    }

    fn try_lock(&self) -> LockState {
        match self {
            LockState::Unlocked => LockState::Locked,
            other => { println!("Can't lock from {:?}", other); self.clone() }
        }
    }

    fn is_passable(&self) -> bool { matches!(self, LockState::Unlocked) }
}

impl Clone for LockState {
    fn clone(&self) -> Self {
        match self {
            LockState::Locked   => LockState::Locked,
            LockState::Unlocked => LockState::Unlocked,
            LockState::Jammed   => LockState::Jammed,
        }
    }
}

fn main() {
    let state = LockState::Locked;
    println!("{:?} passable={}", state, state.is_passable());

    let state = state.try_unlock(9999);
    println!("Wrong pin: {:?}", state);

    let state = state.try_unlock(1234);
    println!("Correct pin: {:?} passable={}", state, state.is_passable());

    let state = state.try_lock();
    println!("Locked again: {:?}", state);
}
```

---

## Quiz

**Q1.** What is the key advantage of Rust's enums over C-style enums?

a) Rust enums are faster  
b) Rust enum variants can hold associated data of different types  
c) Rust enums can have more than 256 variants  
d) Rust enums are always Copy  

**Q2.** Why does Rust use `Option<T>` instead of null?

a) Performance reasons  
b) The type system forces you to explicitly handle the "no value" case, preventing null pointer exceptions  
c) It's more verbose  
d) Option is only for integers  

**Q3.** What does the `matches!` macro do?

a) Runs a match expression and returns the matched arm  
b) Returns a bool — true if the value matches the given pattern  
c) Imports the match module  
d) Validates enum variants  

**Q4.** What happens if you add a new variant to an enum that is used in match expressions elsewhere?

a) Nothing — new variants are ignored  
b) The compiler gives an error on every non-exhaustive match, forcing you to handle the new case  
c) A runtime panic  
d) The wildcard `_` arm handles it automatically  

**Q5.** What does `option.map(|x| x * 2)` return?

a) The doubled value or 0  
b) `Some(doubled)` if Some, `None` if None — transforms the inner value without unwrapping  
c) A bool  
d) Panics if None  

---

## Quiz Answers

**A1.** b) Rust enum variants can hold associated data of different types  
*C enums are just named integers. Rust enums are algebraic data types — each variant can carry data of its own shape. This enables type-safe unions and eliminates entire categories of bugs.*

**A2.** b) The type system forces you to handle the "no value" case  
*If a function returns `Option<String>`, you cannot use it as `String` directly — the compiler rejects it. This eliminates null pointer exceptions at compile time.*

**A3.** b) Returns a bool — true if the value matches the given pattern  
*`matches!(x, Pattern)` is shorthand for `if let Pattern = x { true } else { false }`. It's cleaner when you only need a boolean.*

**A4.** b) The compiler gives an error on every non-exhaustive match  
*This is a major safety benefit. In any language without exhaustive matching, adding a new enum value silently falls through unhandled code paths. Rust's compiler finds every site that needs updating.*

**A5.** b) `Some(doubled)` if Some, `None` if None  
*`map` applies a transformation to the inner value if it exists, leaving `None` as `None`. It's the functional programming way to work with optional values without unwrapping.*

---

## Chapter Summary

- **Enums** define a type that is exactly one of its variants — perfect for states, events, and alternatives
- Enum variants can hold **no data**, **named fields** (struct-like), or **positional fields** (tuple-like)
- **`match` is exhaustive** — the compiler ensures every variant is handled; adding a new variant forces updates everywhere
- **`impl` on enums** works exactly like on structs — methods and associated functions
- **`Option<T>`** replaces null — `Some(T)` for a value, `None` for absence; the type system forces handling both
- Key Option methods: `map`, `and_then`, `unwrap_or`, `filter`, `is_some`, `is_none`
- **`matches!` macro** returns a bool for pattern checks — cleaner than `if let` for boolean tests
- Enums are the foundation of Rust's state machine patterns — transitions are pure functions returning new variants

**Part 3 Complete!** You can now define your own types with structs (data) and enums (alternatives), add behavior with `impl`, and compose them into rich domain models.

In Part 4, we explore Rust's standard collections: `String`, `Vec<T>`, and `HashMap<K, V>`.
