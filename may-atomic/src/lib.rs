pub mod may_atomic;
pub use std::sync::atomic::AtomicUsize;
use core::{
    cell::Cell,
};

use radium::{
    Atom, Radium,
    marker::{Atomic, Nuclear},
};


mod sealed{
    pub trait Sealed {}
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    }
}
