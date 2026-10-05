use crate::{ConstBool, ConstFalse, ConstTrue};

trait FromBool<const B: bool> {
    type Type: ConstBool;
}

impl FromBool<true> for () {
    type Type = ConstTrue;
}

impl FromBool<false> for () {
    type Type = ConstFalse;
}

// 用户用法: <() as FromBool<MY_CONST>>::Type