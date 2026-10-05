use const_bool::{And, BoolFn, BoolFn2, BoolWit, ConstBool, ConstFalse, ConstTrue, Eq, Is, Or, Select, Xor};
use core::{fmt::Debug, marker::PhantomData, mem::transmute};
use core::cmp::Eq as StdEq;
use crate::StaticOption;


// F ↦ F::Select<T, ()>，也就是 StaticOption<T, F> 的存储类型
struct SelFn<T>(PhantomData<T>);
impl<T> BoolFn for SelFn<T> { type Apply<F: ConstBool> = Select<F, T, ()>; }

// F ↦ Or<F, R>::Select<T, ()>
struct OrSelFn<T, R>(PhantomData<(T, R)>);
impl<T, R: ConstBool> BoolFn for OrSelFn<T, R> {
    type Apply<F: ConstBool> = Select<Or<F, R>, T, ()>;
}

// F ↦ And<F, R>::Select<U, ()>
struct AndSelFn<U, R>(PhantomData<(U, R)>);
impl<U, R: ConstBool> BoolFn for AndSelFn<U, R> {
    type Apply<F: ConstBool> = Select<And<F, R>, U, ()>;
}

// (A, B) ↦ Xor<A, B>::Select<T, ()>
struct XorSelFn<T>(PhantomData<T>);
impl<T> BoolFn2 for XorSelFn<T> {
    type Apply<A: ConstBool, B: ConstBool> = Select<Xor<A, B>, T, ()>;
}

impl<T, F: ConstBool> StaticOption<T, F> {
    
    const REPR: Is<Self, F::Select<T, ()>> = unsafe{ Is::proof_unchecked() };
    
    const fn from_some(eq: Is<F, ConstTrue>, value: T) -> Self {
        Self(
            eq
                .project::<SelFn<T>>()
                .flip()
                .cast(value)
        )
    }

    const fn from_none(eq: Is<F, ConstFalse>) -> Self {
        Self(
            eq
                .project::<SelFn<T>>()
                .flip()
                .cast(())
        )
    }

    const fn into_some(self, eq: Is<F, ConstTrue>) -> T {
        //let Self(inner) = self;
        let eq = eq
            .project::<SelFn<T>>();

        Self::REPR
            .then(eq)
            .cast(self)
    }

    const fn into_none(self, eq: Is<F, ConstFalse>) {
        let inner = Self::REPR.cast(self);

        eq.project::<SelFn<T>>()
            .cast(inner)
    }

    const fn ref_some(&self, eq: Is<F, ConstTrue> ) -> &T {
        let inner = Self::REPR.cast_ref(self);
        eq.project::<SelFn<T>>()
            .cast_ref(inner)
    }

    const fn mut_some(&mut self, eq: Is<F, ConstTrue> ) -> &mut T {
        let inner = Self::REPR.cast_mut(self);
        eq.project::<SelFn<T>>()
            .cast_mut(inner)
    }
}

impl<T, F: ConstBool> StaticOption<T, F> {
    pub const fn as_ref(&self) -> StaticOption<&T, F> {
        match F::WIT {
            BoolWit::True(eq)  => StaticOption::from_some(eq, self.ref_some(eq)),
            BoolWit::False(eq) => StaticOption::from_none(eq),
        }
    }

    pub const fn as_mut(&mut self) -> StaticOption<&mut T, F> {
        match F::WIT {
            BoolWit::True(eq) => StaticOption::from_some(eq, self.mut_some(eq)),
            BoolWit::False(eq) => StaticOption::from_none(eq)
        }
    }

    pub const fn into_option(self) -> Option<T> {
        match F::WIT {
            BoolWit::True(eq) => Some(self.into_some(eq)),
            BoolWit::False(eq) => { self.into_none(eq); None }
        }
    }
}

impl<T, F: ConstBool> StaticOption<T, F> {
    pub fn new_some_if(value: T) -> Self{
        match F::WIT {
            BoolWit::True(eq) => StaticOption::from_some(eq, value),
            BoolWit::False(eq) => StaticOption::from_none(eq)
        }
    }

    pub fn map<U>(self, f: impl FnOnce(T) -> U) -> StaticOption<U, F> {
        match F::WIT {
            BoolWit::True(eq) => StaticOption::from_some(eq, f(self.into_some(eq))),
            BoolWit::False(eq) => StaticOption::from_none(eq)
        }
    }

    pub fn unwrap_or(self, value: T) -> T {
        match F::WIT {
            BoolWit::True(eq) => self.into_some(eq),
            BoolWit::False(eq) => { self.into_none(eq); value }
        }
    }

    pub fn unwrap_or_else(self, f: impl FnOnce() -> T) -> T {
        match F::WIT {
            BoolWit::True(eq) => self.into_some(eq),
            BoolWit::False(eq) => { self.into_none(eq); f() }
        }
    }

    pub fn is_some_and(self, f: impl FnOnce(T) -> bool) -> bool {
        match F::WIT {
            BoolWit::True(eq) => f(self.into_some(eq)),
            BoolWit::False(eq) => {self.into_none(eq); false}
        }
    }

    pub fn is_none_or(self, f: impl FnOnce(T) -> bool) -> bool {
        match F::WIT {
            BoolWit::True(eq) => f(self.into_some(eq)),
            BoolWit::False(eq) => {self.into_none(eq); true}
        }
    }

    
}

impl<T, F: ConstBool> StaticOption<T, F> {
    pub fn or<R: ConstBool>(self, other: StaticOption<T, R>) -> StaticOption<T, Or<F, R>> {
        match F::WIT {
            // Or<True,R> = True，存储类型是 T
            BoolWit::True(eq) => {
                let v = self.into_some(eq);
                StaticOption(eq.project::<OrSelFn<T, R>>().flip().cast(v))
            }
            // Or<False,R> = R，存储类型是 R::Select<T,()>，正好是 other.0
            BoolWit::False(eq) => {
                self.into_none(eq);
                StaticOption(eq.project::<OrSelFn<T, R>>().flip().cast(other.0))
            }
        }
    }

    /// 修正后的签名：与 std 一致，结果类型是 other 的值类型，标志是 And<F, R>
    pub fn and<U, R: ConstBool>(self, other: StaticOption<U, R>) -> StaticOption<U, And<F, R>> {
        match F::WIT {
            // And<True,R> = R，存储类型是 R::Select<U,()>，正好是 other.0
            BoolWit::True(eq) => {
                self.into_some(eq);                                  // 丢弃，与 std 一致
                StaticOption(eq.project::<AndSelFn<U, R>>().flip().cast(other.0))
            }
            // And<False,R> = False，存储类型是 ()
            BoolWit::False(eq) => {
                self.into_none(eq);
                StaticOption(eq.project::<AndSelFn<U, R>>().flip().cast(()))
            }
        }
    }

    pub fn and_then<U, RF: ConstBool>(
        self,
        f: impl FnOnce(T) -> StaticOption<U, RF>,
    ) -> StaticOption<U, And<F, RF>> {
        match F::WIT {
            BoolWit::True(eq) => {
                let res = f(self.into_some(eq));
                StaticOption(eq.project::<AndSelFn<U, RF>>().flip().cast(res.0))
            }
            BoolWit::False(eq) => {
                self.into_none(eq);
                StaticOption(eq.project::<AndSelFn<U, RF>>().flip().cast(()))
            }
        }
    }

    pub fn or_else<RF: ConstBool>(self, f: impl FnOnce() -> StaticOption<T, RF>) -> StaticOption<T, Or<F, RF>> {
        match F::WIT {
            BoolWit::True(eq) =>{ 
                let inner = self.into_some(eq);
                StaticOption(eq.project::<OrSelFn<T, RF>>().flip().cast(inner))
            },
            BoolWit::False(eq) => {
                self.into_none(eq);
                let res = f();
                let eq = eq.project::<OrSelFn<T, RF>>().flip();
                StaticOption(eq.cast(res.0))
            }
        }
    }

    /// Xor = Not<Eq<F,R>>，R 未知时化简不了，所以要同时匹配两个见证
    pub fn xor<R: ConstBool>(self, other: StaticOption<T, R>) -> StaticOption<T, Xor<F, R>> {
        match (F::WIT, R::WIT) {
            
            // Xor<True,False> = True
            (BoolWit::True(f), BoolWit::False(r)) => {
                let eq = f.project2::<XorSelFn<T>, _, _>(r);
                let v = self.into_some(f);
                other.into_none(r);
                StaticOption(eq.flip().cast(v))
            }
            // Xor<False,True> = True
            (BoolWit::False(f), BoolWit::True(r)) => {
                let eq = f.project2::<XorSelFn<T>, _, _>(r);
                self.into_none(f);
                let v = other.into_some(r);
                StaticOption(eq.flip().cast(v))
            }
            // Xor<True,True> = Xor<False,False> = False
            (BoolWit::True(f), BoolWit::True(r)) => {
                let eq = f.project2::<XorSelFn<T>, _, _>(r);
                self.into_some(f); other.into_some(r);               // 两个都丢弃
                StaticOption(eq.flip().cast(()))
            }
            (BoolWit::False(f), BoolWit::False(r)) => {
                let eq = f.project2::<XorSelFn<T>, _, _>(r);
                self.into_none(f); other.into_none(r);
                StaticOption(eq.flip().cast(()))
            }
        }
    }
}


impl<T: Clone, F: ConstBool> Clone for StaticOption<T, F> {
    fn clone(&self) -> Self {
        match F::WIT {
            BoolWit::True(eq)  => Self::from_some(eq, self.ref_some(eq).clone()),
            BoolWit::False(eq) => Self::from_none(eq),
        }
    }
}

impl<T: Debug, F: ConstBool> Debug for StaticOption<T, F> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match F::WIT {
            BoolWit::True(eq)  => f.debug_tuple("Some").field(self.ref_some(eq)).finish(),
            BoolWit::False(_)  => f.write_str("None"),
        }
    }
}

impl<T: PartialEq, F: ConstBool> PartialEq for StaticOption<T, F> {
    fn eq(&self, other: &Self) -> bool {
        match F::WIT {
            BoolWit::True(eq)  => self.ref_some(eq) == other.ref_some(eq),
            BoolWit::False(_)  => true,
        }
    }
}
impl<T: StdEq, F: ConstBool> StdEq for StaticOption<T, F> {}