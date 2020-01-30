//! Low-level API generator.
//!
//! Uses either the imprecise or the precise algorithm.

use crate::lib::slice;
use crate::util::*;

// Select the back-end
cfg_if! {
if #[cfg(feature = "correct")] {
    use super::algorithm::correct as algorithm;
} else {
    use super::algorithm::incorrect as algorithm;
}}  // cfg_if

// TRAITS

/// Trait to define parsing of a string to float.
trait StringToFloat: Float {
    /// Serialize string to float, favoring correctness.
    fn default(bytes: &[u8], options: &ParseFloatOptions, sign: Sign) -> ParseResult<(Self, *const u8)>;
}

impl StringToFloat for f32 {
    perftools_inline_always!{
    fn default(bytes: &[u8], options: &ParseFloatOptions, sign: Sign)
        -> ParseResult<(f32, *const u8)>
    {
        algorithm::atof(bytes, options, sign)
    }}
}

impl StringToFloat for f64 {
    perftools_inline_always!{
    fn default(bytes: &[u8], options: &ParseFloatOptions, sign: Sign)
        -> ParseResult<(f64, *const u8)>
    {
        algorithm::atod(bytes, options, sign)
    }}
}

// SPECIAL
// Utilities to filter special values.

// Convert slice to iterator without digit separators.
perftools_inline!{
fn to_iter<'a>(bytes: &'a [u8], _: u8) -> slice::Iter<'a, u8> {
    bytes.iter()
}}

// Convert slice to iterator with digit separators.
perftools_inline!{
#[cfg(feature = "format")]
fn to_iter_s<'a>(bytes: &'a [u8], digit_separator: u8) -> SkipValueIterator<'a, u8> {
    SkipValueIterator::new(bytes, digit_separator)
}}

// PARSER

// Parse infinity from string.
perftools_inline!{
fn parse_infinity<'a, ToIter, StartsWith, Iter, F>(
    bytes: &'a [u8],
    options: &ParseFloatOptions,
    sign: Sign,
    to_iter: ToIter,
    starts_with: StartsWith
)
    -> ParseResult<(F, *const u8)>
    where F: StringToFloat,
          ToIter: Fn(&'a [u8], u8) -> Iter,
          Iter: AsPtrIterator<'a, u8>,
          StartsWith: Fn(Iter, slice::Iter<'a, u8>) -> (bool, Iter)
{
    let bytes_iter = || to_iter(bytes, options.format().digit_separator());
    let inf_iter = options.inf_string().iter();
    let infinity_iter = options.infinity_string().iter();
    if let (true, iter) = starts_with(bytes_iter(), infinity_iter) {
        Ok((F::INFINITY, iter.as_ptr()))
    } else if let (true, iter) = starts_with(bytes_iter(), inf_iter) {
        Ok((F::INFINITY, iter.as_ptr()))
    } else {
        // Not infinity, may be valid with a different radix.
        if cfg!(feature = "radix"){
            F::default(bytes, options, sign)
        } else {
            Err((ErrorCode::InvalidDigit, bytes.as_ptr()))
        }
    }
}}

// Parse NaN from string.
perftools_inline!{
fn parse_nan<'a, ToIter, StartsWith, Iter, F>(
    bytes: &'a [u8],
    options: &ParseFloatOptions,
    sign: Sign,
    to_iter: ToIter,
    starts_with: StartsWith
)
-> ParseResult<(F, *const u8)>
    where F: StringToFloat,
          ToIter: Fn(&'a [u8], u8) -> Iter,
          Iter: AsPtrIterator<'a, u8>,
          StartsWith: Fn(Iter, slice::Iter<'a, u8>) -> (bool, Iter)
{
    let bytes_iter = || to_iter(bytes, options.format().digit_separator());
    let nan_iter = options.nan_string().iter();
    if let (true, iter) = starts_with(bytes_iter(), nan_iter) {
        Ok((F::NAN, iter.as_ptr()))
    } else {
        // Not NaN, may be valid with a different radix.
        if cfg!(feature = "radix"){
            F::default(bytes, options, sign)
        } else {
            Err((ErrorCode::InvalidDigit, bytes.as_ptr()))
        }
    }
}}

// ATOF/ATOD

// Parse special or float values with the standard format.
// Special values are allowed, the match is case-insensitive,
// and no digit separators are allowed.
perftools_inline!{
fn parse_float_standard<F: StringToFloat>(bytes: &[u8], options: &ParseFloatOptions, sign: Sign)
    -> ParseResult<(F, *const u8)>
{
    // Use predictive parsing to filter special cases. This leads to
    // dramatic performance gains.
    let starts_with = case_insensitive_starts_with_iter;
    match index!(bytes[0]) {
        b'i' | b'I' => parse_infinity(bytes, options, sign, to_iter, starts_with),
        b'N' | b'n' => parse_nan(bytes, options, sign, to_iter, starts_with),
        _           => F::default(bytes, options, sign),
    }
}}

// Parse special or float values.
// Special values are allowed, the match is case-sensitive,
// and digit separators are allowed.
perftools_inline!{
#[cfg(feature = "format")]
fn parse_float_cs<F: StringToFloat>(bytes: &[u8], options: &ParseFloatOptions, sign: Sign)
    -> ParseResult<(F, *const u8)>
{
    let digit_separator = options.format().digit_separator();
    let starts_with = starts_with_iter;
    match SkipValueIterator::new(bytes, digit_separator).next()  {
        Some(&b'i') | Some(&b'I')   => parse_infinity(bytes, options, sign, to_iter_s, starts_with),
        Some(&b'n') | Some(&b'N')   => parse_nan(bytes, options, sign, to_iter_s, starts_with),
        _                           => F::default(bytes, options, sign),
    }
}}

// Parse special or float values.
// Special values are allowed, the match is case-sensitive,
// and no digit separators are allowed.
perftools_inline!{
#[cfg(feature = "format")]
fn parse_float_c<F: StringToFloat>(bytes: &[u8], options: &ParseFloatOptions, sign: Sign)
    -> ParseResult<(F, *const u8)>
{
    // Use predictive parsing to filter special cases. This leads to
    // dramatic performance gains.
    let starts_with = starts_with_iter;
    match index!(bytes[0]) {
        b'i' | b'I' => parse_infinity(bytes, options, sign, to_iter, starts_with),
        b'N' | b'n' => parse_nan(bytes, options, sign, to_iter, starts_with),
        _           => F::default(bytes, options, sign),
    }
}}

// Parse special or float values.
// Special values are allowed, the match is case-insensitive,
// and digit separators are allowed.
perftools_inline!{
#[cfg(feature = "format")]
fn parse_float_s<F: StringToFloat>(bytes: &[u8], options: &ParseFloatOptions, sign: Sign)
    -> ParseResult<(F, *const u8)>
{
    let digit_separator = options.format().digit_separator();
    let starts_with = case_insensitive_starts_with_iter;
    match SkipValueIterator::new(bytes, digit_separator).next()  {
        Some(&b'i') | Some(&b'I')   => parse_infinity(bytes, options, sign, to_iter_s, starts_with),
        Some(&b'n') | Some(&b'N')   => parse_nan(bytes, options, sign, to_iter_s, starts_with),
        _                           => F::default(bytes, options, sign),
    }
}}

// Parse special or float values with the default formatter.
perftools_inline!{
#[cfg(not(feature = "format"))]
fn parse_float<F: StringToFloat>(bytes: &[u8], options: &ParseFloatOptions, sign: Sign)
    -> ParseResult<(F, *const u8)>
{
    parse_float_standard(bytes, options, sign)
}}

// Parse special or float values with the default formatter.
perftools_inline!{
#[cfg(feature = "format")]
fn parse_float<F: StringToFloat>(bytes: &[u8], options: &ParseFloatOptions, sign: Sign)
    -> ParseResult<(F, *const u8)>
{
    // Need to consider 3 possibilities:
    //  1). No special values are allowed.
    //  2). Special values are case-sensitive.
    //  3). Digit separators are allowed in the special.
    let no_special = options.format().no_special();
    let case = options.format().case_sensitive_special();
    let has_sep = options.format().special_digit_separator();
    match (no_special, case, has_sep) {
        (true, _, _)            => F::default(bytes, options, sign),
        (false, true, true)     => parse_float_cs(bytes, options, sign),
        (false, false, true)    => parse_float_s(bytes, options, sign),
        (false, true, false)    => parse_float_c(bytes, options, sign),
        (false, false, false)   => parse_float_standard(bytes, options, sign),
    }
}}

// Validate sign byte is valid.
perftools_inline!{
#[cfg(not(feature = "format"))]
fn validate_sign(_: &[u8], _: &[u8], _: Sign, _: NumberFormat)
    -> ParseResult<()>
{
    Ok(())
}}

// Validate sign byte is valid.
perftools_inline!{
#[cfg(feature = "format")]
fn validate_sign(bytes: &[u8], digits: &[u8], sign: Sign, format: NumberFormat)
    -> ParseResult<()>
{
    let has_sign = bytes.as_ptr() != digits.as_ptr();
    if format.no_positive_mantissa_sign() && has_sign && sign == Sign::Positive {
        Err((ErrorCode::InvalidPositiveMantissaSign, bytes.as_ptr()))
    } else if format.required_mantissa_sign() && !has_sign {
        Err((ErrorCode::MissingMantissaSign, bytes.as_ptr()))
    } else {
        Ok(())
    }
}}

// Convert float to signed representation.
perftools_inline!{
fn to_signed<F: StringToFloat>(float: F, sign: Sign) -> F
{
    match sign {
        Sign::Positive => float,
        Sign::Negative => -float
    }
}}

// Standalone atof processor.
perftools_inline!{
fn atof<F: StringToFloat>(bytes: &[u8], options: &ParseFloatOptions)
    -> ParseResult<(F, *const u8)>
{
    let (sign, digits) = parse_sign::<F>(bytes, options.format());
    if digits.is_empty() {
        return Err((ErrorCode::Empty, digits.as_ptr()));
    }
    let (float, ptr): (F, *const u8) = parse_float(digits, options, sign)?;
    validate_sign(bytes, digits, sign, options.format())?;

    Ok((to_signed(float, sign), ptr))
}}

perftools_inline!{
fn parse<F: StringToFloat>(bytes: &[u8])
    -> Result<(F, usize)>
{
    let options = ParseFloatOptions::default();
    let index = | ptr | distance(bytes.as_ptr(), ptr);
    match atof::<F>(bytes, &options) {
        Ok((value, ptr)) => Ok((value, index(ptr))),
        Err((code, ptr)) => Err((code, index(ptr)).into()),
    }
}}

perftools_inline!{
fn parse_with_options<F: StringToFloat>(bytes: &[u8], options: &ParseFloatOptions)
    -> Result<(F, usize)>
{
    let index = | ptr | distance(bytes.as_ptr(), ptr);
    match atof::<F>(bytes, options) {
        Ok((value, ptr)) => Ok((value, index(ptr))),
        Err((code, ptr)) => Err((code, index(ptr)).into()),
    }
}}

// FROM LEXICAL
// ------------

macro_rules! atof_from_lexical {
    ($($type:ident)*) => ($(
        from_lexical!(
            type => $type,
            options => ParseFloatOptions,
            parse => parse,
            parse_with_options => parse_with_options
        );
    )*);
}

macro_rules! atof_from_lexical_lossy {
    ($($type:ident)*) => ($(
        from_lexical_lossy!(
            type => $type,
            options => ParseFloatOptions,
            parse_with_options => parse_with_options
        );
    )*);
}

atof_from_lexical! { f32 f64 }
atof_from_lexical_lossy! { f32 f64 }

// TESTS
// -----

#[cfg(test)]
mod tests {
    use crate::util::*;

    use approx::assert_relative_eq;
    #[cfg(all(feature = "std", feature = "property_tests"))]
    use proptest::{proptest, prop_assert_eq, prop_assert};

    #[test]
    fn f32_decimal_test() {
        // integer test
        assert_f32_eq!(0.0, f32::from_lexical(b"0").unwrap());
        assert_f32_eq!(1.0, f32::from_lexical(b"1").unwrap());
        assert_f32_eq!(12.0, f32::from_lexical(b"12").unwrap());
        assert_f32_eq!(123.0, f32::from_lexical(b"123").unwrap());
        assert_f32_eq!(1234.0, f32::from_lexical(b"1234").unwrap());
        assert_f32_eq!(12345.0, f32::from_lexical(b"12345").unwrap());
        assert_f32_eq!(123456.0, f32::from_lexical(b"123456").unwrap());
        assert_f32_eq!(1234567.0, f32::from_lexical(b"1234567").unwrap());
        assert_f32_eq!(12345678.0, f32::from_lexical(b"12345678").unwrap());

         // No fraction after decimal point test
        assert_f32_eq!(1.0, f32::from_lexical(b"1.").unwrap());
        assert_f32_eq!(12.0, f32::from_lexical(b"12.").unwrap());
        assert_f32_eq!(1234567.0, f32::from_lexical(b"1234567.").unwrap());

        // No integer before decimal point test
        assert_f32_eq!(0.1, f32::from_lexical(b".1").unwrap());
        assert_f32_eq!(0.12, f32::from_lexical(b".12").unwrap());
        assert_f32_eq!(0.1234567, f32::from_lexical(b".1234567").unwrap());

        // decimal test
        assert_f32_eq!(123.1, f32::from_lexical(b"123.1").unwrap());
        assert_f32_eq!(123.12, f32::from_lexical(b"123.12").unwrap());
        assert_f32_eq!(123.123, f32::from_lexical(b"123.123").unwrap());
        assert_f32_eq!(123.1234, f32::from_lexical(b"123.1234").unwrap());
        assert_f32_eq!(123.12345, f32::from_lexical(b"123.12345").unwrap());

        // rounding test
        assert_f32_eq!(123456790.0, f32::from_lexical(b"123456789").unwrap());
        assert_f32_eq!(123456790.0, f32::from_lexical(b"123456789.1").unwrap());
        assert_f32_eq!(123456790.0, f32::from_lexical(b"123456789.12").unwrap());
        assert_f32_eq!(123456790.0, f32::from_lexical(b"123456789.123").unwrap());
        assert_f32_eq!(123456790.0, f32::from_lexical(b"123456789.1234").unwrap());
        assert_f32_eq!(123456790.0, f32::from_lexical(b"123456789.12345").unwrap());

        // exponent test
        assert_f32_eq!(123456789.12345, f32::from_lexical(b"1.2345678912345e8").unwrap());
        assert_f32_eq!(123450000.0, f32::from_lexical(b"1.2345e+8").unwrap());
        assert_f32_eq!(1.2345e+11, f32::from_lexical(b"1.2345e+11").unwrap());
        assert_f32_eq!(1.2345e+11, f32::from_lexical(b"123450000000").unwrap());
        assert_f32_eq!(1.2345e+38, f32::from_lexical(b"1.2345e+38").unwrap());
        assert_f32_eq!(1.2345e+38, f32::from_lexical(b"123450000000000000000000000000000000000").unwrap());
        assert_f32_eq!(1.2345e-8, f32::from_lexical(b"1.2345e-8").unwrap());
        assert_f32_eq!(1.2345e-8, f32::from_lexical(b"0.000000012345").unwrap());
        assert_f32_eq!(1.2345e-38, f32::from_lexical(b"1.2345e-38").unwrap());
        assert_f32_eq!(1.2345e-38, f32::from_lexical(b"0.000000000000000000000000000000000000012345").unwrap());

        assert!(f32::from_lexical(b"NaN").unwrap().is_nan());
        assert!(f32::from_lexical(b"nan").unwrap().is_nan());
        assert!(f32::from_lexical(b"NAN").unwrap().is_nan());
        assert!(f32::from_lexical(b"inf").unwrap().is_infinite());
        assert!(f32::from_lexical(b"INF").unwrap().is_infinite());
        assert!(f32::from_lexical(b"+inf").unwrap().is_infinite());
        assert!(f32::from_lexical(b"-inf").unwrap().is_infinite());

        // Check various expected failures.
        assert_eq!(Err(ErrorCode::Empty.into()), f32::from_lexical(b""));
        assert_eq!(Err((ErrorCode::EmptyMantissa, 0).into()), f32::from_lexical(b"e"));
        assert_eq!(Err((ErrorCode::EmptyMantissa, 0).into()), f32::from_lexical(b"E"));
        assert_eq!(Err(ErrorCode::EmptyMantissa.into()), f32::from_lexical(b".e1"));
        assert_eq!(Err(ErrorCode::EmptyMantissa.into()), f32::from_lexical(b".e-1"));
        assert_eq!(Err((ErrorCode::EmptyMantissa, 0).into()), f32::from_lexical(b"e1"));
        assert_eq!(Err((ErrorCode::EmptyMantissa, 0).into()), f32::from_lexical(b"e-1"));
        assert_eq!(Err((ErrorCode::Empty, 1).into()), f32::from_lexical(b"+"));
        assert_eq!(Err((ErrorCode::Empty, 1).into()), f32::from_lexical(b"-"));

        // Bug fix for Issue #8
        assert_eq!(Ok(5.002868148396374), f32::from_lexical(b"5.002868148396374"));
    }

    #[test]
    #[cfg(feature = "radix")]
    fn f32_radix_test() {
        let options = ParseFloatOptions::builder()
            .exponent_char(b'^')
            .radix(36)
            .build()
            .unwrap();
        assert_f32_eq!(1234.0, f32::from_lexical_with_options(b"YA", &options).unwrap());

        let options = ParseFloatOptions::builder()
            .lossy(true)
            .exponent_char(b'^')
            .radix(36)
            .build()
            .unwrap();
        assert_f32_eq!(1234.0, f32::from_lexical_with_options(b"YA", &options).unwrap());
    }

    #[test]
    fn f64_decimal_test() {
        // integer test
        assert_f64_eq!(0.0, f64::from_lexical(b"0").unwrap());
        assert_f64_eq!(1.0, f64::from_lexical(b"1").unwrap());
        assert_f64_eq!(12.0, f64::from_lexical(b"12").unwrap());
        assert_f64_eq!(123.0, f64::from_lexical(b"123").unwrap());
        assert_f64_eq!(1234.0, f64::from_lexical(b"1234").unwrap());
        assert_f64_eq!(12345.0, f64::from_lexical(b"12345").unwrap());
        assert_f64_eq!(123456.0, f64::from_lexical(b"123456").unwrap());
        assert_f64_eq!(1234567.0, f64::from_lexical(b"1234567").unwrap());
        assert_f64_eq!(12345678.0, f64::from_lexical(b"12345678").unwrap());

        // No fraction after decimal point test
        assert_f64_eq!(1.0, f64::from_lexical(b"1.").unwrap());
        assert_f64_eq!(12.0, f64::from_lexical(b"12.").unwrap());
        assert_f64_eq!(1234567.0, f64::from_lexical(b"1234567.").unwrap());

        // No integer before decimal point test
        assert_f64_eq!(0.1, f64::from_lexical(b".1").unwrap());
        assert_f64_eq!(0.12, f64::from_lexical(b".12").unwrap());
        assert_f64_eq!(0.1234567, f64::from_lexical(b".1234567").unwrap());

        // decimal test
        assert_f64_eq!(123456789.0, f64::from_lexical(b"123456789").unwrap());
        assert_f64_eq!(123456789.1, f64::from_lexical(b"123456789.1").unwrap());
        assert_f64_eq!(123456789.12, f64::from_lexical(b"123456789.12").unwrap());
        assert_f64_eq!(123456789.123, f64::from_lexical(b"123456789.123").unwrap());
        assert_f64_eq!(123456789.1234, f64::from_lexical(b"123456789.1234").unwrap());
        assert_f64_eq!(123456789.12345, f64::from_lexical(b"123456789.12345").unwrap());
        assert_f64_eq!(123456789.123456, f64::from_lexical(b"123456789.123456").unwrap());
        assert_f64_eq!(123456789.1234567, f64::from_lexical(b"123456789.1234567").unwrap());
        assert_f64_eq!(123456789.12345678, f64::from_lexical(b"123456789.12345678").unwrap());

        // rounding test
        assert_f64_eq!(123456789.12345679, f64::from_lexical(b"123456789.123456789").unwrap());
        assert_f64_eq!(123456789.12345679, f64::from_lexical(b"123456789.1234567890").unwrap());
        assert_f64_eq!(123456789.12345679, f64::from_lexical(b"123456789.123456789012").unwrap());
        assert_f64_eq!(123456789.12345679, f64::from_lexical(b"123456789.1234567890123").unwrap());
        assert_f64_eq!(123456789.12345679, f64::from_lexical(b"123456789.12345678901234").unwrap());

        // exponent test
        assert_f64_eq!(123456789.12345, f64::from_lexical(b"1.2345678912345e8").unwrap());
        assert_f64_eq!(123450000.0, f64::from_lexical(b"1.2345e+8").unwrap());
        assert_f64_eq!(1.2345e+11, f64::from_lexical(b"123450000000").unwrap());
        assert_f64_eq!(1.2345e+11, f64::from_lexical(b"1.2345e+11").unwrap());
        assert_f64_eq!(1.2345e+38, f64::from_lexical(b"1.2345e+38").unwrap());
        assert_f64_eq!(1.2345e+38, f64::from_lexical(b"123450000000000000000000000000000000000").unwrap());
        assert_f64_eq!(1.2345e+308, f64::from_lexical(b"1.2345e+308").unwrap());
        assert_f64_eq!(1.2345e+308, f64::from_lexical(b"123450000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000").unwrap());
        assert_f64_eq!(0.000000012345, f64::from_lexical(b"1.2345e-8").unwrap());
        assert_f64_eq!(1.2345e-8, f64::from_lexical(b"0.000000012345").unwrap());
        assert_f64_eq!(1.2345e-38, f64::from_lexical(b"1.2345e-38").unwrap());
        assert_f64_eq!(1.2345e-38, f64::from_lexical(b"0.000000000000000000000000000000000000012345").unwrap());

        // denormalized (try extremely low values)
        assert_f64_eq!(1.2345e-308, f64::from_lexical(b"1.2345e-308").unwrap());
        // These next 3 tests fail on arm-unknown-linux-gnueabi with the
        // incorrect parser.
        #[cfg(all(not(feature = "correct"), not(target_arch = "arm")))]
        assert_eq!(Ok(5e-322), f64::from_lexical(b"5e-322"));
        #[cfg(all(not(feature = "correct"), not(target_arch = "arm")))]
        assert_eq!(Ok(5e-323), f64::from_lexical(b"5e-323"));
        #[cfg(all(not(feature = "correct"), not(target_arch = "arm")))]
        assert_eq!(Ok(5e-324), f64::from_lexical(b"5e-324"));
        // due to issues in how the data is parsed, manually extracting
        // non-exponents of 1.<e-299 is prone to error
        // test the limit of our ability
        // We tend to get relative errors of 1e-16, even at super low values.
        assert_f64_eq!(1.2345e-299, f64::from_lexical(b"0.000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000012345").unwrap(), epsilon=1e-314);

        // Keep pushing from -300 to -324
        assert_f64_eq!(1.2345e-300, f64::from_lexical(b"0.0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000012345").unwrap(), epsilon=1e-315);

        // These next 3 tests fail on arm-unknown-linux-gnueabi with the
        // incorrect parser.
        #[cfg(all(not(feature = "correct"), not(target_arch = "arm")))]
        assert_f64_eq!(1.2345e-310, f64::from_lexical(b"0.00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000012345").unwrap(), epsilon=5e-324);
        #[cfg(all(not(feature = "correct"), not(target_arch = "arm")))]
        assert_f64_eq!(1.2345e-320, f64::from_lexical(b"0.000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000012345").unwrap(), epsilon=5e-324);
        #[cfg(all(not(feature = "correct"), not(target_arch = "arm")))]
        assert_f64_eq!(1.2345e-321, f64::from_lexical(b"0.0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000012345").unwrap(), epsilon=5e-324);
        #[cfg(all(not(feature = "correct"), not(target_arch = "arm")))]
        assert_f64_eq!(1.24e-322, f64::from_lexical(b"0.000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000124").unwrap(), epsilon=5e-324);
        #[cfg(all(not(feature = "correct"), not(target_arch = "arm")))]
        assert_eq!(Ok(1e-323), f64::from_lexical(b"0.00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001"));
        #[cfg(all(not(feature = "correct"), not(target_arch = "arm")))]
        assert_eq!(Ok(5e-324), f64::from_lexical(b"0.000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000005"));

        assert!(f64::from_lexical(b"NaN").unwrap().is_nan());
        assert!(f64::from_lexical(b"nan").unwrap().is_nan());
        assert!(f64::from_lexical(b"NAN").unwrap().is_nan());
        assert!(f64::from_lexical(b"inf").unwrap().is_infinite());
        assert!(f64::from_lexical(b"INF").unwrap().is_infinite());
        assert!(f64::from_lexical(b"+inf").unwrap().is_infinite());
        assert!(f64::from_lexical(b"-inf").unwrap().is_infinite());

        // Check various expected failures.
        assert_eq!(Err(ErrorCode::Empty.into()), f64::from_lexical(b""));
        assert_eq!(Err((ErrorCode::EmptyMantissa, 0).into()), f64::from_lexical(b"e"));
        assert_eq!(Err((ErrorCode::EmptyMantissa, 0).into()), f64::from_lexical(b"E"));
        assert_eq!(Err(ErrorCode::EmptyMantissa.into()), f64::from_lexical(b".e1"));
        assert_eq!(Err(ErrorCode::EmptyMantissa.into()), f64::from_lexical(b".e-1"));
        assert_eq!(Err((ErrorCode::EmptyMantissa, 0).into()), f64::from_lexical(b"e1"));
        assert_eq!(Err((ErrorCode::EmptyMantissa, 0).into()), f64::from_lexical(b"e-1"));

        // Check various reports from a fuzzer.
        assert_eq!(Err((ErrorCode::EmptyExponent, 2).into()), f64::from_lexical(b"0e"));
        assert_eq!(Err((ErrorCode::EmptyExponent, 4).into()), f64::from_lexical(b"0.0e"));
        assert_eq!(Err((ErrorCode::EmptyMantissa, 0).into()), f64::from_lexical(b".E"));
        assert_eq!(Err((ErrorCode::EmptyMantissa, 0).into()), f64::from_lexical(b".e"));
        assert_eq!(Err((ErrorCode::EmptyMantissa, 0).into()), f64::from_lexical(b"E2252525225"));
        assert_eq!(Err((ErrorCode::EmptyMantissa, 0).into()), f64::from_lexical(b"e2252525225"));
        assert_eq!(Ok(f64::INFINITY), f64::from_lexical(b"2E200000000000"));

        // Add various unittests from proptests.
        assert_eq!(Err((ErrorCode::EmptyExponent, 2).into()), f64::from_lexical(b"0e"));
        assert_eq!(Err((ErrorCode::EmptyMantissa, 0).into()), f64::from_lexical(b"."));
        assert_eq!(Err((ErrorCode::EmptyMantissa, 1).into()), f64::from_lexical(b"+."));
        assert_eq!(Err((ErrorCode::EmptyMantissa, 1).into()), f64::from_lexical(b"-."));
        assert_eq!(Err((ErrorCode::Empty, 1).into()), f64::from_lexical(b"+"));
        assert_eq!(Err((ErrorCode::Empty, 1).into()), f64::from_lexical(b"-"));

        // Bug fix for Issue #8
        assert_eq!(Ok(5.002868148396374), f64::from_lexical(b"5.002868148396374"));
    }

    #[test]
    #[should_panic]
    fn limit_test() {
        assert_relative_eq!(1.2345e-320, 0.0, epsilon=5e-324);
    }

    #[test]
    #[cfg(feature = "radix")]
    fn f64_radix_test() {
        let options = ParseFloatOptions::builder()
            .exponent_char(b'^')
            .radix(36)
            .build()
            .unwrap();
        assert_f64_eq!(1234.0, f64::from_lexical_with_options(b"YA", &options).unwrap());

        let options = ParseFloatOptions::builder()
            .lossy(true)
            .exponent_char(b'^')
            .radix(36)
            .build()
            .unwrap();
        assert_f64_eq!(1234.0, f64::from_lexical_with_options(b"YA", &options).unwrap());
    }

    #[test]
    fn f32_lossy_decimal_test() {
        let options = ParseFloatOptions::builder()
            .lossy(true)
            .build()
            .unwrap();
        assert_eq!(Err(ErrorCode::EmptyMantissa.into()), f32::from_lexical_with_options(b".", &options));
        assert_eq!(Err(ErrorCode::Empty.into()), f32::from_lexical_with_options(b"", &options));
        assert_eq!(Ok(0.0), f32::from_lexical_with_options(b"0.0", &options));
        assert_eq!(Err((ErrorCode::InvalidDigit, 1).into()), f32::from_lexical_with_options(b"1a", &options));

        // Bug fix for Issue #8
        assert_eq!(Ok(5.002868148396374), f32::from_lexical_with_options(b"5.002868148396374", &options));
    }

    #[test]
    fn f64_lossy_decimal_test() {
        let options = ParseFloatOptions::builder()
            .lossy(true)
            .build()
            .unwrap();
        assert_eq!(Err(ErrorCode::EmptyMantissa.into()), f64::from_lexical_with_options(b".", &options));
        assert_eq!(Err(ErrorCode::Empty.into()), f64::from_lexical_with_options(b"", &options));
        assert_eq!(Ok(0.0), f64::from_lexical_with_options(b"0.0", &options));
        assert_eq!(Err((ErrorCode::InvalidDigit, 1).into()), f64::from_lexical_with_options(b"1a", &options));

        // Bug fix for Issue #8
        assert_eq!(Ok(5.002868148396374), f64::from_lexical_with_options(b"5.002868148396374", &options));
    }

    #[test]
    #[cfg(feature = "format")]
    fn f64_special_test() {
        //  Comments match (no_special, case_sensitive, has_sep)
        let f1 = NumberFormat::standard().unwrap();         // false, false, false
        let f2 = NumberFormat::ignore(b'_').unwrap();       // false, false, true
        let f3 = f1 | NumberFormat::NO_SPECIAL;             // true, _, _
        let f4 = f1 | NumberFormat::CASE_SENSITIVE_SPECIAL; // false, true, false
        let f5 = f2 | NumberFormat::CASE_SENSITIVE_SPECIAL; // false, true, true

        // Create options from format.
        let o1 = ParseFloatOptions::builder().format(f1).build().unwrap();
        let o2 = ParseFloatOptions::builder().format(f2).build().unwrap();
        let o3 = ParseFloatOptions::builder().format(f3).build().unwrap();
        let o4 = ParseFloatOptions::builder().format(f4).build().unwrap();
        let o5 = ParseFloatOptions::builder().format(f5).build().unwrap();

        // Easy NaN
        assert!(f64::from_lexical_with_options(b"NaN", &o1).unwrap().is_nan());
        assert!(f64::from_lexical_with_options(b"NaN", &o2).unwrap().is_nan());
        assert!(f64::from_lexical_with_options(b"NaN", &o3).is_err());
        assert!(f64::from_lexical_with_options(b"NaN", &o4).unwrap().is_nan());
        assert!(f64::from_lexical_with_options(b"NaN", &o5).unwrap().is_nan());

        // Case-sensitive NaN.
        assert!(f64::from_lexical_with_options(b"nan", &o1).unwrap().is_nan());
        assert!(f64::from_lexical_with_options(b"nan", &o2).unwrap().is_nan());
        assert!(f64::from_lexical_with_options(b"nan", &o3).is_err());
        assert!(f64::from_lexical_with_options(b"nan", &o4).is_err());
        assert!(f64::from_lexical_with_options(b"nan", &o5).is_err());

        // Digit-separator NaN.
        assert!(f64::from_lexical_with_options(b"N_aN", &o1).is_err());
        assert!(f64::from_lexical_with_options(b"N_aN", &o2).unwrap().is_nan());
        assert!(f64::from_lexical_with_options(b"N_aN", &o3).is_err());
        assert!(f64::from_lexical_with_options(b"N_aN", &o4).is_err());
        assert!(f64::from_lexical_with_options(b"N_aN", &o5).unwrap().is_nan());

        // Digit-separator + case-sensitive NaN.
        assert!(f64::from_lexical_with_options(b"n_an", &o1).is_err());
        assert!(f64::from_lexical_with_options(b"n_an", &o2).unwrap().is_nan());
        assert!(f64::from_lexical_with_options(b"n_an", &o3).is_err());
        assert!(f64::from_lexical_with_options(b"n_an", &o4).is_err());
        assert!(f64::from_lexical_with_options(b"n_an", &o5).is_err());
    }

    #[test]
    #[cfg(feature = "format")]
    fn f64_required_integer_digits_test() {
        let format = NumberFormat::REQUIRED_INTEGER_DIGITS;
        let options = ParseFloatOptions::builder()
            .format(format)
            .build()
            .unwrap();
        assert!(f64::from_lexical_with_options(b"+3.0", &options).is_ok());
        assert!(f64::from_lexical_with_options(b"3.0", &options).is_ok());
        assert!(f64::from_lexical_with_options(b".0", &options).is_err());
    }

    #[test]
    #[cfg(feature = "format")]
    fn f64_required_fraction_digits_test() {
        let format = NumberFormat::REQUIRED_FRACTION_DIGITS;
        let options = ParseFloatOptions::builder()
            .format(format)
            .build()
            .unwrap();
        assert!(f64::from_lexical_with_options(b"+3.0", &options).is_ok());
        assert!(f64::from_lexical_with_options(b"3.0", &options).is_ok());
        assert!(f64::from_lexical_with_options(b"3.", &options).is_err());
        assert!(f64::from_lexical_with_options(b"3", &options).is_ok());
    }

    #[test]
    #[cfg(feature = "format")]
    fn f64_required_digits_test() {
        let format = NumberFormat::REQUIRED_DIGITS;
        let options = ParseFloatOptions::builder()
            .format(format)
            .build()
            .unwrap();
        assert!(f64::from_lexical_with_options(b"+3.0", &options).is_ok());
        assert!(f64::from_lexical_with_options(b"3.0", &options).is_ok());
        assert!(f64::from_lexical_with_options(b"3.", &options).is_err());
        assert!(f64::from_lexical_with_options(b"3", &options).is_ok());
        assert!(f64::from_lexical_with_options(b".0", &options).is_err());
    }

    #[test]
    #[cfg(feature = "format")]
    fn f64_no_positive_mantissa_sign_test() {
        let format = NumberFormat::NO_POSITIVE_MANTISSA_SIGN;
        let options = ParseFloatOptions::builder()
            .format(format)
            .build()
            .unwrap();
        assert!(f64::from_lexical_with_options(b"+3.0", &options).is_err());
        assert!(f64::from_lexical_with_options(b"-3.0", &options).is_ok());
        assert!(f64::from_lexical_with_options(b"3.0", &options).is_ok());
    }

    #[test]
    #[cfg(feature = "format")]
    fn f64_required_mantissa_sign_test() {
        let format = NumberFormat::REQUIRED_MANTISSA_SIGN;
        let options = ParseFloatOptions::builder()
            .format(format)
            .build()
            .unwrap();
        assert!(f64::from_lexical_with_options(b"+3.0", &options).is_ok());
        assert!(f64::from_lexical_with_options(b"-3.0", &options).is_ok());
        assert!(f64::from_lexical_with_options(b"3.0", &options).is_err());
    }

    #[test]
    #[cfg(feature = "format")]
    fn f64_no_exponent_notation_test() {
        let format = NumberFormat::NO_EXPONENT_NOTATION;
        let options = ParseFloatOptions::builder()
            .format(format)
            .build()
            .unwrap();
        assert!(f64::from_lexical_with_options(b"+3.0e7", &options).is_err());
        assert!(f64::from_lexical_with_options(b"+3.0e-7", &options).is_err());
        assert!(f64::from_lexical_with_options(b"+3e", &options).is_err());
        assert!(f64::from_lexical_with_options(b"+3e-", &options).is_err());
        assert!(f64::from_lexical_with_options(b"+3.0", &options).is_ok());
        assert!(f64::from_lexical_with_options(b"+3", &options).is_ok());
    }

    #[test]
    #[cfg(feature = "format")]
    fn f64_optional_exponent_test() {
        let format = NumberFormat::permissive().unwrap();
        let options = ParseFloatOptions::builder()
            .format(format)
            .build()
            .unwrap();
        assert!(f64::from_lexical_with_options(b"+3.0e7", &options).is_ok());
        assert!(f64::from_lexical_with_options(b"+3.0e-7", &options).is_ok());
        assert!(f64::from_lexical_with_options(b"+3.0e", &options).is_ok());
        assert!(f64::from_lexical_with_options(b"+3.0e-", &options).is_ok());
        assert!(f64::from_lexical_with_options(b"+3.0", &options).is_ok());
    }

    #[test]
    #[cfg(feature = "format")]
    fn f64_required_exponent_test() {
        let format = NumberFormat::REQUIRED_EXPONENT_DIGITS;
        let options = ParseFloatOptions::builder()
            .format(format)
            .build()
            .unwrap();
        assert!(f64::from_lexical_with_options(b"+3.0e7", &options).is_ok());
        assert!(f64::from_lexical_with_options(b"+3.0e-7", &options).is_ok());
        assert!(f64::from_lexical_with_options(b"+3.0e", &options).is_err());
        assert!(f64::from_lexical_with_options(b"+3.0e-", &options).is_err());
        assert!(f64::from_lexical_with_options(b"+3.0", &options).is_ok());
    }

    #[test]
    #[cfg(feature = "format")]
    fn f64_no_positive_exponent_sign_test() {
        let format = NumberFormat::NO_POSITIVE_EXPONENT_SIGN;
        let options = ParseFloatOptions::builder()
            .format(format)
            .build()
            .unwrap();
        assert!(f64::from_lexical_with_options(b"3.0e7", &options).is_ok());
        assert!(f64::from_lexical_with_options(b"3.0e+7", &options).is_err());
        assert!(f64::from_lexical_with_options(b"3.0e-7", &options).is_ok());
    }

    #[test]
    #[cfg(feature = "format")]
    fn f64_required_exponent_sign_test() {
        let format = NumberFormat::REQUIRED_EXPONENT_SIGN;
        let options = ParseFloatOptions::builder()
            .format(format)
            .build()
            .unwrap();
        assert!(f64::from_lexical_with_options(b"3.0e7", &options).is_err());
        assert!(f64::from_lexical_with_options(b"3.0e+7", &options).is_ok());
        assert!(f64::from_lexical_with_options(b"3.0e-7", &options).is_ok());
    }

    #[test]
    #[cfg(feature = "format")]
    fn f64_no_exponent_without_fraction_test() {
        let format = NumberFormat::NO_EXPONENT_WITHOUT_FRACTION;
        let options = ParseFloatOptions::builder()
            .format(format)
            .build()
            .unwrap();
        assert!(f64::from_lexical_with_options(b"3.0e7", &options).is_ok());
        assert!(f64::from_lexical_with_options(b"3.e7", &options).is_ok());
        assert!(f64::from_lexical_with_options(b"3e7", &options).is_err());

        let format = format | NumberFormat::REQUIRED_FRACTION_DIGITS;
        let options = ParseFloatOptions::builder()
            .format(format)
            .build()
            .unwrap();
        assert!(f64::from_lexical_with_options(b"3.0e7", &options).is_ok());
        assert!(f64::from_lexical_with_options(b"3.e7", &options).is_err());
        assert!(f64::from_lexical_with_options(b"3e7", &options).is_err());
    }

    #[test]
    #[cfg(feature = "format")]
    fn f64_no_leading_zeros_test() {
        let format = NumberFormat::NO_FLOAT_LEADING_ZEROS;
        let options = ParseFloatOptions::builder()
            .format(format)
            .build()
            .unwrap();
        assert!(f64::from_lexical_with_options(b"1.0", &options).is_ok());
        assert!(f64::from_lexical_with_options(b"0.0", &options).is_ok());
        assert!(f64::from_lexical_with_options(b"01.0", &options).is_err());
        assert!(f64::from_lexical_with_options(b"10.0", &options).is_ok());
        assert!(f64::from_lexical_with_options(b"010.0", &options).is_err());
    }

    #[test]
    #[cfg(feature = "format")]
    fn f64_integer_internal_digit_separator_test() {
        let format = NumberFormat::from_separator(b'_') | NumberFormat::INTEGER_INTERNAL_DIGIT_SEPARATOR;
        let options = ParseFloatOptions::builder()
            .format(format)
            .build()
            .unwrap();
        assert!(f64::from_lexical_with_options(b"3_1.0e7", &options).is_ok());
        assert!(f64::from_lexical_with_options(b"_31.0e7", &options).is_err());
        assert!(f64::from_lexical_with_options(b"31_.0e7", &options).is_err());
    }

    #[test]
    #[cfg(feature = "format")]
    fn f64_fraction_internal_digit_separator_test() {
        let format = NumberFormat::from_separator(b'_') | NumberFormat::FRACTION_INTERNAL_DIGIT_SEPARATOR;
        let options = ParseFloatOptions::builder()
            .format(format)
            .build()
            .unwrap();
        assert!(f64::from_lexical_with_options(b"31.0_1e7", &options).is_ok());
        assert!(f64::from_lexical_with_options(b"31._01e7", &options).is_err());
        assert!(f64::from_lexical_with_options(b"31.01_e7", &options).is_err());
    }

    #[test]
    #[cfg(feature = "format")]
    fn f64_exponent_internal_digit_separator_test() {
        let format = NumberFormat::from_separator(b'_') | NumberFormat::EXPONENT_INTERNAL_DIGIT_SEPARATOR;
        let options = ParseFloatOptions::builder()
            .format(format)
            .build()
            .unwrap();
        assert!(f64::from_lexical_with_options(b"31.01e7_1", &options).is_ok());
        assert!(f64::from_lexical_with_options(b"31.01e_71", &options).is_err());
        assert!(f64::from_lexical_with_options(b"31.01e71_", &options).is_err());
    }

    #[test]
    #[cfg(feature = "format")]
    fn f64_integer_leading_digit_separator_test() {
        let format = NumberFormat::from_separator(b'_') | NumberFormat::INTEGER_LEADING_DIGIT_SEPARATOR;
        let options = ParseFloatOptions::builder()
            .format(format)
            .build()
            .unwrap();
        assert!(f64::from_lexical_with_options(b"3_1.0e7", &options).is_err());
        assert!(f64::from_lexical_with_options(b"_31.0e7", &options).is_ok());
        assert!(f64::from_lexical_with_options(b"31_.0e7", &options).is_err());
    }

    #[test]
    #[cfg(feature = "format")]
    fn f64_fraction_leading_digit_separator_test() {
        let format = NumberFormat::from_separator(b'_') | NumberFormat::FRACTION_LEADING_DIGIT_SEPARATOR;
        let options = ParseFloatOptions::builder()
            .format(format)
            .build()
            .unwrap();
        assert!(f64::from_lexical_with_options(b"31.0_1e7", &options).is_err());
        assert!(f64::from_lexical_with_options(b"31._01e7", &options).is_ok());
        assert!(f64::from_lexical_with_options(b"31.01_e7", &options).is_err());
    }

    #[test]
    #[cfg(feature = "format")]
    fn f64_exponent_leading_digit_separator_test() {
        let format = NumberFormat::from_separator(b'_') | NumberFormat::EXPONENT_LEADING_DIGIT_SEPARATOR;
        let options = ParseFloatOptions::builder()
            .format(format)
            .build()
            .unwrap();
        assert!(f64::from_lexical_with_options(b"31.01e7_1", &options).is_err());
        assert!(f64::from_lexical_with_options(b"31.01e_71", &options).is_ok());
        assert!(f64::from_lexical_with_options(b"31.01e71_", &options).is_err());
    }

    #[test]
    #[cfg(feature = "format")]
    fn f64_integer_trailing_digit_separator_test() {
        let format = NumberFormat::from_separator(b'_') | NumberFormat::INTEGER_TRAILING_DIGIT_SEPARATOR;
        let options = ParseFloatOptions::builder()
            .format(format)
            .build()
            .unwrap();
        assert!(f64::from_lexical_with_options(b"3_1.0e7", &options).is_err());
        assert!(f64::from_lexical_with_options(b"_31.0e7", &options).is_err());
        assert!(f64::from_lexical_with_options(b"31_.0e7", &options).is_ok());
    }

    #[test]
    #[cfg(feature = "format")]
    fn f64_fraction_trailing_digit_separator_test() {
        let format = NumberFormat::from_separator(b'_') | NumberFormat::FRACTION_TRAILING_DIGIT_SEPARATOR;
        let options = ParseFloatOptions::builder()
            .format(format)
            .build()
            .unwrap();
        assert!(f64::from_lexical_with_options(b"31.0_1e7", &options).is_err());
        assert!(f64::from_lexical_with_options(b"31._01e7", &options).is_err());
        assert!(f64::from_lexical_with_options(b"31.01_e7", &options).is_ok());
    }

    #[test]
    #[cfg(feature = "format")]
    fn f64_exponent_trailing_digit_separator_test() {
        let format = NumberFormat::from_separator(b'_') | NumberFormat::EXPONENT_TRAILING_DIGIT_SEPARATOR;
        let options = ParseFloatOptions::builder()
            .format(format)
            .build()
            .unwrap();
        assert!(f64::from_lexical_with_options(b"31.01e7_1", &options).is_err());
        assert!(f64::from_lexical_with_options(b"31.01e_71", &options).is_err());
        assert!(f64::from_lexical_with_options(b"31.01e71_", &options).is_ok());
    }

    #[test]
    #[cfg(feature = "format")]
    fn f64_integer_consecutive_digit_separator_test() {
        let format = NumberFormat::from_separator(b'_')
            | NumberFormat::INTEGER_INTERNAL_DIGIT_SEPARATOR
            | NumberFormat::INTEGER_CONSECUTIVE_DIGIT_SEPARATOR;
        let options = ParseFloatOptions::builder()
            .format(format)
            .build()
            .unwrap();
        assert!(f64::from_lexical_with_options(b"3__1.0e7", &options).is_ok());
        assert!(f64::from_lexical_with_options(b"_31.0e7", &options).is_err());
        assert!(f64::from_lexical_with_options(b"31_.0e7", &options).is_err());
    }

    #[test]
    #[cfg(feature = "format")]
    fn f64_fraction_consecutive_digit_separator_test() {
        let format = NumberFormat::from_separator(b'_')
            | NumberFormat::FRACTION_INTERNAL_DIGIT_SEPARATOR
            | NumberFormat::FRACTION_CONSECUTIVE_DIGIT_SEPARATOR;
        let options = ParseFloatOptions::builder()
            .format(format)
            .build()
            .unwrap();
        assert!(f64::from_lexical_with_options(b"31.0__1e7", &options).is_ok());
        assert!(f64::from_lexical_with_options(b"31._01e7", &options).is_err());
        assert!(f64::from_lexical_with_options(b"31.01_e7", &options).is_err());
    }

    #[test]
    #[cfg(feature = "format")]
    fn f64_exponent_consecutive_digit_separator_test() {
        let format = NumberFormat::from_separator(b'_')
            | NumberFormat::EXPONENT_INTERNAL_DIGIT_SEPARATOR
            | NumberFormat::EXPONENT_CONSECUTIVE_DIGIT_SEPARATOR;
        let options = ParseFloatOptions::builder()
            .format(format)
            .build()
            .unwrap();
        assert!(f64::from_lexical_with_options(b"31.01e7__1", &options).is_ok());
        assert!(f64::from_lexical_with_options(b"31.01e_71", &options).is_err());
        assert!(f64::from_lexical_with_options(b"31.01e71_", &options).is_err());
    }

    #[test]
    #[cfg(feature = "format")]
    fn f64_json_exponent_without_dot() {
        // Tests courtesy of @ijl:
        //  https://github.com/Alexhuszagh/rust-lexical/issues/24#issuecomment-578153783
        let format = NumberFormat::JSON;
        let options = ParseFloatOptions::builder()
            .format(format)
            .build()
            .unwrap();
        // JSONTestSuite/test_parsing/y_number_0e1.json
        assert!(f64::from_lexical_with_options(b"0e1", &options).is_ok());
        // JSONTestSuite/test_parsing/y_number_int_with_exp.json
        assert!(f64::from_lexical_with_options(b"20e1", &options).is_ok());
        // JSONTestSuite/test_parsing/y_number_real_capital_e_pos_exp.json
        assert!(f64::from_lexical_with_options(b"1E+2", &options).is_ok());
        // JSONTestSuite/test_transform/number_1e-999.json
        assert!(f64::from_lexical_with_options(b"1E-999", &options).is_ok());
        // nativejson-benchmark/data/jsonchecker/pass01.json
        assert!(f64::from_lexical_with_options(b"23456789012E66", &options).is_ok());
    }
    #[test]
    #[cfg(feature = "format")]
    fn f64_json_exponent_requires_digit() {
        // Tests courtesy of @ijl:
        //  https://github.com/Alexhuszagh/rust-lexical/issues/24#issuecomment-578153783
        let format = NumberFormat::JSON;
        let options = ParseFloatOptions::builder()
            .format(format)
            .build()
            .unwrap();
        assert!(f64::from_lexical_with_options(b"1e", &options).is_err());
        // JSONTestSuite/test_parsing/n_number_9.e+.json
        assert!(f64::from_lexical_with_options(b"9.e+", &options).is_err());
        // JSONTestSuite/test_parsing/n_number_2.e-3.json
        assert!(f64::from_lexical_with_options(b"2.e-3", &options).is_err());
        // JSONTestSuite/test_parsing/n_number_real_without_fractional_part.json
        assert!(f64::from_lexical_with_options(b"1.", &options).is_err());
    }

    #[test]
    #[cfg(feature = "format")]
    fn f64_json_no_leading_zero() {
        let format = NumberFormat::JSON;
        let options = ParseFloatOptions::builder()
            .format(format)
            .build()
            .unwrap();
        assert!(f64::from_lexical_with_options(b"12.0", &options).is_ok());
        assert!(f64::from_lexical_with_options(b"-12.0", &options).is_ok());
        assert!(f64::from_lexical_with_options(b"012.0", &options).is_err());
        assert!(f64::from_lexical_with_options(b"-012.0", &options).is_err());
    }

    #[cfg(all(feature = "std", feature = "property_tests"))]
    proptest! {
        #[test]
        fn f32_invalid_proptest(i in r"[+-]?[0-9]{2}[^\deE]?\.[^\deE]?[0-9]{2}[^\deE]?e[+-]?[0-9]+[^\deE]") {
            let res = f32::from_lexical(i.as_bytes());
            prop_assert!(res.is_err());
            let err = res.err().unwrap();
            prop_assert_eq!(err.code, ErrorCode::InvalidDigit);
        }

        #[test]
        fn f32_double_sign_proptest(i in r"[+-]{2}[0-9]{2}\.[0-9]{2}e[+-]?[0-9]+") {
            let res = f32::from_lexical(i.as_bytes());
            prop_assert!(res.is_err());
            let err = res.err().unwrap();
            prop_assert!(err.code == ErrorCode::InvalidDigit || err.code == ErrorCode::EmptyMantissa);
            prop_assert!(err.index == 0 || err.index == 1);
        }

        #[test]
        fn f32_sign_or_dot_only_proptest(i in r"[+-]?\.?") {
            let res = f32::from_lexical(i.as_bytes());
            prop_assert!(res.is_err());
            let err = res.err().unwrap();
            prop_assert!(err.code == ErrorCode::Empty || err.code == ErrorCode::EmptyMantissa);
            prop_assert!(err.index == 0 || err.index == 1);
        }

        #[test]
        fn f32_double_exponent_sign_proptest(i in r"[+-]?[0-9]{2}\.[0-9]{2}e[+-]{2}[0-9]+") {
            let res = f32::from_lexical(i.as_bytes());
            prop_assert!(res.is_err());
            let err = res.err().unwrap();
            prop_assert_eq!(err.code, ErrorCode::EmptyExponent);
        }

        #[test]
        fn f32_missing_exponent_proptest(i in r"[+-]?[0-9]{2}\.[0-9]{2}e[+-]?") {
            let res = f32::from_lexical(i.as_bytes());
            prop_assert!(res.is_err());
            let err = res.err().unwrap();
            prop_assert_eq!(err.code, ErrorCode::EmptyExponent);
        }

        #[cfg(feature = "correct")]
        #[test]
        fn f32_roundtrip_display_proptest(i in f32::MIN..f32::MAX) {
            let input: String = format!("{}", i);
            prop_assert_eq!(i, f32::from_lexical(input.as_bytes()).unwrap());
        }

        #[cfg(feature = "correct")]
        #[test]
        fn f32_roundtrip_debug_proptest(i in f32::MIN..f32::MAX) {
            let input: String = format!("{:?}", i);
            prop_assert_eq!(i, f32::from_lexical(input.as_bytes()).unwrap());
        }

        #[cfg(feature = "correct")]
        #[test]
        fn f32_roundtrip_scientific_proptest(i in f32::MIN..f32::MAX) {
            let input: String = format!("{:e}", i);
            prop_assert_eq!(i, f32::from_lexical(input.as_bytes()).unwrap());
        }

        #[test]
        fn f64_invalid_proptest(i in r"[+-]?[0-9]{2}[^\deE]?\.[^\deE]?[0-9]{2}[^\deE]?e[+-]?[0-9]+[^\deE]") {
            let res = f64::from_lexical(i.as_bytes());
            prop_assert!(res.is_err());
            let err = res.err().unwrap();
            prop_assert_eq!(err.code, ErrorCode::InvalidDigit);
        }

        #[test]
        fn f64_double_sign_proptest(i in r"[+-]{2}[0-9]{2}\.[0-9]{2}e[+-]?[0-9]+") {
            let res = f64::from_lexical(i.as_bytes());
            prop_assert!(res.is_err());
            let err = res.err().unwrap();
            prop_assert!(err.code == ErrorCode::InvalidDigit || err.code == ErrorCode::EmptyMantissa);
            prop_assert!(err.index == 0 || err.index == 1);
        }

        #[test]
        fn f64_sign_or_dot_only_proptest(i in r"[+-]?\.?") {
            let res = f64::from_lexical(i.as_bytes());
            prop_assert!(res.is_err());
            let err = res.err().unwrap();
            prop_assert!(err.code == ErrorCode::Empty || err.code == ErrorCode::EmptyMantissa);
            prop_assert!(err.index == 0 || err.index == 1);
        }

        #[test]
        fn f64_double_exponent_sign_proptest(i in r"[+-]?[0-9]{2}\.[0-9]{2}e[+-]{2}[0-9]+") {
            let res = f64::from_lexical(i.as_bytes());
            prop_assert!(res.is_err());
            let err = res.err().unwrap();
            prop_assert_eq!(err.code, ErrorCode::EmptyExponent);
        }

        #[test]
        fn f64_missing_exponent_proptest(i in r"[+-]?[0-9]{2}\.[0-9]{2}e[+-]?") {
            let res = f64::from_lexical(i.as_bytes());
            prop_assert!(res.is_err());
            let err = res.err().unwrap();
            prop_assert_eq!(err.code, ErrorCode::EmptyExponent);
        }

        #[cfg(feature = "correct")]
        #[test]
        fn f64_roundtrip_display_proptest(i in f64::MIN..f64::MAX) {
            let input: String = format!("{}", i);
            prop_assert_eq!(i, f64::from_lexical(input.as_bytes()).unwrap());
        }

        #[cfg(feature = "correct")]
        #[test]
        fn f64_roundtrip_debug_proptest(i in f64::MIN..f64::MAX) {
            let input: String = format!("{:?}", i);
            prop_assert_eq!(i, f64::from_lexical(input.as_bytes()).unwrap());
        }

        #[cfg(feature = "correct")]
        #[test]
        fn f64_roundtrip_scientific_proptest(i in f64::MIN..f64::MAX) {
            let input: String = format!("{:e}", i);
            prop_assert_eq!(i, f64::from_lexical(input.as_bytes()).unwrap());
        }
    }
}
