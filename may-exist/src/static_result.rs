use const_bool::{BoolFn, BoolWit, ConstBool, ConstFalse, ConstTrue, Is};

use core::{
    marker::PhantomData, mem::transmute, result::{self, Result},
};

use crate::{Or, StaticOption, utils::transmute_unchecked};

pub struct StaticEither<T, F, B: ConstBool>(pub B::Select<T, F>);

impl<T, F, B: ConstBool> StaticEither<T, F, B> {

    fn flip2(self) -> StaticEither<F, T, B::Not> {
        //proof target: Is< B::Select<T, F>, B::Not::Select<F, T>>
        let eq: Is<B, B> = Is::refl();
        let eq = eq.project::<SelFn<T, F>>()

        todo!()
    }
    
    fn flip(self) -> StaticEither<F, T, B::Not> {
        //proof target: Is< B::Select<T, F>, B::Not::Select<F, T>>
        match B::WIT {
            BoolWit::True(eq) => {
                let eq_1 = eq.project::<SelFn<T, F>>();
                let eq_2 = eq.project::<NotFn>().project::<SelFn<F, T>>().flip();
                
                let eq = Self::REPR
                    .then(eq_1)
                    .then(eq_2)
                    .then(StaticEither::<F, T, B::Not>::FROM_INNER);
                eq.cast(self)
            },
            BoolWit::False(eq) => {
                let eq_1 = eq.project::<SelFn<T, F>>();
                let eq_2 = eq.project::<NotFn>().project::<SelFn<F, T>>().flip();
                
                let eq = Self::REPR
                    .then(eq_1)
                    .then(eq_2)
                    .then(StaticEither::<F, T, B::Not>::FROM_INNER);
                eq.cast(self)
            }
        }
    }

    fn true_as_ref(&self) -> StaticOption<&T, B> {
        todo!()
        // match B::WIT {
        //     BoolWit::True(eq) => eq.project::<SelFn<T, >()
        // }
    }

    fn true_as_mut(&mut self) -> StaticOption<&mut T, B> {
        todo!()
    }

    fn false_as_ref(&self) -> StaticOption<&F, B> {
        todo!()
    }

    fn false_as_mut(&mut self) -> StaticOption<&mut F, B> {
        todo!()
    }
}




impl<T, F, B: ConstBool> StaticEither<T, F, B> {

    const REPR: Is<Self, B::Select<T, F>> = unsafe{ Is::proof_unchecked()};
    
    const FROM_INNER: Is<B::Select<T, F>, Self> = Self::REPR.flip();
    
    const fn from_true(eq: Is<B, ConstTrue>, true_vale: T) -> Self {
        let eq = eq.project::<SelFn<T, F>>().flip().then(Self::REPR.flip());
        eq.cast(true_vale)
    }

    const fn from_false(eq: Is<B, ConstFalse>, false_value: F) -> Self {
        let eq = eq.project::<SelFn<T, F>>().flip().then(Self::REPR.flip());
        eq.cast(false_value)
    }

    const fn into_true(self, eq: Is<B, ConstTrue>) -> T {
        let eq = Self::REPR.then(eq.project::<SelFn<T, F>>());
        eq.cast(self)
    }

    const fn into_false(self, eq: Is<B, ConstFalse>) -> F {
        let eq = Self::REPR.then(eq.project::<SelFn<T, F>>());
        eq.cast(self)
    }

    const fn ref_true(&self, eq: Is<B, ConstTrue>) -> &T {
        let eq = Self::REPR.then(eq.project::<SelFn<T, F>>());
        eq.cast_ref(self)
    }

    const fn mut_true(&mut self, eq: Is<B, ConstTrue>) -> &mut T {
        let eq = Self::REPR.then(eq.project::<SelFn<T, F>>());
        eq.cast_mut(self)
    }

    const fn ref_false(&self, eq: Is<B, ConstFalse>) -> &F {
        let eq = Self::REPR.then(eq.project::<SelFn<T, F>>());
        eq.cast_ref(self)
    }

    const fn mut_false(&mut self, eq: Is<B, ConstFalse>) -> &mut F {
        let eq = Self::REPR.then(eq.project::<SelFn<T, F>>());
        eq.cast_mut(self)
    }
}

// struct NotSelFn<T, F>(PhantomData<(T, F)>);



struct NotFn;

impl BoolFn for NotFn {
    type Apply<F: ConstBool> = F::Not;
}


struct SelFn<T, F>(PhantomData<(T, F)>);

impl<T, F> BoolFn for SelFn<T, F> {
    type Apply<B: ConstBool> = B::Select<T, F>;
}











// //trait
// impl<T, E, F: ConstBool> StaticResult<T, E, F> {
//     pub fn new_ok(value: T) -> StaticResult<T, E, ConstTrue> {
//         StaticResult(value)
//     }

//     pub fn new_err(error: E) -> StaticResult<T, E, ConstFalse> {
//         StaticResult(error)
//     }

//     pub fn ok(self) -> StaticOption<T, F> {
//         unsafe { transmute_unchecked(self) }
//     }

//     pub fn err(self) -> StaticOption<T, F::Not> {
//         unsafe { transmute_unchecked(()) }
//     }

//     pub fn map<U>(self, f: impl FnOnce(T) -> U) -> StaticResult<U, E, F> {
//         if F::VALUE {
//             return unsafe {
//                 let arg = transmute_unchecked(self);
//                 let result = f(arg);
//                 transmute_unchecked(result)
//             };
//         }
//         return unsafe { transmute_unchecked(()) };
//     }

//     pub fn map_err<U>(self, f: impl FnOnce(E) -> U) -> StaticResult<T, U, F> {
//         if !F::VALUE {
//             return unsafe {
//                 let arg = transmute_unchecked(self);
//                 let result = f(arg);
//                 transmute_unchecked(result)
//             };
//         }
//         return unsafe { transmute_unchecked(()) };
//     }

//     pub fn as_ref(&self) -> StaticResult<&T, &E, F> {
//         return unsafe { transmute_unchecked(&self.0) };
//     }

//     pub fn as_mut(&mut self) -> StaticResult<&mut T, &mut E, F> {
//         unsafe { transmute_unchecked(&self.0) }
//     }

//     pub fn and_then<U, RF: ConstBool>(
//         self,
//         f: impl FnOnce(T) -> StaticResult<U, E, RF>,
//     ) -> StaticResult<U, E, F::And<RF>> {
//         if F::VALUE {
//             return unsafe {
//                 let arg = transmute_unchecked(self);
//                 let result = f(arg);
//                 transmute_unchecked(result)
//             };
//         }
//         return unsafe { transmute_unchecked(()) };
//     }

//     //fn and

//     fn or_else<F2: ConstBool, E2>(
//         self,
//         f: impl FnOnce(E) -> StaticResult<T, E2, F2>,
//     ) -> StaticResult<T, E, Or<F, F2>> {
//         if F::VALUE {
//             return unsafe { transmute_unchecked(self) };
//         }

//         let err: E = unsafe { transmute_unchecked(self) };

//         let result = f(err);

//         // if F2::VALUE {
//         //     return unsafe{ transmute_unchecked(result) };
//         // }
//         return unsafe { transmute_unchecked(result) };
//     }

//     //fn bimap
// }

// impl<T, E> StaticResult<T, E, ConstTrue> {
//     fn unwarp(self) -> T {
//         self.0
//     }
// }

// impl<T, E> StaticResult<T, E, ConstFalse> {
//     fn unwarp_err(self) -> E {
//         self.0
//     }
// }
