use crate::{ConstBool, ConstFalse, ConstTrue};


/// A helper trait help use write code like "where Or<A, B>: AssertTrue"
/// or user should write "A::Or<B> = ConstTrue"
pub unsafe trait AssertTrue: ConstBool {}

/// A helper trait help use write code like "where And<A, B>: AssertFalse"
/// or user should write "A::And<B> = ConstFalse"
pub unsafe trait AssertFalse: ConstBool {}

unsafe impl AssertTrue for ConstTrue {}

unsafe impl AssertFalse for ConstFalse{}