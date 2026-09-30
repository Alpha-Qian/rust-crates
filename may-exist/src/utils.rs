use core::mem::forget;

use const_bool::ConstBool;

use crate::StaticOption;

/// Safety:
///   同mem::transmut_unchecked
pub(crate) const unsafe fn transmute_unchecked<T, U>(src: T) -> U {
    let ptr = &src as *const T as *const U;
    let dst = unsafe{ core::ptr::read(ptr) };
    forget(src);
    dst
}

/// #Safety
///   生成带Drop的类型会导致副作用，
///   不可生成Never类型
/// 
pub(crate) const unsafe fn unit<T>() -> T {
    const{ assert!(size_of::<T>() == 0)};
    unsafe{ transmute_unchecked(()) }
}

///#Safety
/// 此函数假设F 为ConstFalse
pub(crate) const unsafe fn new_assume_none<T, F: ConstBool>() -> StaticOption<T, F> {
    unsafe{ transmute_unchecked(()) }
}

///Mark a Struct which is a MarkerType cant be construct
pub(crate) enum MarkerType{}