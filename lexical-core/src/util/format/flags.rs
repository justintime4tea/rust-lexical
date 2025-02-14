//! Shared flags for number formats.

#![cfg_attr(not(feature = "format"), allow(dead_code))]
#![cfg_attr(rustfmt, rustfmt::skip::macros(const_fn))]

// We have a lot of flags that may not be enabled when the format
// feature is off, but we don't want to add cfg_if feature gates
// for every one.

use static_assertions::const_assert;

// MACROS
// ------

/// Add flag to flags
#[cfg(feature = "format")]
macro_rules! add_flag {
    ($flags:ident, $bool:expr, $flag:ident) => {
        if $bool {
            $flags.bits |= NumberFormat::$flag.bits();
        }
    };
}

/// Check if a defined flag is invalid.
#[cfg(feature = "format")]
macro_rules! check_flag {
    ($flags:ident, $mask:ident, $invalid:ident) => {
        $flags.bits & NumberFormat::$mask.bits() == NumberFormat::$invalid.bits()
    };
}

// NON-DIGIT SEPARATOR FLAGS & MASKS
// ---------------------------------

/// Digits are required before the decimal point.
pub(crate) const REQUIRED_INTEGER_DIGITS: u64 =
    0b0000000000000000000000000000000000000000000000000000000000000001;

/// Digits are required after the decimal point.
/// This check will only occur if the decimal point is present.
pub(crate) const REQUIRED_FRACTION_DIGITS: u64 =
    0b0000000000000000000000000000000000000000000000000000000000000010;

/// Digits are required after the exponent character.
/// This check will only occur if the exponent character is present.
pub(crate) const REQUIRED_EXPONENT_DIGITS: u64 =
    0b0000000000000000000000000000000000000000000000000000000000000100;

/// Digits are required before or after the control characters.
pub(crate) const REQUIRED_DIGITS: u64 =
    REQUIRED_INTEGER_DIGITS | REQUIRED_FRACTION_DIGITS | REQUIRED_EXPONENT_DIGITS;

/// Positive sign before the mantissa is not allowed.
pub(crate) const NO_POSITIVE_MANTISSA_SIGN: u64 =
    0b0000000000000000000000000000000000000000000000000000000000001000;

/// Positive sign before the mantissa is required.
pub(crate) const REQUIRED_MANTISSA_SIGN: u64 =
    0b0000000000000000000000000000000000000000000000000000000000010000;

/// Exponent notation is not allowed.
pub(crate) const NO_EXPONENT_NOTATION: u64 =
    0b0000000000000000000000000000000000000000000000000000000000100000;

/// Positive sign before the exponent is not allowed.
pub(crate) const NO_POSITIVE_EXPONENT_SIGN: u64 =
    0b0000000000000000000000000000000000000000000000000000000001000000;

/// Positive sign before the exponent is required.
pub(crate) const REQUIRED_EXPONENT_SIGN: u64 =
    0b0000000000000000000000000000000000000000000000000000000010000000;

/// Exponent without a fraction component is not allowed.
///
/// This only checks if a decimal point precedes the exponent character.
/// To require fraction digits or exponent digits with this check,
/// please use the appropriate flags.
pub(crate) const NO_EXPONENT_WITHOUT_FRACTION: u64 =
    0b0000000000000000000000000000000000000000000000000000000100000000;

/// Special (non-finite) values are not allowed.
pub(crate) const NO_SPECIAL: u64 =
    0b0000000000000000000000000000000000000000000000000000001000000000;

/// Special (non-finite) values are case-sensitive.
pub(crate) const CASE_SENSITIVE_SPECIAL: u64 =
    0b0000000000000000000000000000000000000000000000000000010000000000;

/// Leading zeros before an integer value are not allowed.
///
/// If the value is a literal, then this distinction applies
/// when the value is treated like an integer literal, typically
/// when there is no decimal point. If the value is parsed,
/// then this distinction applies when the value as parsed
/// as an integer.
///
/// # Warning
///
/// This also does not mean that the value parsed will be correct,
/// for example, in languages like C, this will not auto-
/// deduce that the radix is 8 with leading zeros, for an octal
/// literal.
pub(crate) const NO_INTEGER_LEADING_ZEROS: u64 =
    0b0000000000000000000000000000000000000000000000000000100000000000;

/// Leading zeros before a float value are not allowed.
///
/// If the value is a literal, then this distinction applies
/// when the value is treated like an integer float, typically
/// when there is a decimal point. If the value is parsed,
/// then this distinction applies when the value as parsed
/// as a float.
///
/// # Warning
///
/// This also does not mean that the value parsed will be correct,
/// for example, in languages like C, this will not auto-
/// deduce that the radix is 8 with leading zeros, for an octal
/// literal.
pub(crate) const NO_FLOAT_LEADING_ZEROS: u64 =
    0b0000000000000000000000000000000000000000000000000001000000000000;

/// Exponent notation is required.
///
/// Valid floats must contain an exponent notation character, and if
/// applicable, a sign character and digits afterwards.
pub(crate) const REQUIRED_EXPONENT_NOTATION: u64 =
    0b0000000000000000000000000000000000000000000000000010000000000000;

// DIGIT SEPARATOR FLAGS & MASKS
// -----------------------------

/// Digit separators are allowed between integer digits.
pub(crate) const INTEGER_INTERNAL_DIGIT_SEPARATOR: u64 =
    0b0000000000000000000000000000000100000000000000000000000000000000;

/// A digit separator is allowed before any integer digits.
pub(crate) const INTEGER_LEADING_DIGIT_SEPARATOR: u64 =
    0b0000000000000000000000000000001000000000000000000000000000000000;

/// A digit separator is allowed after any integer digits.
pub(crate) const INTEGER_TRAILING_DIGIT_SEPARATOR: u64 =
    0b0000000000000000000000000000010000000000000000000000000000000000;

/// Multiple consecutive integer digit separators are allowed.
pub(crate) const INTEGER_CONSECUTIVE_DIGIT_SEPARATOR: u64 =
    0b0000000000000000000000000000100000000000000000000000000000000000;

/// Digit separators are allowed between fraction digits.
pub(crate) const FRACTION_INTERNAL_DIGIT_SEPARATOR: u64 =
    0b0000000000000000000000000001000000000000000000000000000000000000;

/// A digit separator is allowed before any fraction digits.
pub(crate) const FRACTION_LEADING_DIGIT_SEPARATOR: u64 =
    0b0000000000000000000000000010000000000000000000000000000000000000;

/// A digit separator is allowed after any fraction digits.
pub(crate) const FRACTION_TRAILING_DIGIT_SEPARATOR: u64 =
    0b0000000000000000000000000100000000000000000000000000000000000000;

/// Multiple consecutive fraction digit separators are allowed.
pub(crate) const FRACTION_CONSECUTIVE_DIGIT_SEPARATOR: u64 =
    0b0000000000000000000000001000000000000000000000000000000000000000;

/// Digit separators are allowed between exponent digits.
pub(crate) const EXPONENT_INTERNAL_DIGIT_SEPARATOR: u64 =
    0b0000000000000000000000010000000000000000000000000000000000000000;

/// A digit separator is allowed before any exponent digits.
pub(crate) const EXPONENT_LEADING_DIGIT_SEPARATOR: u64 =
    0b0000000000000000000000100000000000000000000000000000000000000000;

/// A digit separator is allowed after any exponent digits.
pub(crate) const EXPONENT_TRAILING_DIGIT_SEPARATOR: u64 =
    0b0000000000000000000001000000000000000000000000000000000000000000;

/// Multiple consecutive exponent digit separators are allowed.
pub(crate) const EXPONENT_CONSECUTIVE_DIGIT_SEPARATOR: u64 =
    0b0000000000000000000010000000000000000000000000000000000000000000;

/// Digit separators are allowed between digits.
pub(crate) const INTERNAL_DIGIT_SEPARATOR: u64 = INTEGER_INTERNAL_DIGIT_SEPARATOR
    | FRACTION_INTERNAL_DIGIT_SEPARATOR
    | EXPONENT_INTERNAL_DIGIT_SEPARATOR;

/// A digit separator is allowed before any digits.
pub(crate) const LEADING_DIGIT_SEPARATOR: u64 = INTEGER_LEADING_DIGIT_SEPARATOR
    | FRACTION_LEADING_DIGIT_SEPARATOR
    | EXPONENT_LEADING_DIGIT_SEPARATOR;

/// A digit separator is allowed after any digits.
pub(crate) const TRAILING_DIGIT_SEPARATOR: u64 = INTEGER_TRAILING_DIGIT_SEPARATOR
    | FRACTION_TRAILING_DIGIT_SEPARATOR
    | EXPONENT_TRAILING_DIGIT_SEPARATOR;

/// Multiple consecutive digit separators are allowed.
pub(crate) const CONSECUTIVE_DIGIT_SEPARATOR: u64 = INTEGER_CONSECUTIVE_DIGIT_SEPARATOR
    | FRACTION_CONSECUTIVE_DIGIT_SEPARATOR
    | EXPONENT_CONSECUTIVE_DIGIT_SEPARATOR;

/// Any digit separators are allowed in special (non-finite) values.
pub(crate) const SPECIAL_DIGIT_SEPARATOR: u64 =
    0b0000000000000000000100000000000000000000000000000000000000000000;

// FLAG ASSERTIONS
// ---------------

// Ensure all our bit flags are valid.
macro_rules! check_subsequent_flags {
    ($x:ident, $y:ident) => {
        const_assert!($x << 1 == $y);
    };
}

// Non-digit separator flags.
const_assert!(REQUIRED_INTEGER_DIGITS == 1);
check_subsequent_flags!(REQUIRED_INTEGER_DIGITS, REQUIRED_FRACTION_DIGITS);
check_subsequent_flags!(REQUIRED_FRACTION_DIGITS, REQUIRED_EXPONENT_DIGITS);
check_subsequent_flags!(REQUIRED_EXPONENT_DIGITS, NO_POSITIVE_MANTISSA_SIGN);
check_subsequent_flags!(NO_POSITIVE_MANTISSA_SIGN, REQUIRED_MANTISSA_SIGN);
check_subsequent_flags!(REQUIRED_MANTISSA_SIGN, NO_EXPONENT_NOTATION);
check_subsequent_flags!(NO_EXPONENT_NOTATION, NO_POSITIVE_EXPONENT_SIGN);
check_subsequent_flags!(NO_POSITIVE_EXPONENT_SIGN, REQUIRED_EXPONENT_SIGN);
check_subsequent_flags!(REQUIRED_EXPONENT_SIGN, NO_EXPONENT_WITHOUT_FRACTION);
check_subsequent_flags!(NO_EXPONENT_WITHOUT_FRACTION, NO_SPECIAL);
check_subsequent_flags!(NO_SPECIAL, CASE_SENSITIVE_SPECIAL);
check_subsequent_flags!(NO_SPECIAL, CASE_SENSITIVE_SPECIAL);
check_subsequent_flags!(CASE_SENSITIVE_SPECIAL, NO_INTEGER_LEADING_ZEROS);
check_subsequent_flags!(NO_INTEGER_LEADING_ZEROS, NO_FLOAT_LEADING_ZEROS);
check_subsequent_flags!(NO_FLOAT_LEADING_ZEROS, REQUIRED_EXPONENT_NOTATION);

// Digit separator flags.
const_assert!(INTEGER_INTERNAL_DIGIT_SEPARATOR == 1 << 32);
check_subsequent_flags!(INTEGER_INTERNAL_DIGIT_SEPARATOR, INTEGER_LEADING_DIGIT_SEPARATOR);
check_subsequent_flags!(INTEGER_LEADING_DIGIT_SEPARATOR, INTEGER_TRAILING_DIGIT_SEPARATOR);
check_subsequent_flags!(INTEGER_TRAILING_DIGIT_SEPARATOR, INTEGER_CONSECUTIVE_DIGIT_SEPARATOR);
check_subsequent_flags!(INTEGER_CONSECUTIVE_DIGIT_SEPARATOR, FRACTION_INTERNAL_DIGIT_SEPARATOR);
check_subsequent_flags!(FRACTION_INTERNAL_DIGIT_SEPARATOR, FRACTION_LEADING_DIGIT_SEPARATOR);
check_subsequent_flags!(FRACTION_LEADING_DIGIT_SEPARATOR, FRACTION_TRAILING_DIGIT_SEPARATOR);
check_subsequent_flags!(FRACTION_TRAILING_DIGIT_SEPARATOR, FRACTION_CONSECUTIVE_DIGIT_SEPARATOR);
check_subsequent_flags!(FRACTION_CONSECUTIVE_DIGIT_SEPARATOR, EXPONENT_INTERNAL_DIGIT_SEPARATOR);
check_subsequent_flags!(EXPONENT_INTERNAL_DIGIT_SEPARATOR, EXPONENT_LEADING_DIGIT_SEPARATOR);
check_subsequent_flags!(EXPONENT_LEADING_DIGIT_SEPARATOR, EXPONENT_TRAILING_DIGIT_SEPARATOR);
check_subsequent_flags!(EXPONENT_TRAILING_DIGIT_SEPARATOR, EXPONENT_CONSECUTIVE_DIGIT_SEPARATOR);
check_subsequent_flags!(EXPONENT_CONSECUTIVE_DIGIT_SEPARATOR, SPECIAL_DIGIT_SEPARATOR);

// VALIDATORS
// ----------

const_fn!(
/// Determine if the digit separator is valid.
#[inline]
#[cfg(not(feature = "power_of_two"))]
pub(crate) const fn is_valid_digit_separator(ch: u8) -> bool {
    match ch {
        b'0'..=b'9' => false,
        b'+' | b'-' => false,
        _ => ch.is_ascii(),
    }
});

const_fn!(
/// Determine if the digit separator is valid.
#[inline]
#[cfg(feature = "power_of_two")]
pub(crate) const fn is_valid_digit_separator(ch: u8) -> bool {
    match ch {
        b'A'..=b'Z' => false,
        b'a'..=b'z' => false,
        b'0'..=b'9' => false,
        b'+' | b'-' => false,
        _ => ch.is_ascii(),
    }
});

const_fn!(
/// Determine if the decimal point is valid.
#[inline]
pub(crate) const fn is_valid_decimal_point(ch: u8) -> bool {
    is_valid_digit_separator(ch)
});

const_fn!(
/// Determine if the exponent decimal character is valid.
#[inline]
pub(crate) const fn is_valid_exponent_decimal(ch: u8) -> bool {
    match ch {
        b'0'..=b'9' => false,
        b'+' | b'-' => false,
        _ => ch.is_ascii(),
    }
});

const_fn!(
    /// Determine if the exponent backup character is valid.
    #[inline]
    pub(crate) const fn is_valid_exponent_backup(ch: u8) -> bool {
        is_valid_digit_separator(ch)
    }
);

const_fn!(
/// Determine if all of the "punctuation" characters are valid.
#[inline]
pub(crate) const fn is_valid_punctuation(
    digit_separator: u8,
    decimal_point: u8,
    exponent_decimal: u8,
    exponent_backup: u8,
) -> bool {
    if digit_separator == decimal_point {
        false
    } else if digit_separator == exponent_decimal {
        false
    } else if digit_separator == exponent_backup {
        false
    } else if decimal_point == exponent_decimal {
        false
    } else if decimal_point == exponent_backup {
        false
    } else {
        // exponent_decimal and exponent_backup can be the same as long as
        // both are valid: in case someone always wants b'^' to be
        // the exponent character.
        true
    }
});

// FLAG FUNCTIONS
// --------------

/// Convert a character, shift and mask to flags.
macro_rules! to_flags {
    ($ch:ident, $shift:ident, $mask:ident) => {
        (($ch & $mask) as u64) << $shift
    };
}

/// Convert a flag, shift and mask to a character.
macro_rules! from_flags {
    ($flag:ident, $shift:ident, $mask:ident) => {
        (($flag >> $shift) as u8) & $mask
    };
}

/// Bit shift for the decimal exponent from the start of the format flags.
const EXPONENT_DECIMAL_SHIFT: u32 = 18;

/// Mask to extract the decimal exponent after shifting.
const EXPONENT_DECIMAL_MASK: u8 = 0x7F;

/// Convert decimal exponent to flags.
#[inline]
pub(crate) const fn exponent_decimal_to_flags(ch: u8) -> u64 {
    to_flags!(ch, EXPONENT_DECIMAL_SHIFT, EXPONENT_DECIMAL_MASK)
}

/// Extract decimal exponent from flags.
#[inline]
pub(crate) const fn exponent_decimal_from_flags(flag: u64) -> u8 {
    from_flags!(flag, EXPONENT_DECIMAL_SHIFT, EXPONENT_DECIMAL_MASK)
}

/// Bit shift for the exponent backup from the start of the format flags.
const EXPONENT_BACKUP_SHIFT: u32 = 25;

/// Mask to extract the exponent backup after shifting.
const EXPONENT_BACKUP_MASK: u8 = 0x7F;

/// Convert exponent backup to flags.
#[inline]
pub(crate) const fn exponent_backup_to_flags(ch: u8) -> u64 {
    to_flags!(ch, EXPONENT_BACKUP_SHIFT, EXPONENT_BACKUP_MASK)
}

/// Extract exponent backup from flags.
#[inline]
pub(crate) const fn exponent_backup_from_flags(flag: u64) -> u8 {
    from_flags!(flag, EXPONENT_BACKUP_SHIFT, EXPONENT_BACKUP_MASK)
}

/// Bit shift for the decimal point from the start of the format flags.
const DECIMAL_POINT_SHIFT: u32 = 50;

/// Mask to extract the decimal point after shifting.
const DECIMAL_POINT_MASK: u8 = 0x7F;

/// Convert decimal point to flags.
#[inline]
pub(crate) const fn decimal_point_to_flags(ch: u8) -> u64 {
    to_flags!(ch, DECIMAL_POINT_SHIFT, DECIMAL_POINT_MASK)
}

/// Extract decimal point from flags.
#[inline]
pub(crate) const fn decimal_point_from_flags(flag: u64) -> u8 {
    from_flags!(flag, DECIMAL_POINT_SHIFT, DECIMAL_POINT_MASK)
}

/// Bit shift for the digit separator from the start of the format flags.
const DIGIT_SEPARATOR_SHIFT: u32 = 57;

/// Mask to extract the digit separator after shifting.
const DIGIT_SEPARATOR_MASK: u8 = 0x7F;

/// Convert digit separator to flags.
#[inline]
pub(crate) const fn digit_separator_to_flags(ch: u8) -> u64 {
    to_flags!(ch, DIGIT_SEPARATOR_SHIFT, DIGIT_SEPARATOR_MASK)
}

/// Extract digit separator from flags.
#[inline]
#[cfg(any(test, feature = "format"))]
pub(crate) const fn digit_separator_from_flags(flag: u64) -> u8 {
    from_flags!(flag, DIGIT_SEPARATOR_SHIFT, DIGIT_SEPARATOR_MASK)
}

// MASK ASSERTIONS
// ---------------

// Ensure all our bit masks don't overlap.
macro_rules! check_subsequent_masks {
    ($xm:ident, $xs:ident, $ym:ident, $ys:ident) => {
        const_assert!((($xm as u64) << $xs) & (($ym as u64) << $ys) == 0);
    };
}

// Ensure all our bit masks don't overlap with existing flags.
macro_rules! check_masks_and_flags {
    ($xm:ident, $xs:ident, $f:ident) => {
        const_assert!((($xm as u64) << $xs) & $f == 0);
    };
}

// Masks do not overlap.
check_subsequent_masks!(
    EXPONENT_DECIMAL_MASK,
    EXPONENT_DECIMAL_SHIFT,
    EXPONENT_BACKUP_MASK,
    EXPONENT_BACKUP_SHIFT
);
check_subsequent_masks!(
    EXPONENT_BACKUP_MASK,
    EXPONENT_BACKUP_SHIFT,
    DECIMAL_POINT_MASK,
    DECIMAL_POINT_SHIFT
);
check_subsequent_masks!(
    DECIMAL_POINT_MASK,
    DECIMAL_POINT_SHIFT,
    DIGIT_SEPARATOR_MASK,
    DIGIT_SEPARATOR_SHIFT
);

// Check masks don't overlap with neighboring flags.
check_masks_and_flags!(
    EXPONENT_BACKUP_MASK,
    EXPONENT_BACKUP_SHIFT,
    INTEGER_INTERNAL_DIGIT_SEPARATOR
);

// DIGIT FUNCTIONS
// ---------------

/// Convert a character to ASCII lowercase as a const fn.
const_fn!(
#[inline(always)]
pub(crate) const fn to_ascii_lowercase(c: u8) -> u8 {
    match c {
        b'A'..=b'Z' => c - b'A' + b'a',
        _ => c,
    }
});

// TESTS
// -----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_digit_separator() {
        assert_eq!(is_valid_digit_separator(b'_'), true);
        assert_eq!(is_valid_digit_separator(b'\''), true);
        assert_eq!(is_valid_digit_separator(b'.'), true);
        if cfg!(feature = "power_of_two") {
            assert_eq!(is_valid_digit_separator(b'e'), false);
        } else {
            assert_eq!(is_valid_digit_separator(b'e'), true);
        }
        assert_eq!(is_valid_digit_separator(b'0'), false);
        assert_eq!(is_valid_digit_separator(128), false);
    }

    #[test]
    fn test_is_valid_decimal_point() {
        assert_eq!(is_valid_decimal_point(b'_'), true);
        assert_eq!(is_valid_decimal_point(b'\''), true);
        assert_eq!(is_valid_decimal_point(b'.'), true);
        if cfg!(feature = "power_of_two") {
            assert_eq!(is_valid_decimal_point(b'e'), false);
        } else {
            assert_eq!(is_valid_decimal_point(b'e'), true);
        }
        assert_eq!(is_valid_decimal_point(b'0'), false);
        assert_eq!(is_valid_decimal_point(128), false);
    }

    #[test]
    fn test_is_valid_exponent_decimal() {
        assert_eq!(is_valid_exponent_decimal(b'_'), true);
        assert_eq!(is_valid_exponent_decimal(b'\''), true);
        assert_eq!(is_valid_exponent_decimal(b'.'), true);
        assert_eq!(is_valid_exponent_decimal(b'^'), true);
        assert_eq!(is_valid_exponent_decimal(b'e'), true);
        assert_eq!(is_valid_exponent_decimal(b'0'), false);
        assert_eq!(is_valid_exponent_decimal(128), false);
    }

    #[test]
    fn test_is_valid_exponent_backup() {
        assert_eq!(is_valid_exponent_backup(b'_'), true);
        assert_eq!(is_valid_exponent_backup(b'\''), true);
        assert_eq!(is_valid_exponent_backup(b'.'), true);
        assert_eq!(is_valid_exponent_backup(b'^'), true);
        assert_eq!(is_valid_exponent_backup(b'0'), false);
        assert_eq!(is_valid_exponent_backup(128), false);

        #[cfg(feature = "power_of_two")]
        assert_eq!(is_valid_exponent_backup(b'e'), false);
    }

    #[test]
    fn test_is_valid_punctuation() {
        assert_eq!(is_valid_punctuation(b'_', b'.', b'e', b'^'), true);
        assert_eq!(is_valid_punctuation(b'_', b'.', b'^', b'^'), true);
        assert_eq!(is_valid_punctuation(b'_', b'e', b'^', b'^'), true);
        assert_eq!(is_valid_punctuation(b'e', b'.', b'^', b'^'), true);
        assert_eq!(is_valid_punctuation(b'e', b'.', b'e', b'^'), false);
        assert_eq!(is_valid_punctuation(b'^', b'.', b'e', b'^'), false);
        assert_eq!(is_valid_punctuation(b'\'', b'^', b'e', b'^'), false);
        assert_eq!(is_valid_punctuation(b'\'', b'e', b'e', b'^'), false);
    }

    #[test]
    fn test_exponent_decimal_to_flags() {
        assert_eq!(exponent_decimal_to_flags(b'e'), 0x1940000);
        assert_eq!(exponent_decimal_to_flags(b'^'), 0x1780000);
        assert_eq!(exponent_decimal_to_flags(b'.'), 0xB80000);
        assert_eq!(exponent_decimal_to_flags(b'\x00'), 0x0);
    }

    #[test]
    fn test_exponent_decimal_from_flags() {
        assert_eq!(exponent_decimal_from_flags(0x1940000), b'e');
        assert_eq!(exponent_decimal_from_flags(0x1780000), b'^');
        assert_eq!(exponent_decimal_from_flags(0xB80000), b'.');
        assert_eq!(exponent_decimal_from_flags(0x0), b'\x00');

        // Test hybrid, to test mask
        let flags = 0x1940000 | 0xBC000000 | 0xB8000000000000 | 0xBE00000000000000;
        assert_eq!(exponent_decimal_from_flags(flags), b'e');
    }

    #[test]
    fn test_exponent_backup_to_flags() {
        assert_eq!(exponent_backup_to_flags(b'e'), 0xCA000000);
        assert_eq!(exponent_backup_to_flags(b'^'), 0xBC000000);
        assert_eq!(exponent_backup_to_flags(b'.'), 0x5C000000);
        assert_eq!(exponent_backup_to_flags(b'\x00'), 0x0);
    }

    #[test]
    fn test_exponent_backup_from_flags() {
        assert_eq!(exponent_backup_from_flags(0xCA000000), b'e');
        assert_eq!(exponent_backup_from_flags(0xBC000000), b'^');
        assert_eq!(exponent_backup_from_flags(0x5C000000), b'.');
        assert_eq!(exponent_backup_from_flags(0x0), b'\x00');

        // Test hybrid, to test mask
        let flags = 0x1940000 | 0xBC000000 | 0xB8000000000000 | 0xBE00000000000000;
        assert_eq!(exponent_backup_from_flags(flags), b'^');
    }

    #[test]
    fn test_decimal_point_to_flags() {
        assert_eq!(decimal_point_to_flags(b'e'), 0x194000000000000);
        assert_eq!(decimal_point_to_flags(b'^'), 0x178000000000000);
        assert_eq!(decimal_point_to_flags(b'.'), 0xB8000000000000);
        assert_eq!(decimal_point_to_flags(b'\x00'), 0x0);
    }

    #[test]
    fn test_decimal_point_from_flags() {
        assert_eq!(decimal_point_from_flags(0x194000000000000), b'e');
        assert_eq!(decimal_point_from_flags(0x178000000000000), b'^');
        assert_eq!(decimal_point_from_flags(0xB8000000000000), b'.');
        assert_eq!(decimal_point_from_flags(0x0), b'\x00');

        // Test hybrid, to test mask
        let flags = 0x1940000 | 0xBC000000 | 0xB8000000000000 | 0xBE00000000000000;
        assert_eq!(decimal_point_from_flags(flags), b'.');
    }

    #[test]
    fn test_digit_separator_to_flags() {
        assert_eq!(digit_separator_to_flags(b'e'), 0xCA00000000000000);
        assert_eq!(digit_separator_to_flags(b'^'), 0xBC00000000000000);
        assert_eq!(digit_separator_to_flags(b'.'), 0x5C00000000000000);
        assert_eq!(digit_separator_to_flags(b'\x00'), 0x0);
    }

    #[test]
    fn test_digit_separator_from_flags() {
        assert_eq!(digit_separator_from_flags(0xCA00000000000000), b'e');
        assert_eq!(digit_separator_from_flags(0xBC00000000000000), b'^');
        assert_eq!(digit_separator_from_flags(0x5C00000000000000), b'.');
        assert_eq!(digit_separator_from_flags(0x0), b'\x00');

        // Test hybrid, to test mask
        let flags = 0x1940000 | 0xBC000000 | 0xB8000000000000 | 0xBE00000000000000;
        assert_eq!(digit_separator_from_flags(flags), b'_');
    }

    #[test]
    fn test_to_ascii_lowercase() {
        assert_eq!(to_ascii_lowercase(b'E'), b'e');
        assert_eq!(to_ascii_lowercase(b'e'), b'e');
        assert_eq!(to_ascii_lowercase(b'Z'), b'z');
        assert_eq!(to_ascii_lowercase(b'+'), b'+');
        assert_eq!(to_ascii_lowercase(b'\t'), b'\t');
    }
}
