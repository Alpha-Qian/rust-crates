//!
//! 

use core::marker::PhantomData;

use crate::{ConstBool, ConstFalse, ConstTrue};

/// 一个在运行时不占用任何内存的标记，用于承载编译期布尔状态
/// 
/// #example
/// 
/// '''
/// fn foo<F: ConstBool>(flag: ConstFlag<F>) -> bool { F::VALUE }
/// fn bar<F: ConstBool>() -> bool { F::VALUE }
/// 
/// foo(ConstFlag::TRUE);
/// // similar to:
/// 
/// bar::<ConstTrue>()
/// '''
pub struct ConstFlag<B: ConstBool>(PhantomData<B>);

impl ConstFlag<ConstTrue> {
    pub const TRUE: Self = Self(PhantomData);
}

impl ConstFlag<ConstFalse> {
    pub const FALSE: Self = Self(PhantomData);
}