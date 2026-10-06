//!Basic lemma proof by crate

mod swap_law;
mod bool_select;

use core::marker::PhantomData;

use crate::{Not, Or};
use crate::witness::{BoolFn, BoolWit, Is, TypeFn};
use crate::const_bool::{ConstBool, ConstTrue, ConstFalse};

struct NotFn;
impl BoolFn for NotFn { type Apply<F: ConstBool> = Not<F>;} 



const fn not_not_eq<A: ConstBool>() -> Is<Not<Not<A>>, A> {
    match A::WIT {
        BoolWit::True(eq) => eq.project::<NotFn>().project::<NotFn>().then(eq.flip()),
        BoolWit::False(eq) => eq.project::<NotFn>().project::<NotFn>().then(eq.flip()),
    }
}

struct OrNotFn<A>(PhantomData<A>);
impl<A: ConstBool> BoolFn for OrNotFn<A> {
    type Apply<F: ConstBool> = Or<F, A>;
}
const fn or_not_eq<A: ConstBool>() -> Is<Or<A, Not<A>>, ConstTrue> {
    match A::WIT {
        BoolWit::True(eq) => eq.project::<OrNotFn<A>>()
    }
}

struct AndNotFn<A>(Pha)