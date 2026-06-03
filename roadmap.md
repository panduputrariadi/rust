# Rust Complete Learning Path (Beginner to Expert)

## Goal

By the end of this roadmap, the student should be able to:

* Build production-grade CLI applications.
* Build REST APIs.
* Build async services.
* Create reusable crates.
* Write concurrent and parallel programs.
* Debug memory issues.
* Work with unsafe Rust responsibly.
* Interface with C/C++ libraries.
* Optimize performance.
* Build systems software.
* Read Rust compiler source code.
* Contribute to open source Rust projects.

Expected Duration:

* Beginner: 1–2 Months
* Intermediate: 2–3 Months
* Advanced: 3–6 Months
* Expert: 6–12+ Months

---

# LEVEL 1 — BEGINNER

## PART 1 — Rust Fundamentals

### Chapter 1

Introduction to Rust

### Chapter 2

Variables and Data Types

### Chapter 3

Control Flow

### Chapter 4

Functions

---

## PART 2 — Ownership and Memory

### Chapter 5

Ownership

### Chapter 6

Borrowing

### Chapter 7

Slices

---

## PART 3 — Structs and Enums

### Chapter 8

Structs

### Chapter 9

Methods

### Chapter 10

Enums

---

## PART 4 — Collections

### Chapter 11

Strings

### Chapter 12

Vectors

### Chapter 13

HashMaps

---

# LEVEL 2 — INTERMEDIATE

## PART 5 — Error Handling

### Chapter 14

panic!

### Chapter 15

Result

### Chapter 16

Custom Errors

Crates:

* anyhow
* thiserror

---

## PART 6 — Generics and Traits

### Chapter 17

Generics

### Chapter 18

Traits

### Chapter 19

Trait Bounds

### Chapter 20

Associated Types

### Chapter 21

Operator Overloading

---

## PART 7 — Lifetimes

### Chapter 22

Lifetime Fundamentals

### Chapter 23

Lifetime Elision

### Chapter 24

Advanced Lifetimes

---

## PART 8 — Modules and Crates

### Chapter 25

Modules

### Chapter 26

Packages

### Chapter 27

Workspaces

### Chapter 28

Publishing Crates

---

## PART 9 — Functional Rust

### Chapter 29

Iterators

### Chapter 30

Closures

### Chapter 31

Functional Patterns

### Chapter 32

Iterator Optimization

---

## PART 10 — Smart Pointers

### Chapter 33

Box

### Chapter 34

Rc

### Chapter 35

Arc

### Chapter 36

RefCell

### Chapter 37

Cell

### Chapter 38

Interior Mutability

---

# LEVEL 3 — ADVANCED

## PART 11 — Concurrency

### Chapter 39

Threads

### Chapter 40

Channels

### Chapter 41

Mutex

### Chapter 42

RwLock

### Chapter 43

Atomics

### Chapter 44

Lock-Free Concepts

---

## PART 12 — Async Rust

### Chapter 45

Futures

### Chapter 46

Async/Await

### Chapter 47

Pin

### Chapter 48

Task Scheduling

### Chapter 49

Executors

### Chapter 50

Tokio Runtime

### Chapter 51

Async Streams

Crates:

* tokio
* futures
* async-trait

---

## PART 13 — Networking

### Chapter 52

TCP

### Chapter 53

UDP

### Chapter 54

HTTP

### Chapter 55

WebSockets

### Chapter 56

gRPC

Crates:

* reqwest
* hyper
* tonic

---

## PART 14 — Backend Development

### Chapter 57

Axum

### Chapter 58

Actix Web

### Chapter 59

Middleware

### Chapter 60

Authentication

### Chapter 61

Authorization

### Chapter 62

JWT

### Chapter 63

Rate Limiting

### Chapter 64

Caching

### Chapter 65

Background Jobs

---

## PART 15 — Database

### Chapter 66

PostgreSQL

### Chapter 67

SQLx

### Chapter 68

Diesel

### Chapter 69

Transactions

### Chapter 70

Connection Pools

### Chapter 71

Migrations

---

## PART 16 — Testing

### Chapter 72

Unit Tests

### Chapter 73

Integration Tests

### Chapter 74

Property Testing

### Chapter 75

Benchmarking

Crates:

* criterion
* proptest

---

## PART 17 — Design Patterns

### Chapter 76

Builder Pattern

### Chapter 77

Repository Pattern

### Chapter 78

Factory Pattern

### Chapter 79

Dependency Injection

### Chapter 80

Domain Driven Design

---

# LEVEL 4 — EXPERT

## PART 18 — Unsafe Rust

### Chapter 81

Unsafe Blocks

### Chapter 82

Raw Pointers

### Chapter 83

Pointer Arithmetic

### Chapter 84

Unsafe Traits

### Chapter 85

Unsafe Functions

### Chapter 86

Memory Layout

### Chapter 87

Drop Check

### Chapter 88

Variance

---

## PART 19 — Memory Internals

### Chapter 89

Stack vs Heap

### Chapter 90

Memory Alignment

### Chapter 91

Cache Locality

### Chapter 92

Allocation Strategies

### Chapter 93

Custom Allocators

### Chapter 94

Arena Allocation

---

## PART 20 — Advanced Type System

### Chapter 95

PhantomData

### Chapter 96

Marker Traits

### Chapter 97

Auto Traits

### Chapter 98

Negative Trait Bounds

### Chapter 99

Higher-Rank Trait Bounds

### Chapter 100

Generic Associated Types

### Chapter 101

Type-Level Programming

---

## PART 21 — Macros

### Chapter 102

Declarative Macros

### Chapter 103

Macro Rules

### Chapter 104

Procedural Macros

### Chapter 105

Derive Macros

### Chapter 106

Attribute Macros

### Chapter 107

Function-Like Macros

---

## PART 22 — Compiler Knowledge

### Chapter 108

How rustc Works

### Chapter 109

HIR

### Chapter 110

MIR

### Chapter 111

LLVM

### Chapter 112

Borrow Checker Internals

### Chapter 113

Monomorphization

---

## PART 23 — FFI

### Chapter 114

Calling C from Rust

### Chapter 115

Calling Rust from C

### Chapter 116

Bindgen

### Chapter 117

cbindgen

### Chapter 118

Memory Safety Across Boundaries

---

## PART 24 — Systems Programming

### Chapter 119

Processes

### Chapter 120

Signals

### Chapter 121

File Systems

### Chapter 122

Memory Mapping

### Chapter 123

Operating System Interfaces

Crates:

* nix
* libc

---

## PART 25 — Embedded Rust

### Chapter 124

Embedded Basics

### Chapter 125

no_std

### Chapter 126

Microcontrollers

### Chapter 127

RTIC

### Chapter 128

Embedded HAL

---

## PART 26 — Performance Engineering

### Chapter 129

Profiling

### Chapter 130

Flamegraphs

### Chapter 131

SIMD

### Chapter 132

Zero-Cost Abstractions

### Chapter 133

Benchmark Analysis

### Chapter 134

Performance Tuning

---

## PART 27 — Distributed Systems

### Chapter 135

Message Queues

### Chapter 136

Kafka

### Chapter 137

Event Sourcing

### Chapter 138

CQRS

### Chapter 139

Distributed Transactions

### Chapter 140

Consensus Concepts

---

## PART 28 — Expert Projects

### Project 1

Production REST API

Stack:

* Axum
* SQLx
* PostgreSQL
* Redis
* Docker

---

### Project 2

Distributed Job Queue

---

### Project 3

gRPC Microservice Platform

---

### Project 4

High Performance TCP Server

---

### Project 5

Custom Async Runtime

---

### Project 6

Build a Mini Database

Features:

* WAL
* Indexing
* B-Tree
* Transactions

---

### Project 7

Build a Mini Redis

Features:

* TCP Protocol
* Persistence
* Pub/Sub

---

### Project 8

Build a Programming Language

Features:

* Lexer
* Parser
* AST
* Interpreter

---

### Project 9

Build a Rust Web Framework

---

### Project 10

Build an Operating System Kernel (Experimental)

Topics:

* Bootloader
* Memory Manager
* Scheduler
* Drivers

---

# Expert Milestone

You can consider yourself Expert when you can:

* Explain the borrow checker internally.
* Read MIR output.
* Read LLVM IR.
* Write procedural macros.
* Use unsafe Rust correctly.
* Design high-performance systems.
* Build custom runtimes.
* Contribute to Rust ecosystem crates.
* Review complex Rust code confidently.
* Teach advanced Rust concepts to others.
* Understand performance implications of abstractions.
* Debug memory and concurrency issues without guesswork.

```
```
