//! C-compatible format builders.

use lexical_core;

// NumberFormatBuilder
create_builder!(
    fn lexical_number_format_builder -> NumberFormat;
    fn lexical_number_format_build;
);

/// Get the permissive number format.
#[doc(hidden)]
#[no_mangle]
#[cfg(feature = "format")]
pub unsafe extern fn lexical_number_format_permissive()
    -> crate::option::Option<lexical_core::NumberFormat>
{
    lexical_core::NumberFormat::permissive().into()
}

/// Get the standard number format.
#[doc(hidden)]
#[no_mangle]
#[cfg(feature = "format")]
pub unsafe extern fn lexical_number_format_standard()
    -> crate::option::Option<lexical_core::NumberFormat>
{
    lexical_core::NumberFormat::standard().into()
}

/// Get the ignore number format.
#[doc(hidden)]
#[no_mangle]
#[cfg(feature = "format")]
pub unsafe extern fn lexical_number_format_ignore(digit_separator: u8)
    -> crate::option::Option<lexical_core::NumberFormat>
{
    lexical_core::NumberFormat::ignore(digit_separator).into()
}
