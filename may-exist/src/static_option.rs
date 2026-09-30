use core::fmt::{Debug, Display};
use core::mem::{self, forget, transmute};
use core::num::{NonZero, NonZeroUsize};
use core::ops::DerefMut;
use core::pin::Pin;
use core::ptr::NonNull;

use core::option::Option;

use crate::sealed::Sealed;
use crate::utils::{transmute_unchecked, unit};
use const_bool::{ConstBool, ConstFalse, ConstTrue, Select, Xor};

// pub type IsSome = ConstTrue;
// pub type IsNone = ConstFalse;

// trait Selector: ConstBool{}
// impl<T: ConstBool> Selector for T{}

///Repr方案，人体工学更好
#[repr(transparent)]
pub struct StaticOption<T, F: ConstBool>(pub F::Select<T, ()>);

// unsafe impl<M, F: StaticBool> Core for Repr<M, F> {
//     type ExistFlag = F;
//     type Me = M;
//     type ConStucter<T> = Repr<T, F>;
// }
//

const fn addr_cast<F: ConstBool>(addr: usize) -> Option<NonZeroUsize> {
    if F::VALUE {
        return unsafe { transmute(addr) };
    }
    return None;
}

// provide compile time type info
// pub trait MetaData: Sealed {
//     type Flag: ConstBool;
//     type Inner;
// }

// Since the compiler does not know that True | False = impl StaticBool,
// do not define methods with the same names for True and False.
// Generic methods should be defined within impl StaticBool,
// and then transmute_unchecked can be used to treat them as ConstTrue or ConstFalse types.

///methods be shared of Some<T> and None
impl<T, F: ConstBool> StaticOption<T, F> {

    ///#Safety
    /// this method assume F = ConstFalse
    const unsafe fn new_assume_none() -> Self{
        unsafe{ transmute_unchecked(()) }
    }

    ///#Safety
    /// this method assume F = ConstTrue
    const unsafe fn new_assume_some(value: T) -> Self {
        unsafe{ transmute_unchecked(value) }
    }

    ///#Safety
    /// this method assume F = ConstTrue
    const unsafe fn unwarp_assume_some(self) -> T {
        unsafe{ transmute_unchecked(self) }
    }

    ///#Safety
    ///  this method assume F = ConstTrue
    const unsafe fn as_ref_assume_some(&self) -> &T {
        unsafe{ transmute(self) }
    }

    ///#Safety
    ///  this method assume F = ConstTrue
    const unsafe fn as_mut_assume_some(&mut self) -> &mut T {
        unsafe{ transmute(self) }
    }
    
    /// 
    pub fn new_none_or(value: T) -> Self {
        if F::VALUE {
            return unsafe { Self::new_assume_some(value)};
        }
        // forget(value);
        return unsafe { Self::new_assume_none() };
    }

    //=================================
    // cast the static type to dynamic type
    //

    /// return the result of “is_none"
    /// '''rust
    /// fn foobar<T, F: ConstBool>(value: StaticOption<T, F>) {
    ///     if StaticOption<(), F>::is_some() {
    ///         //do something
    ///     }
    ///     //another way is use F::VALUE directly
    ///     
    /// }
    /// '''
    pub const fn is_some() -> bool {
        F::VALUE
    }

    
    pub fn is_some_and(self, f: impl FnOnce(T) -> bool) -> bool {
        if F::VALUE {
            return f(unsafe { self.unwarp_assume_some() });
        }
        return false;
    }

    /// return the result of “is_none"
    /// '''rust
    /// fn foobar<T, F: ConstBool>(value: StaticOption<T, F>) {
    ///     if StaticOption<(), F>::is_none() {
    ///         //do something
    ///     }
    ///     //another way is use !F::VALUE directly
    /// }
    /// '''
    pub const fn is_none() -> bool {
        !F::VALUE
    }

    ///see as ["Option::is_none_of"]
    pub fn is_none_or(self, f: impl FnOnce(T) -> bool) -> bool {
        if F::VALUE {
            return f(unsafe { self.unwarp_assume_some() });
        }
        return true;
    }

    pub const fn as_ref(&self) -> StaticOption<&T, F> {
        if F::VALUE {
            return unsafe { transmute_unchecked(self) };
        }

        return unsafe { transmute_unchecked(())};
    }

    pub const fn as_mut(&mut self) -> StaticOption<&mut T, F> {
        if F::VALUE {
            return unsafe { transmute_unchecked(self) };
        }
        unsafe { transmute_unchecked(()) }
    }

    pub const fn as_pin_ref(self: Pin<&Self>) -> StaticOption<Pin<&T>, F> {
        if F::VALUE {
            return unsafe{ transmute_unchecked(self) };
        }
        unsafe{ transmute_unchecked(()) }
    }

    pub const fn as_pin_mut(self: Pin<&mut Self>) -> StaticOption<Pin<&mut T>, F> {
        if F::VALUE {
            return unsafe{ transmute_unchecked(self) }
        }

        unsafe{ transmute_unchecked(())}
    }

    pub fn into_option(self) -> Option<T> {
        if F::VALUE {
            Some(unsafe { transmute_unchecked(self) })
        } else {
            // drop cant be call in compile time now, but None can be forget directly
            //mem::forget(self);
            None
        }
    }

    pub fn unwarp_or(self, value: T) -> T {
        if F::VALUE {
            unsafe { transmute_unchecked(self) }
        } else {
            value
        }
    }

    pub fn unwarp_or_else(self, f: impl FnOnce() -> T) -> T {
        if F::VALUE {
            unsafe { transmute_unchecked(self) }
        } else {
            f()
        }
    }

    pub fn unwarp_or_default(self) -> T
    where
        T: Default,
    {
        if F::VALUE {
            unsafe { transmute_unchecked(self) }
        } else {
            T::default()
        }
    }

    pub fn or<R: ConstBool>(self, other: StaticOption<T, R>) -> StaticOption<T, F::Or<R>> {
        if F::VALUE {
            return unsafe { transmute_unchecked(self) };
        }

        if R::VALUE {
            return unsafe { transmute_unchecked(other) };
        }

        return unsafe { transmute_unchecked(()) };
    }

    pub fn or_else<R: ConstBool>(
        self,
        f: impl FnOnce() -> StaticOption<T, R>,
    ) -> StaticOption<T, F::Or<R>> {
        if F::VALUE {
            return unsafe { transmute_unchecked(self) };
        }

        let result = f();
        if R::VALUE {
            return unsafe { transmute_unchecked(result) };
        }

        return unsafe { transmute_unchecked(()) };
    }

    pub fn and<R: ConstBool>(self, other: StaticOption<T, R>) -> StaticOption<T, F> {
        if F::VALUE {
            if R::VALUE {
                return unsafe { transmute_unchecked(other) };
            }
            return unsafe { transmute_unchecked(self) };
        }

        return unsafe { transmute_unchecked(()) };
    }

    pub fn and_then<RF: ConstBool, R>(
        self,
        f: impl FnOnce(T) -> StaticOption<R, RF>,
    ) -> StaticOption<R, F::And<RF>> {
        if F::VALUE {
            unsafe {
                let arg = transmute_unchecked(self);
                let result = f(arg);
                return transmute_unchecked(result);
            }
        };

        unsafe { transmute_unchecked(self) }
    }

    pub fn map<U>(self, f: impl FnOnce(T) -> U) -> StaticOption<U, F> {
        if F::VALUE {
            unsafe {
                let arg = transmute_unchecked(self);
                let result = f(arg);
                transmute_unchecked(result)
            }
        } else {
            unsafe { transmute_unchecked(()) }
        }
    }

    //fn replace

    //fn zip

    pub fn xor<R: ConstBool>(self, other: StaticOption<T, R>) -> StaticOption<T, Xor<F, R>> {
        match (F::VALUE, R::VALUE) {
            (true, false) => unsafe { transmute_unchecked(self) },
            (false, true) => unsafe { transmute_unchecked(other) },
            _ => unsafe { transmute_unchecked(()) },
        }
    }
}

impl<T> StaticOption<T, ConstTrue> {
    pub const fn new(value: T) -> Self {
        StaticOption(value)
    }

    pub const fn unwarp(self) -> T {
        unsafe { transmute_unchecked(self) }
    }

    //const fn assert_some()
}

impl<T> StaticOption<T, ConstFalse> {
    pub const fn new_none() -> Self {
        StaticOption(())
    }

    pub const fn assert_none(self) { /*do nothing */
    }
}

// impl<T, F: StaticBool> StaticOption<&T, F> {
//     pub const fn cloned(self) ->
// }

// impl<T> Deref for StaticOption<T, ConstTrue> {

// }

// impl<T> DerefMut for StaticOption<T, ConstTrue> {

// }

/// 编译器不知道 T: Debug => F::Select<T, ()>: Debug，所以我们必须代替编译器推断 
impl<T, F: ConstBool> Debug for StaticOption<T, F> 
where
    T: Debug
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if F::VALUE {
            // ConstTrue: 内部是 T
            let value: &T = unsafe{ transmute(&self.0) };
            f.debug_tuple("Some").field(value).finish()
        } else {
            // ConstFalse: 内部是 ()
            f.write_str("None")
        }
    }
}

/// 类似std::option,仅为Some实现
impl<T: Display> Display for StaticOption<T, ConstTrue> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.0.fmt(f)
    }
}

///
/// 编译器不知道 T: Clone => F::Select<T, ()>: Clone，所以我们必须代替编译器推断 
impl<T, F: ConstBool> Clone for StaticOption<T, F>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        //Self(self.0.clone())
        if F::VALUE {
            unsafe { 
                let value: &T = transmute(self);
                return transmute_unchecked(value.clone())
            };
        } else {
            return unsafe { transmute_unchecked(()) };
        }
    }
}

///由于编译器限制，无法直接从T: Copy 中推出StaticOption: Copy
impl<T, F: ConstBool> Copy for StaticOption<T, F> 
where 
    F::Select<T, ()>: Copy,
    T: Copy,
{}

//impl<T, F: ConstBool> Copy for StaticOption<T, F> where F::Select<T, ()>: Copy {}

//暂时还不能确定Eq等比较操作的语义
impl<T: PartialEq, F: ConstBool> PartialEq for StaticOption<T, F> {
    fn eq(&self, other: &Self) -> bool {
        if F::VALUE {
            let a: &T = unsafe{ transmute(self) };
            let b: &T = unsafe{ transmute(other) };
            return a.eq(b)
        }
        return true
    }
}

impl<T: Eq, F: ConstBool> Eq for StaticOption<T, F> {}
