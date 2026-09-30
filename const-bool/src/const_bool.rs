use crate::sealed::Sealed;

/// # Safety
/// The implementors can and must only be `ConstTrue` and `ConstFalse`.
/// The implemented content must be correct.
/// Other `unsafe` code can assume the invariants in the trait.
pub unsafe trait ConstBool: Sealed {
    const VALUE: bool;

    // allow T: ?Size, F: ?Sized is imposble;
    // because the compeliter can not understand 'T: Sized & F: Sized => <B as ConstBool>::Select<T, F>: Sized'
    // it makes write genic code both of ConstTrue and ConstFalse is imposible
    ///True => T; False => F
    type Select<T, F>;

    type Not: ConstBool;

    type Or<R: ConstBool>: ConstBool;

    type And<R: ConstBool>: ConstBool;

    type Eq<R: ConstBool>: ConstBool;

    //type BiggerThan<R: ConstBool>: ConstBool;
}

pub enum ConstTrue{}

impl Sealed for ConstTrue {}

unsafe impl ConstBool for ConstTrue {
    const VALUE: bool = true;

    type Select<T, F> = T;

    type Not = ConstFalse;

    type Or<R: ConstBool> = ConstTrue;

    type And<R: ConstBool> = R;

    type Eq<R: ConstBool> = R;

    //type BiggerThan<R: ConstBool> = R::Neg;
}

pub enum ConstFalse{}

impl Sealed for ConstFalse {}

unsafe impl ConstBool for ConstFalse {
    const VALUE: bool = false;

    type Select<T, F> = F;

    type Not = ConstTrue;

    type Or<R: ConstBool> = R;

    type And<R: ConstBool> = ConstFalse;

    type Eq<R: ConstBool> = R::Not;

    //type BiggerThan<R: ConstBool> = ConstFalse;
}

pub type Select<B, T, F> = <B as ConstBool>::Select<T, F>;

pub type Not<B> = <B as ConstBool>::Not;

pub type Or<A, B> = <A as ConstBool>::Or<B>;

pub type And<A, B> = <A as ConstBool>::And<B>;

pub type Eq<A, B> = <A as ConstBool>::Eq<B>;

pub type Xor<A, B> = Not<Eq<A, B>>;

pub type Nor<A, B> = Not<Or<A, B>>;

pub type Nand<A, B> = Not<And<A, B>>;

