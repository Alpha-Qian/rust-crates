//use radium::Atom

use std::mem::ManuallyDrop;
use std::ptr::drop_in_place;
use std::{cell::Cell, ops::Deref, ptr::NonNull};

use may_exist::{ConstBool, ConstFalse, ConstTrue, StaticOption};
use std::rc::{Rc as StdRc, Weak as StdWeak};
use std::sync::{Arc as StdArc, Weak as StdArcWeak};
type Wakable = ConstTrue;
type UnUwakable = ConstFalse;

//#[repr(C)]
struct RcInner<T: ?Sized, W: ConstBool> {
    strong: Cell<usize>,
    weak: StaticOption<Cell<usize>, W>,
    data: T,
}

// ==========================
// Rc

struct Rc<T: ?Sized, W: ConstBool>(NonNull<RcInner<T, W>>);

impl<T: ?Sized, W: ConstBool> Deref for Rc<T, W> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &self.0.as_ref().data }
    }
}

///通用方法
impl<T: ?Sized, W: ConstBool> Rc<T, W> {
    fn new(data: T) -> Self
    where
        T: Sized,
    {
        let a = Box::leak(Box::new(RcInner {
            strong: 1.into(),
            weak: StaticOption::new_none_or(1.into()),
            data,
        }));
        Self(NonNull::from_mut(a))
    }
}

///Weakable only方法
impl<T: ?Sized> Rc<T, ConstTrue> {
    fn create_weak(&self) -> Weak<T> {
        let inner = unsafe { self.0.as_ref() };
        inner.weak.0.update(|x| x + 1); //fetch_add relax
        Weak(self.0)
    }

    fn downgrade(self) -> Weak<T> {
        self.create_weak()
    }
}

impl<T: ?Sized, W: ConstBool> Clone for Rc<T, W> {
    fn clone(&self) -> Self {
        let inner = unsafe { self.0.as_ref() };
        inner.strong.update(|x| x + 1); //fetch_add relax
        Self(self.0)
    }
}

impl<T: ?Sized, W: ConstBool> Drop for Rc<T, W> {
    fn drop(&mut self) {
        let mut inner = unsafe { self.0.as_mut() };
        let strong = inner.strong.get_mut();
        *strong -= 1; //fetch_sub ?
        if *strong > 0 {
            return;
        }

        //drop value
        unsafe {
            drop_in_place(&mut inner.data as *mut _);
        }

        let weak = inner.weak.as_ref();

        weak.map(|weak| weak.update(|x| x - 1)); //fetch_sub ?

        //the last one is me
        if weak.map(|r| r.get()).unwarp_or(1) > 0 {
            return;
        }

        //free
        unsafe {
            let ptr = inner as *mut _ as *mut ManuallyDrop<RcInner<T, W>>;
            Box::from_raw(ptr);
        }
    }
}

// ==================================
// Weak
//

struct Weak<T: ?Sized>(NonNull<RcInner<T, ConstTrue>>);

impl<T: ?Sized> Weak<T> {
    fn create_strong(&self) -> Option<Rc<T, ConstTrue>> {
        let inner = unsafe { self.0.as_ref() };
        let strong_count = inner.strong.get();
        if strong_count > 0 {
            inner.strong.set(strong_count + 1); //fetch_add relax
            return Some(Rc(self.0));
        }
        return None;
    }

    fn upgrade(self) -> Option<Rc<T, ConstTrue>> {
        self.create_strong()
    }
}

impl<T: ?Sized> Clone for Weak<T> {
    fn clone(&self) -> Self {
        let inner = unsafe { self.0.as_ref() };
        inner.weak.0.update(|x| x + 1); //fetch_add relax
        Self(self.0)
    }
}

impl<T: ?Sized> Drop for Weak<T> {
    fn drop(&mut self) {
        let mut inner = unsafe { self.0.as_mut() };
        *inner.weak.0.get_mut() -= 1; //fetch AcqRel

        if inner.weak.0.get() == 0 {
            // && inner.strong.get() == 0 {
            //free
            unsafe {
                let ptr = inner as *mut _ as *mut ManuallyDrop<RcInner<T, ConstTrue>>;
                Box::from_raw(ptr);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
