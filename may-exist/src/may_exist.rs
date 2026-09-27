//! 结合 `static_option`（类型级零开销选择）与 `MayExist`（统一访问语义 + 编译期误用检测）
//! 两者优点的实现。
//!
//! 设计要点：
//! 1. `Selected<T>` 在 "Some" 分支下是 `#[repr(transparent)] struct Exist<T>(T)`，
//!    与裸 `T` 布局完全相同，运行时零开销；在 "None" 分支下是零大小的 `NoExist<T>`。
//!    —— 这是 `static_option` 的思路：编译期就决定字段的物理表示。
//! 2. 两个分支都实现同一个 `MayExist` trait，因此调用方可以对
//!    `StaticOption<S, T>`（`S` 是泛型参数）写完全泛型的代码，
//!    不必对 `S` 做 match/if 就能拿到统一的 `.exist_or(..)`/`.unwrap()` 等 API。
//! 3. `MayExist::unwrap()` / `unwrap_none()` 在"当前分支不该被调用"的那一侧
//!    用内联 `const { panic!(..) }` 实现：一旦该方法被单态化，就是**编译期错误**，
//!    而不是等到运行时才 panic —— 这是原 `MayExist`/`NoExit` 代码的核心思路，被保留下来。
//!
//! 需要 Rust >= 1.79（GAT 于 1.65 稳定，内联 `const` 表达式于 1.79 稳定）。

//use std::option::Option;




use std::mem::forget;
use std::num::NonZero;
use std::{fmt, mem};
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::ptr::NonNull;


use crate::static_bool::{StaticBool, ConstTrue, ConstFalse};
use crate::utils::transmute_unchecked;
use crate::sealed::Sealed;

// ======================================
// const bool
// ======================================

// =====================================================================
// 1. 行为层：统一的访问语义（对应原来的 `MayExist`）
// =====================================================================


/// Trait方案，人体工学较Repr方案更差但但trait允许自己实现，内部结构不是黑盒
///Safety:
/// 如果const EXIST为true
/// Self的内存布局必须与Self::Me相同
/// 换言之，const EXIST为true的所有实现必须能相互transport
/// Self不可以假设Self::Me内存在任何不变量
/// 如果const EXIST为false
/// Self的内存布局必须与()相同
/// 
/// 任意两个Me, ExistFlag相同的impl Core都是可以转换的
pub unsafe trait Core {

    /// Static Option Flag
    /// associated_const_equality still unstable
    type ExistFlag: StaticBool;

    /// 用来标记自己
    /// ConStructer只要ExistFlag和Me相同，本质就是相同的，所以感觉根本不需要
    type ConStucter<T>: Core<ExistFlag = Self::ExistFlag, Me = T>;
    
    type Me;
}


// struct StaticResult<T, E, F: StaticBool>(pub F::Select<T, E>);

// impl<M, F: StaticBool> StaticResult<>




static ASSERT_IS_SOME_FAILED: &str = "assert is some failed";
static ASSERT_IS_NONE_FAILED: &str = "assert is none failed";

trait CoreExt: Core + Sized{

    //========================
    //  static type to run time type 
    // =======================

    fn is_some() -> bool{
        Self::ExistFlag::VALUE
    }

    fn is_none() -> bool{
        !Self::ExistFlag::VALUE
    }
    
    ///处理原始的地址转换
    #[inline]
    fn addr_cast(addr: usize) -> Option<NonZero<usize>>{
        if Self::ExistFlag::VALUE {
            NonZero::new(addr)
        } else { None }
    }
    
    fn raw_cast(this: *mut Self) -> Option<NonNull<Self::Me>> {
        unsafe{ mem::transmute(Self::addr_cast(this as usize)) }
    }

    fn as_ref(&self) -> Option<&Self::Me>{
        unsafe{ mem::transmute(Self::addr_cast(self as *const _ as usize)) }
    }

    fn as_mut(&mut self) -> Option<&mut Self::Me>{
        unsafe{ mem::transmute(Self::addr_cast(self as *mut _ as usize))}
    }

    fn into_option(self) -> Option<Self::Me>{
        if Self::ExistFlag::VALUE {
            unsafe{ Some(transmute_unchecked(self)) }
        } else { None }
    }


    // =================
    // const assert 编译期unwarp
    // =================
    fn unwarp(self) -> Self::Me{
        const { assert!(Self::ExistFlag::VALUE) };
        unsafe{ transmute_unchecked(self) }
    }

    fn unwarp_or(self, value: Self::Me) -> Self::Me{
        if Self::ExistFlag::VALUE {
            unsafe{ transmute_unchecked(self) }
        } else { value }
    }

    fn unwarp_or_else(self, f: impl FnOnce() -> Self::Me) -> Self::Me{
        if Self::ExistFlag::VALUE {
            unsafe{ transmute_unchecked(self) }
        } else { f() }
    }

    fn unwarp_or_default(self) -> Self::Me
    where Self::Me: Default
    {
        if Self::ExistFlag::VALUE {
            unsafe{ transmute_unchecked(self) }
        } else { Self::Me::default() }
    }

    fn unwarp_none(self){
        const{assert!(!Self::ExistFlag::VALUE)}
    }

    fn assert_is_some() {
        const{ assert!(Self::ExistFlag::VALUE) }
    }

    fn assert_is_none() {
        const{ assert!(!Self::ExistFlag::VALUE) }
    }

    fn unwarp_const(this: *const Self) -> *const Self::Me {
        const{ assert!(Self::ExistFlag::VALUE) };
        unsafe{ mem::transmute(this) }
    }

    fn unwarp_nonull(this: *mut Self) -> NonNull<Self::Me> {
        const{ assert!(Self::ExistFlag::VALUE) };
        unsafe{ mem::transmute(this) }
    }

    fn unwarp_ref(&self) -> &Self::Me{
        const{ assert!(Self::ExistFlag::VALUE) };
        unsafe{ mem::transmute(self)}
    }

    fn unwarp_mut(&mut self) -> &mut Self::Me{
        const{ assert!(Self::ExistFlag::VALUE) };
        unsafe{ mem::transmute(self) }
    }

    // =======================
    // simple transmute

    fn transmute<T: Core<ExistFlag = Self::ExistFlag, Me = Self::Me>>(self) -> T{
        unsafe{ transmute_unchecked(self) }
    }

    // ///从类型擦除中恢复
    // fn named

    // ///类型擦除
    // fn unnamed

    
    //方案1：要什么类型自己填
    fn map<T, R: CoreExt<Me = T>>(self, f: impl FnOnce(Self::Me) -> T) -> R {
        if Self::ExistFlag::VALUE {
            let t: Self::Me = unsafe{ transmute_unchecked(self) };
            let r = f(t);
            unsafe{ transmute_unchecked(r) }
        } else { 
            unsafe{ transmute_unchecked(self) }
        }
    }

    //方案2: 构造器
    fn map2<T>(self, f: impl FnOnce(Self::Me) -> T) -> Self::ConStucter<T> {
        if Self::ExistFlag::VALUE {
            let t: Self::Me = unsafe{ transmute_unchecked(Self::Me) };
            let result = f(t);
            unsafe{ transmute_unchecked(result) }
        } else {
            unsafe{ transmute_unchecked(self) }
        }
    }

    //方案3：类型擦除，因为Safety条款中规定了可以任意转换所以没事
    fn map3<T>(self, f: impl FnOnce(Self::Me) -> T) -> impl Core<Me = T, ExistFlag = Self::ExistFlag> {
        todo!()
    }

    // 从impl Core<ExistFlag = true>中恢复为具名类型Exist
    // fn renamed_some(self) -> Exist<Self::Me>{
    //     const{ assert!(todo)}
    // }

    // 从impl Core<ExistFlag = true>中恢复为具名类型NoExist
    // fn renamed_some(self) -> NoExist<Self::Me>{
    //     const{ assert!(todo)}
    // }
    
    //
    //fn map_to_addr(self) -> impl Core<Me = NonZero<usize>>;

    // fn map_to_nonnull(self) -> impl Core<Me = NonNull, ExistFlag = Self::ExistFlag> {
        
    // }

    //fn map_to_const(self) -> impl Core
    
    //fn map_to_raw

    //fn map_to_ref

    //fn map_to_mut

    
    ///如果self为Some，用other更新字段
    fn update<T: Core<Me = Self::Me>>(self, other: T) -> Self {
        if !Self::ExistFlag::VALUE {
            return self
        };
        
        if T::ExistFlag::VALUE {
            return unsafe{ transmute_unchecked(other) }
        };

        return self
    }

    //猎奇编译期计算
    fn or<T: Core<Me = Self::Me>(self, other: T) -> impl Core<Me = Self::Me, ExistFlag = <Self::ExistFlag as StaticBool>::Or<T::ExistFlag>> {
        if Self::ExistFlag::VALUE {
            return unsafe{ transmute_unchecked(self) }
        }

        if T::ExistFlag::VALUE {
            return unsafe{ transmute_unchecked(other) }
        }

        return unsafe{ transmute_unchecked(()) }
    }

    ///方案1: 类型自己填
    fn and_then<T: Core<Me = Self::Me>, R: Core<Me = Self::Me>>(self, f: impl FnOnce(Self::Me) -> T) -> R {
        
        // const type safety check:
        const{
            let excepted_resule_flag = Self::ExistFlag::VALUE && T::ExistFlag::VALUE;
            assert_eq!(excepted_resule_flag, R::ExistFlag::VALUE)
        }
        
        if Self::ExistFlag::VALUE {
            unsafe{
                let t = f(transmute_unchecked(self));
                return transmute_unchecked(t)
            }
        } else {
            unsafe{
                return transmute_unchecked(())
            }
        }
    }

    ///方案二：猎奇编译期间计算
    fn and_then2<T: Core>(self, f: impl FnOnce(Self::Me) -> T) -> impl Core<Me = T::Me, ExistFlag = <Self::ExistFlag as StaticBool>::And<T::ExistFlag>> {
        todo!()
    }

    // ==================
    // unchecked //感觉没必要，因为完全可以在编译时检查
    // 
    // unsafe fn unwarp_unchecked
}

impl<T: Core> CoreExt for T {}

/// 编译期可空类型的统一访问接口。
/// `Me` 是"如果存在，值的类型"，类似 `Option<Me>` 里的 `Me`。
pub trait MayExist {
    type Me;

    /// 存在则返回内部值，否则返回 `value`。
    fn exist_or(self, value: Self::Me) -> Self::Me;

    /// 存在则返回内部值，否则调用 `f()`。
    fn exist_or_else(self, f: impl FnOnce() -> Self::Me) -> Self::Me;

    /// 存在则返回内部值，否则返回 `Self::Me::default()`。
    fn exist_or_default(self) -> Self::Me
    where
        Self::Me: Default;

    /// 存在则返回内部值；在"静态上一定不存在"的分支上，
    /// 这个方法一旦被单态化调用，就是**编译期错误**。
    fn unwrap(self) -> Self::Me;

    /// 断言值不存在；在"静态上一定存在"的分支上，
    /// 这个方法一旦被单态化调用，就是**编译期错误**。
    fn unwrap_none(self)
    where
        Self: Sized,
    {
        const { panic!("unwrap_none 失败：值本应不存在，但当前分支持有值") }
    }
}

// =====================================================================
// 2. 存储层：两个具体分支（对应原来的 `Exist` / `NoExit`）
// =====================================================================

/// "存在" 分支。`#[repr(transparent)]` 保证它与裸 `T` 布局完全一致，
/// 因此和 `static_option` 里直接令 `Selected<T> = T` 一样是零运行时开销，
/// 只是多包一层，好挂 `MayExist` 的实现。
#[repr(transparent)]
pub struct Exist<T>(pub T);

impl<T> MayExist for Exist<T> {
    type Me = T;

    fn exist_or(self, _value: T) -> T {
        self.0
    }

    fn exist_or_else(self, _f: impl FnOnce() -> T) -> T {
        self.0
    }

    fn exist_or_default(self) -> T
    where
        T: Default,
    {
        self.0
    }

    fn unwrap(self) -> T {
        self.0
    }

    // `unwrap_none` 不覆写：继承 trait 默认实现，
    // 一旦被调用（单态化）就是编译期错误 —— 因为这个分支下值确实存在。
}

impl<T: Default> Default for Exist<T> {
    fn default() -> Self {
        Self(T::default())
    }
}

// impl<T> Deref for Exist<T> {
//     type Target = T;
//     fn deref(&self) -> &T {
//         &self.0
//     }
// }

// impl<T> DerefMut for Exist<T> {
//     fn deref_mut(&mut self) -> &mut T {
//         &mut self.0
//     }
// }

impl<T: Clone> Clone for Exist<T> {
    fn clone(&self) -> Self {
        Exist(self.0.clone())
    }
}

impl<T: fmt::Debug> fmt::Debug for Exist<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Exist").field(&self.0).finish()
    }
}

/// "不存在" 分支。零大小类型（ZST），只携带 `T` 的类型信息，不占运行时空间，
/// 对应 `static_option` 里 `Selected<T> = StaticOptionNone`。
pub struct NoExist<T>(PhantomData<T>);

impl<T: Default> Default for NoExist<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<T: Clone> Clone for NoExist<T> {
    fn clone(&self) -> Self {
        Self(PhantomData)
    }
}
impl<T: Copy> Copy for NoExist<T> {}

impl<T: fmt::Debug> fmt::Debug for NoExist<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("NoExist<?>")
    }
}

impl<T> MayExist for NoExist<T> {
    type Me = T;

    fn exist_or(self, value: T) -> T {
        value
    }

    fn exist_or_else(self, f: impl FnOnce() -> T) -> T {
        f()
    }

    fn exist_or_default(self) -> T
    where
        T: Default,
    {
        T::default()
    }

    fn unwrap(self) -> T {
        const { panic!("unwrap 失败：值本应存在，但当前分支不持有值") }
    }

    fn unwrap_none(self) {
        // 正确用法：值确实不存在，什么也不做。
    }
}

/// 构造 "存在" 分支的辅助函数。
pub fn some<T>(value: T) -> Exist<T> {
    Exist(value)
}

/// 构造 "不存在" 分支的辅助函数。
pub fn none<T>() -> NoExist<T> {
    NoExist(PhantomData)
}



// // ======================
// // test Selector
// // =======================

// struct 







// =====================================================================
// 3. 类型选择层：`Selector` 静态决定用哪个分支（对应原来的 `static_option`）
// =====================================================================

mod sealed {
    pub trait Sealed {}
}

/// 静态选择器：`StaticOptionSome` 或 `StaticOptionNone`。
/// 密封（sealed），外部无法新增第三种分支。
///
/// 关键约束：`Selected<T>: MayExist<Me = T>`。
/// 这一行把 `static_option` 的零开销存储和 `MayExist` 的统一访问语义粘合在一起：
/// 无论选中哪个分支，都保证能用同一套 `MayExist` API 访问它。
pub trait StaticOptionSelector: sealed::Sealed {
    type Selected<T>: MayExist<Me = T>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StaticOptionSome;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StaticOptionNone;

impl sealed::Sealed for StaticOptionSome {}
impl sealed::Sealed for StaticOptionNone {}

impl StaticOptionSelector for StaticOptionSome {
    type Selected<T> = Exist<T>;
}

impl StaticOptionSelector for StaticOptionNone {
    type Selected<T> = NoExist<T>;
}







/// 编译期 Option：`Selector` 是 `StaticOptionSome` / `StaticOptionNone`，
/// `T` 是内部值类型。
pub type StaticOption<Selector, T> = <Selector as StaticOptionSelector>::Selected<T>;



// =====================================================================
// 4. 用法示例：typestate 连接
// =====================================================================

/// 一个典型的 typestate：拿到 token 前后是不同的类型状态，
/// 但字段访问用统一的 `MayExist` API，不需要对状态做 match。
pub struct Connection<S: StaticOptionSelector> {
    pub token: StaticOption<S, String>,
}

impl Connection<StaticOptionNone> {
    pub fn new() -> Self {
        Self { token: none() }
    }

    pub fn with_token(self, token: String) -> Connection<StaticOptionSome> {
        Connection { token: some(token) }
    }
}

/// 完全泛型：不管 `S` 是 Some 还是 None 都能编译通过，
/// 因为 trait 定义已经保证 `StaticOption<S, String>: MayExist<Me = String>`。
pub fn token_or_anonymous<S: StaticOptionSelector>(conn: Connection<S>) -> String {
    conn.token.exist_or_else(|| "anonymous".to_string())
}




// 下面两个函数如果取消注释，会在编译期（单态化时）直接报错，
// 而不是等到运行时才 panic：
//
// fn bug(conn: Connection<StaticOptionNone>) -> String {
//     conn.token.unwrap() // error: unwrap 失败：值本应存在，但当前分支不持有值
// }
//
// fn bug2(conn: Connection<StaticOptionSome>) {
//     conn.token.unwrap_none() // error: unwrap_none 失败：值本应不存在，但当前分支持有值
// }



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_cost_layout() {
        // Exist<T> 与裸 T 大小完全一致（#[repr(transparent)] 的效果）
        assert_eq!(
            std::mem::size_of::<Exist<u64>>(),
            std::mem::size_of::<u64>()
        );
        // NoExist<T> 不占运行时空间
        assert_eq!(std::mem::size_of::<NoExist<u64>>(), 0);
    }

    #[test]
    fn generic_access() {
        let a = Connection::<StaticOptionNone>::new();
        assert_eq!(token_or_anonymous(a), "anonymous");

        let b = Connection::<StaticOptionNone>::new().with_token("secret".into());
        assert_eq!(token_or_anonymous(b), "secret");
    }

    #[test]
    fn exist_or_default() {
        let x: NoExist<i32> = none();
        assert_eq!(x.exist_or_default(), 0);

        let y: Exist<i32> = some(42);
        assert_eq!(y.exist_or_default(), 42);
    }
}