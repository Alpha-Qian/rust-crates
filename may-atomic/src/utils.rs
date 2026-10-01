use std::mem::{size_of, MaybeUninit};


///Safty: No Padding
pub(crate) unsafe fn bits_eq<T>(a: &T, b: &T) -> bool {
    let size = std::mem::size_of::<T>();
    unsafe {
        let pa = std::slice::from_raw_parts(a as *const _ as *const u8, size);
        let pb = std::slice::from_raw_parts(b as *const _ as *const u8, size);
        pa == pb
    }
}




/// 内部辅助：对两个 T 的内存逐字节应用 `f`。
///
/// # Safety
/// 1. `T` 不能有 padding 字节（否则读取未初始化内存是 UB）；
/// 2. `f` 作用于任意两个合法 `T` 的逐字节结果，必须仍是合法的 `T`。
#[inline]
unsafe fn bytewise<T: Copy>(a: &T, b: &T, f: impl Fn(u8, u8) -> u8) -> T {
    let mut out = MaybeUninit::<T>::uninit();
    let pa = a as *const T as *const u8;
    let pb = b as *const T as *const u8;
    let po = out.as_mut_ptr() as *mut u8;

    for i in 0..size_of::<T>() {
        *po.add(i) = f(*pa.add(i), *pb.add(i));
    }
    out.assume_init()
}

/// 按位或
///
/// # Safety
/// 同 `bytewise`：`T` 无 padding，且 OR 结果必须是合法的 `T`。
pub unsafe fn bitor_raw<T: Copy>(a: &T, b: &T) -> T {
    bytewise(a, b, |x, y| x | y)
}

/// 按位与非：!(a & b)
///
/// # Safety
/// 同 `bytewise`：`T` 无 padding，且 NAND 结果必须是合法的 `T`。
pub unsafe fn bitnand_raw<T: Copy>(a: &T, b: &T) -> T {
    bytewise(a, b, |x, y| !(x & y))
}

/// 按位或非：!(a | b)
///
/// # Safety
/// 同 `bytewise`：`T` 无 padding，且 NOR 结果必须是合法的 `T`。
pub unsafe fn bitnor_raw<T: Copy>(a: &T, b: &T) -> T {
    bytewise(a, b, |x, y| !(x | y))
}

pub unsafe fn bitxor_raw<T: Copy>(a: &T, b: &T) -> T {
    bytewise(a, b, |x, y| x ^ y)
}

fn main() {
    let x: u32 = 0xFF00_FF00;
    let y: u32 = 0x0F0F_0F0F;

    unsafe {
        println!("OR   = {:#010X}", bitor_raw(&x, &y));   // 0xFF0FFF0F
        println!("NAND = {:#010X}", bitnand_raw(&x, &y)); // 0xF0FFF0FF
        println!("NOR  = {:#010X}", bitnor_raw(&x, &y));  // 0x00F000F0
    }

    // 数组同样适用（无 padding）
    let a = [1u8, 3, 7, 15];
    let b = [2u8, 2, 2, 2];
    unsafe {
        println!("{:?}", bitor_raw(&a, &b));   // [3, 3, 7, 15]
        println!("{:?}", bitnand_raw(&a, &b)); // [255, 253, 253, 253]
        println!("{:?}", bitnor_raw(&a, &b));  // [252, 252, 248, 240]
    }
}