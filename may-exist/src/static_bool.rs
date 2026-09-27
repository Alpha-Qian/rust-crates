use crate::{sealed::Sealed, utils::MarkerType};

pub trait ConstBool: Sealed {
    const VALUE: bool;

    //True => T; False => F
    type Select<T, F>;

    type Neg: ConstBool;

    type Or<R: ConstBool>: ConstBool;

    type And<R: ConstBool>: ConstBool;

    type Eq<R: ConstBool>: ConstBool;

    type BiggerThan<R: ConstBool>: ConstBool;
}

pub struct ConstTrue(MarkerType);

impl Sealed for ConstTrue {}

impl ConstBool for ConstTrue {
    const VALUE: bool = true;

    type Select<T, F> = T;

    type Neg = ConstFalse;

    type Or<R: ConstBool> = ConstTrue;

    type And<R: ConstBool> = R;

    type Eq<R: ConstBool> = R;

    type BiggerThan<R: ConstBool> = R::Neg;
}

pub struct ConstFalse(MarkerType);

impl Sealed for ConstFalse {}

impl ConstBool for ConstFalse {
    const VALUE: bool = false;

    type Select<T, F> = F;

    type Neg = ConstTrue;

    type Or<R: ConstBool> = R;

    type And<R: ConstBool> = ConstFalse;

    type Eq<R: ConstBool> = R::Neg;

    type BiggerThan<R: ConstBool> = ConstFalse;
}

pub type Select<B: ConstBool, T, F> = B::Select<T, F>;

pub type Neg<B: ConstBool> = B::Neg;

pub type Or<A: ConstBool, B: ConstBool> = A::Or<B>;

pub type And<A: ConstBool, B: ConstBool> = A::And<B>;

pub type Xor<A: ConstBool, B: ConstBool> = And<Or<A, B>, Neg<And<A, B>>>;

pub type Eq<A: ConstBool, B: ConstBool> = Neg<Xor<A, B>>;

pub type Nor<A: ConstBool, B: ConstBool> = <A::Or<B> as ConstBool>::Neg;

pub type Nand<A: ConstBool, B: ConstBool> = <A::And<B> as ConstBool>::Neg;

pub type Or3<A: ConstBool, B: ConstBool, C: ConstBool> = A::Or<B::Or<C>>;

pub type And3<A: ConstBool, B: ConstBool, C: ConstBool> = A::And<B::And<C>>;

pub type Bigger<L: ConstBool, R: ConstBool> = L::BiggerThan<R>;

pub type Less<L: ConstBool, R: ConstBool> = R::BiggerThan<L>;

pub type StaticOption<B: ConstBool, T> = B::Select<T, ()>;

pub type StaticResult<B: ConstBool, T, E> = B::Select<T, E>;
