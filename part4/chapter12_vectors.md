# Chapter 12: Vectors

## Learning Objectives

By the end of this chapter, you will:

- Create and populate `Vec<T>` in multiple ways
- Access, modify, and iterate over vector elements safely
- Understand how Vec manages memory (capacity, reallocation)
- Use common Vec methods: push, pop, insert, remove, retain, sort, dedup
- Build a Todo List CLI using vectors

---

## Theory

### 12.1 Vec\<T\>

`Vec<T>` (pronounced "vector") is Rust's standard growable array. It stores elements of a single type `T` contiguously on the heap and can grow or shrink at runtime.

#### Creating a Vec

```rust
fn main() {
    // Empty vec with type annotation:
    let v: Vec<i32> = Vec::new();

    // Using the vec! macro:
    let v = vec![1, 2, 3, 4, 5];

    // With capacity pre-allocated (avoids reallocations):
    let mut v: Vec<i32> = Vec::with_capacity(100);
    println!("len={}, capacity={}", v.len(), v.capacity());

    // Collect from an iterator:
    let squares: Vec<i32> = (1..=5).map(|x| x * x).collect();
    println!("{:?}", squares);  // [1, 4, 9, 16, 25]

    // Repeat a value:
    let zeros = vec![0; 10];    // [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
    println!("{:?}", zeros);

    // From a slice:
    let arr = [10, 20, 30];
    let v: Vec<i32> = arr.to_vec();
    // OR:
    let v = arr.iter().cloned().collect::<Vec<_>>();
}
```

#### Memory Layout

```
Vec<i32>:
Stack:                    Heap:
┌──────────────────┐     ┌────┬────┬────┬────┬────┬────┐
│ ptr ──────────────┼────►│ 1  │ 2  │ 3  │ 4  │ 5  │    │
│ len: 5           │     └────┴────┴────┴────┴────┴────┘
│ cap: 6           │      ^                       ^
└──────────────────┘      │ used (len=5)          │ allocated (cap=6)
```

- **`len`**: number of elements currently in the Vec
- **`cap`**: total allocated space (may be larger than len)
- When `len == cap` and you push, Rust doubles the capacity (allocates new memory, copies elements)

#### Accessing Elements

```rust
fn main() {
    let v = vec![10, 20, 30, 40, 50];

    // Index (panics on out of bounds):
    println!("{}", v[0]);   // 10
    println!("{}", v[4]);   // 50
    // println!("{}", v[5]); // PANIC: index out of bounds

    // Safe access with get (returns Option):
    match v.get(2) {
        Some(x) => println!("element: {}", x),
        None    => println!("out of bounds"),
    }

    println!("{:?}", v.get(10));  // None (no panic)

    // First and last:
    println!("{:?}", v.first());  // Some(10)
    println!("{:?}", v.last());   // Some(50)
}
```

**Use `v[i]` when you know the index is valid. Use `v.get(i)` when you need to handle the out-of-bounds case.**

---

### 12.2 Iteration

#### Iterating Over a Vec

```rust
fn main() {
    let v = vec![100, 200, 300];

    // for loop — borrows elements:
    for x in &v {
        println!("{}", x);  // x is &i32
    }
    // v is still valid after the loop

    // for loop — moves (consumes) elements:
    let v2 = vec![1, 2, 3];
    for x in v2 {         // v2 is consumed
        println!("{}", x); // x is i32
    }
    // v2 is no longer valid

    // for loop — mutable references:
    let mut v3 = vec![1, 2, 3, 4, 5];
    for x in &mut v3 {
        *x *= 2;          // dereference to modify
    }
    println!("{:?}", v3);  // [2, 4, 6, 8, 10]

    // With index:
    let v4 = vec!["a", "b", "c"];
    for (i, val) in v4.iter().enumerate() {
        println!("{}: {}", i, val);
    }
}
```

#### Iterator Adaptors on Vec

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // filter:
    let evens: Vec<i32> = numbers.iter().filter(|&&x| x % 2 == 0).cloned().collect();
    println!("{:?}", evens);  // [2, 4, 6, 8, 10]

    // map:
    let doubled: Vec<i32> = numbers.iter().map(|&x| x * 2).collect();
    println!("{:?}", doubled);  // [2, 4, 6, 8, 10, 12, 14, 16, 18, 20]

    // filter + map:
    let even_squares: Vec<i32> = numbers.iter()
        .filter(|&&x| x % 2 == 0)
        .map(|&x| x * x)
        .collect();
    println!("{:?}", even_squares);  // [4, 16, 36, 64, 100]

    // fold (reduce):
    let sum: i32 = numbers.iter().sum();
    let product: i32 = numbers.iter().product();
    println!("sum={}, product={}", sum, product);

    // find:
    let first_big = numbers.iter().find(|&&x| x > 7);
    println!("{:?}", first_big);  // Some(8)

    // any / all:
    println!("{}", numbers.iter().any(|&x| x > 9));   // true
    println!("{}", numbers.iter().all(|&x| x > 0));   // true
}
```

---

### 12.3 Mutation

#### Modifying a Vec

```rust
fn main() {
    let mut v: Vec<i32> = Vec::new();

    // Push (append to end):
    v.push(1);
    v.push(2);
    v.push(3);
    println!("{:?}", v);  // [1, 2, 3]

    // Pop (remove from end, returns Option):
    let last = v.pop();
    println!("{:?}", last);  // Some(3)
    println!("{:?}", v);     // [1, 2]

    // Insert at index:
    v.insert(1, 99);          // insert 99 at index 1
    println!("{:?}", v);      // [1, 99, 2]

    // Remove by index (returns the element):
    let removed = v.remove(1);
    println!("removed: {}", removed);  // 99
    println!("{:?}", v);               // [1, 2]

    // Swap:
    let mut v = vec![1, 2, 3, 4, 5];
    v.swap(0, 4);
    println!("{:?}", v);  // [5, 2, 3, 4, 1]

    // Extend:
    let mut v1 = vec![1, 2, 3];
    let v2 = vec![4, 5, 6];
    v1.extend(&v2);   // v2 still valid (borrowed)
    v1.extend(v2);    // v2 consumed (moved)
    println!("{:?}", v1);

    // Clear:
    v1.clear();
    println!("len after clear: {}", v1.len());  // 0

    // Truncate:
    let mut v = vec![1, 2, 3, 4, 5];
    v.truncate(3);  // keep first 3 elements
    println!("{:?}", v);  // [1, 2, 3]

    // Retain (keep only matching elements):
    let mut v = vec![1, 2, 3, 4, 5, 6];
    v.retain(|&x| x % 2 == 0);
    println!("{:?}", v);  // [2, 4, 6]

    // Dedup (remove consecutive duplicates, sort first):
    let mut v = vec![1, 1, 2, 3, 3, 3, 4, 2, 2];
    v.dedup();
    println!("{:?}", v);  // [1, 2, 3, 4, 2] — only consecutive deduped

    let mut v = vec![1, 1, 2, 3, 3, 3, 4, 2, 2];
    v.sort();
    v.dedup();
    println!("{:?}", v);  // [1, 2, 3, 4] — all dupes removed
}
```

#### Sorting

```rust
fn main() {
    let mut v = vec![3, 1, 4, 1, 5, 9, 2, 6];

    // Sort ascending (in place):
    v.sort();
    println!("{:?}", v);  // [1, 1, 2, 3, 4, 5, 6, 9]

    // Sort descending:
    v.sort_by(|a, b| b.cmp(a));
    println!("{:?}", v);  // [9, 6, 5, 4, 3, 2, 1, 1]

    // Sort by key:
    let mut words = vec!["banana", "apple", "cherry", "date"];
    words.sort_by_key(|w| w.len());  // sort by length
    println!("{:?}", words);  // ["date", "apple", "banana", "cherry"]

    // Sort floats (use partial_cmp):
    let mut floats = vec![3.1, 1.4, 2.7, 0.5];
    floats.sort_by(|a, b| a.partial_cmp(b).unwrap());
    println!("{:?}", floats);  // [0.5, 1.4, 2.7, 3.1]
}
```

#### Splitting and Draining

```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5, 6];

    // Split at index (no allocation, returns slices):
    let (left, right) = v.split_at(3);
    println!("{:?} {:?}", left, right);  // [1, 2, 3] [4, 5, 6]

    // Windows and chunks (slice methods, work on Vec too):
    for window in v.windows(3) {
        print!("{:?} ", window);  // overlapping
    }
    println!();

    for chunk in v.chunks(2) {
        print!("{:?} ", chunk);   // non-overlapping
    }
    println!();

    // Drain (remove a range and return those elements):
    let mut v = vec![1, 2, 3, 4, 5];
    let drained: Vec<i32> = v.drain(1..3).collect();
    println!("drained: {:?}", drained);  // [2, 3]
    println!("remaining: {:?}", v);      // [1, 4, 5]
}
```

---

## Code Example

### Practice: Todo List

```rust
use std::io;
use std::io::Write;
use std::fmt;

#[derive(Debug, Clone)]
struct Task {
    id: u32,
    title: String,
    done: bool,
    priority: Priority,
}

#[derive(Debug, Clone, PartialEq)]
enum Priority { Low, Medium, High }

impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Priority::Low    => write!(f, "Low"),
            Priority::Medium => write!(f, "Medium"),
            Priority::High   => write!(f, "High"),
        }
    }
}

impl Task {
    fn new(id: u32, title: &str, priority: Priority) -> Task {
        Task { id, title: String::from(title), done: false, priority }
    }

    fn display(&self) {
        let status = if self.done { "✓" } else { "○" };
        println!("  [{}] #{} [{:<6}] {}", status, self.id, self.priority.to_string(), self.title);
    }
}

struct TodoList {
    tasks: Vec<Task>,
    next_id: u32,
}

impl TodoList {
    fn new() -> TodoList {
        TodoList { tasks: Vec::new(), next_id: 1 }
    }

    fn add(&mut self, title: &str, priority: Priority) -> u32 {
        let id = self.next_id;
        self.tasks.push(Task::new(id, title, priority));
        self.next_id += 1;
        id
    }

    fn complete(&mut self, id: u32) -> bool {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.done = true;
            true
        } else {
            false
        }
    }

    fn remove(&mut self, id: u32) -> bool {
        let before = self.tasks.len();
        self.tasks.retain(|t| t.id != id);
        self.tasks.len() < before
    }

    fn clear_done(&mut self) -> usize {
        let before = self.tasks.len();
        self.tasks.retain(|t| !t.done);
        before - self.tasks.len()
    }

    fn list_all(&self) {
        if self.tasks.is_empty() {
            println!("  (no tasks)");
            return;
        }
        let done = self.tasks.iter().filter(|t| t.done).count();
        println!("  Tasks: {} total, {} done, {} pending", self.tasks.len(), done, self.tasks.len() - done);
        println!();

        // Show pending first (sorted by priority: High > Medium > Low)
        let mut pending: Vec<&Task> = self.tasks.iter().filter(|t| !t.done).collect();
        pending.sort_by_key(|t| match t.priority { Priority::High => 0, Priority::Medium => 1, Priority::Low => 2 });

        let done_tasks: Vec<&Task> = self.tasks.iter().filter(|t| t.done).collect();

        if !pending.is_empty() {
            println!("  --- Pending ---");
            for t in &pending { t.display(); }
        }
        if !done_tasks.is_empty() {
            println!("  --- Done ---");
            for t in done_tasks { t.display(); }
        }
    }

    fn find_by_keyword(&self, keyword: &str) -> Vec<&Task> {
        let kw = keyword.to_ascii_lowercase();
        self.tasks.iter()
            .filter(|t| t.title.to_ascii_lowercase().contains(&kw))
            .collect()
    }

    fn stats(&self) {
        let total = self.tasks.len();
        let done = self.tasks.iter().filter(|t| t.done).count();
        let high = self.tasks.iter().filter(|t| t.priority == Priority::High && !t.done).count();
        println!("  Total: {}  Done: {}  Pending: {}  High priority: {}", total, done, total - done, high);
    }
}

fn prompt(msg: &str) -> String {
    print!("{}", msg);
    io::stdout().flush().unwrap();
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s.trim().to_string()
}

fn parse_priority(s: &str) -> Priority {
    match s.to_ascii_lowercase().as_str() {
        "h" | "high"   => Priority::High,
        "l" | "low"    => Priority::Low,
        _              => Priority::Medium,
    }
}

fn main() {
    let mut list = TodoList::new();

    // Seed
    list.add("Buy groceries", Priority::High);
    list.add("Read Rust Book chapter 12", Priority::Medium);
    list.add("Call dentist", Priority::High);
    list.add("Update resume", Priority::Low);
    list.add("Exercise", Priority::Medium);

    println!("=== Todo List ===");

    loop {
        println!("\n1.List  2.Add  3.Complete  4.Remove  5.Search  6.Clear done  7.Stats  q.Quit");
        match prompt("Choice: ").as_str() {
            "1" => { println!(); list.list_all(); }
            "2" => {
                let title = prompt("  Title: ");
                let pri = parse_priority(&prompt("  Priority (h/m/l): "));
                let id = list.add(&title, pri);
                println!("  Added task #{}", id);
            }
            "3" => {
                let id: u32 = match prompt("  Task ID: ").parse() {
                    Ok(n) => n,
                    Err(_) => { println!("  Invalid ID."); continue; }
                };
                if list.complete(id) { println!("  Task #{} marked done.", id); }
                else { println!("  Task not found."); }
            }
            "4" => {
                let id: u32 = match prompt("  Task ID to remove: ").parse() {
                    Ok(n) => n,
                    Err(_) => { println!("  Invalid ID."); continue; }
                };
                if list.remove(id) { println!("  Task #{} removed.", id); }
                else { println!("  Task not found."); }
            }
            "5" => {
                let kw = prompt("  Keyword: ");
                let results = list.find_by_keyword(&kw);
                if results.is_empty() { println!("  No matches."); }
                else { for t in results { t.display(); } }
            }
            "6" => {
                let n = list.clear_done();
                println!("  Cleared {} completed tasks.", n);
            }
            "7" => { list.stats(); }
            "q" | "quit" => { println!("Goodbye!"); break; }
            _ => println!("  Unknown option."),
        }
    }
}
```

### Line-by-Line Explanation

```rust
fn clear_done(&mut self) -> usize {
    let before = self.tasks.len();
    self.tasks.retain(|t| !t.done);
    before - self.tasks.len()
}
```
- `retain` keeps only elements matching the predicate — removes done tasks in-place
- Returns how many were removed by comparing lengths before and after

```rust
let mut pending: Vec<&Task> = self.tasks.iter().filter(|t| !t.done).collect();
pending.sort_by_key(|t| match t.priority { Priority::High => 0, Priority::Medium => 1, Priority::Low => 2 });
```
- `Vec<&Task>` — vector of references into `self.tasks`, no cloning
- `sort_by_key` assigns a numeric key for sorting — High=0 sorts before Medium=1 and Low=2
- The mapping converts an enum into a sortable integer without deriving `Ord`

```rust
fn find_by_keyword(&self, keyword: &str) -> Vec<&Task> {
    let kw = keyword.to_ascii_lowercase();
    self.tasks.iter()
        .filter(|t| t.title.to_ascii_lowercase().contains(&kw))
        .collect()
}
```
- Returns borrowed references — no allocation of new Task structs
- Case-insensitive search by lowercasing both query and title

---

## Common Mistakes

### Mistake 1: Holding an index reference while mutating

```rust
let mut v = vec![1, 2, 3];
let first = &v[0];   // immutable borrow
v.push(4);           // ERROR: mutable borrow while immutable borrow exists
println!("{}", first);
```

### Mistake 2: Using `remove` in a loop (shifts indices)

```rust
let mut v = vec![1, 2, 3, 4, 5];

// WRONG: after removing index 0, all others shift
for i in 0..v.len() {
    if v[i] % 2 == 0 {
        v.remove(i);  // might panic or skip elements
    }
}

// CORRECT: use retain
v.retain(|&x| x % 2 != 0);
```

### Mistake 3: Iterating with index manually when for works

```rust
// Avoid:
let mut i = 0;
while i < v.len() {
    println!("{}", v[i]);
    i += 1;
}

// Prefer:
for x in &v {
    println!("{}", x);
}
```

### Mistake 4: Unnecessarily collecting into Vec when iterating suffices

```rust
// Unnecessary allocation:
let doubled: Vec<i32> = v.iter().map(|&x| x * 2).collect();
for x in &doubled { println!("{}", x); }

// Better: iterate the map directly:
for x in v.iter().map(|&x| x * 2) {
    println!("{}", x);
}
```

---

## Best Practices

1. **Pre-allocate with `Vec::with_capacity`** when you know the expected size
2. **Use `retain` instead of `remove` in loops** — safer and more idiomatic
3. **Prefer `iter()` over manual index loops** — cleaner, bounds-safe
4. **Use `get()` for fallible access** — returns `Option<&T>` instead of panicking
5. **Collect into `Vec<&T>`** when you need a temporary reordering without cloning
6. **Use `sort_unstable`** when equal elements don't need stable order — it's faster

---

## Exercises

### Exercise 1: Vec Statistics

Given a `Vec<f64>`, compute: min, max, mean, median, and mode. Handle the empty case.

### Exercise 2: Two Sum

Write `two_sum(nums: &[i32], target: i32) -> Option<(usize, usize)>` that returns the indices of two numbers that sum to target.

### Exercise 3: Flatten Nested

Given a `Vec<Vec<i32>>`, write `flatten(v: Vec<Vec<i32>>) -> Vec<i32>` that combines all inner vecs into one. Do it without `.iter().flatten()` (manually), then redo it with iterators.

### Exercise 4: Rotate

Write `rotate_left(v: &mut Vec<i32>, n: usize)` that rotates the vector left by n positions in-place. `[1,2,3,4,5]` rotated left by 2 → `[3,4,5,1,2]`.

### Exercise 5: Group by Predicate

Write `partition_by<T, F: Fn(&T) -> bool>(v: Vec<T>, predicate: F) -> (Vec<T>, Vec<T>)` that splits a Vec into two: elements matching the predicate, and the rest. (Hint: `Vec::partition` exists, but implement it manually first.)

---

## Solutions

### Solution 1

```rust
fn stats(v: &[f64]) -> Option<(f64, f64, f64, f64)> {
    if v.is_empty() { return None; }
    let min = v.iter().cloned().fold(f64::INFINITY, f64::min);
    let max = v.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let mean = v.iter().sum::<f64>() / v.len() as f64;
    let mut sorted = v.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let median = if sorted.len() % 2 == 0 {
        (sorted[sorted.len()/2 - 1] + sorted[sorted.len()/2]) / 2.0
    } else {
        sorted[sorted.len()/2]
    };
    Some((min, max, mean, median))
}

fn main() {
    let data = vec![3.0, 1.0, 4.0, 1.0, 5.0, 9.0, 2.0, 6.0];
    if let Some((min, max, mean, median)) = stats(&data) {
        println!("min={}, max={}, mean={:.2}, median={:.1}", min, max, mean, median);
    }
}
```

### Solution 2

```rust
fn two_sum(nums: &[i32], target: i32) -> Option<(usize, usize)> {
    for i in 0..nums.len() {
        for j in (i+1)..nums.len() {
            if nums[i] + nums[j] == target {
                return Some((i, j));
            }
        }
    }
    None
}

fn main() {
    let nums = vec![2, 7, 11, 15];
    println!("{:?}", two_sum(&nums, 9));   // Some((0, 1))
    println!("{:?}", two_sum(&nums, 100)); // None
}
```

### Solution 3

```rust
fn flatten_manual(v: Vec<Vec<i32>>) -> Vec<i32> {
    let mut result = Vec::new();
    for inner in v {
        for x in inner {
            result.push(x);
        }
    }
    result
}

fn flatten_iter(v: Vec<Vec<i32>>) -> Vec<i32> {
    v.into_iter().flatten().collect()
}

fn main() {
    let nested = vec![vec![1, 2, 3], vec![4, 5], vec![6, 7, 8, 9]];
    println!("{:?}", flatten_iter(nested));  // [1, 2, 3, 4, 5, 6, 7, 8, 9]
}
```

### Solution 4

```rust
fn rotate_left(v: &mut Vec<i32>, n: usize) {
    if v.is_empty() { return; }
    let n = n % v.len();
    v.rotate_left(n);  // stdlib method
    // Manual implementation:
    // let tail = v.split_off(n);
    // v.splice(0..0, tail);  or: v = [&v[n..], &v[..n]].concat()
}

fn main() {
    let mut v = vec![1, 2, 3, 4, 5];
    rotate_left(&mut v, 2);
    println!("{:?}", v);  // [3, 4, 5, 1, 2]
}
```

### Solution 5

```rust
fn partition_by<T, F: Fn(&T) -> bool>(v: Vec<T>, predicate: F) -> (Vec<T>, Vec<T>) {
    let mut yes = Vec::new();
    let mut no  = Vec::new();
    for item in v {
        if predicate(&item) { yes.push(item); }
        else { no.push(item); }
    }
    (yes, no)
}

fn main() {
    let nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let (evens, odds) = partition_by(nums, |x| x % 2 == 0);
    println!("evens: {:?}", evens);  // [2, 4, 6, 8, 10]
    println!("odds:  {:?}", odds);   // [1, 3, 5, 7, 9]
}
```

---

## Quiz

**Q1.** What does `Vec::with_capacity(100)` do?

a) Creates a Vec with 100 zero elements  
b) Creates an empty Vec pre-allocating heap space for 100 elements to avoid reallocations  
c) Limits the Vec to 100 elements  
d) Fills the Vec with 100 default values  

**Q2.** What is the difference between `v.remove(i)` and `v.swap_remove(i)`?

a) No difference  
b) `remove` preserves order (O(n)); `swap_remove` swaps with the last element and pops (O(1)) — doesn't preserve order  
c) `swap_remove` is deprecated  
d) `remove` is O(1)  

**Q3.** Why does `v.retain(|x| condition)` fail inside a `for` loop iterating `v`?

a) `retain` doesn't exist  
b) You cannot mutably borrow while iterating — but `retain` is called on the Vec directly, not inside a `for x in &v` loop  
c) The loop would crash due to shifting indices  
d) `retain` needs an immutable reference  

**Q4.** What does `v.iter()` return compared to `v.into_iter()`?

a) Same result  
b) `iter()` yields `&T` (borrows); `into_iter()` yields `T` (consumes the Vec)  
c) `into_iter()` is faster  
d) `iter()` consumes the Vec  

**Q5.** What happens if you call `push` on a Vec when `len == capacity`?

a) Panics — out of memory  
b) The push is ignored  
c) Rust allocates a new, larger buffer (typically double), copies all elements, then pushes  
d) The capacity is increased by 1  

---

## Quiz Answers

**A1.** b) Creates an empty Vec pre-allocating heap space for 100 elements  
*The Vec has `len=0` but `capacity=100`. The first 100 `push` calls won't reallocate. This is a performance optimization when you know the expected size.*

**A2.** b) `remove` preserves order (O(n) shift); `swap_remove` is O(1) but changes order  
*`v.remove(i)` shifts all elements after index i left — O(n). `v.swap_remove(i)` puts the last element in position i and pops — O(1). Use `swap_remove` when order doesn't matter.*

**A3.** b) Cannot mutably borrow while iterating  
*`retain` takes `&mut self` — a mutable borrow. If you're already borrowing via `for x in &v`, you can't also mutably borrow. `retain` must be called outside the loop, or use a different pattern.*

**A4.** b) `iter()` yields `&T`; `into_iter()` yields `T` and consumes the Vec  
*After `for x in v.iter()`, `v` is still valid (borrowed). After `for x in v` (or `v.into_iter()`), `v` is consumed and invalid. Use `iter()` when you need `v` afterwards.*

**A5.** c) Rust allocates new memory, copies all elements, then pushes  
*This is called "amortized O(1)" growth. Doubling the capacity means reallocations happen logarithmically rarely as the Vec grows. Pre-allocating with `with_capacity` avoids this entirely.*

---

## Chapter Summary

- `Vec<T>` is a growable, heap-allocated sequence — the workhorse collection of Rust
- Create with `Vec::new()`, `vec![...]`, `Vec::with_capacity(n)`, or `.collect()`
- Access safely with `.get(i)` → `Option<&T>` or unsafely with `v[i]` (panics on OOB)
- Iterate with `&v` (borrow), `v` (consume), or `&mut v` (mutate)
- **Mutation**: `push`, `pop`, `insert`, `remove`, `retain`, `sort`, `dedup`, `drain`, `extend`
- **`retain`** is the idiomatic way to remove elements matching a condition
- **`sort_by_key`** is the clean way to sort by a derived property
- **`with_capacity`** avoids reallocations when you know the expected size
- `iter()` borrows (`&T`), `iter_mut()` mutably borrows (`&mut T`), `into_iter()` consumes (`T`)

In Chapter 13, we explore `HashMap<K, V>` — Rust's key-value store for efficient lookups.
