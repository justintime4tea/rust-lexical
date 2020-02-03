//! C-compatible format builders.

use lexical_core;

/// Create a builder specialized for the number format.
macro_rules! create_number_format_builder {
    (
        $(
            fn $field_fn:ident ($argname:ident : $argtype:ty);
        )*
    ) => (
        create_builder!(
            fn lexical_number_format_builder -> NumberFormat;
            fn lexical_number_format_build;

            $(
                @field
                #[cfg(feature = "format")]
                fn $field_fn ($argname: $argtype);
            )*
        );
    );
}

// NumberFormatBuilder
create_number_format_builder!(
    fn lexical_number_format_builder_digit_separator(digit_separator: u8);
    fn lexical_number_format_builder_required_integer_digits(required_integer_digits: bool);
    fn lexical_number_format_builder_required_fraction_digits(required_fraction_digits: bool);
    fn lexical_number_format_builder_required_exponent_digits(required_exponent_digits: bool);
    fn lexical_number_format_builder_no_positive_mantissa_sign(no_positive_mantissa_sign: bool);
    fn lexical_number_format_builder_required_mantissa_sign(required_mantissa_sign: bool);
    fn lexical_number_format_builder_no_exponent_notation(no_exponent_notation: bool);
    fn lexical_number_format_builder_no_positive_exponent_sign(no_positive_exponent_sign: bool);
    fn lexical_number_format_builder_required_exponent_sign(required_exponent_sign: bool);
    fn lexical_number_format_builder_no_exponent_without_fraction(no_exponent_without_fraction: bool);
    fn lexical_number_format_builder_no_special(no_special: bool);
    fn lexical_number_format_builder_case_sensitive_special(case_sensitive_special: bool);
    fn lexical_number_format_builder_no_integer_leading_zeros(no_integer_leading_zeros: bool);
    fn lexical_number_format_builder_no_float_leading_zeros(no_float_leading_zeros: bool);
    fn lexical_number_format_builder_integer_internal_digit_separator(integer_internal_digit_separator: bool);
    fn lexical_number_format_builder_fraction_internal_digit_separator(fraction_internal_digit_separator: bool);
    fn lexical_number_format_builder_exponent_internal_digit_separator(exponent_internal_digit_separator: bool);
    fn lexical_number_format_builder_integer_leading_digit_separator(integer_leading_digit_separator: bool);
    fn lexical_number_format_builder_fraction_leading_digit_separator(fraction_leading_digit_separator: bool);
    fn lexical_number_format_builder_exponent_leading_digit_separator(exponent_leading_digit_separator: bool);
    fn lexical_number_format_builder_integer_trailing_digit_separator(integer_trailing_digit_separator: bool);
    fn lexical_number_format_builder_fraction_trailing_digit_separator(fraction_trailing_digit_separator: bool);
    fn lexical_number_format_builder_exponent_trailing_digit_separator(exponent_trailing_digit_separator: bool);
    fn lexical_number_format_builder_integer_consecutive_digit_separator(integer_consecutive_digit_separator: bool);
    fn lexical_number_format_builder_fraction_consecutive_digit_separator(fraction_consecutive_digit_separator: bool);
    fn lexical_number_format_builder_exponent_consecutive_digit_separator(exponent_consecutive_digit_separator: bool);
    fn lexical_number_format_builder_special_digit_separator(special_digit_separator: bool);
);

////
//#[doc(hidden)]
//#[no_mangle]
//#[cfg(feature = "format")]
