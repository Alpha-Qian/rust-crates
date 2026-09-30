#![no_std]
//mod may_exist;

pub mod preload;
pub mod static_option;
mod static_result;
mod utils;

pub use const_bool::*;
pub use crate::static_option::{StaticOption};

#[cfg(test)]
mod test {
    #[test]
    fn test() {}
}

mod sealed {
    pub trait Sealed {}
}
