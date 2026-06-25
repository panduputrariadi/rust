# Chapter 18: Modules and Crates

## Learning Objectives

By the end of this chapter, you will be able to:

- Explain what modules are in Rust and why they exist
- Declare and nest modules using the `mod` keyword
- Control visibility with `pub`, `pub(crate)`, and `pub(super)`
- Bring items into scope using `use`, `use as`, and glob imports
- Organize Rust code across multiple files using the Rust 2018 edition file layout
- Understand the difference between a module and a crate
- Describe what a Cargo workspace is and when to use one
- Build a multi-module application from scratch
- Complete a full mini project: a Library Management System split across multiple modules

---

## Theory

### 18.1 Modules

#### What Problem Do Modules Solve?

Imagine writing a 5,000-line Rust program entirely in one file. Every function, every type, every constant lives in the same flat namespace. You would face three serious problems:

1. **Name collision** — Two functions named `parse` or `validate` would conflict.
2. **Cognitive overload** — Every reader of every function must understand the entire program to understand any one part.
3. **Poor encapsulation** — Internal implementation details are exposed globally. A caller can accidentally use a helper that was never meant to be a public API.

Modules are Rust's answer to all three problems. A module is a named container for items (functions, types, constants, other modules). It creates:

- A **namespace** — items inside the module are accessed by a path like `inventory::Product`.
- An **encapsulation boundary** — items are private by default. Only what you explicitly mark `pub` becomes part of the public API.
- A **logical grouping** — readers can understand the structure of the program at a high level before diving into details.

#### Comparison with Other Languages

| Concept | Rust | Python | Java |
|---|---|---|---|
| Grouping unit | `mod` block or file | `.py` file | `.java` file / package |
| Visibility default | **private** | public (name mangling with `_`) | package-private |
| Namespace syntax | `crate::module::item` | `module.item` | `com.example.package.Class` |
| Declaration needed | Yes — `mod foo;` | No — import auto-discovers | No — directory = package |

The key Rust difference: **you must explicitly declare that a module exists**. Python will find a `.py` file automatically. Java will find a `.java` file automatically. Rust will not. You must write `mod foo;` to tell the compiler "there is a module named `foo`, go find it."

#### Declaring an Inline Module

The simplest form of a module is declared directly inside a file:

```rust
mod greetings {
    pub fn hello() {
        println!("Hello!");
    }

    fn internal_helper() {
        // private — cannot be called from outside this module
    }
}

fn main() {
    greetings::hello();
    // greetings::internal_helper(); // ERROR: function is private
}
```

The `mod greetings { ... }` syntax creates a module whose body is inlined. The `::` operator is the path separator — it reads "inside of". `greetings::hello` means "the item named `hello` inside the module named `greetings`".

#### Declaring a File Module

For larger codebases, you split modules into separate files. Rust 2018 edition (the current standard) uses this layout:

```
src/
  main.rs          ← crate root (or lib.rs for a library)
  greetings.rs     ← module body for `mod greetings;`
  inventory/
    mod.rs         ← (old style, still works)
    product.rs
```

Actually in Rust 2018+, there are **two valid styles** for a module that itself contains submodules:

**Style 1 (Rust 2018 preferred):** Named file, no `mod.rs`

```
src/
  main.rs
  inventory.rs        ← module body for `mod inventory;`
  inventory/
    product.rs        ← submodule body for `mod product;` inside inventory.rs
```

**Style 2 (Old style, Rust 2015):** Uses `mod.rs`

```
src/
  main.rs
  inventory/
    mod.rs            ← module body for `mod inventory;`
    product.rs        ← submodule body for `mod product;` inside mod.rs
```

Both work today. The 2018 style is preferred because you avoid a flood of files all named `mod.rs`, which makes editors and file searches confusing.

#### Module Paths

Every item in Rust has a full path. There are two root anchors:

- `crate::` — the root of the current crate
- `super::` — the parent module of the current module
- `self::` — the current module itself (sometimes needed for disambiguation)

```
crate
  └── main
  └── inventory
        └── product
              └── Product (struct)
              └── validate_isbn (private fn)
        └── service
              └── add_book (pub fn)
```

From `inventory::service`, you can write:
- `super::product::Product` — go up one level (to `inventory`), then into `product`
- `crate::inventory::product::Product` — absolute path from crate root

#### Nested Modules

Modules can nest arbitrarily:

```rust
mod network {
    pub mod http {
        pub mod request {
            pub fn get(url: &str) {
                println!("GET {}", url);
            }
        }
    }
}

fn main() {
    network::http::request::get("https://example.com");
}
```

Deep nesting is possible but not always desirable. Good Rust code rarely goes beyond two or three levels of nesting.

---

### 18.2 pub — Visibility Control

#### The Default: Private

Every item in Rust is **private by default** — this is the opposite of most languages. Private means: accessible within the module it is defined in, and within any child modules of that module.

```
Module A (defines `fn foo`)
  ├── Can call `foo`
  └── Module B (child of A)
        └── Can call `foo` (children can see parent privates)

Module C (sibling of A)
  └── CANNOT call `foo` (siblings cannot see each other's privates)
```

This is a deliberate design decision. Rust's philosophy: **everything is private until you explicitly decide to share it**. This makes APIs explicit and helps prevent accidental coupling.

#### `pub` — Public to Everyone

```rust
pub fn visible_everywhere() { }
```

Any code that can name this module can call this function. This includes other crates if this item is exported from a library crate.

#### `pub(crate)` — Public within This Crate Only

```rust
pub(crate) fn visible_in_this_crate_only() { }
```

This is extremely useful for internal APIs. You want multiple modules within your crate to share a helper, but you do not want external users of your library to see it. `pub(crate)` threads this needle perfectly.

```
Your library crate
  ├── Module auth — can call pub(crate) fn hash_password()
  ├── Module users — can call pub(crate) fn hash_password()
  └── External users of your library — CANNOT call hash_password()
```

#### `pub(super)` — Public to the Parent Module Only

```rust
pub(super) fn visible_to_parent_only() { }
```

Less common but useful when you have an internal submodule that needs to expose something to its immediate parent but not to the whole crate.

```
inventory (parent)           ← can see pub(super) items from product
  └── product (child)        ← defines pub(super) fn internal_validate()
        └── details (grandchild) ← CANNOT see pub(super) items from product
```

#### `pub(in path)` — Public to a Specific Ancestor

The most precise form. Rarely needed but exists for unusual module structures:

```rust
pub(in crate::inventory) fn visible_only_in_inventory_tree() { }
```

#### Summary of Visibility Levels

```
Visibility        | Who Can See It
------------------|--------------------------------------------------
(no pub)          | Current module + all child modules
pub(super)        | Parent module (and its children)
pub(crate)        | Anywhere inside this crate
pub               | Anywhere (including external crates)
pub(in path)      | Only modules under the given path
```

#### Struct Field Visibility

Visibility applies to struct fields independently:

```rust
pub struct User {
    pub username: String,       // public field
    pub(crate) email: String,   // crate-internal field
    password_hash: String,      // private field
}
```

A struct being `pub` does NOT make its fields public. Each field has its own visibility. This is a common source of confusion for beginners coming from other languages.

---

### 18.3 use — Bringing Items Into Scope

#### The Basic `use` Statement

Fully qualified paths are precise but verbose. Writing `crate::inventory::product::Product` every time is tedious. The `use` keyword brings an item into the current scope under a shorter name:

```rust
use crate::inventory::product::Product;

fn main() {
    let p = Product::new("Book", 29.99); // no need for full path
}
```

`use` does NOT copy or move anything. It is purely a compile-time directive that says "when I write `Product`, resolve it as `crate::inventory::product::Product`".

#### Conventional `use` Style

Rust has strong conventions about what level of path to import:

**For functions:** import the parent module, not the function itself:

```rust
// Preferred — makes it clear where the function comes from
use std::fmt;
fmt::Display

// Avoid — makes it unclear whether `format` is local or imported
use std::fmt::format;
format(...)
```

**For types, structs, enums:** import the type directly:

```rust
// Preferred
use std::collections::HashMap;
let map: HashMap<String, i32> = HashMap::new();
```

The exception is when two items from different modules have the same name — then you must disambiguate.

#### `use as` — Aliasing

When you need to import two items with the same name, or when a name is awkward, use `as`:

```rust
use std::fmt::Result as FmtResult;
use std::io::Result as IoResult;

fn format_something() -> FmtResult { Ok(()) }
fn read_file() -> IoResult<String> { Ok(String::new()) }
```

Without `as`, you cannot have both `Result` variants in scope simultaneously.

#### Nested Paths — Grouping Multiple Imports

Rather than writing many `use` lines:

```rust
// Verbose — avoid
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::BTreeMap;
```

Group them:

```rust
// Concise
use std::collections::{HashMap, HashSet, BTreeMap};
```

You can also combine a parent with children:

```rust
use std::{fmt, io};
use std::io::{self, Write}; // `self` brings `io` itself in addition to `Write`
```

#### Glob Imports — `use module::*`

A glob import brings ALL public items from a module into scope:

```rust
use std::collections::*;
```

This is **generally discouraged** in production code because it makes it unclear where any given name came from. However, it is idiomatic in two specific situations:

1. **Test modules** — `use super::*;` is the standard way to bring the entire parent module into a test module.
2. **Prelude modules** — Many crates define a `prelude` module meant to be glob-imported: `use my_crate::prelude::*;`

#### Re-exporting with `pub use`

You can import an item AND re-export it, making it available to external callers through a different path:

```rust
// In src/lib.rs
pub use crate::inventory::product::Product;

// External caller can now write:
use my_library::Product;
// instead of:
use my_library::inventory::product::Product;
```

This is a powerful API design tool. Your internal module structure can be as deep as you like, but you can present a clean, flat public API by re-exporting the right items from your crate root.

---

### 18.4 Crate Structure

#### What is a Crate?

A **crate** is the smallest compilation unit in Rust. Every time you run `cargo build`, Cargo compiles one or more crates. A crate is either:

- A **binary crate**: has a `main` function, compiles to an executable. Entry point: `src/main.rs`.
- A **library crate**: has no `main` function, compiles to a `.rlib` that other crates can depend on. Entry point: `src/lib.rs`.

A single Cargo package (`Cargo.toml`) can contain:
- One optional library crate (`src/lib.rs`)
- Multiple binary crates (`src/main.rs` and/or files in `src/bin/`)

#### The Crate Root

The **crate root** is the source file the compiler starts with. For a binary, it is `src/main.rs`. For a library, it is `src/lib.rs`. Every `mod` declaration must form a tree rooted here.

```
src/lib.rs                     ← crate root
  mod inventory;               ← must have src/inventory.rs OR src/inventory/mod.rs
    mod product;               ← must have src/inventory/product.rs
    mod service;               ← must have src/inventory/service.rs
  mod users;                   ← must have src/users.rs
```

The compiler does NOT automatically discover files. If `src/forgotten_module.rs` exists but you never wrote `mod forgotten_module;`, that file is completely ignored.

#### Cargo.toml — The Package Manifest

```toml
[package]
name = "my_library"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = { version = "1", features = ["derive"] }

[dev-dependencies]
pretty_assertions = "1"
```

Key fields:
- `[package]` — metadata about this crate
- `[dependencies]` — external crates your code needs at runtime
- `[dev-dependencies]` — external crates needed only for tests and benchmarks
- `[build-dependencies]` — crates needed by the build script (`build.rs`)

#### Typical File Structure for a Library Crate

```
my_library/
  Cargo.toml
  src/
    lib.rs              ← crate root, declares modules and re-exports
    models/
      mod.rs or models.rs
      user.rs
      product.rs
    services/
      services.rs or services/mod.rs
      user_service.rs
      product_service.rs
    errors.rs
    utils.rs
```

#### The `extern crate` Keyword (Historical Note)

In Rust 2015 edition, you had to write `extern crate serde;` to use external crates. In Rust 2018+, this is no longer needed — the compiler automatically makes declared dependencies available. You will see `extern crate` in old code, but do not write it in new code.

---

### 18.5 Workspace

#### What is a Workspace?

As projects grow, a single crate is not always enough. You may need to split your project into:

- `core` — shared types and traits
- `server` — the HTTP API binary
- `cli` — a command-line tool
- `client` — an SDK for external users

Rather than making each a separate Git repository, Rust lets you group them into a **workspace**. A workspace is a directory containing a root `Cargo.toml` that lists member crates.

#### ASCII Diagram — Workspace Structure

```
my_workspace/                    ← workspace root
  Cargo.toml                     ← workspace manifest
  target/                        ← shared build output (ONE target dir for all)
  Cargo.lock                     ← shared lock file
  core/
    Cargo.toml
    src/
      lib.rs
  server/
    Cargo.toml
    src/
      main.rs
  cli/
    Cargo.toml
    src/
      main.rs
```

#### Workspace Root Cargo.toml

The workspace root `Cargo.toml` does NOT define a package — it only lists members:

```toml
[workspace]
members = [
    "core",
    "server",
    "cli",
]
```

#### Member Crate Cargo.toml

Each member has its own `Cargo.toml`:

```toml
# server/Cargo.toml
[package]
name = "server"
version = "0.1.0"
edition = "2021"

[dependencies]
core = { path = "../core" }     # depends on sibling workspace member
tokio = { version = "1", features = ["full"] }
```

The `path = "../core"` dependency tells Cargo to use the local `core` crate rather than downloading one from crates.io.

#### Benefits of a Workspace

1. **Shared `target/` directory** — Cargo compiles each dependency only once even if multiple workspace members need it. Without a workspace, you would compile `serde` once per member.
2. **Shared `Cargo.lock`** — All members use the same locked dependency versions. This prevents the impossible-to-debug situation where `server` uses `serde 1.0.150` and `cli` uses `serde 1.0.152` and they disagree on struct layouts.
3. **Cross-crate refactoring** — `cargo test --workspace` runs tests in all members. `cargo build --workspace` builds all.
4. **Logical separation** — The `core` crate has no knowledge of HTTP or CLI. The `server` crate has no knowledge of CLI specifics. Each crate has a single responsibility.

#### When to Use a Workspace vs a Single Crate

| Situation | Recommendation |
|---|---|
| Small project, one binary | Single crate, multiple modules |
| Medium project, one library + one binary | Single package with `src/lib.rs` + `src/main.rs` |
| Large project, multiple binaries sharing code | Workspace |
| Open-source library with integration tests | Workspace (separate `integration_tests` member) |
| Monorepo with microservices | Workspace |

---

## Code Example

The following complete example demonstrates a multi-file module system. We will build a small `bookstore` library with modules for models, services, and errors, plus a binary that uses it.

### File Tree

```
bookstore/
  Cargo.toml
  src/
    lib.rs
    errors.rs
    models/
      mod.rs          (we use mod.rs style here to show both styles)
      book.rs
      author.rs
    services/
      book_service.rs
      author_service.rs
    services.rs       (declares the services submodules)
    utils.rs
  src/bin/
    main.rs
```

### `Cargo.toml`

```toml
[package]
name = "bookstore"
version = "0.1.0"
edition = "2021"

[[bin]]
name = "bookstore_cli"
path = "src/bin/main.rs"
```

### `src/lib.rs`

```rust
// src/lib.rs
// This is the crate root for the bookstore library.
// It declares which modules exist and re-exports the public API.

pub mod errors;
pub mod models;
pub mod services;
pub(crate) mod utils;

// Re-export the most commonly used items at the crate root
// so callers can write `bookstore::Book` instead of
// `bookstore::models::book::Book`.
pub use models::book::Book;
pub use models::author::Author;
pub use errors::BookstoreError;
```

### `src/errors.rs`

```rust
// src/errors.rs
// Defines the error type used throughout the crate.
// Having a single errors.rs avoids circular imports.

use std::fmt;

#[derive(Debug)]
pub enum BookstoreError {
    NotFound(String),
    DuplicateIsbn(String),
    InvalidIsbn(String),
    InvalidTitle(String),
}

impl fmt::Display for BookstoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BookstoreError::NotFound(msg) => write!(f, "Not found: {}", msg),
            BookstoreError::DuplicateIsbn(isbn) => write!(f, "Duplicate ISBN: {}", isbn),
            BookstoreError::InvalidIsbn(isbn) => write!(f, "Invalid ISBN: {}", isbn),
            BookstoreError::InvalidTitle(title) => write!(f, "Invalid title: {}", title),
        }
    }
}

impl std::error::Error for BookstoreError {}
```

### `src/models/mod.rs`

```rust
// src/models/mod.rs
// This file makes `models` a module that itself contains submodules.
// It declares which submodules exist inside `models`.

pub mod book;
pub mod author;
```

### `src/models/author.rs`

```rust
// src/models/author.rs

#[derive(Debug, Clone)]
pub struct Author {
    pub id: u32,
    pub name: String,
    pub birth_year: u16,
}

impl Author {
    pub fn new(id: u32, name: &str, birth_year: u16) -> Self {
        Author {
            id,
            name: name.to_string(),
            birth_year,
        }
    }
}

impl std::fmt::Display for Author {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} (b. {})", self.name, self.birth_year)
    }
}
```

### `src/models/book.rs`

```rust
// src/models/book.rs

use crate::models::author::Author;

#[derive(Debug, Clone)]
pub struct Book {
    pub isbn: String,
    pub title: String,
    pub author: Author,
    pub price: f64,
    pub(crate) in_stock: bool,
}

impl Book {
    pub fn new(isbn: &str, title: &str, author: Author, price: f64) -> Self {
        Book {
            isbn: isbn.to_string(),
            title: title.to_string(),
            author,
            price,
            in_stock: true,
        }
    }

    pub fn is_available(&self) -> bool {
        self.in_stock
    }
}

impl std::fmt::Display for Book {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}] {} by {} — ${:.2} {}",
            self.isbn,
            self.title,
            self.author.name,
            self.price,
            if self.in_stock { "(in stock)" } else { "(out of stock)" }
        )
    }
}
```

### `src/utils.rs`

```rust
// src/utils.rs
// Internal utilities. pub(crate) means only code in this crate can use these.

/// Validates an ISBN-13 format (simplified: must be 13 digits).
pub(crate) fn validate_isbn(isbn: &str) -> bool {
    isbn.chars().all(|c| c.is_ascii_digit()) && isbn.len() == 13
}

/// Validates a title is non-empty and under 200 characters.
pub(crate) fn validate_title(title: &str) -> bool {
    !title.trim().is_empty() && title.len() <= 200
}
```

### `src/services.rs`

```rust
// src/services.rs
// Declares the service submodules.
// This file is the module body for `pub mod services;` in lib.rs.

pub mod book_service;
pub mod author_service;
```

### `src/services/book_service.rs`

```rust
// src/services/book_service.rs

use crate::errors::BookstoreError;
use crate::models::book::Book;
use crate::utils::{validate_isbn, validate_title};

pub struct BookService {
    books: Vec<Book>,
}

impl BookService {
    pub fn new() -> Self {
        BookService { books: Vec::new() }
    }

    pub fn add_book(&mut self, book: Book) -> Result<(), BookstoreError> {
        if !validate_isbn(&book.isbn) {
            return Err(BookstoreError::InvalidIsbn(book.isbn.clone()));
        }
        if !validate_title(&book.title) {
            return Err(BookstoreError::InvalidTitle(book.title.clone()));
        }
        if self.books.iter().any(|b| b.isbn == book.isbn) {
            return Err(BookstoreError::DuplicateIsbn(book.isbn.clone()));
        }
        self.books.push(book);
        Ok(())
    }

    pub fn find_by_isbn(&self, isbn: &str) -> Result<&Book, BookstoreError> {
        self.books
            .iter()
            .find(|b| b.isbn == isbn)
            .ok_or_else(|| BookstoreError::NotFound(format!("ISBN {}", isbn)))
    }

    pub fn list_all(&self) -> &[Book] {
        &self.books
    }

    pub fn remove_by_isbn(&mut self, isbn: &str) -> Result<Book, BookstoreError> {
        let pos = self
            .books
            .iter()
            .position(|b| b.isbn == isbn)
            .ok_or_else(|| BookstoreError::NotFound(format!("ISBN {}", isbn)))?;
        Ok(self.books.remove(pos))
    }
}

impl Default for BookService {
    fn default() -> Self {
        Self::new()
    }
}
```

### `src/services/author_service.rs`

```rust
// src/services/author_service.rs

use crate::errors::BookstoreError;
use crate::models::author::Author;

pub struct AuthorService {
    authors: Vec<Author>,
    next_id: u32,
}

impl AuthorService {
    pub fn new() -> Self {
        AuthorService {
            authors: Vec::new(),
            next_id: 1,
        }
    }

    pub fn add_author(&mut self, name: &str, birth_year: u16) -> Author {
        let author = Author::new(self.next_id, name, birth_year);
        self.next_id += 1;
        self.authors.push(author.clone());
        author
    }

    pub fn find_by_id(&self, id: u32) -> Result<&Author, BookstoreError> {
        self.authors
            .iter()
            .find(|a| a.id == id)
            .ok_or_else(|| BookstoreError::NotFound(format!("author id {}", id)))
    }

    pub fn list_all(&self) -> &[Author] {
        &self.authors
    }
}

impl Default for AuthorService {
    fn default() -> Self {
        Self::new()
    }
}
```

### `src/bin/main.rs`

```rust
// src/bin/main.rs
// The binary crate entry point.
// It uses the bookstore library crate.

use bookstore::services::book_service::BookService;
use bookstore::services::author_service::AuthorService;
use bookstore::Book;   // re-exported from lib.rs
use bookstore::BookstoreError;

fn main() {
    let mut author_svc = AuthorService::new();
    let mut book_svc = BookService::new();

    // Add some authors
    let tolkien = author_svc.add_author("J.R.R. Tolkien", 1892);
    let orwell = author_svc.add_author("George Orwell", 1903);

    println!("=== Authors ===");
    for author in author_svc.list_all() {
        println!("  {}", author);
    }

    // Add some books
    let book1 = Book::new("9780261103573", "The Lord of the Rings", tolkien.clone(), 29.99);
    let book2 = Book::new("9780451524935", "1984", orwell.clone(), 9.99);
    let bad_book = Book::new("123", "Invalid Book", tolkien.clone(), 5.00); // bad ISBN

    match book_svc.add_book(book1) {
        Ok(()) => println!("\nAdded: The Lord of the Rings"),
        Err(e) => println!("\nError: {}", e),
    }

    match book_svc.add_book(book2) {
        Ok(()) => println!("Added: 1984"),
        Err(e) => println!("Error: {}", e),
    }

    match book_svc.add_book(bad_book) {
        Ok(()) => println!("Added: Invalid Book"),
        Err(e) => println!("Error adding invalid book: {}", e),
    }

    println!("\n=== All Books ===");
    for book in book_svc.list_all() {
        println!("  {}", book);
    }

    // Search
    println!("\n=== Search by ISBN ===");
    match book_svc.find_by_isbn("9780451524935") {
        Ok(book) => println!("Found: {}", book),
        Err(e) => println!("Error: {}", e),
    }

    match book_svc.find_by_isbn("0000000000000") {
        Ok(book) => println!("Found: {}", book),
        Err(BookstoreError::NotFound(msg)) => println!("Not found: {}", msg),
        Err(e) => println!("Error: {}", e),
    }
}
```

---

## Line-by-Line Explanation

### `src/lib.rs`

```rust
pub mod errors;
```
Declares a public module named `errors`. The compiler looks for `src/errors.rs` or `src/errors/mod.rs`. The `pub` makes this module visible to external users of the library.

```rust
pub mod models;
```
Declares the `models` module. Since `models` contains submodules, the compiler finds `src/models/mod.rs` (old style) which in turn declares `pub mod book;` and `pub mod author;`.

```rust
pub mod services;
```
Declares the `services` module. Because we are using the 2018 style here, the compiler finds `src/services.rs` (not `src/services/mod.rs`). That file declares the service submodules.

```rust
pub(crate) mod utils;
```
The `utils` module is crate-internal. External users of the `bookstore` library cannot access `validate_isbn` or `validate_title`. This prevents them from depending on internal implementation details that we might change.

```rust
pub use models::book::Book;
```
This re-exports `Book` at the crate root. External users write `use bookstore::Book;` — they do not need to know about the internal `models::book` path. This is called a "flat API" style, common in well-designed crates.

### `src/models/book.rs`

```rust
use crate::models::author::Author;
```
`crate::` is an absolute path from the crate root. This is the preferred way to write cross-module imports inside a crate — it is unambiguous regardless of where this file lives.

```rust
pub(crate) in_stock: bool,
```
The `in_stock` field is crate-internal. Users of the library see a `Book` struct and can call `is_available()`, but they cannot directly read or write `in_stock`. This preserves our ability to change the internal representation (maybe `in_stock` becomes an `Option<u32>` representing quantity).

### `src/utils.rs`

```rust
pub(crate) fn validate_isbn(isbn: &str) -> bool {
```
These functions are `pub(crate)` — visible anywhere inside the `bookstore` crate, but invisible to external callers. `book_service.rs` uses them via `use crate::utils::{validate_isbn, validate_title};`.

### `src/services.rs`

```rust
pub mod book_service;
pub mod author_service;
```
Because `src/services.rs` is the module body for `services`, and `services` contains further submodules, the compiler looks for `src/services/book_service.rs` and `src/services/author_service.rs`.

### `src/bin/main.rs`

```rust
use bookstore::services::book_service::BookService;
```
The binary crate imports from the library crate `bookstore`. Because `BookService` is not re-exported at the crate root, callers use the full internal path.

```rust
use bookstore::Book;
```
`Book` IS re-exported at the crate root via `pub use` in `lib.rs`, so this short import works.

---

## Common Mistakes

### 1. Forgetting to Declare `mod`

**Symptom:** You create `src/helpers.rs` and write code in it. You import from it. The compiler says the module does not exist.

**Cause:** The compiler does not automatically discover files. You MUST write `mod helpers;` in the crate root (or the appropriate parent module).

**Fix:**
```rust
// src/lib.rs or src/main.rs
mod helpers; // ADD THIS LINE
```

### 2. Confusing `mod` and `use`

Beginners often write:
```rust
use utils; // WRONG — this does not declare the module
```

`use` brings an already-declared module's items into scope. `mod` declares that a module exists. You need BOTH:
```rust
mod utils;         // declare the module exists
use utils::something; // bring an item into scope
```

### 3. Privacy Confusion — Parent Cannot See Child's Private Items

A common mental model mistake: "the parent module created me, so it can see everything inside me."

This is WRONG. Privacy in Rust flows DOWNWARD (parents can see children's private items? No — actually the rule is: private items are visible to the module they are defined in AND to descendants).

Wait — let's be precise:

```
Module A defines `fn private_fn()`
  ├── Module A itself CAN call private_fn
  └── Module B (child of A) CAN also call private_fn
            (children can access parent's private items)

Module C (sibling of A, or anywhere else) CANNOT call private_fn
```

The surprise for many is direction: a CHILD can see its parent's privates, but a PARENT cannot see its children's privates without `pub`.

### 4. Struct is `pub` but Fields Are Not

```rust
pub struct Config {
    timeout: u32,  // private! even though Config is pub
}

// External code:
let c = Config { timeout: 30 }; // ERROR: field `timeout` is private
```

Fix: Either mark fields `pub`, or provide a constructor `pub fn new(timeout: u32) -> Config`.

### 5. Circular Module Dependencies

Modules that depend on each other circularly will cause compilation errors:

```
module_a uses module_b
module_b uses module_a    ← CIRCULAR — compiler error
```

Fix: Extract shared types into a third module (often called `types`, `models`, or `common`) that both depend on. Neither depends on the other.

```
models (shared types)
  ↑         ↑
module_a   module_b
```

### 6. Using `mod.rs` Inconsistently with New Style

If you mix old style (`mod.rs`) and new style within the same directory, you get confusing errors. Pick one style and stick with it for a given directory.

### 7. Forgetting `pub` on a Module Re-export

```rust
// src/lib.rs
mod models; // private module — external callers cannot access it at all
pub use models::Book; // ERROR: `models` is private, cannot re-export from it
```

Fix:
```rust
pub mod models; // or keep it private and re-export specific items correctly
```

Actually, Rust does allow re-exporting from a private module:
```rust
mod models;                   // private
pub use models::book::Book;   // re-export — this WORKS
```
The module `models` remains private, but `Book` is accessible through the re-export. External callers can use `Book` but cannot write `use mycrate::models::book::Book` — they can only use the re-exported path.

---

## Best Practices

### 1. One Concept per Module

Each module should have a single, clear responsibility. If you find yourself unsure whether a function belongs in module A or B, that is often a sign that those modules are not cleanly separated.

### 2. Start with `mod` in One File, Split When It Grows

Begin with inline modules:
```rust
// lib.rs
mod models { ... }
mod services { ... }
```

When a module grows beyond ~100–150 lines, extract it to its own file.

### 3. Design the Public API First

Before writing any implementation, think about what `pub use` statements you want in `lib.rs`. Design the API your callers will use. Then structure the internal modules to support that API. Internal structure is your concern; the public API is your user's concern.

### 4. Prefer `pub(crate)` Over `pub` for Internal Helpers

Any function that does not need to be part of the library's public API should be `pub(crate)` at most. This prevents accidental API stability commitments. Once you make something `pub`, changing or removing it is a breaking change for your users.

### 5. Use `pub use` to Create a Clean Public Interface

Deep module hierarchies are fine internally. Use `pub use` in `lib.rs` to flatten the public API:
```rust
// lib.rs
pub use models::user::User;
pub use models::product::Product;
pub use services::user_service::UserService;
pub use errors::AppError;
```

Now external callers write `use my_crate::User;` rather than navigating your internal hierarchy.

### 6. Keep `errors.rs` at the Crate Root

Having a single `errors.rs` (or `error.rs`) at the top level prevents circular dependency problems. If `models` defines errors and `services` imports from `models` and also defines errors that `models` needs, you have a circular dependency. A shared `errors` module avoids this.

### 7. Use `use super::*` in Tests

Inside `#[cfg(test)]` test modules, write:
```rust
#[cfg(test)]
mod tests {
    use super::*; // import everything from the parent module under test

    #[test]
    fn test_something() { ... }
}
```

This is the idiomatic Rust testing pattern.

---

## Practice: Split Application into Modules

The following exercise walks you through taking a monolithic single-file program and splitting it into a proper multi-module structure.

### Starting Point: `monolith.rs`

```rust
// Everything in one file — messy and hard to maintain

struct Product {
    name: String,
    price: f64,
    quantity: u32,
}

impl Product {
    fn new(name: &str, price: f64, quantity: u32) -> Self {
        Product { name: name.to_string(), price, quantity }
    }
    fn total_value(&self) -> f64 {
        self.price * self.quantity as f64
    }
}

struct Inventory {
    products: Vec<Product>,
}

impl Inventory {
    fn new() -> Self { Inventory { products: Vec::new() } }
    fn add(&mut self, p: Product) { self.products.push(p); }
    fn total_value(&self) -> f64 {
        self.products.iter().map(|p| p.total_value()).sum()
    }
    fn find_by_name(&self, name: &str) -> Option<&Product> {
        self.products.iter().find(|p| p.name == name)
    }
}

fn validate_price(price: f64) -> bool { price > 0.0 }

fn format_currency(amount: f64) -> String { format!("${:.2}", amount) }

fn main() {
    let mut inv = Inventory::new();
    inv.add(Product::new("Widget", 9.99, 100));
    inv.add(Product::new("Gadget", 24.99, 50));

    println!("Total value: {}", format_currency(inv.total_value()));
    if let Some(p) = inv.find_by_name("Widget") {
        println!("Found: {} at {}", p.name, format_currency(p.price));
    }
}
```

### Target Structure

```
inventory_app/
  Cargo.toml
  src/
    main.rs
    models/
      mod.rs
      product.rs
    services/
      mod.rs
      inventory_service.rs
    utils.rs
```

### Step 1: Create the Cargo project

```bash
cargo new inventory_app
cd inventory_app
mkdir -p src/models src/services
```

### Step 2: `src/utils.rs`

```rust
pub fn validate_price(price: f64) -> bool {
    price > 0.0
}

pub fn format_currency(amount: f64) -> String {
    format!("${:.2}", amount)
}
```

### Step 3: `src/models/product.rs`

```rust
#[derive(Debug, Clone)]
pub struct Product {
    pub name: String,
    pub price: f64,
    pub quantity: u32,
}

impl Product {
    pub fn new(name: &str, price: f64, quantity: u32) -> Self {
        Product {
            name: name.to_string(),
            price,
            quantity,
        }
    }

    pub fn total_value(&self) -> f64 {
        self.price * self.quantity as f64
    }
}
```

### Step 4: `src/models/mod.rs`

```rust
pub mod product;
```

### Step 5: `src/services/inventory_service.rs`

```rust
use crate::models::product::Product;
use crate::utils::validate_price;

pub struct InventoryService {
    products: Vec<Product>,
}

impl InventoryService {
    pub fn new() -> Self {
        InventoryService { products: Vec::new() }
    }

    pub fn add(&mut self, product: Product) -> Result<(), String> {
        if !validate_price(product.price) {
            return Err(format!("Invalid price for '{}'", product.name));
        }
        self.products.push(product);
        Ok(())
    }

    pub fn total_value(&self) -> f64 {
        self.products.iter().map(|p| p.total_value()).sum()
    }

    pub fn find_by_name(&self, name: &str) -> Option<&Product> {
        self.products.iter().find(|p| p.name == name)
    }
}
```

### Step 6: `src/services/mod.rs`

```rust
pub mod inventory_service;
```

### Step 7: `src/main.rs`

```rust
mod models;
mod services;
mod utils;

use services::inventory_service::InventoryService;
use models::product::Product;
use utils::format_currency;

fn main() {
    let mut inv = InventoryService::new();
    inv.add(Product::new("Widget", 9.99, 100)).unwrap();
    inv.add(Product::new("Gadget", 24.99, 50)).unwrap();

    println!("Total value: {}", format_currency(inv.total_value()));

    if let Some(p) = inv.find_by_name("Widget") {
        println!("Found: {} at {}", p.name, format_currency(p.price));
    }
}
```

The logic is identical to the monolith. The difference: each piece of logic lives in its own file, owns a clear responsibility, and has explicit visibility. Adding a new service, model, or utility no longer requires touching existing files — you add a new file and declare it.

---

## Exercises

### Exercise 1: Module Declaration

Given this file tree, write the `mod` declarations needed in `src/main.rs` and `src/networking/mod.rs`:

```
src/
  main.rs
  networking/
    mod.rs
    http.rs
    websocket.rs
  storage.rs
```

### Exercise 2: Visibility

Given the following code, identify every compilation error and explain why it occurs:

```rust
mod outer {
    struct Secret {
        value: i32,
    }

    pub mod inner {
        pub fn reveal() -> i32 {
            let s = super::Secret { value: 42 };
            s.value
        }
    }
}

fn main() {
    let _ = outer::inner::reveal();
}
```

### Exercise 3: `use` Statements

Rewrite these imports using nested path syntax:

```rust
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::BTreeMap;
use std::io::Read;
use std::io::Write;
use std::io::BufReader;
```

### Exercise 4: Re-exporting

You have a library with the following internal structure:

```
src/lib.rs
src/types/user.rs      (defines pub struct User)
src/types/session.rs   (defines pub struct Session)
src/types/mod.rs
```

Write the `lib.rs` content to re-export `User` and `Session` at the crate root so callers can write:
```rust
use my_lib::User;
use my_lib::Session;
```

### Exercise 5: Workspace

Design (write the `Cargo.toml` files for) a workspace called `todo_platform` with three crates:
- `todo_core` — library with shared types
- `todo_api` — binary (HTTP server)
- `todo_cli` — binary (command-line tool)

Both `todo_api` and `todo_cli` depend on `todo_core`.

---

## Solutions

### Solution 1

`src/main.rs`:
```rust
mod networking;  // finds src/networking/mod.rs
mod storage;     // finds src/storage.rs
```

`src/networking/mod.rs`:
```rust
pub mod http;       // finds src/networking/http.rs
pub mod websocket;  // finds src/networking/websocket.rs
```

### Solution 2

There are **two errors**:

1. `super::Secret { value: 42 }` — `Secret` is private (no `pub`). Even though `inner` is inside `outer`, it cannot construct a type from `outer` that is not `pub`. The struct itself needs `pub struct Secret`.

2. `s.value` — Even if `Secret` were `pub struct`, the field `value` is private. It needs `pub value: i32`.

Fixed code:
```rust
mod outer {
    pub struct Secret {  // pub added
        pub value: i32,  // pub added
    }

    pub mod inner {
        pub fn reveal() -> i32 {
            let s = super::Secret { value: 42 };
            s.value
        }
    }
}

fn main() {
    let _ = outer::inner::reveal();
}
```

### Solution 3

```rust
use std::collections::{HashMap, HashSet, BTreeMap};
use std::io::{Read, Write, BufReader};
```

### Solution 4

```rust
// src/lib.rs
pub mod types;

pub use types::user::User;
pub use types::session::Session;
```

```rust
// src/types/mod.rs
pub mod user;
pub mod session;
```

### Solution 5

`todo_platform/Cargo.toml` (workspace root):
```toml
[workspace]
members = [
    "todo_core",
    "todo_api",
    "todo_cli",
]
```

`todo_platform/todo_core/Cargo.toml`:
```toml
[package]
name = "todo_core"
version = "0.1.0"
edition = "2021"
```

`todo_platform/todo_api/Cargo.toml`:
```toml
[package]
name = "todo_api"
version = "0.1.0"
edition = "2021"

[dependencies]
todo_core = { path = "../todo_core" }
```

`todo_platform/todo_cli/Cargo.toml`:
```toml
[package]
name = "todo_cli"
version = "0.1.0"
edition = "2021"

[dependencies]
todo_core = { path = "../todo_core" }
```

---

## Mini Project: Library Management System

### Project Overview

We will build a **Library Management System** — a library crate (`libms`) that supports:
- Managing books and members
- Checking books in and out
- Searching the catalog
- A binary that uses the library

This project demonstrates: multiple modules, shared models, reusable services, clean public API design, and a workspace layout.

### Functional Requirements

1. Add books to the library catalog
2. Register library members
3. Check out a book to a member (if available)
4. Return a book
5. Search books by title or author
6. List all books checked out by a member
7. Validate all inputs (ISBNs, names, member IDs)

### Project Structure

```
libms/
  Cargo.toml
  src/
    lib.rs
    errors.rs
    models/
      mod.rs
      book.rs
      member.rs
      loan.rs
    services/
      mod.rs
      catalog_service.rs
      member_service.rs
      loan_service.rs
    utils/
      mod.rs
      validation.rs
      formatting.rs
  src/bin/
    main.rs
```

### Step-by-Step Development

#### Step 1: Initialize the Project

```bash
cargo new libms --lib
cd libms
mkdir -p src/models src/services src/utils src/bin
```

#### Step 2: Define `Cargo.toml`

```toml
[package]
name = "libms"
version = "0.1.0"
edition = "2021"

[[bin]]
name = "libms_cli"
path = "src/bin/main.rs"
```

#### Step 3: Define errors — `src/errors.rs`

```rust
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum LibmsError {
    BookNotFound(String),
    MemberNotFound(String),
    BookAlreadyCheckedOut(String),
    BookNotCheckedOut(String),
    DuplicateIsbn(String),
    DuplicateMemberId(String),
    InvalidIsbn(String),
    InvalidMemberId(String),
    InvalidName(String),
    LoanNotFound(String),
}

impl fmt::Display for LibmsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LibmsError::BookNotFound(s) => write!(f, "Book not found: {}", s),
            LibmsError::MemberNotFound(s) => write!(f, "Member not found: {}", s),
            LibmsError::BookAlreadyCheckedOut(s) => write!(f, "Book already checked out: {}", s),
            LibmsError::BookNotCheckedOut(s) => write!(f, "Book is not checked out: {}", s),
            LibmsError::DuplicateIsbn(s) => write!(f, "Duplicate ISBN: {}", s),
            LibmsError::DuplicateMemberId(s) => write!(f, "Duplicate member ID: {}", s),
            LibmsError::InvalidIsbn(s) => write!(f, "Invalid ISBN: {}", s),
            LibmsError::InvalidMemberId(s) => write!(f, "Invalid member ID: {}", s),
            LibmsError::InvalidName(s) => write!(f, "Invalid name: {}", s),
            LibmsError::LoanNotFound(s) => write!(f, "Loan not found: {}", s),
        }
    }
}

impl std::error::Error for LibmsError {}
```

#### Step 4: Utility modules

`src/utils/validation.rs`:

```rust
/// Validate an ISBN-13: exactly 13 ASCII digits.
pub(crate) fn is_valid_isbn(isbn: &str) -> bool {
    isbn.len() == 13 && isbn.chars().all(|c| c.is_ascii_digit())
}

/// Validate a member ID: 4–10 alphanumeric characters.
pub(crate) fn is_valid_member_id(id: &str) -> bool {
    let len = id.len();
    (4..=10).contains(&len) && id.chars().all(|c| c.is_alphanumeric())
}

/// Validate a non-empty name under 100 characters.
pub(crate) fn is_valid_name(name: &str) -> bool {
    let trimmed = name.trim();
    !trimmed.is_empty() && trimmed.len() <= 100
}
```

`src/utils/formatting.rs`:

```rust
use crate::models::book::Book;
use crate::models::member::Member;
use crate::models::loan::Loan;

pub(crate) fn format_book(book: &Book) -> String {
    format!(
        "  [{}] \"{}\" by {} — {}",
        book.isbn,
        book.title,
        book.author,
        if book.is_available() { "Available" } else { "Checked out" }
    )
}

pub(crate) fn format_member(member: &Member) -> String {
    format!("  [{}] {}", member.id, member.name)
}

pub(crate) fn format_loan(loan: &Loan) -> String {
    format!(
        "  Loan: ISBN {} → Member {} (checked out: {})",
        loan.isbn, loan.member_id, loan.checkout_date
    )
}
```

`src/utils/mod.rs`:

```rust
pub(crate) mod validation;
pub(crate) mod formatting;
```

#### Step 5: Model definitions

`src/models/book.rs`:

```rust
#[derive(Debug, Clone)]
pub struct Book {
    pub isbn: String,
    pub title: String,
    pub author: String,
    pub(crate) available: bool,
}

impl Book {
    pub fn new(isbn: &str, title: &str, author: &str) -> Self {
        Book {
            isbn: isbn.to_string(),
            title: title.to_string(),
            author: author.to_string(),
            available: true,
        }
    }

    pub fn is_available(&self) -> bool {
        self.available
    }

    pub(crate) fn check_out(&mut self) {
        self.available = false;
    }

    pub(crate) fn check_in(&mut self) {
        self.available = true;
    }
}
```

`src/models/member.rs`:

```rust
#[derive(Debug, Clone)]
pub struct Member {
    pub id: String,
    pub name: String,
    pub(crate) active: bool,
}

impl Member {
    pub fn new(id: &str, name: &str) -> Self {
        Member {
            id: id.to_string(),
            name: name.to_string(),
            active: true,
        }
    }

    pub fn is_active(&self) -> bool {
        self.active
    }
}
```

`src/models/loan.rs`:

```rust
#[derive(Debug, Clone)]
pub struct Loan {
    pub id: String,
    pub isbn: String,
    pub member_id: String,
    pub checkout_date: String,
    pub returned: bool,
}

impl Loan {
    pub fn new(id: &str, isbn: &str, member_id: &str, checkout_date: &str) -> Self {
        Loan {
            id: id.to_string(),
            isbn: isbn.to_string(),
            member_id: member_id.to_string(),
            checkout_date: checkout_date.to_string(),
            returned: false,
        }
    }
}
```

`src/models/mod.rs`:

```rust
pub mod book;
pub mod member;
pub mod loan;
```

#### Step 6: Service implementations

`src/services/catalog_service.rs`:

```rust
use crate::errors::LibmsError;
use crate::models::book::Book;
use crate::utils::validation::is_valid_isbn;
use crate::utils::validation::is_valid_name;

pub struct CatalogService {
    books: Vec<Book>,
}

impl CatalogService {
    pub fn new() -> Self {
        CatalogService { books: Vec::new() }
    }

    pub fn add_book(&mut self, isbn: &str, title: &str, author: &str) -> Result<(), LibmsError> {
        if !is_valid_isbn(isbn) {
            return Err(LibmsError::InvalidIsbn(isbn.to_string()));
        }
        if !is_valid_name(title) {
            return Err(LibmsError::InvalidName(format!("title: {}", title)));
        }
        if !is_valid_name(author) {
            return Err(LibmsError::InvalidName(format!("author: {}", author)));
        }
        if self.books.iter().any(|b| b.isbn == isbn) {
            return Err(LibmsError::DuplicateIsbn(isbn.to_string()));
        }
        self.books.push(Book::new(isbn, title, author));
        Ok(())
    }

    pub fn find_by_isbn(&self, isbn: &str) -> Result<&Book, LibmsError> {
        self.books
            .iter()
            .find(|b| b.isbn == isbn)
            .ok_or_else(|| LibmsError::BookNotFound(isbn.to_string()))
    }

    pub(crate) fn find_by_isbn_mut(&mut self, isbn: &str) -> Result<&mut Book, LibmsError> {
        self.books
            .iter_mut()
            .find(|b| b.isbn == isbn)
            .ok_or_else(|| LibmsError::BookNotFound(isbn.to_string()))
    }

    pub fn search_by_title(&self, query: &str) -> Vec<&Book> {
        let query_lower = query.to_lowercase();
        self.books
            .iter()
            .filter(|b| b.title.to_lowercase().contains(&query_lower))
            .collect()
    }

    pub fn search_by_author(&self, query: &str) -> Vec<&Book> {
        let query_lower = query.to_lowercase();
        self.books
            .iter()
            .filter(|b| b.author.to_lowercase().contains(&query_lower))
            .collect()
    }

    pub fn list_all(&self) -> &[Book] {
        &self.books
    }
}

impl Default for CatalogService {
    fn default() -> Self {
        Self::new()
    }
}
```

`src/services/member_service.rs`:

```rust
use crate::errors::LibmsError;
use crate::models::member::Member;
use crate::utils::validation::{is_valid_member_id, is_valid_name};

pub struct MemberService {
    members: Vec<Member>,
}

impl MemberService {
    pub fn new() -> Self {
        MemberService { members: Vec::new() }
    }

    pub fn register(&mut self, id: &str, name: &str) -> Result<(), LibmsError> {
        if !is_valid_member_id(id) {
            return Err(LibmsError::InvalidMemberId(id.to_string()));
        }
        if !is_valid_name(name) {
            return Err(LibmsError::InvalidName(name.to_string()));
        }
        if self.members.iter().any(|m| m.id == id) {
            return Err(LibmsError::DuplicateMemberId(id.to_string()));
        }
        self.members.push(Member::new(id, name));
        Ok(())
    }

    pub fn find_by_id(&self, id: &str) -> Result<&Member, LibmsError> {
        self.members
            .iter()
            .find(|m| m.id == id)
            .ok_or_else(|| LibmsError::MemberNotFound(id.to_string()))
    }

    pub fn list_all(&self) -> &[Member] {
        &self.members
    }
}

impl Default for MemberService {
    fn default() -> Self {
        Self::new()
    }
}
```

`src/services/loan_service.rs`:

```rust
use crate::errors::LibmsError;
use crate::models::loan::Loan;
use crate::services::catalog_service::CatalogService;
use crate::services::member_service::MemberService;

pub struct LoanService {
    loans: Vec<Loan>,
    next_id: u64,
}

impl LoanService {
    pub fn new() -> Self {
        LoanService {
            loans: Vec::new(),
            next_id: 1,
        }
    }

    pub fn checkout(
        &mut self,
        isbn: &str,
        member_id: &str,
        catalog: &mut CatalogService,
        members: &MemberService,
        date: &str,
    ) -> Result<String, LibmsError> {
        // Verify member exists
        members.find_by_id(member_id)?;

        // Verify book exists and is available
        let book = catalog.find_by_isbn(isbn)?;
        if !book.is_available() {
            return Err(LibmsError::BookAlreadyCheckedOut(isbn.to_string()));
        }

        // Mark the book as checked out
        catalog.find_by_isbn_mut(isbn)?.check_out();

        // Record the loan
        let loan_id = format!("LN{:06}", self.next_id);
        self.next_id += 1;
        self.loans.push(Loan::new(&loan_id, isbn, member_id, date));
        Ok(loan_id)
    }

    pub fn return_book(
        &mut self,
        isbn: &str,
        member_id: &str,
        catalog: &mut CatalogService,
    ) -> Result<(), LibmsError> {
        // Find the active loan
        let loan = self.loans
            .iter_mut()
            .find(|l| l.isbn == isbn && l.member_id == member_id && !l.returned)
            .ok_or_else(|| LibmsError::LoanNotFound(format!("isbn={} member={}", isbn, member_id)))?;

        loan.returned = true;

        // Mark the book as available
        catalog.find_by_isbn_mut(isbn)?.check_in();
        Ok(())
    }

    pub fn loans_by_member(&self, member_id: &str) -> Vec<&Loan> {
        self.loans
            .iter()
            .filter(|l| l.member_id == member_id && !l.returned)
            .collect()
    }

    pub fn all_active_loans(&self) -> Vec<&Loan> {
        self.loans.iter().filter(|l| !l.returned).collect()
    }
}

impl Default for LoanService {
    fn default() -> Self {
        Self::new()
    }
}
```

`src/services/mod.rs`:

```rust
pub mod catalog_service;
pub mod member_service;
pub mod loan_service;
```

#### Step 7: `src/lib.rs` — Crate Root and Public API

```rust
// src/lib.rs

pub mod errors;
pub mod models;
pub mod services;
pub(crate) mod utils;

// Flat re-exports for the most commonly used items
pub use errors::LibmsError;
pub use models::book::Book;
pub use models::member::Member;
pub use models::loan::Loan;
pub use services::catalog_service::CatalogService;
pub use services::member_service::MemberService;
pub use services::loan_service::LoanService;
```

#### Step 8: `src/bin/main.rs` — The CLI binary

```rust
// src/bin/main.rs

use libms::{
    CatalogService,
    MemberService,
    LoanService,
};
use libms::utils::formatting::{format_book, format_member, format_loan};

fn main() {
    let mut catalog = CatalogService::new();
    let mut members = MemberService::new();
    let mut loans = LoanService::new();

    println!("=== Library Management System ===\n");

    // --- Add books to catalog ---
    println!("--- Adding Books ---");
    add_book(&mut catalog, "9780261103573", "The Lord of the Rings", "J.R.R. Tolkien");
    add_book(&mut catalog, "9780451524935", "1984", "George Orwell");
    add_book(&mut catalog, "9780743273565", "The Great Gatsby", "F. Scott Fitzgerald");
    add_book(&mut catalog, "9780062315007", "The Alchemist", "Paulo Coelho");
    add_book(&mut catalog, "BAD_ISBN_HERE", "Bad Book", "Nobody"); // invalid ISBN

    // --- Register members ---
    println!("\n--- Registering Members ---");
    register_member(&mut members, "M001", "Alice Johnson");
    register_member(&mut members, "M002", "Bob Smith");
    register_member(&mut members, "M003", "Carol White");
    register_member(&mut members, "XY", "Short ID"); // invalid ID

    // --- List catalog ---
    println!("\n--- Full Catalog ---");
    for book in catalog.list_all() {
        println!("{}", format_book(book));
    }

    // --- List members ---
    println!("\n--- Registered Members ---");
    for member in members.list_all() {
        println!("{}", format_member(member));
    }

    // --- Check out books ---
    println!("\n--- Checkouts ---");
    checkout_book(&mut loans, "9780261103573", "M001", &mut catalog, &members, "2026-01-15");
    checkout_book(&mut loans, "9780451524935", "M001", &mut catalog, &members, "2026-01-15");
    checkout_book(&mut loans, "9780261103573", "M002", &mut catalog, &members, "2026-01-16"); // already out!
    checkout_book(&mut loans, "9780743273565", "M002", &mut catalog, &members, "2026-01-16");
    checkout_book(&mut loans, "9780062315007", "M003", &mut catalog, &members, "2026-01-16");

    // --- Catalog after checkouts ---
    println!("\n--- Catalog after checkouts ---");
    for book in catalog.list_all() {
        println!("{}", format_book(book));
    }

    // --- Active loans ---
    println!("\n--- Active Loans ---");
    for loan in loans.all_active_loans() {
        println!("{}", format_loan(loan));
    }

    // --- Loans by member ---
    println!("\n--- Alice's loans ---");
    for loan in loans.loans_by_member("M001") {
        println!("{}", format_loan(loan));
    }

    // --- Search ---
    println!("\n--- Search: 'the' ---");
    for book in catalog.search_by_title("the") {
        println!("{}", format_book(book));
    }

    // --- Return a book ---
    println!("\n--- Returning 1984 (Alice → M001) ---");
    match loans.return_book("9780451524935", "M001", &mut catalog) {
        Ok(()) => println!("  Return successful."),
        Err(e) => println!("  Error: {}", e),
    }

    println!("\n--- Catalog after return ---");
    for book in catalog.list_all() {
        println!("{}", format_book(book));
    }
}

fn add_book(catalog: &mut CatalogService, isbn: &str, title: &str, author: &str) {
    match catalog.add_book(isbn, title, author) {
        Ok(()) => println!("  Added: \"{}\" by {}", title, author),
        Err(e) => println!("  Error: {}", e),
    }
}

fn register_member(members: &mut MemberService, id: &str, name: &str) {
    match members.register(id, name) {
        Ok(()) => println!("  Registered: [{}] {}", id, name),
        Err(e) => println!("  Error: {}", e),
    }
}

fn checkout_book(
    loans: &mut LoanService,
    isbn: &str,
    member_id: &str,
    catalog: &mut CatalogService,
    members: &MemberService,
    date: &str,
) {
    match loans.checkout(isbn, member_id, catalog, members, date) {
        Ok(loan_id) => println!("  [{}] Checked out ISBN {} to member {}", loan_id, isbn, member_id),
        Err(e) => println!("  Error: {}", e),
    }
}
```

**Note:** The binary imports `format_book`, `format_member`, and `format_loan` from `libms::utils::formatting`. These are `pub(crate)` inside the library, which means they are NOT accessible from the binary crate this way. To fix this for a real project, you have two options:
1. Make `utils::formatting` fully `pub` (expose it as a utility API).
2. Move the formatting logic into the `Display` trait implementations on each model.

The cleanest solution for production code is always `Display`:

```rust
// Replace format_book() calls with:
println!("  {}", book);

// And implement Display on Book:
impl std::fmt::Display for Book {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] \"{}\" by {} — {}",
            self.isbn, self.title, self.author,
            if self.available { "Available" } else { "Checked out" })
    }
}
```

This is the refactoring we address in the next section.

#### Expected Output

```
=== Library Management System ===

--- Adding Books ---
  Added: "The Lord of the Rings" by J.R.R. Tolkien
  Added: "1984" by George Orwell
  Added: "The Great Gatsby" by F. Scott Fitzgerald
  Added: "The Alchemist" by Paulo Coelho
  Error: Invalid ISBN: BAD_ISBN_HERE

--- Registering Members ---
  Registered: [M001] Alice Johnson
  Registered: [M002] Bob Smith
  Registered: [M003] Carol White
  Error: Invalid member ID: XY

--- Full Catalog ---
  [9780261103573] "The Lord of the Rings" by J.R.R. Tolkien — Available
  [9780451524935] "1984" by George Orwell — Available
  [9780743273565] "The Great Gatsby" by F. Scott Fitzgerald — Available
  [9780062315007] "The Alchemist" by Paulo Coelho — Available

--- Registered Members ---
  [M001] Alice Johnson
  [M002] Bob Smith
  [M003] Carol White

--- Checkouts ---
  [LN000001] Checked out ISBN 9780261103573 to member M001
  [LN000002] Checked out ISBN 9780451524935 to member M001
  Error: Book already checked out: 9780261103573
  [LN000003] Checked out ISBN 9780743273565 to member M002
  [LN000004] Checked out ISBN 9780062315007 to member M003

--- Catalog after checkouts ---
  [9780261103573] "The Lord of the Rings" by J.R.R. Tolkien — Checked out
  [9780451524935] "1984" by George Orwell — Checked out
  [9780743273565] "The Great Gatsby" by F. Scott Fitzgerald — Checked out
  [9780062315007] "The Alchemist" by Paulo Coelho — Checked out

--- Active Loans ---
  Loan: ISBN 9780261103573 → Member M001 (checked out: 2026-01-15)
  Loan: ISBN 9780451524935 → Member M001 (checked out: 2026-01-15)
  Loan: ISBN 9780743273565 → Member M002 (checked out: 2026-01-16)
  Loan: ISBN 9780062315007 → Member M003 (checked out: 2026-01-16)

--- Alice's loans ---
  Loan: ISBN 9780261103573 → Member M001 (checked out: 2026-01-15)
  Loan: ISBN 9780451524935 → Member M001 (checked out: 2026-01-15)

--- Search: 'the' ---
  [9780261103573] "The Lord of the Rings" by J.R.R. Tolkien — Checked out
  [9780743273565] "The Great Gatsby" by F. Scott Fitzgerald — Checked out
  [9780062315007] "The Alchemist" by Paulo Coelho — Checked out

--- Returning 1984 (Alice → M001) ---
  Return successful.

--- Catalog after return ---
  [9780261103573] "The Lord of the Rings" by J.R.R. Tolkien — Checked out
  [9780451524935] "1984" by George Orwell — Available
  [9780743273565] "The Great Gatsby" by F. Scott Fitzgerald — Checked out
  [9780062315007] "The Alchemist" by Paulo Coelho — Checked out
```

---

### Module Tree Diagram

```
crate (libms)
│
├── errors          (pub)   — LibmsError enum
│
├── models          (pub)
│   ├── book        (pub)   — Book struct
│   ├── member      (pub)   — Member struct
│   └── loan        (pub)   — Loan struct
│
├── services        (pub)
│   ├── catalog_service  (pub) — CatalogService
│   ├── member_service   (pub) — MemberService
│   └── loan_service     (pub) — LoanService
│
└── utils        (pub(crate))  — internal helpers
    ├── validation  (pub(crate)) — is_valid_isbn, etc.
    └── formatting  (pub(crate)) — format_book, etc.
```

### Code Explanation — Key Design Decisions

**Why is `LoanService::checkout` so complex?**
It takes mutable references to both `CatalogService` and `MemberService`. This is because Rust's borrow checker ensures we cannot have two simultaneous mutable borrows of the same value. By taking them as separate parameters, we explicitly declare what we need and the borrow checker can verify correctness at compile time.

In a more advanced design, you might wrap everything in a single `Library` struct that owns all three services and has methods that coordinate them internally. This avoids passing services around as parameters.

**Why put `check_out` and `check_in` as `pub(crate)`?**
These methods change the availability of a book. We do not want external callers to call them directly — they should go through `LoanService::checkout` and `LoanService::return_book`, which enforce all the business rules (validating the member, recording the loan, etc.). `pub(crate)` exposes the mutation only to code within the `libms` crate.

**Why a separate `errors.rs` at the crate root?**
If `BookNotFound` were defined in `catalog_service.rs` and `MemberNotFound` in `member_service.rs`, then `loan_service.rs` (which calls both) would need to import from both and return a union type. A single `LibmsError` enum in a shared location keeps error handling clean and avoids import tangles.

---

### Refactoring Suggestions

#### Refactoring 1: Implement `Display` on Models

Replace the `format_*` functions in `utils/formatting.rs` with `Display` implementations on each model. This is idiomatic Rust — you print a value with `{}` and the `Display` trait handles it.

```rust
// In src/models/book.rs
impl std::fmt::Display for Book {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}] \"{}\" by {} — {}",
            self.isbn,
            self.title,
            self.author,
            if self.available { "Available" } else { "Checked out" }
        )
    }
}
```

#### Refactoring 2: Introduce a `Library` Facade

Instead of passing three separate services around, create a `Library` struct that owns all of them:

```rust
pub struct Library {
    pub catalog: CatalogService,
    pub members: MemberService,
    pub loans: LoanService,
}

impl Library {
    pub fn new() -> Self {
        Library {
            catalog: CatalogService::new(),
            members: MemberService::new(),
            loans: LoanService::new(),
        }
    }

    pub fn checkout(&mut self, isbn: &str, member_id: &str, date: &str) -> Result<String, LibmsError> {
        self.loans.checkout(isbn, member_id, &mut self.catalog, &self.members, date)
    }
}
```

The binary then becomes much cleaner:
```rust
let mut lib = Library::new();
lib.checkout("9780261103573", "M001", "2026-01-15")?;
```

#### Refactoring 3: Add a Prelude Module

For convenience, add a `prelude.rs`:
```rust
// src/prelude.rs
pub use crate::{LibmsError, Book, Member, Loan, CatalogService, MemberService, LoanService, Library};
```

External users then write:
```rust
use libms::prelude::*;
```

#### Refactoring 4: Use `chrono` for Dates

Replace `String` date fields with `chrono::NaiveDate` for proper date arithmetic (computing overdue books, due dates, etc.):

```toml
[dependencies]
chrono = "0.4"
```

```rust
use chrono::NaiveDate;
pub struct Loan {
    pub checkout_date: NaiveDate,
    pub due_date: NaiveDate,
    // ...
}
```

---

### Challenge Exercises

1. **Due dates and overdue detection.** Add a `due_date` field to `Loan`. Add a method `LoanService::overdue_loans(today: &str) -> Vec<&Loan>` that returns all active loans past their due date.

2. **Book reservations.** Add a `ReservationService`. If a book is checked out, a member can reserve it. When the book is returned, the first member in the reservation queue is notified (print a message). Implement this without breaking the existing service API.

3. **Multiple copies.** Modify `Book` to track `available_copies: u32` and `total_copies: u32`. A book is available if `available_copies > 0`. Update `checkout` and `return_book` accordingly.

4. **Persistence.** Add a `save(path: &str)` method to the `Library` that serializes the state to a JSON file, and a `load(path: &str)` class method that deserializes it. Use `serde` and `serde_json`.

5. **Workspace split.** Convert the project into a workspace:
   - `libms_core` — models and errors only
   - `libms_services` — services (depends on `libms_core`)
   - `libms_cli` — the binary (depends on `libms_services`)

6. **Integration tests.** Create a `tests/` directory (outside `src/`). Write integration tests for the full checkout → return flow. Verify that error cases (checkout twice, return a book you did not borrow) produce the correct error variants.

---

## Quiz

### Question 1

What is the difference between `mod foo;` and `use foo;`?

### Question 2

In Rust, which of the following is TRUE about default (no `pub`) visibility?

A) Items are visible to everyone in the crate  
B) Items are visible only within the module they are defined in  
C) Items are visible within the module and all its descendants  
D) Items are never visible — you must always use `pub`

### Question 3

Given:
```
crate
  └── outer
        └── inner (defines `fn secret()`)
```
Which of the following modules can call `secret()` if it has NO `pub` annotation?

A) Only `inner`  
B) `inner` and `outer`  
C) `inner`, `outer`, and the crate root  
D) Only the crate root

### Question 4

What does `pub(crate)` mean?

A) The item is public to the current module only  
B) The item is public to all modules in the current crate, but not to external crates  
C) The item is public to the parent module only  
D) The item is public to everyone

### Question 5

You have a file `src/animals/dog.rs`. What `mod` declaration do you need to add, and where?

### Question 6

What is the purpose of `pub use` in `src/lib.rs`?

### Question 7

What is a Cargo workspace?

### Question 8

In Rust 2018 edition, if you have a module `config` that contains a submodule `parser`, which of the following file layouts is preferred (new style)?

A) `src/config/mod.rs` contains `pub mod parser;`, `src/config/parser.rs` exists  
B) `src/config.rs` contains `pub mod parser;`, `src/config/parser.rs` exists  
C) `src/config.rs` contains `pub mod parser;`, `src/parser.rs` exists  
D) No file needed — just declare `mod config::parser;` in main.rs

### Question 9

What is wrong with this code?
```rust
mod animals {
    pub struct Dog {
        name: String,
    }
}

fn main() {
    let d = animals::Dog { name: String::from("Buddy") };
}
```

### Question 10

Why should you generally avoid `use module::*` (glob imports) in production code?

---

## Quiz Answers

### Answer 1

`mod foo;` declares that a module named `foo` exists and tells the compiler to find its definition in `foo.rs` or `foo/mod.rs`. It brings the module itself into the module tree.

`use foo;` or `use foo::something;` brings an already-existing module (or item within it) into the current scope, allowing you to refer to it by a shorter name. `use` does NOT declare modules — it is purely a naming convenience.

You typically need BOTH: `mod` to declare a module exists, and `use` to conveniently refer to its items.

### Answer 2

**C** — Items with no visibility annotation are visible within the module they are defined in AND all descendant (child, grandchild, etc.) modules of that module. Sibling modules and parent modules cannot access them.

### Answer 3

**B** — `inner` and `outer`. Private items in `inner` are visible to `inner` itself and to `outer` (the parent). The crate root is not a descendant of `outer` → `inner`, so it cannot see `secret()` directly.

### Answer 4

**B** — `pub(crate)` means the item is public to all code within the current crate, but is not part of the public API visible to other crates that depend on yours.

### Answer 5

You need to add `pub mod dog;` (or `mod dog;` if it should be private) inside the file that defines the `animals` module. If `animals` is declared as `mod animals;` in `main.rs`, then `animals` has its body in `src/animals.rs` (new style) or `src/animals/mod.rs` (old style). In either case, you add `pub mod dog;` to that file.

```rust
// src/animals.rs (or src/animals/mod.rs)
pub mod dog; // this tells the compiler to look for src/animals/dog.rs
```

### Answer 6

`pub use` in `src/lib.rs` re-exports an item from an internal module path, making it accessible to external callers at a shorter, cleaner path. For example, `pub use models::user::User;` lets callers write `use my_crate::User;` instead of `use my_crate::models::user::User;`. This is the primary tool for designing a clean, stable public API that is independent of your internal module structure.

### Answer 7

A Cargo workspace is a directory containing a root `Cargo.toml` with a `[workspace]` section that lists member packages. All workspace members share a single `target/` build output directory and a single `Cargo.lock` file. This means dependencies are compiled only once even if multiple members use them, and all members are guaranteed to use the same locked versions of external dependencies. Workspaces are used when you want to split a large project into multiple crates that are developed together.

### Answer 8

**B** — In Rust 2018 preferred style, `src/config.rs` contains the module body for `config` (including `pub mod parser;`), and the submodule parser lives in `src/config/parser.rs`. This avoids `mod.rs` files named identically across the project.

### Answer 9

The struct field `name` is private (no `pub`). Even though `Dog` is a `pub struct`, its fields are private by default. Code outside the `animals` module cannot access or set `name`.

Fix:
```rust
mod animals {
    pub struct Dog {
        pub name: String, // add pub
    }
}
```

Or, keep the field private and add a constructor:
```rust
mod animals {
    pub struct Dog {
        name: String,
    }
    impl Dog {
        pub fn new(name: &str) -> Self {
            Dog { name: name.to_string() }
        }
    }
}
fn main() {
    let d = animals::Dog::new("Buddy");
}
```

### Answer 10

Glob imports (`use module::*`) make it unclear where any given name came from. If two glob-imported modules both export a type named `Error`, you get an ambiguity error. If a future version of a glob-imported module adds a new name that conflicts with one of yours, your code silently breaks. The exception is `use super::*` inside test modules (idiomatic for bringing the module under test into scope) and `use crate::prelude::*` for crates that explicitly design a prelude module for glob import.

---

## Chapter Summary

In this chapter, you learned one of Rust's most important organizational tools: the module system, alongside crates and workspaces.

**Modules** (`mod`) are named containers for Rust items. They serve three purposes simultaneously: namespace isolation (no name collisions), visibility control (items are private by default), and logical organization (readers understand program structure at a glance).

**Visibility** in Rust defaults to private and flows downward — child modules can see parent private items, but nothing outside the declaring module can. You make items visible with `pub` (everyone), `pub(crate)` (this crate only), `pub(super)` (parent module only), or `pub(in path)` (a specific ancestor). Struct fields have independent visibility from the struct itself.

**`use`** brings items into scope without copying or moving them. By convention, import functions via their parent module and import types directly. Use `as` to rename imports when names conflict. Use `pub use` to re-export items, enabling you to design a clean flat public API over a deep internal module hierarchy.

**File layout**: In Rust 2018+, a module `foo` that has submodules lives in `src/foo.rs`, with submodule files in `src/foo/`. This is preferred over the old `src/foo/mod.rs` style. The compiler discovers nothing automatically — every module must be declared with `mod`.

**Crates** are the unit of compilation. A binary crate has a `main` function; a library crate does not. The crate root (`main.rs` or `lib.rs`) is where the module tree begins. External crates are declared as `[dependencies]` in `Cargo.toml`.

**Workspaces** group multiple related crates that are developed together. They share a `target/` directory (efficient compilation), a `Cargo.lock` file (consistent dependency versions), and can reference each other with `path = "../sibling"` dependencies. Use workspaces when you have a large project that naturally separates into a library, a CLI tool, an API server, and so on.

The mini project demonstrated all of these concepts together: a library crate split across `models`, `services`, and `utils` modules, with a public API designed via `pub use` re-exports, crate-internal utilities via `pub(crate)`, and a binary crate that uses the library. This architecture separates concerns, prevents internal implementation details from leaking, and makes each piece independently understandable and testable.

Key takeaways:
- **`mod` declares existence; `use` brings items into scope.** Both are needed.
- **Private by default is a feature, not a restriction.** It forces explicit API design.
- **`pub(crate)` is your best tool for internal shared utilities.**
- **`pub use` at the crate root is how you design a clean public API.**
- **Workspaces solve the multi-crate project problem cleanly.**
- **The borrow checker and module system work together** — mutable access to internal service fields is controlled via `pub(crate)` methods, not raw field access.

In the next chapter, you will learn about **Testing in Rust** — unit tests, integration tests, and how the module system makes testing straightforward with `#[cfg(test)]` and `use super::*`.
