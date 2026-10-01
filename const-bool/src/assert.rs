use crate::{ConstBool, ConstFalse, ConstTrue};

/// #Safety
///   必须仅有ConstTrue实现，其他unsafe代码可以依赖此代码
/// 
/// A helper trait help use write code like "where Or<A, B>: AssertTrue"
/// or user should write "where A::Or<B> = ConstTrue"
/// 
/// 该trait仅用于简化写法，编译器并不知道只有ConstTrue实现了AssertTrue，如果编译器无法正确推断，应尝试改回原来的写法
pub unsafe trait AssertTrue: ConstBool {}

/// #Safety
///   必须仅有ConstFalse实现，其他unsafe代码可以依赖此代码
/// 
/// A helper trait help use write code like "where And<A, B>: AssertFalse"
/// or user should write "A::And<B> = ConstFalse"
/// 
/// 该trait仅用于简化写法，编译器并不知道只有ConstTrue实现了AssertTrue，如果编译器无法正确推断，应尝试改回原来的写法
pub unsafe trait AssertFalse: ConstBool {}

unsafe impl AssertTrue for ConstTrue {}

unsafe impl AssertFalse for ConstFalse {}