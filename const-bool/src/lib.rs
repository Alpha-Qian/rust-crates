#![no_std]
//! A `no_std` crate providing type-level booleans.
//! 
//! This crate is useful when you want to use const generics in bounds, 
//! but hit the limitations of the current Rust compiler (e.g., you cannot write 
//! `where A::FLAG == B::FLAG`).
//!
//! # Example
//! 
//! ```rust
//! use const_bool::*;
//! 
//! trait StateMachine {
//!     type IsActive: ConstBool;
//! }
//! 
//! // Use type bounds to simulate const generic equality/logic
//! fn transition<A, B>() 
//! where 
//!     A: StateMachine,
//!     B: StateMachine,
//!     And<A::IsActive, B::IsActive>: Eq<ConstTrue, ConstTrue> // Simulated Logic
//! {
//!     // ...
//! }
//! ```

mod const_bool;
mod assert;

//暂不公开
mod bool_fn;
mod witness;
mod bool_select_wits;
mod lemma;

pub use const_bool::{
    ConstBool,
    ConstTrue,
    ConstFalse,
    Not,
    And,
    Or,
    Nand,
    Nor,
    Xor,
    Eq,
    Select,
};

pub use assert::{
    AssertTrue,
    AssertFalse,
};

pub use witness::{
    BoolFn,
    BoolWit,
};

mod sealed {
    pub trait Sealed {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logic() {
        
        // test VALUE
        assert_eq!(ConstTrue::VALUE, true);
        assert_eq!(ConstFalse::VALUE, false);
        
        assert_eq!(<And<ConstTrue, ConstFalse> as ConstBool>::VALUE, false);
        assert_eq!(<Or<ConstTrue, ConstFalse> as ConstBool>::VALUE, true);
        assert_eq!(<Eq<ConstTrue, ConstTrue> as ConstBool>::VALUE, true);
        
        // test type select
        assert_eq!(<Select<ConstTrue, i32, f64>>::default(), 0i32);
        assert_eq!(<Select<ConstFalse, i32, f64>>::default(), 0.0f64);
    }
}