use std::marker::PhantomData;

use crate::{And, ConstBool, Eq, Or, sealed::Sealed, static_bool::{Bigger, Less}, utils::MarkerType};

struct StaticU8<
F0: ConstBool,
F1: ConstBool,
F2: ConstBool,
F3: ConstBool,
F4: ConstBool,
F5: ConstBool,
F6: ConstBool,
F7: ConstBool>(
    PhantomData<(F0, F1, F2, F3, F4, F5, F6, F7)>,
    MarkerType
);

trait ConstU8: Sealed{
    const VALUE: u8;

    type F0: ConstBool;
    type F1: ConstBool;
    type F2: ConstBool;
    type F3: ConstBool;
    type F4: ConstBool;
    type F5: ConstBool;
    type F6: ConstBool;
    type F7: ConstBool;

    type BitOr<T: ConstU8>: ConstU8;
    type BitAnd<T: ConstU8>: ConstU8;
    type BitXor<T: ConstU8>: ConstU8;
    type Eq<T: ConstU8>: ConstBool;
    type BiggerThan<T: ConstU8>: ConstBool;
    type LessThan<T: ConstU8>: ConstBool;
}

impl<
F0: ConstBool,
F1: ConstBool,
F2: ConstBool,
F3: ConstBool,
F4: ConstBool,
F5: ConstBool,
F6: ConstBool,
F7: ConstBool> Sealed for StaticU8<F0,F1,F2,F3,F4,F5,F6,F7>{
    
}

impl<
F0: ConstBool,
F1: ConstBool,
F2: ConstBool,
F3: ConstBool,
F4: ConstBool,
F5: ConstBool,
F6: ConstBool,
F7: ConstBool> ConstU8 for StaticU8<F0,F1,F2,F3,F4,F5,F6,F7>{
    const VALUE: u8 = 
    (F0::VALUE as u8) << 0 |
    (F1::VALUE as u8) << 1 |
    (F2::VALUE as u8) << 2 |
    (F3::VALUE as u8) << 3 |
    (F4::VALUE as u8) << 4 |
    (F5::VALUE as u8) << 5 |
    (F6::VALUE as u8) << 6 |
    (F7::VALUE as u8) << 7;

    type F0 = F0;
    type F1 = F1;
    type F2 = F2;
    type F3 = F3;
    type F4 = F4;
    type F5 = F5;
    type F6 = F6;
    type F7 = F7;

    type BitOr<T: ConstU8> = StaticU8<
        Or<Self::F0, T::F0>,
        Or<Self::F1, T::F1>,
        Or<Self::F2, T::F2>,
        Or<Self::F3, T::F3>,
        Or<Self::F4, T::F4>,
        Or<Self::F5, T::F5>,
        Or<Self::F6, T::F6>,
        Or<Self::F7, T::F7>,
    >;
    type BiggerThan<T: ConstU8> = Or<Bigger<Self::F7, T::F7>, And<Eq<Self::F7, T::F7>, Bigger<Self::F6, T::F6>>>>
}
