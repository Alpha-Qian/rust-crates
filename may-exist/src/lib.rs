#![no_std]
//! A is Some or None Option Type know in compile time,surpot map(), and_then() and(), or, into_option
//! 
//! #警告
//!   该库不是一个以unsafe代码较少而著称的crate；
//!   尽管编译期Option有多种unsafe 较少甚至完全不需要unsafe的实现方式，但这些实现方式无一例外都牺牲了少量或大量使用者的人体工程学；
//!   该库的解决方式是通过大量使用unsafe，最大程度地换取使用者地人体工程。虽然现在已知有一种大量减少unsafe代码且对使用者友好地模式(即type witness)模式，
//!   但该模式也是对实现者最不友好地模式，是否使用该模式还在考虑中 ：） ；
//!   如果你比较在意，建议使用前先初步审查该库的实现方法，或使用unsafe较少的crate，比如 StaticOption
//! 

// 下面是一些能实现StataOption的方法和不使用的原因
// unsafe强制转换（当前正在使用）
// 分别定义Some<T>和None结构体，通用方法定义在trait中    不支持const方法
// 按具体类型写 inherent impl                         编译器不知道只有ConstTrue和ConstFalse实现ConstBool，会使使用者不得不将相同的方法在ConstTrue和ConstFalse下各定义一遍
// 使用 typewitness证明类型相等性                        未来可能会使用此方法

//mod may_exist;

pub mod preload;
pub mod static_option;
mod witness;
mod wit_fn;
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
