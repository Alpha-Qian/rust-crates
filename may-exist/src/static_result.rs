use const_bool::{ConstBool, ConstFalse, ConstTrue};

use core::{
    mem::transmute,
    result::{self, Result},
};

use crate::{
    Or, StaticOption,
    utils::transmute_unchecked,
};

pub struct StaticResult<T, E, F: ConstBool>(pub F::Select<T, E>);

//trait
impl<T, E, F: ConstBool> StaticResult<T, E, F> {
    pub fn new_ok(value: T) -> StaticResult<T, E, ConstTrue> {
        StaticResult(value)
    }

    pub fn new_err(error: E) -> StaticResult<T, E, ConstFalse> {
        StaticResult(error)
    }

    pub fn ok(self) -> StaticOption<T, F> {
        unsafe { transmute_unchecked(self) }
    }

    pub fn err(self) -> StaticOption<T, F::Not> {
        unsafe { transmute_unchecked(()) }
    }

    pub fn map<U>(self, f: impl FnOnce(T) -> U) -> StaticResult<U, E, F> {
        if F::VALUE {
            return unsafe {
                let arg = transmute_unchecked(self);
                let result = f(arg);
                transmute_unchecked(result)
            };
        }
        return unsafe { transmute_unchecked(()) };
    }

    pub fn map_err<U>(self, f: impl FnOnce(E) -> U) -> StaticResult<T, U, F> {
        if !F::VALUE {
            return unsafe {
                let arg = transmute_unchecked(self);
                let result = f(arg);
                transmute_unchecked(result)
            };
        }
        return unsafe { transmute_unchecked(()) };
    }

    pub fn as_ref(&self) -> StaticResult<&T, &E, F> {
        return unsafe { transmute_unchecked(&self.0) };
    }

    pub fn as_mut(&mut self) -> StaticResult<&mut T, &mut E, F> {
        unsafe { transmute_unchecked(&self.0) }
    }

    pub fn and_then<U, RF: ConstBool>(
        self,
        f: impl FnOnce(T) -> StaticResult<U, E, RF>,
    ) -> StaticResult<U, E, F::And<RF>> {
        if F::VALUE {
            return unsafe {
                let arg = transmute_unchecked(self);
                let result = f(arg);
                transmute_unchecked(result)
            };
        }
        return unsafe { transmute_unchecked(()) };
    }

    //fn and

    fn or_else<F2: ConstBool, E2>(
        self,
        f: impl FnOnce(E) -> StaticResult<T, E2, F2>,
    ) -> StaticResult<T, E, Or<F, F2>> {
        if F::VALUE {
            return unsafe { transmute_unchecked(self) };
        }

        let err: E = unsafe { transmute_unchecked(self) };

        let result = f(err);

        // if F2::VALUE {
        //     return unsafe{ transmute_unchecked(result) };
        // }
        return unsafe { transmute_unchecked(result) };
    }

    //fn bimap
}

impl<T, E> StaticResult<T, E, ConstTrue> {
    fn unwarp(self) -> T {
        self.0
    }
}

impl<T, E> StaticResult<T, E, ConstFalse> {
    fn unwarp_err(self) -> E {
        self.0
    }
}
