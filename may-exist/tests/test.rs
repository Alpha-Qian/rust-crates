use std::marker::PhantomData;

use may_exist::{ConstBool, StaticOption};



#[test]
fn test_some() {
    let a = StaticOption::new(1);
    let b = a.0;
    let c = a.unwarp();
}

#[test]
fn test_none() {
    let a = StaticOption::new_none();
    a.assert_none();
    let b = a.unwarp_or(1);
    let c = a.unwarp_or_default();
    assert!(b == 1);
    assert!(c == 0);
}

fn hander<T, F: ConstBool>(mut value: StaticOption<T, F>) {
    let mut_ref = value.as_mut();
    let optional = mut_ref.into_option();
}

trait Foo{
    fn foobar();
}

struct Bar;

impl Foo for Bar {
    fn foobar() {
        
    }
}


fn call_foobar<T: Foo>(ty: TypeOf<T>) {
    T::foobar();
}

struct TypeOf<T>(PhantomData<T>);

impl<T> TypeOf<T> {
    
    pub fn by_const(ptr: *const T) -> Self{
        Self(PhantomData)
    }
}

#[test]
fn test_hander() {
    let a = Bar;
    let ptr = &a as *const _;
    let type_of = TypeOf::by_const(ptr);
    call_foobar(type_of);
}