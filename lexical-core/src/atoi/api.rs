//! Fast lexical string-to-integer conversion routines.

use crate::util::*;
use super::generic::*;

// ATOI TRAIT
// ----------

pub(crate) trait Atoi: Integer {
    // Parse integer from string.
    fn parse(bytes: &[u8]) -> ParseResult<(Self, *const u8)>;

    // Parse integer from string with options.
    fn parse_with_options(bytes: &[u8], options: &ParseIntegerOptions) -> ParseResult<(Self, *const u8)>;
}

// Implement atoi for type.
macro_rules! atoi_impl {
    ($($t:ty)*) => ($(
        impl Atoi for $t {
            perftools_inline_always!{
            fn parse(bytes: &[u8])
                -> ParseResult<($t, *const u8)>
            {
                standalone_no_options(bytes)
            }}

            perftools_inline_always!{
            fn parse_with_options(bytes: &[u8], options: &ParseIntegerOptions)
                -> ParseResult<($t, *const u8)>
            {
                standalone_options(bytes, options)
            }}
        }
    )*);
}

atoi_impl! { u8 u16 u32 u64 usize i8 i16 i32 i64 isize }

impl Atoi for u128 {
    perftools_inline_always!{
    fn parse(bytes: &[u8])
        -> ParseResult<(u128, *const u8)>
    {
        standalone_128_no_options::<u128, u64>(bytes)
    }}

    perftools_inline_always!{
    fn parse_with_options(bytes: &[u8], options: &ParseIntegerOptions)
        -> ParseResult<(u128, *const u8)>
    {
        standalone_128_options::<u128, u64>(bytes, options)
    }}
}

impl Atoi for i128 {
    perftools_inline_always!{
    fn parse(bytes: &[u8])
        -> ParseResult<(i128, *const u8)>
    {
        standalone_128_no_options::<i128, i64>(bytes)
    }}

    perftools_inline_always!{
    fn parse_with_options(bytes: &[u8], options: &ParseIntegerOptions)
        -> ParseResult<(i128, *const u8)>
    {
        standalone_128_options::<i128, i64>(bytes, options)
    }}
}

// ATOI
// ----

perftools_inline_always!{
pub(crate) fn parse<'a, T>(bytes: &'a [u8])
    -> Result<(T, usize)>
    where T: Atoi
{
    let index = | ptr | distance(bytes.as_ptr(), ptr);
    match T::parse(bytes) {
        Ok((value, ptr)) => Ok((value, index(ptr))),
        Err((code, ptr)) => Err((code, index(ptr)).into()),
    }
}}

perftools_inline_always!{
pub(crate) fn parse_with_options<'a, T>(bytes: &'a [u8], options: &ParseIntegerOptions)
    -> Result<(T, usize)>
    where T: Atoi
{
    let index = | ptr | distance(bytes.as_ptr(), ptr);
    match T::parse_with_options(bytes, &options) {
        Ok((value, ptr)) => Ok((value, index(ptr))),
        Err((code, ptr)) => Err((code, index(ptr)).into()),
    }
}}

// FROM LEXICAL
// ------------

macro_rules! atoi_from_lexical {
    ($($type:ident)*) => ($(
        from_lexical!(
            type => $type,
            options => ParseIntegerOptions,
            parse => parse,
            parse_with_options => parse_with_options
        );
    )*);
}

atoi_from_lexical! { u8 u16 u32 u64 u128 usize i8 i16 i32 i64 isize i128 }

// TESTS
// -----

#[cfg(test)]
mod tests {
    use crate::util::*;

    #[cfg(all(feature = "std", feature = "property_tests"))]
    use proptest::{proptest, prop_assert_eq, prop_assert};

    #[cfg(feature = "radix")]
    const DATA: [(u8, &'static str); 35] = [
        (2, "100101"),
        (3, "1101"),
        (4, "211"),
        (5, "122"),
        (6, "101"),
        (7, "52"),
        (8, "45"),
        (9, "41"),
        (10, "37"),
        (11, "34"),
        (12, "31"),
        (13, "2B"),
        (14, "29"),
        (15, "27"),
        (16, "25"),
        (17, "23"),
        (18, "21"),
        (19, "1I"),
        (20, "1H"),
        (21, "1G"),
        (22, "1F"),
        (23, "1E"),
        (24, "1D"),
        (25, "1C"),
        (26, "1B"),
        (27, "1A"),
        (28, "19"),
        (29, "18"),
        (30, "17"),
        (31, "16"),
        (32, "15"),
        (33, "14"),
        (34, "13"),
        (35, "12"),
        (36, "11"),
    ];

    #[test]
    fn u8_decimal_test() {
        assert_eq!(Ok(0), u8::from_lexical(b"0"));
        assert_eq!(Ok(127), u8::from_lexical(b"127"));
        assert_eq!(Ok(128), u8::from_lexical(b"128"));
        assert_eq!(Ok(255), u8::from_lexical(b"255"));
        assert_eq!(Err((ErrorCode::InvalidDigit, 0).into()), u8::from_lexical(b"-1"));
        assert_eq!(Err((ErrorCode::InvalidDigit, 1).into()), u8::from_lexical(b"1a"));
    }

    #[test]
    #[cfg(feature = "radix")]
    fn u8_radix_test() {
        for (b, s) in DATA.iter() {
            let options = ParseIntegerOptions::builder().radix(*b).build().unwrap();
            assert_eq!(u8::from_lexical_with_options(s.as_bytes(), &options), Ok(37));
        }
    }

    #[test]
    fn i8_decimal_test() {
        assert_eq!(Ok(0), i8::from_lexical(b"0"));
        assert_eq!(Ok(127), i8::from_lexical(b"127"));
        assert_eq!(Err((ErrorCode::Overflow, 2).into()), i8::from_lexical(b"128"));
        assert_eq!(Err((ErrorCode::Overflow, 2).into()), i8::from_lexical(b"255"));
        assert_eq!(Ok(-1), i8::from_lexical(b"-1"));
        assert_eq!(Err((ErrorCode::InvalidDigit, 1).into()), i8::from_lexical(b"1a"));
    }

    #[test]
    #[cfg(feature = "radix")]
    fn i8_radix_test() {
        for (b, s) in DATA.iter() {
            let options = ParseIntegerOptions::builder().radix(*b).build().unwrap();
            assert_eq!(i8::from_lexical_with_options(s.as_bytes(), &options), Ok(37));
        }
    }

    #[test]
    fn u16_decimal_test() {
        assert_eq!(Ok(0), u16::from_lexical(b"0"));
        assert_eq!(Ok(32767), u16::from_lexical(b"32767"));
        assert_eq!(Ok(32768), u16::from_lexical(b"32768"));
        assert_eq!(Ok(65535), u16::from_lexical(b"65535"));
        assert_eq!(Err((ErrorCode::InvalidDigit, 0).into()), u16::from_lexical(b"-1"));
        assert_eq!(Err((ErrorCode::InvalidDigit, 1).into()), u16::from_lexical(b"1a"));
    }

    #[test]
    fn i16_decimal_test() {
        assert_eq!(Ok(0), i16::from_lexical(b"0"));
        assert_eq!(Ok(32767), i16::from_lexical(b"32767"));
        assert_eq!(Err((ErrorCode::Overflow, 4).into()), i16::from_lexical(b"32768"));
        assert_eq!(Err((ErrorCode::Overflow, 4).into()), i16::from_lexical(b"65535"));
        assert_eq!(Ok(-1), i16::from_lexical(b"-1"));
        assert_eq!(Err((ErrorCode::InvalidDigit, 1).into()), i16::from_lexical(b"1a"));
    }

    #[test]
    #[cfg(feature = "radix")]
    fn i16_radix_test() {
        for (b, s) in DATA.iter() {
            let options = ParseIntegerOptions::builder().radix(*b).build().unwrap();
            assert_eq!(i16::from_lexical_with_options(s.as_bytes(), &options), Ok(37));
        }
        let options = ParseIntegerOptions::builder().radix(36).build().unwrap();
        assert_eq!(i16::from_lexical_with_options(b"YA", &options), Ok(1234));
    }

    #[test]
    fn u32_decimal_test() {
        assert_eq!(Ok(0), u32::from_lexical(b"0"));
        assert_eq!(Ok(2147483647), u32::from_lexical(b"2147483647"));
        assert_eq!(Ok(2147483648), u32::from_lexical(b"2147483648"));
        assert_eq!(Ok(4294967295), u32::from_lexical(b"4294967295"));
        assert_eq!(Err((ErrorCode::InvalidDigit, 0).into()), u32::from_lexical(b"-1"));
        assert_eq!(Err((ErrorCode::InvalidDigit, 1).into()), u32::from_lexical(b"1a"));
    }

    #[test]
    fn i32_decimal_test() {
        assert_eq!(Ok(0), i32::from_lexical(b"0"));
        assert_eq!(Ok(2147483647), i32::from_lexical(b"2147483647"));
        assert_eq!(Err((ErrorCode::Overflow, 9).into()), i32::from_lexical(b"2147483648"));
        assert_eq!(Err((ErrorCode::Overflow, 9).into()), i32::from_lexical(b"4294967295"));
        assert_eq!(Ok(-1), i32::from_lexical(b"-1"));
        assert_eq!(Err((ErrorCode::InvalidDigit, 1).into()), i32::from_lexical(b"1a"));
    }

    #[test]
    fn u64_decimal_test() {
        assert_eq!(Ok(0), u64::from_lexical(b"0"));
        assert_eq!(Ok(9223372036854775807), u64::from_lexical(b"9223372036854775807"));
        assert_eq!(Ok(9223372036854775808), u64::from_lexical(b"9223372036854775808"));
        assert_eq!(Ok(18446744073709551615), u64::from_lexical(b"18446744073709551615"));
        assert_eq!(Err((ErrorCode::InvalidDigit, 0).into()), u64::from_lexical(b"-1"));
        assert_eq!(Err((ErrorCode::InvalidDigit, 1).into()), u64::from_lexical(b"1a"));
    }

    #[test]
    fn i64_decimal_test() {
        assert_eq!(Ok(0), i64::from_lexical(b"0"));
        assert_eq!(Ok(9223372036854775807), i64::from_lexical(b"9223372036854775807"));
        assert_eq!(Err((ErrorCode::Overflow, 18).into()), i64::from_lexical(b"9223372036854775808"));
        assert_eq!(Err((ErrorCode::Overflow, 19).into()), i64::from_lexical(b"18446744073709551615"));
        assert_eq!(Ok(-1), i64::from_lexical(b"-1"));
        assert_eq!(Err((ErrorCode::InvalidDigit, 1).into()), i64::from_lexical(b"1a"));

        // Add tests discovered via fuzzing.
        assert_eq!(Err((ErrorCode::Overflow, 19).into()), i64::from_lexical(b"406260572150672006000066000000060060007667760000000000000000000+00000006766767766666767665670000000000000000000000666"));
    }

    #[test]
    fn u128_decimal_test() {
        assert_eq!(Ok(0), u128::from_lexical(b"0"));
        assert_eq!(Ok(170141183460469231731687303715884105727), u128::from_lexical(b"170141183460469231731687303715884105727"));
        assert_eq!(Ok(170141183460469231731687303715884105728), u128::from_lexical(b"170141183460469231731687303715884105728"));
        assert_eq!(Ok(340282366920938463463374607431768211455), u128::from_lexical(b"340282366920938463463374607431768211455"));
        assert_eq!(Err((ErrorCode::InvalidDigit, 0).into()), u128::from_lexical(b"-1"));
        assert_eq!(Err((ErrorCode::InvalidDigit, 1).into()), u128::from_lexical(b"1a"));
    }

    #[test]
    fn i128_decimal_test() {
        assert_eq!(Ok(0), i128::from_lexical(b"0"));
        assert_eq!(Ok(170141183460469231731687303715884105727), i128::from_lexical(b"170141183460469231731687303715884105727"));
        assert_eq!(Err((ErrorCode::Overflow, 39).into()), i128::from_lexical(b"170141183460469231731687303715884105728"));
        assert_eq!(Err((ErrorCode::Overflow, 39).into()), i128::from_lexical(b"340282366920938463463374607431768211455"));
        assert_eq!(Ok(-1), i128::from_lexical(b"-1"));
        assert_eq!(Err((ErrorCode::InvalidDigit, 1).into()), i128::from_lexical(b"1a"));
    }

    #[test]
    #[cfg(feature = "format")]
    fn i32_no_leading_zeros_test() {
        let format = NumberFormat::NO_INTEGER_LEADING_ZEROS;
        let options = ParseIntegerOptions::builder()
            .format(format)
            .build()
            .unwrap();
        assert!(i32::from_lexical_with_options(b"1", &options).is_ok());
        assert!(i32::from_lexical_with_options(b"0", &options).is_ok());
        assert!(i32::from_lexical_with_options(b"01", &options).is_err());
        assert!(i32::from_lexical_with_options(b"10", &options).is_ok());
        assert!(i32::from_lexical_with_options(b"010", &options).is_err());
    }

    #[test]
    #[cfg(feature = "format")]
    fn i32_integer_internal_digit_separator_test() {
        let format = NumberFormat::from_separator(b'_') | NumberFormat::INTEGER_INTERNAL_DIGIT_SEPARATOR;
        let options = ParseIntegerOptions::builder()
            .format(format)
            .build()
            .unwrap();
        assert!(i32::from_lexical_with_options(b"3_1", &options).is_ok());
        assert!(i32::from_lexical_with_options(b"_31", &options).is_err());
        assert!(i32::from_lexical_with_options(b"31_", &options).is_err());
    }

    #[test]
    #[cfg(feature = "format")]
    fn i32_integer_leading_digit_separator_test() {
        let format = NumberFormat::from_separator(b'_') | NumberFormat::INTEGER_LEADING_DIGIT_SEPARATOR;
        let options = ParseIntegerOptions::builder()
            .format(format)
            .build()
            .unwrap();
        assert!(i32::from_lexical_with_options(b"3_1", &options).is_err());
        assert!(i32::from_lexical_with_options(b"_31", &options).is_ok());
        assert!(i32::from_lexical_with_options(b"31_", &options).is_err());
    }

    #[test]
    #[cfg(feature = "format")]
    fn i32_integer_trailing_digit_separator_test() {
        let format = NumberFormat::from_separator(b'_') | NumberFormat::INTEGER_TRAILING_DIGIT_SEPARATOR;
        let options = ParseIntegerOptions::builder()
            .format(format)
            .build()
            .unwrap();
        assert!(i32::from_lexical_with_options(b"3_1", &options).is_err());
        assert!(i32::from_lexical_with_options(b"_31", &options).is_err());
        assert!(i32::from_lexical_with_options(b"31_", &options).is_ok());
    }

    #[test]
    #[cfg(feature = "format")]
    fn i32_integer_consecutive_digit_separator_test() {
        let format = NumberFormat::from_separator(b'_')
            | NumberFormat::INTEGER_INTERNAL_DIGIT_SEPARATOR
            | NumberFormat::INTEGER_CONSECUTIVE_DIGIT_SEPARATOR;
        let options = ParseIntegerOptions::builder()
            .format(format)
            .build()
            .unwrap();
        assert!(i32::from_lexical_with_options(b"3_1", &options).is_ok());
        assert!(i32::from_lexical_with_options(b"3__1", &options).is_ok());
        assert!(i32::from_lexical_with_options(b"_31", &options).is_err());
        assert!(i32::from_lexical_with_options(b"31_", &options).is_err());
    }

    #[test]
    #[cfg(feature = "format")]
    fn i32_json_no_leading_zero() {
        let format = NumberFormat::JSON;
        let options = ParseIntegerOptions::builder()
            .format(format)
            .build()
            .unwrap();
        assert!(i32::from_lexical_with_options(b"12", &options).is_ok());
        assert!(i32::from_lexical_with_options(b"-12", &options).is_ok());
        assert!(i32::from_lexical_with_options(b"012", &options).is_err());
        assert!(i32::from_lexical_with_options(b"-012", &options).is_err());
    }

    #[cfg(all(feature = "std", feature = "property_tests"))]
    proptest! {
        #[test]
        fn u8_invalid_proptest(i in r"[+]?[0-9]{2}\D") {
            let result = u8::from_lexical(i.as_bytes());
            prop_assert!(result.is_err());
            let index = result.err().unwrap().index;
            prop_assert!(index == 2 || index == 3);
        }

        #[test]
        fn u8_overflow_proptest(i in r"[+]?[1-9][0-9]{3}") {
            let result = u8::from_lexical(i.as_bytes());
            prop_assert!(result.is_err());
            let code = result.err().unwrap().code;
            prop_assert_eq!(code, ErrorCode::Overflow);
        }

        #[test]
        fn u8_negative_proptest(i in r"[-][1-9][0-9]{2}") {
            let result = u8::from_lexical(i.as_bytes());
            prop_assert!(result.is_err());
            let code = result.err().unwrap().code;
            prop_assert_eq!(code, ErrorCode::InvalidDigit);
        }

        #[test]
        fn u8_double_sign_proptest(i in r"[+]{2}[0-9]{2}") {
            let result = u8::from_lexical(i.as_bytes());
            prop_assert!(result.is_err());
            let error = result.err().unwrap();
            prop_assert_eq!(error.code, ErrorCode::InvalidDigit);
            prop_assert!(error.index == 1);
        }

        #[test]
        fn u8_sign_only_proptest(i in r"[+]") {
            let result = u8::from_lexical(i.as_bytes());
            prop_assert!(result.is_err());
            let code = result.err().unwrap().code;
            prop_assert_eq!(code, ErrorCode::Empty);
        }

        #[test]
        fn u8_trailing_digits_proptest(i in r"[+]?[0-9]{2}\D[0-9]{2}") {
            let result = u8::from_lexical(i.as_bytes());
            prop_assert!(result.is_err());
            let error = result.err().unwrap();
            prop_assert_eq!(error.code, ErrorCode::InvalidDigit);
            prop_assert!(error.index == 2 || error.index == 3);
        }

        #[test]
        fn i8_invalid_proptest(i in r"[+-]?[0-9]{2}\D") {
            let result = i8::from_lexical(i.as_bytes());
            prop_assert!(result.is_err());
            let error = result.err().unwrap();
            prop_assert_eq!(error.code, ErrorCode::InvalidDigit);
            prop_assert!(error.index == 2 || error.index == 3);
        }

        #[test]
        fn i8_overflow_proptest(i in r"[+]?[1-9][0-9]{3}\D") {
            let result = i8::from_lexical(i.as_bytes());
            let code = result.err().unwrap().code;
            prop_assert_eq!(code, ErrorCode::Overflow);
        }

        #[test]
        fn i8_underflow_proptest(i in r"[-][1-9][0-9]{3}\D") {
            let result = i8::from_lexical(i.as_bytes());
            let code = result.err().unwrap().code;
            prop_assert_eq!(code, ErrorCode::Underflow);
        }

        #[test]
        fn i8_double_sign_proptest(i in r"[+-]{2}[0-9]{2}") {
            let result = i8::from_lexical(i.as_bytes());
            let error = result.err().unwrap();
            prop_assert_eq!(error.code, ErrorCode::InvalidDigit);
            prop_assert!(error.index == 1);
        }

        #[test]
        fn i8_sign_only_proptest(i in r"[+-]") {
            let result = i8::from_lexical(i.as_bytes());
            let error = result.err().unwrap();
            prop_assert_eq!(error.code, ErrorCode::Empty);
        }

        #[test]
        fn i8_trailing_digits_proptest(i in r"[+-]?[0-9]{2}\D[0-9]{2}") {
            let result = i8::from_lexical(i.as_bytes());
            let error = result.err().unwrap();
            prop_assert_eq!(error.code, ErrorCode::InvalidDigit);
            prop_assert!(error.index == 2 || error.index == 3);
        }

        #[test]
        fn u16_invalid_proptest(i in r"[+]?[0-9]{4}\D") {
            let result = u16::from_lexical(i.as_bytes());
            let error = result.err().unwrap();
            prop_assert_eq!(error.code, ErrorCode::InvalidDigit);
            prop_assert!(error.index == 4 || error.index == 5);
        }

        #[test]
        fn u16_overflow_proptest(i in r"[+]?[1-9][0-9]{5}\D") {
            let result = u16::from_lexical(i.as_bytes());
            let code = result.err().unwrap().code;
            prop_assert_eq!(code, ErrorCode::Overflow);
        }

        #[test]
        fn u16_negative_proptest(i in r"[-][1-9][0-9]{4}") {
            let result = u16::from_lexical(i.as_bytes());
            prop_assert!(result.is_err());
            let code = result.err().unwrap().code;
            prop_assert_eq!(code, ErrorCode::InvalidDigit);
        }

        #[test]
        fn u16_double_sign_proptest(i in r"[+]{2}[0-9]{4}") {
            let result = u16::from_lexical(i.as_bytes());
            prop_assert!(result.is_err());
            let error = result.err().unwrap();
            prop_assert_eq!(error.code, ErrorCode::InvalidDigit);
            prop_assert!(error.index == 1);
        }

        #[test]
        fn u16_sign_only_proptest(i in r"[+]") {
            let result = u16::from_lexical(i.as_bytes());
            prop_assert!(result.is_err());
            let code = result.err().unwrap().code;
            prop_assert_eq!(code, ErrorCode::Empty);
        }

        #[test]
        fn u16_trailing_digits_proptest(i in r"[+]?[0-9]{4}\D[0-9]{2}") {
            let result = u16::from_lexical(i.as_bytes());
            prop_assert!(result.is_err());
            let error = result.err().unwrap();
            prop_assert_eq!(error.code, ErrorCode::InvalidDigit);
            prop_assert!(error.index == 4 || error.index == 5);
        }

        #[test]
        fn i16_invalid_proptest(i in r"[+-]?[0-9]{4}\D") {
            let result = i16::from_lexical(i.as_bytes());
            prop_assert!(result.is_err());
            let error = result.err().unwrap();
            prop_assert_eq!(error.code, ErrorCode::InvalidDigit);
            prop_assert!(error.index == 4 || error.index == 5);
        }

        #[test]
        fn i16_overflow_proptest(i in r"[+]?[1-9][0-9]{5}\D") {
            let result = i16::from_lexical(i.as_bytes());
            let code = result.err().unwrap().code;
            prop_assert_eq!(code, ErrorCode::Overflow);
        }

        #[test]
        fn i16_underflow_proptest(i in r"[-][1-9][0-9]{5}\DD") {
            let result = i16::from_lexical(i.as_bytes());
            let code = result.err().unwrap().code;
            prop_assert_eq!(code, ErrorCode::Underflow);
        }

        #[test]
        fn i16_double_sign_proptest(i in r"[+-]{2}[0-9]{4}") {
            let result = i16::from_lexical(i.as_bytes());
            let error = result.err().unwrap();
            prop_assert_eq!(error.code, ErrorCode::InvalidDigit);
            prop_assert!(error.index == 1);
        }

        #[test]
        fn i16_sign_only_proptest(i in r"[+-]") {
            let result = i16::from_lexical(i.as_bytes());
            prop_assert!(result.is_err());
            let code = result.err().unwrap().code;
            prop_assert_eq!(code, ErrorCode::Empty);
        }

        #[test]
        fn i16_trailing_digits_proptest(i in r"[+-]?[0-9]{4}\D[0-9]{2}") {
            let result = i16::from_lexical(i.as_bytes());
            let error = result.err().unwrap();
            prop_assert_eq!(error.code, ErrorCode::InvalidDigit);
            prop_assert!(error.index == 4 || error.index == 5);
        }

        #[test]
        fn u32_invalid_proptest(i in r"[+]?[0-9]{9}\D") {
            let result = u32::from_lexical(i.as_bytes());
            let error = result.err().unwrap();
            prop_assert_eq!(error.code, ErrorCode::InvalidDigit);
            prop_assert!(error.index == 9 || error.index == 10);
        }

        #[test]
        fn u32_overflow_proptest(i in r"[+]?[1-9][0-9]{10}\D") {
            let result = u32::from_lexical(i.as_bytes());
            let code = result.err().unwrap().code;
            prop_assert_eq!(code, ErrorCode::Overflow);
        }

        #[test]
        fn u32_negative_proptest(i in r"[-][1-9][0-9]{9}") {
            let result = u32::from_lexical(i.as_bytes());
            prop_assert!(result.is_err());
            let code = result.err().unwrap().code;
            prop_assert_eq!(code, ErrorCode::InvalidDigit);
        }

        #[test]
        fn u32_double_sign_proptest(i in r"[+]{2}[0-9]{9}") {
            let result = u32::from_lexical(i.as_bytes());
            prop_assert!(result.is_err());
            let error = result.err().unwrap();
            prop_assert_eq!(error.code, ErrorCode::InvalidDigit);
            prop_assert!(error.index == 1);
        }

        #[test]
        fn u32_sign_only_proptest(i in r"[+]") {
            let result = u32::from_lexical(i.as_bytes());
            prop_assert!(result.is_err());
            let code = result.err().unwrap().code;
            prop_assert_eq!(code, ErrorCode::Empty);
        }

        #[test]
        fn u32_trailing_digits_proptest(i in r"[+]?[0-9]{9}\D[0-9]{2}") {
            let result = u32::from_lexical(i.as_bytes());
            prop_assert!(result.is_err());
            let error = result.err().unwrap();
            prop_assert_eq!(error.code, ErrorCode::InvalidDigit);
            prop_assert!(error.index == 9 || error.index == 10);
        }

        #[test]
        fn i32_invalid_proptest(i in r"[+-]?[0-9]{9}\D") {
            let result = i32::from_lexical(i.as_bytes());
            prop_assert!(result.is_err());
            let error = result.err().unwrap();
            prop_assert_eq!(error.code, ErrorCode::InvalidDigit);
            prop_assert!(error.index == 9 || error.index == 10);
        }

        #[test]
        fn i32_overflow_proptest(i in r"[+]?[1-9][0-9]{10}\D") {
            let result = i32::from_lexical(i.as_bytes());
            let code = result.err().unwrap().code;
            prop_assert_eq!(code, ErrorCode::Overflow);
        }

        #[test]
        fn i32_underflow_proptest(i in r"-[1-9][0-9]{10}\D") {
            let result = i32::from_lexical(i.as_bytes());
            let code = result.err().unwrap().code;
            prop_assert_eq!(code, ErrorCode::Underflow);
        }

        #[test]
        fn i32_double_sign_proptest(i in r"[+-]{2}[0-9]{9}") {
            let result = i32::from_lexical(i.as_bytes());
            let error = result.err().unwrap();
            prop_assert_eq!(error.code, ErrorCode::InvalidDigit);
            prop_assert!(error.index == 1);
        }

        #[test]
        fn i32_sign_only_proptest(i in r"[+-]") {
            let result = i32::from_lexical(i.as_bytes());
            prop_assert!(result.is_err());
            let code = result.err().unwrap().code;
            prop_assert_eq!(code, ErrorCode::Empty);
        }

        #[test]
        fn i32_trailing_digits_proptest(i in r"[+-]?[0-9]{9}\D[0-9]{2}") {
            let result = i32::from_lexical(i.as_bytes());
            let error = result.err().unwrap();
            prop_assert_eq!(error.code, ErrorCode::InvalidDigit);
            prop_assert!(error.index == 9 || error.index == 10);
        }

        #[test]
        fn u64_invalid_proptest(i in r"[+]?[0-9]{19}\D") {
            let result = u64::from_lexical(i.as_bytes());
            let error = result.err().unwrap();
            prop_assert_eq!(error.code, ErrorCode::InvalidDigit);
            prop_assert!(error.index == 19 || error.index == 20);
        }

        #[test]
        fn u64_overflow_proptest(i in r"[+]?[1-9][0-9]{21}\D") {
            let result = u64::from_lexical(i.as_bytes());
            let code = result.err().unwrap().code;
            prop_assert_eq!(code, ErrorCode::Overflow);
        }

        #[test]
        fn u64_negative_proptest(i in r"[-][1-9][0-9]{21}") {
            let result = u64::from_lexical(i.as_bytes());
            prop_assert!(result.is_err());
            let code = result.err().unwrap().code;
            prop_assert_eq!(code, ErrorCode::InvalidDigit);
        }

        #[test]
        fn u64_double_sign_proptest(i in r"[+]{2}[0-9]{19}") {
            let result = u64::from_lexical(i.as_bytes());
            prop_assert!(result.is_err());
            let error = result.err().unwrap();
            prop_assert_eq!(error.code, ErrorCode::InvalidDigit);
            prop_assert!(error.index == 1);
        }

        #[test]
        fn u64_sign_only_proptest(i in r"[+]") {
            let result = u64::from_lexical(i.as_bytes());
            prop_assert!(result.is_err());
            let code = result.err().unwrap().code;
            prop_assert_eq!(code, ErrorCode::Empty);
        }

        #[test]
        fn u64_trailing_digits_proptest(i in r"[+]?[0-9]{19}\D[0-9]{2}") {
            let result = u64::from_lexical(i.as_bytes());
            prop_assert!(result.is_err());
            let error = result.err().unwrap();
            prop_assert_eq!(error.code, ErrorCode::InvalidDigit);
            prop_assert!(error.index == 19 || error.index == 20);
        }

        #[test]
        fn i64_invalid_proptest(i in r"[+-]?[0-9]{18}\D") {
            let result = i64::from_lexical(i.as_bytes());
            prop_assert!(result.is_err());
            let error = result.err().unwrap();
            prop_assert_eq!(error.code, ErrorCode::InvalidDigit);
            prop_assert!(error.index == 18 || error.index == 19);
        }

        #[test]
        fn i64_overflow_proptest(i in r"[+]?[1-9][0-9]{19}\D") {
            let result = i64::from_lexical(i.as_bytes());
            let code = result.err().unwrap().code;
            prop_assert_eq!(code, ErrorCode::Overflow);
        }

        #[test]
        fn i64_underflow_proptest(i in r"-[1-9][0-9]{19}\D") {
            let result = i64::from_lexical(i.as_bytes());
            let code = result.err().unwrap().code;
            prop_assert_eq!(code, ErrorCode::Underflow);
        }

        #[test]
        fn i64_double_sign_proptest(i in r"[+-]{2}[0-9]{18}") {
            let result = i64::from_lexical(i.as_bytes());
            let error = result.err().unwrap();
            prop_assert_eq!(error.code, ErrorCode::InvalidDigit);
            prop_assert!(error.index == 1);
        }

        #[test]
        fn i64_sign_only_proptest(i in r"[+-]") {
            let result = i32::from_lexical(i.as_bytes());
            prop_assert!(result.is_err());
            let code = result.err().unwrap().code;
            prop_assert_eq!(code, ErrorCode::Empty);
        }

        #[test]
        fn i64_trailing_digits_proptest(i in r"[+-]?[0-9]{18}\D[0-9]{2}") {
            let result = i64::from_lexical(i.as_bytes());
            let error = result.err().unwrap();
            prop_assert_eq!(error.code, ErrorCode::InvalidDigit);
            prop_assert!(error.index == 18 || error.index == 19);
        }
    }
}
