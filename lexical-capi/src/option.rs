//! C-compatible option type.

use crate::lib::option::Option as StdOption;

/// Tag for the FFI-compatible option.
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
enum OptionTag {
    Some = 0,
    None = 1
}

/// Union for the FFI-compatible option.
#[repr(C)]
#[derive(Copy, Clone)]
union OptionUnion<T: Copy> {
    value: T,
    none: ()
}

/// C-compatible option type.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Option<T: Copy> {
    tag: OptionTag,
    data: OptionUnion<T>,
}

impl<T: Copy> From<StdOption<T>> for Option<T> {
    fn from(res: StdOption<T>) -> Option<T> {
        match res {
            Some(v) => {
                let data = OptionUnion { value: v };
                Option { tag: OptionTag::Some, data }
            },
            None    => {
                let data = OptionUnion { none: () };
                Option { tag: OptionTag::None, data }
            },
        }
    }
}

impl<T: Copy> Into<StdOption<T>> for Option<T> {
    fn into(self) -> StdOption<T> {
        unsafe {
            match self.tag {
                OptionTag::Some => Some(self.data.value),
                OptionTag::None => None,
            }
        }
    }
}
