use std::fmt::{Debug, Display};
use std::num::{NonZero, NonZeroUsize};
use std::mem::{self, forget, transmute};
use std::ops::DerefMut;
use std::ptr::NonNull;

use std::option::Option;

use crate::sealed::Sealed;
use crate::static_bool::{ConstFalse, ConstTrue, ConstBool, Xor};
use crate::utils::{transmute_unchecked, unit};

///Repr方案，人体工学更好
#[repr(transparent)]
pub struct StaticOption<T, F: ConstBool>(pub F::Select<T, ()>);

// unsafe impl<M, F: StaticBool> Core for Repr<M, F> {
//     type ExistFlag = F;
//     type Me = M;
//     type ConStucter<T> = Repr<T, F>;
// }
// 

const fn addr_cast<F: ConstBool>(addr: usize) -> Option<NonZeroUsize>{
    if F::VALUE {
        return unsafe{ transmute(addr) }
    } 
    return None
}


///编译期类型信息
pub trait MetaData: Sealed{
    type Flag: ConstBool;
    type Inner;
}

// 因为编译器不知道 True | False = impl StaticBool
// 所以不要为True和False定义相同的方法名
// 通用方法应该定义在impl StaticBool中

///Some和None共享方法
impl<T, F: ConstBool> StaticOption<T, F> {

    pub const fn new_or_forget(value: T) -> Self {
        if F::VALUE {
            return unsafe{ transmute_unchecked(value) };
        }
        forget(value);
        return unsafe{ transmute_unchecked(()) }
    }


    //=================================
    // 将静态类型转为动态类型
    // 
    
    pub const fn is_some() -> bool {
        F::VALUE
    }

    pub fn is_some_and(self, f: impl FnOnce(T) -> bool) -> bool {
        if F::VALUE {
            return f(unsafe{ transmute_unchecked(self) })
        }
        return false
    }

    pub const fn is_none() -> bool {
        F::VALUE
    }

    pub fn is_none_or(self, f: impl FnOnce(T) -> bool) -> bool {
        if F::VALUE {
            return f(unsafe{ transmute_unchecked(self) });
        }
        return true
    }

    pub const fn as_ref(&self) -> StaticOption<&T, F> {
        if F::VALUE {
            return unsafe{ transmute_unchecked(self) }
        }

        return unsafe{ transmute_unchecked(()) }
    }

    pub const fn as_mut(&mut self) -> StaticOption<&mut T, F> {
        if F::VALUE{
            return unsafe{ transmute_unchecked(self) }
        }

        return unsafe{ transmute_unchecked(()) }
    }

    //pub const fn as_raw

    //pub const fn as_const

    // pub const fn as_nonnull


    pub const fn into_option(self) -> Option<T> {
        if F::VALUE {
            Some(unsafe{ transmute_unchecked(self) })
        } else {
            //强制忽略drop无法在编译期调用错误
            mem::forget(self);
            None
        }
    }

    pub fn unwarp_or(self, value: T) -> T {
        if F::VALUE {
            unsafe{ transmute_unchecked(self) }
        } else {
            mem::forget(self);
            value
        }
    }

    pub fn unwarp_or_else(self, f: impl FnOnce() -> T) -> T{
        if F::VALUE {
            unsafe{ transmute_unchecked(self) }
        } else { f() }
    }

    pub fn unwarp_or_default(self) -> T
    where T: Default
    {
        if F::VALUE {
            unsafe{ transmute_unchecked(self) }
        } else { T::default() }
    }

    pub fn or<R: ConstBool>(self, other: StaticOption<T, R>) -> StaticOption<T, F::Or<R>> {
        if F::VALUE {
            return unsafe{ transmute_unchecked(self) }
        }

        if R::VALUE {
            return unsafe{ transmute_unchecked(other) }
        }

        return unsafe{ transmute_unchecked(()) }
    }

    pub fn or_else<R: ConstBool>(self, f: impl FnOnce() -> StaticOption<T, R>) -> StaticOption<T, F::Or<R>> {
        if F::VALUE {
            return unsafe{ transmute_unchecked(self) }
        }

        let result = f();
        if R::VALUE {
            return unsafe { transmute_unchecked(result)} 
        }

        return unsafe{ transmute_unchecked(()) }
    }

    pub fn and<R: ConstBool>(self, other: StaticOption<T, R>) -> StaticOption<T, F::And<R>> {
        if F::VALUE {
            if R::VALUE {
                return unsafe{ transmute_unchecked(other )}
            }
            return unsafe{ transmute_unchecked(self) }
        }

        return unsafe{ transmute_unchecked(())}
    }
    
    pub fn and_then<RF: ConstBool, R>(self, f: impl FnOnce(T) -> StaticOption<R, RF>) -> StaticOption<T, F::And<RF>> {
        if F::VALUE {
            unsafe{
                let arg = transmute_unchecked(self);
                let result = f(arg);
                return transmute_unchecked(result);
            }
        };

        unsafe{ transmute_unchecked(self) }
    }
    
    pub fn map<U>(self, f: impl FnOnce(T) -> U) -> StaticOption<U, F> {
        if F::VALUE {
            unsafe {
                let arg = transmute_unchecked(self);
                let result = f(arg);
                transmute_unchecked(result)
            }
        } else { unsafe{ transmute_unchecked(())} }
    }

    //fn replace

    //fn zip
    
    pub fn xor<R: ConstBool>(self, other: StaticOption<T, R>) -> StaticOption<T, Xor<F, R>> {
        match (F::VALUE, R::VALUE) {
            (true, false) => unsafe{ transmute_unchecked(self) },
            (false, true) => unsafe{ transmute_unchecked(other) },
            _ => unsafe{ transmute_unchecked(()) }
        }
    }
    
}

impl<T> StaticOption<T, ConstTrue> {

    pub const fn new(value: T) -> Self {
        StaticOption(value)
    }

    pub const fn unwarp(self) -> T {
        unsafe{ transmute_unchecked(self) }
    }

    //const fn assert_some()
    
}


impl<T> StaticOption<T, ConstFalse> {

    pub const fn new_none() -> Self {
        StaticOption(())
    }
    
    pub const fn assert_none(self) {/*do nothing */}
}

// impl<T, F: StaticBool> StaticOption<&T, F> {
//     pub const fn cloned(self) -> 
// }


// impl<T> Deref for StaticOption<T, ConstTrue> {
    
// }

// impl<T> DerefMut for StaticOption<T, ConstTrue> {
    
// }


impl<T: Debug, F: ConstBool> Debug for StaticOption<T, F> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl<T: Display, F: ConstBool> Display for StaticOption<T, F> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl<T, F: ConstBool> Clone for StaticOption<T, F> 
where 
    F::Select<T, ()>: Clone
{
    fn clone(&self) -> Self {
        if F::VALUE {
            return unsafe{ Self(transmute_unchecked(&self.0)).clone()}
        } else {
            return unsafe{ transmute_unchecked(()) }
        }
    }
}

impl<T, F: ConstBool> Copy for StaticOption<T, F>
where
    F::Select<T, ()>: Copy
{}


//暂时还不能确定Eq等比较操作的语义
impl<T: PartialEq, F: ConstBool> PartialEq for StaticOption<T, F> {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

impl<T: Eq, F: ConstBool> Eq for StaticOption<T, F> {}
