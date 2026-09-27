use std::mem::forget;

/// Safety:
///   同mem::transmut_unchecked
pub(crate) const unsafe fn transmute_unchecked<T, U>(src: T) -> U {
    let ptr = &src as *const T as *const U;
    let dst = unsafe{ std::ptr::read(ptr) };
    forget(src);
    dst
}

pub(crate) const unsafe fn unit<T>() -> T {
    const{ assert!(size_of::<T>() == 0)};
    unsafe{ transmute_unchecked(()) }
}

///Mark a Struct which is a MarkerType cant be construct
pub(crate) enum MarkerType{}