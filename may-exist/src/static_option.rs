use core::fmt::{Debug, Display};
use core::mem::{self, forget, transmute};
use core::num::{NonZero, NonZeroUsize};
use core::ops::DerefMut;
use core::pin::Pin;
use core::ptr::NonNull;

use core::option::Option;

use crate::sealed::Sealed;
use crate::utils::{transmute_unchecked, unit};
use const_bool::{BoolWit, ConstBool, ConstFalse, ConstTrue, Select, Xor};

///Repr方案，人体工学更好
#[repr(transparent)]
pub struct StaticOption<T, F: ConstBool>(pub F::Select<T, ()>);

impl<T, F: ConstBool> StaticOption<T, F> {
    pub const IS_SOME: bool = F::VALUE;
    pub const IS_NONE: bool = !F::VALUE;
    pub const WIT: BoolWit<F> = F::WIT; 
}

impl<T> StaticOption<T, ConstTrue> {
    fn into_inner(self) -> T {
        self.0
    }

    fn assert_some() {}
}

impl<T> StaticOption<T, ConstFalse> {
    fn assert_none() {}
}