# const-bool

[![Crates.io](https://img.shields.io/crates/v/const-bool.svg)](https://crates.io/crates/const-bool)
[![Docs.rs](https://docs.rs/const-bool/badge.svg)](https://docs.rs/const-bool)
[![License](https://img.shields.io/crates/l/const-bool.svg)](https://crates.io/crates/const-bool)

A `#![no_std]` Rust crate providing type-level booleans. 

This crate is primarily designed to bypass the current limitations of Rust's `const generics`. It allows you to perform boolean logic and equality checks directly within type bounds (`where` clauses).

## ⚠️ The Problem

Currently in Rust, you **cannot** evaluate const expressions or compare const associated values directly in trait bounds. 

For example, this is **invalid** Rust and will not compile:

```rust
// ❌ THIS DOES NOT COMPILE
trait State {
    const IS_ACTIVE: bool;
}

fn do_something<A: State, B: State>() 
where 
    A::IS_ACTIVE == B::IS_ACTIVE, // Error: expected trait bound
    { A::IS_ACTIVE && B::IS_ACTIVE } == true // Error: complex expressions not allowed
{
    // ...
}
```

## ✨ The Solution

`const-bool` solves this by lifting booleans to the **type level**. By using types like `ConstTrue` and `ConstFalse`, you can enforce logic at compile time using standard trait bounds.

### Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
const-bool = "0.1.0"
```

Or using `cargo`:

```bash
cargo add const-bool
```

## 🚀 Usage Examples

### 1. Basic Compile-Time Logic in Bounds

```rust
use const_bool::*;

trait State {
    // Lift the boolean to an associated type
    type IsActive: ConstBool; 
}

struct DeviceA;
impl State for DeviceA { type IsActive = ConstTrue; }

struct DeviceB;
impl State for DeviceB { type IsActive = ConstFalse; }

// We only want this function to be callable if BOTH devices are active.
// Using `And<A, B>` and requiring it to equal `ConstTrue`.
fn operate_devices<A, B>()
where
    A: State,
    B: State,
    And<A::IsActive, B::IsActive>: Eq<ConstTrue, ConstTrue>, // Type-level logic!
{
    println!("Both devices are active!");
}

fn main() {
    // operate_devices::<DeviceA, DeviceB>(); // ❌ Compile error: DeviceB is false
    operate_devices::<DeviceA, DeviceA>();    // ✅ Compiles!
}
```

### 2. Conditional Type Selection (`Select`)

You can use a type-level boolean to choose between two different types at compile time.

```rust
use const_bool::*;

// If True, select `i32`. If False, select `f64`.
type Number1 = Select<ConstTrue, i32, f64>;  // Resolves to i32
type Number2 = Select<ConstFalse, i32, f64>; // Resolves to f64

let a: Number1 = 42;
let b: Number2 = 3.14;

struct RcInner<T, F: ConstBool>{
    strong: usize,
    weak: Select<F, usize, ()>,
    data: T
}
```

### 3. Extracting the Value at Runtime

If you ever need the actual `bool` value at runtime, you can access the `VALUE` constant:

```rust
use const_bool::*;

assert_eq!(ConstTrue::VALUE, true);
assert_eq!(ConstFalse::VALUE, false);

assert_eq!(<Xor<ConstTrue, ConstFalse> as ConstBool>::VALUE, true);
```

## 🛠️ Provided Logic Operators

This crate provides the following type-level aliases for boolean logic:

*   `Not<A>`
*   `And<A, B>`
*   `Or<A, B>`
*   `Xor<A, B>`
*   `Nand<A, B>`
*   `Nor<A, B>`
*   `Eq<A, B>`

## Safety and Soundness

The `ConstBool` trait is **Sealed**. This means that users of this crate cannot implement `ConstBool` for their own types. `ConstTrue` and `ConstFalse` are the strictly only two types that will ever implement this trait, guaranteeing soundness in your unsafe code if you rely on these bounds.

## License

Licensed under either of

* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)

at your option.