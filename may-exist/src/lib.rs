
//mod may_exist;

pub mod static_bool;
pub mod static_option;
mod static_u8;
mod static_result;
mod utils;
pub mod preload;

pub use crate::static_option::{MetaData, StaticOption};
pub use crate::static_bool::{ConstBool, ConstTrue, ConstFalse, Neg, And, Or, Xor, Eq};


#[cfg(test)]
mod test {
    #[test]
    fn test() {
      
    }
}




mod sealed{
    pub trait Sealed {}
}