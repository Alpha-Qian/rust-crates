#![no_std]
//! A is Some or None Option Type know in compile time,surpot map(), and_then() and(), or, into_option
//! 
//! #警告
//!   该库不是一个以unsafe代码较少而著称的crate；
//!   尽管编译期Option有多种unsafe 较少甚至完全不需要unsafe的实现方式，但这些实现方式无一例外都牺牲了少量或大量使用者的人体工程学；
//!   该库的解决方式是通过大量使用unsafe，最大程度地换取使用者地人体工程。虽然现在已知有一种大量减少unsafe代码且对使用者友好地模式(即type witness)模式
//! ，但该模式也是对实现者最不友好地模式，是否使用该模式还在考虑中 ：） ；
//!   如果你比较在意，建议使用前先初步审查该库的实现方法，或使用unsafe较少的crate，比如 StaticOption
//! 

//mod may_exist;

pub mod preload;
pub mod static_option;
mod flag;
mod static_result;
mod utils;

pub use const_bool::*;
pub use crate::static_option::{StaticOption};

#[cfg(test)]
mod test {
    #[test]
    fn test() {}
}

mod sealed {
    pub trait Sealed {}
}
