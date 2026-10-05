use crate::{BoolWit, ConstBool, ConstFalse, ConstTrue, witness::Is};

type Select<A, T, F> = <A as ConstBool>::SelectBool<T, F>;

type Not<A> = <A as ConstBool>::SelectBool<ConstFalse, ConstTrue>;

type Or<A, B> = <A as ConstBool>::SelectBool<ConstTrue, B>;

type SelectAnd<A, B> = <A as ConstBool>::SelectBool<B, ConstFalse>;

type Eq<A, B> = <A as ConstBool>::SelectBool<B, Not<B>>;

type Xor<A, B> = <A as ConstBool>::SelectBool<Not<B>, B>;

type SelectNor<A, B> = <A as ConstBool>::SelectBool<ConstFalse, Not<B>>;

type SelectNand<A, B> = <A as ConstBool>::SelectBool<Not<B>, ConstTrue>;

type Me<A> = Select<A, ConstTrue, ConstFalse>;



fn proof_select_flip<A: ConstBool, T: ConstBool, F: ConstBool>() -> Is<Select<A, T, F>, Select<Not<A>, F, T>> {
    unsafe{ Is::proof_unchecked() }
}

fn proof_not_not<A: ConstBool>() -> Is<Not<Not<A>>, A> {
    match A::WIT {
        BoolWit::True(is_true) => todo!(),
        BoolWit::False(is_false) => todo!()
    }
    unsafe{ Is::proof_unchecked() }
}

fn proof_xor_swap<A: ConstBool, B: ConstBool>() -> Is<Xor<A, B>, Xor<B, A>> {
    match <Xor<A, B> as ConstBool>::WIT{
        BoolWit::True(is_true) => unsafe {
            
        }
        BoolWit::False(is_false) => todo!(),
    }
}

fn proof_not_flip<A: ConstBool, B: ConstBool>() -> Is<Select<A, B, Not<B>>, Select<Not<A>, Not<B>, B>> {
    let eq = unsafe{ Is::proof_unchecked() };
    eq
}