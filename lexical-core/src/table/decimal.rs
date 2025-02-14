//! Cached tables for precalculated values for decimal strings.

use crate::traits::*;
use static_assertions::const_assert;

#[cfg(feature = "radix")]
use super::radix::*;

/// Precalculated table for a digit to a character.
///
/// Unoptimized table for radix N always, which translates a single digit to a
/// character, and also useful for radix-N float encoding.
const DIGIT_TO_CHAR: [u8; 36] = [
    b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'A', b'B', b'C', b'D', b'E', b'F',
    b'G', b'H', b'I', b'J', b'K', b'L', b'M', b'N', b'O', b'P', b'Q', b'R', b'S', b'T', b'U', b'V',
    b'W', b'X', b'Y', b'Z',
];

/// Get character from digit.
#[inline(always)]
#[allow(dead_code)]
pub(crate) fn digit_to_char<T: Integer>(digit: T) -> u8 {
    debug_assert!(digit.as_i32() >= 0 && digit.as_i32() < 36, "digit_to_char() invalid character.");
    DIGIT_TO_CHAR[digit.as_usize()]
}

// RADIX^2 TABLES
// --------------

// Conditionally compile the precompiled radix**2 tables.
// These tables take `2 * (value % (radix^2))`, and return
// two consecutive values corresponding to both digits.
//
// Total array storage:
//  Without radix: ~430 B:
//      200 u8
//      11 f32
//      23 f64
//  With radix: ~55 KB.
//      32210 u8
//      518 f32
//      2610 f64
// Provides ~5x performance enhancement.
//
// These arrays are cache-friendly, for each BASE[2-36] table,
// elements can be access sequentially 2-at-a-time, preventing as many
// cache misses inside inner loops. For example, accessing the two elements
// for a remainder of `3` for the radix^2 in radix 2 will give you `1` and `1`,
// at indexes 6 and 7.

pub(crate) const DIGIT_TO_BASE10_SQUARED: [u8; 200] = [
    b'0', b'0', b'0', b'1', b'0', b'2', b'0', b'3', b'0', b'4', b'0', b'5', b'0', b'6', b'0', b'7',
    b'0', b'8', b'0', b'9', b'1', b'0', b'1', b'1', b'1', b'2', b'1', b'3', b'1', b'4', b'1', b'5',
    b'1', b'6', b'1', b'7', b'1', b'8', b'1', b'9', b'2', b'0', b'2', b'1', b'2', b'2', b'2', b'3',
    b'2', b'4', b'2', b'5', b'2', b'6', b'2', b'7', b'2', b'8', b'2', b'9', b'3', b'0', b'3', b'1',
    b'3', b'2', b'3', b'3', b'3', b'4', b'3', b'5', b'3', b'6', b'3', b'7', b'3', b'8', b'3', b'9',
    b'4', b'0', b'4', b'1', b'4', b'2', b'4', b'3', b'4', b'4', b'4', b'5', b'4', b'6', b'4', b'7',
    b'4', b'8', b'4', b'9', b'5', b'0', b'5', b'1', b'5', b'2', b'5', b'3', b'5', b'4', b'5', b'5',
    b'5', b'6', b'5', b'7', b'5', b'8', b'5', b'9', b'6', b'0', b'6', b'1', b'6', b'2', b'6', b'3',
    b'6', b'4', b'6', b'5', b'6', b'6', b'6', b'7', b'6', b'8', b'6', b'9', b'7', b'0', b'7', b'1',
    b'7', b'2', b'7', b'3', b'7', b'4', b'7', b'5', b'7', b'6', b'7', b'7', b'7', b'8', b'7', b'9',
    b'8', b'0', b'8', b'1', b'8', b'2', b'8', b'3', b'8', b'4', b'8', b'5', b'8', b'6', b'8', b'7',
    b'8', b'8', b'8', b'9', b'9', b'0', b'9', b'1', b'9', b'2', b'9', b'3', b'9', b'4', b'9', b'5',
    b'9', b'6', b'9', b'7', b'9', b'8', b'9', b'9',
];

// EXACT EXPONENT
// --------------

// Calculating the exponent limit requires determining the largest exponent
// we can calculate for a radix that can be **exactly** store in the
// float type. If the value is a power-of-two, then we simply
// need to scale the minimum, denormal exp and maximum exp to the type
// size. Otherwise, we need to calculate the number of digits
// that can fit into the type's precision, after removing a power-of-two
// (since these values can be represented exactly).
//
// The mantissa limit is the number of digits we can remove from
// the exponent into the mantissa, and is therefore is the
// `⌊ precision / log2(radix) ⌋`, where precision does not include
// the hidden bit.
//
// The algorithm for calculating both `exponent_limit` and `mantissa_limit`,
// in Python, can be done as follows:
//
// ```python
// import math
//
// def is_pow2(value):
//     '''Calculate if a value is a power of 2.'''
//
//     floor = int(math.log2(value))
//     return value == 2**floor
//
//
// def remove_pow2(value):
//     '''Remove a power of 2 from the value.'''
//
//     while math.floor(value / 2) == value / 2:
//         value //= 2
//     return value
//
//
// def exponent_limit(radix, mantissa_size, min_exp, max_exp):
//     '''
//     Calculate the exponent limit for a float, for a given
//     float type, where `radix` is the numerical base
//     for the float type, and mantissa size is the length
//     of the mantissa in bits. min_exp is the minimum,
//     denormal binary exponent, and max_exp is the maximum
//     binary exponent.
//     '''
//
//     if is_pow2(radix):
//         # Can always be exactly represented, calculate relative
//         # to min and max exp.
//         scaled_min_exp = int(min_exp / math.log2(radix))
//         scaled_max_exp = int(max_exp / math.log2(radix))
//         return (scaled_min_exp, scaled_max_exp)
//     else:
//         # Positive and negative should be the same,
//         # since we need to find the maximum digit
//         # representable with mantissa digits.
//         # We first need to remove the highest power-of-
//         # two from the radix, since these will be represented
//         # with exponent digits.
//         base = remove_pow2(radix)
//         precision = mantissa_size + 1
//         exp_limit = int(precision / math.log2(base))
//         return (-exp_limit, exp_limit)
//
//
// def mantissa_limit(radix, mantissa_size):
//     '''
//     Calculate mantissa limit for a float type, given
//     the radix and the length of the mantissa in bits.
//     '''
//
//     precision = mantissa_size + 1
//     return int(precision / math.log2(radix))
//
//
// def all_limits(mantissa_size, min_exp, max_exp):
//     '''Print limits for all radixes.'''
//
//     print('match radix.as_i32() {')
//     for radix in range(2, 37):
//         exp_limit = exponent_limit(radix, mantissa_size, min_exp, max_exp)
//         print(f'    {radix} => {exp_limit},')
//     print('}')
//
//     print('match radix.as_i32() {')
//     for radix in range(2, 37):
//         mant_limit = mantissa_limit(radix, mantissa_size)
//         print(f'    {radix} => {mant_limit},')
//     print('}')
// ```

/// Get exact exponent limit for radix.
pub trait ExactExponent {
    /// Get min and max exponent limits (exact) from radix.
    fn exponent_limit<T: Integer>(radix: T) -> (i32, i32);

    /// Get the number of digits that can be shifted from exponent to mantissa.
    fn mantissa_limit<T: Integer>(radix: T) -> i32;
}

#[cfg(feature = "f16")]
impl ExactExponent for f16 {
    #[inline]
    fn exponent_limit<T: Integer>(radix: T) -> (i32, i32) {
        debug_assert_radix!(radix);
        #[cfg(not(feature = "power_of_two"))]
        {
            (-4, 4)
        }

        #[cfg(all(feature = "power_of_two", not(feature = "radix")))]
        {
            match radix.as_i32() {
                2 => (-24, 15),
                4 => (-12, 7),
                8 => (-8, 5),
                10 => (-4, 4),
                16 => (-6, 3),
                32 => (-4, 3),
                // Invalid radix
                _ => unreachable!(),
            }
        }

        #[cfg(feature = "radix")]
        {
            match radix.as_i32() {
                2 => (-24, 15),
                3 => (-6, 6),
                4 => (-12, 7),
                5 => (-4, 4),
                6 => (-6, 6),
                7 => (-3, 3),
                8 => (-8, 5),
                9 => (-3, 3),
                10 => (-4, 4),
                11 => (-3, 3),
                12 => (-6, 6),
                13 => (-2, 2),
                14 => (-3, 3),
                15 => (-2, 2),
                16 => (-6, 3),
                17 => (-2, 2),
                18 => (-3, 3),
                19 => (-2, 2),
                20 => (-4, 4),
                21 => (-2, 2),
                22 => (-3, 3),
                23 => (-2, 2),
                24 => (-6, 6),
                25 => (-2, 2),
                26 => (-2, 2),
                27 => (-2, 2),
                28 => (-3, 3),
                29 => (-2, 2),
                30 => (-2, 2),
                31 => (-2, 2),
                32 => (-4, 3),
                33 => (-2, 2),
                34 => (-2, 2),
                35 => (-2, 2),
                36 => (-3, 3),
                // Invalid radix
                _ => unreachable!(),
            }
        }
    }

    #[inline]
    fn mantissa_limit<T: Integer>(radix: T) -> i32 {
        debug_assert_radix!(radix);
        #[cfg(not(feature = "power_of_two"))]
        {
            3
        }

        #[cfg(all(feature = "power_of_two", not(feature = "radix")))]
        {
            match radix.as_i32() {
                2 => 11,
                4 => 5,
                8 => 3,
                10 => 3,
                16 => 2,
                32 => 2,
                // Invalid radix
                _ => unreachable!(),
            }
        }

        #[cfg(feature = "radix")]
        {
            match radix.as_i32() {
                2 => 11,
                3 => 6,
                4 => 5,
                5 => 4,
                6 => 4,
                7 => 3,
                8 => 3,
                9 => 3,
                10 => 3,
                11 => 3,
                12 => 3,
                13 => 2,
                14 => 2,
                15 => 2,
                16 => 2,
                17 => 2,
                18 => 2,
                19 => 2,
                20 => 2,
                21 => 2,
                22 => 2,
                23 => 2,
                24 => 2,
                25 => 2,
                26 => 2,
                27 => 2,
                28 => 2,
                29 => 2,
                30 => 2,
                31 => 2,
                32 => 2,
                33 => 2,
                34 => 2,
                35 => 2,
                36 => 2,
                // Invalid radix
                _ => unreachable!(),
            }
        }
    }
}

#[cfg(feature = "f16")]
impl ExactExponent for bf16 {
    #[inline]
    fn exponent_limit<T: Integer>(radix: T) -> (i32, i32) {
        debug_assert_radix!(radix);
        #[cfg(not(feature = "power_of_two"))]
        {
            (-3, 3)
        }

        #[cfg(all(feature = "power_of_two", not(feature = "radix")))]
        {
            match radix.as_i32() {
                2 => (-133, 127),
                4 => (-66, 63),
                8 => (-44, 42),
                10 => (-3, 3),
                16 => (-33, 31),
                32 => (-26, 25),
                // Invalid radix
                _ => unreachable!(),
            }
        }

        #[cfg(feature = "radix")]
        {
            match radix.as_i32() {
                2 => (-133, 127),
                3 => (-5, 5),
                4 => (-66, 63),
                5 => (-3, 3),
                6 => (-5, 5),
                7 => (-2, 2),
                8 => (-44, 42),
                9 => (-2, 2),
                10 => (-3, 3),
                11 => (-2, 2),
                12 => (-5, 5),
                13 => (-2, 2),
                14 => (-2, 2),
                15 => (-2, 2),
                16 => (-33, 31),
                17 => (-1, 1),
                18 => (-2, 2),
                19 => (-1, 1),
                20 => (-3, 3),
                21 => (-1, 1),
                22 => (-2, 2),
                23 => (-1, 1),
                24 => (-5, 5),
                25 => (-1, 1),
                26 => (-2, 2),
                27 => (-1, 1),
                28 => (-2, 2),
                29 => (-1, 1),
                30 => (-2, 2),
                31 => (-1, 1),
                32 => (-26, 25),
                33 => (-1, 1),
                34 => (-1, 1),
                35 => (-1, 1),
                36 => (-2, 2),
                // Invalid radix
                _ => unreachable!(),
            }
        }
    }

    #[inline]
    fn mantissa_limit<T: Integer>(radix: T) -> i32 {
        debug_assert_radix!(radix);
        #[cfg(not(feature = "power_of_two"))]
        {
            2
        }

        #[cfg(all(feature = "power_of_two", not(feature = "radix")))]
        {
            match radix.as_i32() {
                2 => 8,
                4 => 4,
                8 => 2,
                10 => 2,
                16 => 2,
                32 => 1,
                // Invalid radix
                _ => unreachable!(),
            }
        }

        #[cfg(feature = "radix")]
        {
            match radix.as_i32() {
                2 => 8,
                3 => 5,
                4 => 4,
                5 => 3,
                6 => 3,
                7 => 2,
                8 => 2,
                9 => 2,
                10 => 2,
                11 => 2,
                12 => 2,
                13 => 2,
                14 => 2,
                15 => 2,
                16 => 2,
                17 => 1,
                18 => 1,
                19 => 1,
                20 => 1,
                21 => 1,
                22 => 1,
                23 => 1,
                24 => 1,
                25 => 1,
                26 => 1,
                27 => 1,
                28 => 1,
                29 => 1,
                30 => 1,
                31 => 1,
                32 => 1,
                33 => 1,
                34 => 1,
                35 => 1,
                36 => 1,
                // Invalid radix
                _ => unreachable!(),
            }
        }
    }
}

impl ExactExponent for f32 {
    #[inline]
    fn exponent_limit<T: Integer>(radix: T) -> (i32, i32) {
        debug_assert_radix!(radix);
        #[cfg(not(feature = "power_of_two"))]
        {
            (-10, 10)
        }

        #[cfg(all(feature = "power_of_two", not(feature = "radix")))]
        {
            match radix.as_i32() {
                2 => (-149, 127),
                4 => (-74, 63),
                8 => (-49, 42),
                10 => (-10, 10),
                16 => (-37, 31),
                32 => (-29, 25),
                // Invalid radix
                _ => unreachable!(),
            }
        }

        #[cfg(feature = "radix")]
        {
            match radix.as_i32() {
                2 => (-149, 127),
                3 => (-15, 15),
                4 => (-74, 63),
                5 => (-10, 10),
                6 => (-15, 15),
                7 => (-8, 8),
                8 => (-49, 42),
                9 => (-7, 7),
                10 => (-10, 10),
                11 => (-6, 6),
                12 => (-15, 15),
                13 => (-6, 6),
                14 => (-8, 8),
                15 => (-6, 6),
                16 => (-37, 31),
                17 => (-5, 5),
                18 => (-7, 7),
                19 => (-5, 5),
                20 => (-10, 10),
                21 => (-5, 5),
                22 => (-6, 6),
                23 => (-5, 5),
                24 => (-15, 15),
                25 => (-5, 5),
                26 => (-6, 6),
                27 => (-5, 5),
                28 => (-8, 8),
                29 => (-4, 4),
                30 => (-6, 6),
                31 => (-4, 4),
                32 => (-29, 25),
                33 => (-4, 4),
                34 => (-5, 5),
                35 => (-4, 4),
                36 => (-7, 7),
                // Invalid radix
                _ => unreachable!(),
            }
        }
    }

    #[inline]
    fn mantissa_limit<T: Integer>(radix: T) -> i32 {
        debug_assert_radix!(radix);
        #[cfg(not(feature = "power_of_two"))]
        {
            7
        }

        #[cfg(all(feature = "power_of_two", not(feature = "radix")))]
        {
            match radix.as_i32() {
                2 => 24,
                4 => 12,
                8 => 8,
                10 => 7,
                16 => 6,
                32 => 4,
                // Invalid radix
                _ => unreachable!(),
            }
        }

        #[cfg(feature = "radix")]
        {
            match radix.as_i32() {
                2 => 24,
                3 => 15,
                4 => 12,
                5 => 10,
                6 => 9,
                7 => 8,
                8 => 8,
                9 => 7,
                10 => 7,
                11 => 6,
                12 => 6,
                13 => 6,
                14 => 6,
                15 => 6,
                16 => 6,
                17 => 5,
                18 => 5,
                19 => 5,
                20 => 5,
                21 => 5,
                22 => 5,
                23 => 5,
                24 => 5,
                25 => 5,
                26 => 5,
                27 => 5,
                28 => 4,
                29 => 4,
                30 => 4,
                31 => 4,
                32 => 4,
                33 => 4,
                34 => 4,
                35 => 4,
                36 => 4,
                // Invalid radix
                _ => unreachable!(),
            }
        }
    }
}

impl ExactExponent for f64 {
    #[inline]
    fn exponent_limit<T: Integer>(radix: T) -> (i32, i32) {
        debug_assert_radix!(radix);
        #[cfg(not(feature = "power_of_two"))]
        {
            (-22, 22)
        }

        #[cfg(all(feature = "power_of_two", not(feature = "radix")))]
        {
            match radix.as_i32() {
                2 => (-1074, 1023),
                4 => (-537, 511),
                8 => (-358, 341),
                10 => (-22, 22),
                16 => (-268, 255),
                32 => (-214, 204),
                // Invalid radix
                _ => unreachable!(),
            }
        }
        #[cfg(feature = "radix")]
        {
            match radix.as_i32() {
                2 => (-1074, 1023),
                3 => (-33, 33),
                4 => (-537, 511),
                5 => (-22, 22),
                6 => (-33, 33),
                7 => (-18, 18),
                8 => (-358, 341),
                9 => (-16, 16),
                10 => (-22, 22),
                11 => (-15, 15),
                12 => (-33, 33),
                13 => (-14, 14),
                14 => (-18, 18),
                15 => (-13, 13),
                16 => (-268, 255),
                17 => (-12, 12),
                18 => (-16, 16),
                19 => (-12, 12),
                20 => (-22, 22),
                21 => (-12, 12),
                22 => (-15, 15),
                23 => (-11, 11),
                24 => (-33, 33),
                25 => (-11, 11),
                26 => (-14, 14),
                27 => (-11, 11),
                28 => (-18, 18),
                29 => (-10, 10),
                30 => (-13, 13),
                31 => (-10, 10),
                32 => (-214, 204),
                33 => (-10, 10),
                34 => (-12, 12),
                35 => (-10, 10),
                36 => (-16, 16),
                // Invalid radix
                _ => unreachable!(),
            }
        }
    }

    #[inline]
    fn mantissa_limit<T: Integer>(radix: T) -> i32 {
        debug_assert_radix!(radix);
        #[cfg(not(feature = "power_of_two"))]
        {
            15
        }

        #[cfg(all(feature = "power_of_two", not(feature = "radix")))]
        {
            match radix.as_i32() {
                2 => 53,
                4 => 26,
                8 => 17,
                10 => 15,
                16 => 13,
                32 => 10,
                // Invalid radix
                _ => unreachable!(),
            }
        }

        #[cfg(feature = "radix")]
        {
            match radix.as_i32() {
                2 => 53,
                3 => 33,
                4 => 26,
                5 => 22,
                6 => 20,
                7 => 18,
                8 => 17,
                9 => 16,
                10 => 15,
                11 => 15,
                12 => 14,
                13 => 14,
                14 => 13,
                15 => 13,
                16 => 13,
                17 => 12,
                18 => 12,
                19 => 12,
                20 => 12,
                21 => 12,
                22 => 11,
                23 => 11,
                24 => 11,
                25 => 11,
                26 => 11,
                27 => 11,
                28 => 11,
                29 => 10,
                30 => 10,
                31 => 10,
                32 => 10,
                33 => 10,
                34 => 10,
                35 => 10,
                36 => 10,
                // Invalid radix
                _ => unreachable!(),
            }
        }
    }
}

#[cfg(feature = "f128")]
impl ExactExponent for f128 {
    #[inline]
    fn exponent_limit<T: Integer>(radix: T) -> (i32, i32) {
        debug_assert_radix!(radix);
        #[cfg(not(feature = "power_of_two"))]
        {
            (-48, 48)
        }

        #[cfg(all(feature = "power_of_two", not(feature = "radix")))]
        {
            match radix.as_i32() {
                2 => (-16494, 16383),
                4 => (-8247, 8191),
                8 => (-5498, 5461),
                10 => (-48, 48),
                16 => (-4123, 4095),
                32 => (-3298, 3276),
                // Invalid radix
                _ => unreachable!(),
            }
        }

        #[cfg(feature = "radix")]
        {
            match radix.as_i32() {
                2 => (-16494, 16383),
                3 => (-71, 71),
                4 => (-8247, 8191),
                5 => (-48, 48),
                6 => (-71, 71),
                7 => (-40, 40),
                8 => (-5498, 5461),
                9 => (-35, 35),
                10 => (-48, 48),
                11 => (-32, 32),
                12 => (-71, 71),
                13 => (-30, 30),
                14 => (-40, 40),
                15 => (-28, 28),
                16 => (-4123, 4095),
                17 => (-27, 27),
                18 => (-35, 35),
                19 => (-26, 26),
                20 => (-48, 48),
                21 => (-25, 25),
                22 => (-32, 32),
                23 => (-24, 24),
                24 => (-71, 71),
                25 => (-24, 24),
                26 => (-30, 30),
                27 => (-23, 23),
                28 => (-40, 40),
                29 => (-23, 23),
                30 => (-28, 28),
                31 => (-22, 22),
                32 => (-3298, 3276),
                33 => (-22, 22),
                34 => (-27, 27),
                35 => (-22, 22),
                36 => (-35, 35),
                // Invalid radix
                _ => unreachable!(),
            }
        }
    }

    #[inline]
    fn mantissa_limit<T: Integer>(radix: T) -> i32 {
        debug_assert_radix!(radix);
        #[cfg(not(feature = "power_of_two"))]
        {
            34
        }

        #[cfg(all(feature = "power_of_two", not(feature = "radix")))]
        {
            match radix.as_i32() {
                2 => 113,
                4 => 56,
                8 => 37,
                10 => 34,
                16 => 28,
                32 => 22,
                // Invalid radix
                _ => unreachable!(),
            }
        }

        #[cfg(feature = "radix")]
        {
            match radix.as_i32() {
                2 => 113,
                3 => 71,
                4 => 56,
                5 => 48,
                6 => 43,
                7 => 40,
                8 => 37,
                9 => 35,
                10 => 34,
                11 => 32,
                12 => 31,
                13 => 30,
                14 => 29,
                15 => 28,
                16 => 28,
                17 => 27,
                18 => 27,
                19 => 26,
                20 => 26,
                21 => 25,
                22 => 25,
                23 => 24,
                24 => 24,
                25 => 24,
                26 => 24,
                27 => 23,
                28 => 23,
                29 => 23,
                30 => 23,
                31 => 22,
                32 => 22,
                33 => 22,
                34 => 22,
                35 => 22,
                36 => 21,
                // Invalid radix
                _ => unreachable!(),
            }
        }
    }
}

// Conditionally compile the radix POWI tables.
// These tables contain all the values that can be exactly represented
// by a given float of a certain size.
//
// Total array storage: 2.1 KB (f32) + 21.5 KB (f64).
// The total performance enhancements save ~350+ clock cycles (x86) or
// ~100 clock cycles (x87) for the FYL2X and F2XM1 instructions, require
// to compute a power. This should be a significant performance win.

// TABLE POW

/// Calculate powers using pre-calculated lookup tables.
/// No error-checking occurs, these methods are not safe.
pub trait TablePower {
    /// Get power of 2 from exponent.
    #[cfg(feature = "power_of_two")]
    fn table_pow2(exponent: i32) -> Self;

    /// Get power of 2 from exponent.
    fn table_pow<T: Integer>(radix: T, exponent: i32) -> Self;
}

/// Calculate 2^exponent assigned straight from bits.
#[cfg(feature = "power_of_two")]
macro_rules! bitwise_pow2 {
    ($exponent:ident, $float:ty, $unsigned:ty) => {{
        debug_assert!(
            $exponent + <$float>::EXPONENT_BIAS - 1 >= 0,
            "table_pow2() have negative exponent."
        );

        // Say we have (for f32):
        //     BIAS = 127
        //     MANT_SIZE = 23
        // Then, we have denormal floats and normal floats that take
        // the following form:
        //
        // Denormal floats are [-BIAS-MANT_SIZE, -BIAS]
        //     Take form of S00000000MMMMMMMMMMMMMMMMMMMMMMM
        // Normal floats are [-BIAS+1, BIAS]
        //     Take form of SEEEEEEE100000000000000000000000
        //     Where S = Sign, E = Exponent, and M = Mantissa.

        // We adjust our exp bias here so we can find denormal floats.
        const MIN_EXP: i32 = <$float>::EXPONENT_BIAS - 1;
        const BIAS: i32 = <$float>::EXPONENT_BIAS - <$float>::MANTISSA_SIZE;
        if $exponent <= -BIAS {
            // Denormal float, can calculate it based off the shift.
            let shift = $exponent + MIN_EXP;
            <$float>::from_bits(1 as $unsigned << shift)
        } else {
            // Normal float, just shift to the bias.
            // Remember: we're not using the EXPONENT_BIAS here because
            // we assume we're having a value in the hidden bit,
            // which is `1 << MANTISSA_SIZE`. We therefore
            // need to subtract MANTISSA_SIZE from our bias to calculate
            // the float as the form `2^exponent`.
            let biased_e = ($exponent + BIAS) as $unsigned;
            <$float>::from_bits(biased_e << <$float>::MANTISSA_SIZE)
        }
    }};
}

// F32

/// Precalculated values of radix**i for i in range [0, arr.len()-1].
/// Each value can be **exactly** represented as that type.
const F32_POW10: [f32; 11] = [
    1.0,
    10.0,
    100.0,
    1000.0,
    10000.0,
    100000.0,
    1000000.0,
    10000000.0,
    100000000.0,
    1000000000.0,
    10000000000.0,
];

// Compile-time guarantees for our tables.
const_assert!(F32_POW10[1] / F32_POW10[0] == 10.0);

impl TablePower for f32 {
    #[inline]
    #[cfg(feature = "power_of_two")]
    fn table_pow2(exponent: i32) -> f32 {
        bitwise_pow2!(exponent, f32, u32)
    }

    #[inline]
    fn table_pow<T: Integer>(radix: T, exponent: i32) -> f32 {
        debug_assert!(exponent >= 0, "table_pow() have negative exponent.");
        debug_assert_radix!(radix);
        let exponent = exponent as usize;

        #[cfg(not(feature = "radix"))]
        {
            debug_assert!(radix.as_i32() == 10, "radix must be 10");
            F32_POW10[exponent]
        }

        #[cfg(feature = "radix")]
        {
            match radix.as_i32() {
                3 => F32_POW3[exponent],
                5 => F32_POW5[exponent],
                6 => F32_POW6[exponent],
                7 => F32_POW7[exponent],
                9 => F32_POW9[exponent],
                10 => F32_POW10[exponent],
                11 => F32_POW11[exponent],
                12 => F32_POW12[exponent],
                13 => F32_POW13[exponent],
                14 => F32_POW14[exponent],
                15 => F32_POW15[exponent],
                17 => F32_POW17[exponent],
                18 => F32_POW18[exponent],
                19 => F32_POW19[exponent],
                20 => F32_POW20[exponent],
                21 => F32_POW21[exponent],
                22 => F32_POW22[exponent],
                23 => F32_POW23[exponent],
                24 => F32_POW24[exponent],
                25 => F32_POW25[exponent],
                26 => F32_POW26[exponent],
                27 => F32_POW27[exponent],
                28 => F32_POW28[exponent],
                29 => F32_POW29[exponent],
                30 => F32_POW30[exponent],
                31 => F32_POW31[exponent],
                33 => F32_POW33[exponent],
                34 => F32_POW34[exponent],
                35 => F32_POW35[exponent],
                36 => F32_POW36[exponent],
                // Invalid radix
                _ => unreachable!(),
            }
        }
    }
}

/// Precalculated values of radix**i for i in range [0, arr.len()-1].
/// Each value can be **exactly** represented as that type.
const F64_POW10: [f64; 23] = [
    1.0,
    10.0,
    100.0,
    1000.0,
    10000.0,
    100000.0,
    1000000.0,
    10000000.0,
    100000000.0,
    1000000000.0,
    10000000000.0,
    100000000000.0,
    1000000000000.0,
    10000000000000.0,
    100000000000000.0,
    1000000000000000.0,
    10000000000000000.0,
    100000000000000000.0,
    1000000000000000000.0,
    10000000000000000000.0,
    100000000000000000000.0,
    1000000000000000000000.0,
    10000000000000000000000.0,
];

// Compile-time guarantees for our tables.
const_assert!(F64_POW10[1] / F64_POW10[0] == 10.0);

impl TablePower for f64 {
    #[inline]
    #[cfg(feature = "power_of_two")]
    fn table_pow2(exponent: i32) -> f64 {
        bitwise_pow2!(exponent, f64, u64)
    }

    #[inline]
    fn table_pow<T: Integer>(radix: T, exponent: i32) -> f64 {
        debug_assert!(exponent >= 0, "table_pow() have negative exponent.");
        debug_assert_radix!(radix);
        let exponent = exponent as usize;

        #[cfg(not(feature = "radix"))]
        {
            debug_assert!(radix.as_i32() == 10, "radix must be 10");
            F64_POW10[exponent]
        }

        #[cfg(feature = "radix")]
        {
            match radix.as_i32() {
                3 => F64_POW3[exponent],
                5 => F64_POW5[exponent],
                6 => F64_POW6[exponent],
                7 => F64_POW7[exponent],
                9 => F64_POW9[exponent],
                10 => F64_POW10[exponent],
                11 => F64_POW11[exponent],
                12 => F64_POW12[exponent],
                13 => F64_POW13[exponent],
                14 => F64_POW14[exponent],
                15 => F64_POW15[exponent],
                17 => F64_POW17[exponent],
                18 => F64_POW18[exponent],
                19 => F64_POW19[exponent],
                20 => F64_POW20[exponent],
                21 => F64_POW21[exponent],
                22 => F64_POW22[exponent],
                23 => F64_POW23[exponent],
                24 => F64_POW24[exponent],
                25 => F64_POW25[exponent],
                26 => F64_POW26[exponent],
                27 => F64_POW27[exponent],
                28 => F64_POW28[exponent],
                29 => F64_POW29[exponent],
                30 => F64_POW30[exponent],
                31 => F64_POW31[exponent],
                33 => F64_POW33[exponent],
                34 => F64_POW34[exponent],
                35 => F64_POW35[exponent],
                36 => F64_POW36[exponent],
                // Invalid radix
                _ => unreachable!(),
            }
        }
    }
}

#[cfg(all(test, feature = "power_of_two"))]
mod tests {
    use super::*;

    // These tests are ignored so we can test them on x86_64, where
    // we know powi has some guarantees. table_pow2 assigns directly
    // from bits, and therefore will always be accurate, we
    // just do a smoke test here.

    #[test]
    #[ignore]
    fn test_f32_roundtrip() {
        // Check our logic is correct: by using a large type, we should
        // ensure our table_pow2 function is valid.
        for exp in -149i32..127 {
            let float = f32::table_pow2(exp);
            assert_eq!(float, f64::powi(2.0, exp) as f32);
        }
    }

    #[test]
    #[ignore]
    fn test_f64_roundtrip() {
        for exp in -1074i32..1023 {
            let float = f64::table_pow2(exp);
            if exp > -1023 {
                // Only check for normal floats, powi isn't stable for
                // denormal floats.
                assert_eq!(float, f64::powi(2.0, exp));
            }
        }
    }
}
