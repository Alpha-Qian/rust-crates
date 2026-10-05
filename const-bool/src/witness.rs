use core::{marker::PhantomData, mem::ManuallyDrop};

use crate::{And, ConstBool, ConstFalse, ConstTrue, Not, Or, Select, Xor};


pub enum BoolWit<F>{
    True(Is<F, ConstTrue>),
    False(Is<F, ConstFalse>),
}

impl<F> Clone for BoolWit<F> { fn clone(&self) -> Self { *self } }
impl<F> Copy for BoolWit<F> {}



/// A marker type proof Type A is Eq with Type B
pub struct Is<A, B>(PhantomData<(fn(A) -> A, fn(B) -> B)>);

impl<A> Is<A, A> {
    pub const fn refl() -> Self { Is(PhantomData) }
}

impl<A, B> Clone for Is<A, B> { fn clone(&self) -> Self { *self } }
impl<A, B> Copy for Is<A, B> {}

impl<A, B> Is<A, B> {
    pub const fn flip(self) -> Is<B, A> { Is(PhantomData) }

    
    pub const fn then<C>(self, _: Is<B, C>) -> Is<A, C> { Is(PhantomData) }

    
    const fn on_left<Z>(self, _: Is<Z, A>) -> Is<Z, B> { Is(PhantomData) }
    
    pub const fn cast(self, a: A) -> B {
        union U<A, B> { a: ManuallyDrop<A>, b: ManuallyDrop<B> }
        unsafe { ManuallyDrop::into_inner(U { a: ManuallyDrop::new(a) }.b) }
    }
    
    pub const fn cast_ref(self, a: &A) -> &B { unsafe { &*(a as *const A as *const B) } }
    pub const fn cast_mut(self, a: &mut A) -> &mut B { unsafe { &mut *(a as *mut A as *mut B) } }

}

/// 类型级函数，stable 上没有 HKT，只能每个函数一个结构体
pub trait BoolFn { type Apply<F: ConstBool>; }

pub trait BoolFn2 { type Apply<A: ConstBool, B: ConstBool>; }


impl<A: ConstBool, B: ConstBool> Is<A, B> {
    /// A == B => G<A> == G<B>
    pub const fn project<G: BoolFn>(self) -> Is<G::Apply<A>, G::Apply<B>> { Is(PhantomData) }

    ///A == B -> C == D -> G<A, C> == G<B, D>
    pub const fn project2<G: BoolFn2, C: ConstBool, D: ConstBool>(
        self, _: Is<C, D>,
    ) -> Is<G::Apply<A, C>, G::Apply<B, D>> { Is(PhantomData) }

}

// impl<F1: BoolFn, F2: BoolFn> Is<F1, F2> {
//     pub const fn apply<A: ConstBool, B: ConstBool>(self, _: Is<A, B>)
//         -> Is<F1::Apply<A>, F2::Apply<B>>
//     { Is(PhantomData) }
// }

impl<A, B> Is<A, B> {
    pub const unsafe fn proof_unchecked() -> Self {
        Is(PhantomData)
    }
}

struct SelFn<T, F>(PhantomData<(T, F)>);

impl<T, F> BoolFn for SelFn<T, F> {
    type Apply<A: ConstBool> = A::Select<T, F>;
}

struct NotFn;

impl BoolFn for NotFn {
    type Apply<F: ConstBool> = F::Not;
}

struct OrFn<R: ConstBool>(PhantomData<R>);

impl<R: ConstBool> BoolFn for OrFn<R> {
    type Apply<F: ConstBool> = Or<F, R>;
}

struct AndFn<R: ConstBool>(PhantomData<R>);

impl<R: ConstBool> BoolFn for AndFn<R> {
    type Apply<F: ConstBool> = And<F, R>;
}

struct XorFn<R: ConstBool>(PhantomData<R>);

impl<R: ConstBool> BoolFn for XorFn<R> {
    type Apply<F: ConstBool> = Xor<F, R>;
}


// ===========================
// 基本定理证明


///证明双重否定
const fn not_not_eq<F: ConstBool>() -> Is<Not<Not<F>>, F> {
    
    ///证明 A == Not<Not<A>>
    struct NotNot;
    impl BoolFn for NotNot { type Apply<F: ConstBool> = Not<Not<F>>; }

    match F::WIT {
        BoolWit::True(eq) => {            // eq : F = True
            // Not<Not<F>> = Not<Not<True>>，后者被编译器规范化为 True
            eq.project::<NotNot>()        // Is<Not<Not<F>>, True>
              .then(eq.flip())            // Is<True, F>  ⟹  Is<Not<Not<F>>, F>
        }
        BoolWit::False(eq) => eq.project::<NotNot>().then(eq.flip()),
    }
}



///证明排中律
const fn excluded_middle_eq<A: ConstBool>() -> Is<Or<A, Not<A>>, ConstTrue> {

    /// 类型函数：A ↦ Or<A, Not<A>> 证明排中律
    struct ExcludedMiddle;
    impl BoolFn for ExcludedMiddle {
        type Apply<A: ConstBool> = Or<A, Not<A>>;
    }
    
    match A::WIT {
        // eq : A = True
        // project 后：Or<A, Not<A>> = Or<True, Not<True>>
        // 编译器化简：Or<True, False> = True
        BoolWit::True(eq) => eq.project::<ExcludedMiddle>(),

        // eq : A = False
        // project 后：Or<A, Not<A>> = Or<False, Not<False>>
        // 编译器化简：Or<False, True> = True
        BoolWit::False(eq) => eq.project::<ExcludedMiddle>(),
    }
}

// F ↦ Select<F, T, U>
struct SelL<T, U>(PhantomData<(T, U)>);
impl<T, U> BoolFn for SelL<T, U> {
    type Apply<F: ConstBool> = Select<F, T, U>;
}

// F ↦ Select<Not<F>, U, T>
struct SelR<T, U>(PhantomData<(T, U)>);
impl<T, U> BoolFn for SelR<T, U> {
    type Apply<F: ConstBool> = Select<Not<F>, U, T>;
}

///证明Select翻转
const fn select_flip_eq<A: ConstBool, T, U>()
    -> Is<Select<A, T, U>, Select<Not<A>, U, T>>
{
    match A::WIT {
        // eq : A = True
        BoolWit::True(eq) => {
            let l = eq.project::<SelL<T, U>>();   // Is<Select<A,T,U>,     Select<True,T,U>>      = Is<LHS, T>
            let r = eq.project::<SelR<T, U>>();   // Is<Select<Not<A>,U,T>, Select<Not<True>,U,T>> = Is<RHS, T>
            l.then(r.flip())                      // Is<LHS, RHS>
        }
        // eq : A = False
        BoolWit::False(eq) => {
            let l = eq.project::<SelL<T, U>>();   // Is<LHS, U>
            let r = eq.project::<SelR<T, U>>();   // Select<Not<False>,U,T> = Select<True,U,T> = U，Is<RHS, U>
            l.then(r.flip())
        }
    }
}

//证明两个德摩根

struct NotAndFn;   // (A,B) ↦ Not<And<A,B>>
impl BoolFn2 for NotAndFn { type Apply<A: ConstBool, B: ConstBool> = Not<And<A, B>>; }
struct OrNotFn;    // (A,B) ↦ Or<Not<A>,Not<B>>
impl BoolFn2 for OrNotFn  { type Apply<A: ConstBool, B: ConstBool> = Or<Not<A>, Not<B>>; }

struct NotOrFn;    // (A,B) ↦ Not<Or<A,B>>
impl BoolFn2 for NotOrFn  { type Apply<A: ConstBool, B: ConstBool> = Not<Or<A, B>>; }
struct AndNotFn;   // (A,B) ↦ And<Not<A>,Not<B>>
impl BoolFn2 for AndNotFn { type Apply<A: ConstBool, B: ConstBool> = And<Not<A>, Not<B>>; }


const fn de_morgan_and_eq<A: ConstBool, B: ConstBool>()
    -> Is<Not<And<A, B>>, Or<Not<A>, Not<B>>>
{
    let b = Is::<B, B>::refl();           // B 不分情形，直接用自反性
    match A::WIT {
        BoolWit::True(eq)  => eq.project2::<NotAndFn, B, B>(b)
                                .then(eq.project2::<OrNotFn, B, B>(b).flip()),
        BoolWit::False(eq) => eq.project2::<NotAndFn, B, B>(b)
                                .then(eq.project2::<OrNotFn, B, B>(b).flip()),
    }
}

pub const fn de_morgan_or_eq<A: ConstBool, B: ConstBool>()
    -> Is<Not<Or<A, B>>, And<Not<A>, Not<B>>>
{
    let b = Is::<B, B>::refl();
    match A::WIT {
        BoolWit::True(eq)  => eq.project2::<NotOrFn, B, B>(b)
                                .then(eq.project2::<AndNotFn, B, B>(b).flip()),
        BoolWit::False(eq) => eq.project2::<NotOrFn, B, B>(b)
                                .then(eq.project2::<AndNotFn, B, B>(b).flip()),
    }
}

enum Void {}                       // ⊥：没有任何值

struct Discr;                          // F ↦ F::Select<(), Void>
impl BoolFn for Discr { type Apply<F: ConstBool> = Select<F, (), Void>; }

/// 命题：True ≠ False，即 Is<True, False> → ⊥
const fn true_ne_false(h: Is<ConstTrue, ConstFalse>) -> Void {
    // project 后：Is<Select<True,(),Void>, Select<False,(),Void>> = Is<(), Void>
    h.project::<Discr>().cast(())      // 用「() = Void」把 () 变成 Void
}


const fn A_ne_NotA<A: ConstBool>(h: Is<A, Not<A>>) -> Is<Is<A, Not<A>>, Void> {
    match A::WIT {
        BoolWit::True(eq) => {
            let a = eq.project::<Discr>();
            todo!()
        }
        BoolWit::False(eq) => todo!(),
    }
}