use std::{cell::{self, Cell, UnsafeCell}, fmt::Debug, mem::{forget, transmute}, range::RangeInclusive, result, sync::atomic::{Atomic, Ordering, fence}};

use may_exist::{self, ConstBool, ConstFalse, ConstTrue};
use atomig::{Atom, AtomInteger, AtomLogic, Atomic, impls::{PrimitiveAtom, PrimitiveAtomInteger, PrimitiveAtomLogic}};
use crate::utils::{bitnand_raw, bitor_raw, bits_eq, bitxor_raw};

#[repr(transparent)]
struct MayAtomic<T: Atom, A: ConstBool>(
    pub A::Select<Atomic<T>, Cell<T::Repr>>
);

impl<T: Atom, A: ConstBool> MayAtomic<T, A> {
    
    fn new(value: T) -> Self{
        if A::VALUE {
            return unsafe{ transmute_unchecked(Atomic::new(value)) }
        }
        return unsafe{ transmute_unchecked(Cell::new(value.pack())) }
    }

    fn fence(order: Ordering) {
        if A::VALUE {
            fence(order);
        }
    }

    fn load(&self, order: Ordering) -> T{
         
        if A::VALUE {
            //let a = self.0;
            let atomic: &Atomic<T> = unsafe{ transmute(self) };
            return atomic.load(order);
        }
        
        let cell: &Cell<T::Repr>= unsafe{ transmute(self) };
        T::unpack(cell.get())
    }

    fn store(&self, value: T, order: Ordering) {
        if A::VALUE {
            let atomic: &Atomic<T> = unsafe{ transmute(self) };
            return atomic.store(value, order);
        }
        
        let cell: &Cell<T::Repr> = unsafe{ transmute(self) };
        return cell.set(value.pack());
    }

    fn swap(&self, value: T, order: Ordering) -> T{
        if A::VALUE {
            let atomic: &Atomic<T> = unsafe{ transmute(self) };
            return atomic.swap(value, order);
        }

        let cell: &Cell<T::Repr> = unsafe{ transmute(self) };
        return T::unpack(cell.replace(value.pack()));
    }

    fn compare_exchange(&self, current: T, new: T, success: Ordering, failure: Ordering) -> Result<T, T>{
        if A::VALUE {
            let atomic: &Atomic<T> = unsafe{ transmute(self) };
            return atomic.compare_exchange(current, new, success, failure);
        }

        let cell: &Cell<T::Repr> = unsafe{ transmute(self) };
        
        let old = cell.get();
        if unsafe{ bits_eq(&current.pack(), &old) } {
            cell.set(new.pack());
            Ok(T::unpack(old))
        } else {
            Err(T::unpack(old))
        }
    }

    fn compare_exchange_weak(&self, current: T, new: T, success: Ordering, failure: Ordering) -> Result<T, T> {
        if A::VALUE {
            let atomic: &Atomic<T> = unsafe{ transmute(self) };
            return atomic.compare_exchange_weak(current, new, success, failure)
        }

        let cell: &Cell<T::Repr> = unsafe{ transmute(self) };
        let old = cell.get();
        if unsafe{ bits_eq(&current.pack(), &old) } {
            cell.set(new.pack());
            Ok(T::unpack(old))
        } else {
            Err(T::unpack(old))
        }
    }

    fn fetch_update(&self, set_order: Ordering, fetch_order: Ordering,mut f: impl FnMut(T) -> Option<T>) -> Result<T, T>{
        if A::VALUE {
            let atomic: &Atomic<T> = unsafe{ transmute(self) };
            return atomic.fetch_update(set_order, fetch_order, f)
        }

        let cell: &Cell<T::Repr> = unsafe{ transmute(self) };
        let old = cell.get();
        match f(T::unpack(old)) {
            Some(new) => {
                cell.set(new.pack());
                return Ok(T::unpack(old))
            },
            None => return Err(T::unpack(old))
        }
    }

    ///仅在原子条件下compare
    fn optional_compare_exchange(&self, current: T, new: T, success: Ordering, failure: Ordering) -> Result<T, T>{
        if A::VALUE {
            let atomic: &Atomic<T> = unsafe{ transmute_unchecked(self) };
            return atomic.compare_exchange(current, new, success, failure);
        }

        let cell: &Cell<T::Repr> = unsafe{ transmute_unchecked(self) };
        cell.set(new.pack());
        return Ok(current)
    }

    ///仅在原子条件下compare
    fn optional_compare_exchange_weak(&self, current: T, new: T, success: Ordering, failure: Ordering) -> Result<T, T> {
        if A::VALUE {
            let atomic: &Atomic<T> = unsafe{ transmute_unchecked(self) };
            return atomic.compare_exchange_weak(current, new, success, failure)
        }

        let cell: &Cell<T::Repr> = unsafe{ transmute_unchecked(self) };
        cell.set(new.pack());
        return Ok(current)
    }

    fn into_inner(self) -> T {
        if A::VALUE {
            let atomic: Atomic<T> = unsafe{ transmute_unchecked(self)};
            return atomic.into_inner()
        }

        let cell: Cell<T::Repr> = unsafe{ transmute_unchecked(self) };
        T::unpack(cell.into_inner())
    }

    fn get_repr_mut(&mut self) -> &mut T::Repr {
        if A::VALUE {
            let atomic: &mut Atomic<T> = unsafe{ transmute(self) };
            return unsafe{
                // Atomic没有#[repr(transparent)]，但crate没有提供其他Api，我们不得不这么做
                transmute(atomic)
            }
        }
        let cell: &mut UnsafeCell<T::Repr> = unsafe{ transmute_unchecked(self) };
        cell.get_mut()
    }

    
    const fn is_atomic() -> bool {
        A::VALUE
    }

    const fn is_cell() -> bool {
        !A::VALUE
    }
}

impl<T: AtomLogic, A: ConstBool> MayAtomic<T, A>
where
    T::Repr: PrimitiveAtomLogic
{
    fn fetch_and(&self, val: T, order: Ordering) -> T {
        if A::VALUE {
            let atomic: &Atomic<T> = unsafe{ transmute(self) };
            return atomic.fetch_and(val, order)
        }

        let cell: &Cell<T::Repr> = unsafe{ transmute(self) };
        let current = cell.get();
        let result = unsafe{ bitnand_raw(&val.pack(), &current) };
        cell.set(result);
        return T::unpack(result)
    }

    fn fetch_or(&self, val: T,order: Ordering) -> T {
        if A::VALUE {
            let atomic: &Atomic<T> = unsafe{ transmute(self) };
            return atomic.fetch_or(val, order)
        }

        let cell: &Cell<T::Repr> = unsafe{ transmute(self) };
        let current = cell.get();
        let result = unsafe{ bitor_raw(&current, &val.pack()) };
        cell.set(result);
        return T::unpack(result)
    }

    fn fetch_xor(&self, val: T, order: Ordering) -> T{
        if A::VALUE {
            let atomic: &Atomic<T> = unsafe{ transmute(self) };
            return atomic.fetch_xor(val, order)
        }

        let cell: &Cell<T::Repr> = unsafe{ transmute(self) };
        let current = cell.get();
        let result = unsafe{ bitxor_raw(&val.pack(), &current) };
        cell.set(result);
        return T::unpack(result)
    }

    fn fetch_nand(&self, val: T, order: Ordering) -> T {
        if A::VALUE {
            let atomic: &Atomic<T> = unsafe{ transmute(self) };
            return atomic.fetch_nand(val, order)
        }

        let cell: &Cell<T::Repr> = unsafe{ transmute(self) };
        let current = cell.get();
        let result = unsafe{ bitnand_raw(&val.pack(), &current) };
        return T::unpack(result)
        
    }
}

impl<T: AtomInteger, A: ConstBool> MayAtomic<T, A>
where
    T::Repr: PrimitiveAtomInteger
{
    
}

impl<T: Atom + Default, A: ConstBool> Default for MayAtomic<T, A> {
    fn default() -> Self {
        let value = T::default();
        Self::new(value)
    }
}

impl<T: Atom + Debug, A: ConstBool> Debug for MayAtomic<T, A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.load(Ordering::SeqCst).fmt(f)
    }
}

impl<T: Atom, A: ConstBool> From<T> for MayAtomic<T, A> {
    fn from(value: T) -> Self {
        if A::VALUE {
            let atomic = Atomic::from(value);
            return unsafe{ transmute_unchecked(atomic)}
        }

        let cell = Cell::from(value);
        unsafe{ transmute_unchecked(cell) }
    }
}

impl<T: Atom> MayAtomic<T, ConstTrue> {
    const fn assert_atomic() {}
}

impl<T: Atom> MayAtomic<T, ConstFalse> {
    const fn assert_cell() {}
}

/// Safety:
///  同mem::transmut_unchecked
pub(crate) const unsafe fn transmute_unchecked<T, U>(src: T) -> U {
    let ptr = &src as *const T as *const U;
    let dst = unsafe{ std::ptr::read(ptr) };
    forget(src);
    dst
}

