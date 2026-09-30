use crate::sealed::Sealed;

///Safty:
/// 实现者有且仅能有ConstTrue和ConstFalse两个
/// 实现的内容必须正确
/// 其他unsafe代码可以假设trait中的不变量
pub unsafe trait ConstBool: Sealed {
    const VALUE: bool;

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

pub type Select<B: ConstBool, T, F> = B::Select<T, F>;

pub type Not<B: ConstBool> = B::Not;

pub type Or<A: ConstBool, B: ConstBool> = A::Or<B>;

pub type And<A: ConstBool, B: ConstBool> = A::And<B>;

pub type Xor<A: ConstBool, B: ConstBool> = And<Or<A, B>, Not<And<A, B>>>;

pub type Eq<A: ConstBool, B: ConstBool> = Not<Xor<A, B>>;

pub type Nor<A: ConstBool, B: ConstBool> = <A::Or<B> as ConstBool>::Not;

pub type Nand<A: ConstBool, B: ConstBool> = <A::And<B> as ConstBool>::Not;

