//! Cached powers trait for extended-precision floats.

use crate::float::*;

// Cached powers
mod float80;
mod float80_decimal;
#[cfg(feature = "radix")]
mod float80_radix;

cfg_if! {
if #[cfg(feature = "f128")] {
    mod float160;
    mod float160_decimal;
    #[cfg(feature = "radix")]
    mod float160_radix;
}} // cfg_if

// POWERS
// ------

/// Precalculated powers that uses two-separate arrays for memory-efficiency.
#[doc(hidden)]
pub(crate) struct ExtendedFloatArray<M: Mantissa> {
    // Pre-calculated mantissa for the powers.
    pub mant: &'static [M],
    // Pre-calculated binary exponents for the powers.
    pub exp: &'static [i32],
}

/// Allow indexing of values without bounds checking
impl<M: Mantissa> ExtendedFloatArray<M> {
    #[inline]
    pub fn get_extended_float(&self, index: usize) -> ExtendedFloat<M> {
        let mant = self.mant[index];
        let exp = self.exp[index];
        ExtendedFloat {
            mant,
            exp,
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.mant.len()
    }
}

// MODERATE PATH POWERS
// --------------------

/// Precalculated powers of base N for the moderate path.
#[doc(hidden)]
pub(crate) struct ModeratePathPowers<M: Mantissa> {
    // Pre-calculated small powers.
    pub small: ExtendedFloatArray<M>,
    // Pre-calculated large powers.
    pub large: ExtendedFloatArray<M>,
    /// Pre-calculated small powers as 64-bit integers
    pub small_int: &'static [M],
    // Step between large powers and number of small powers.
    pub step: i32,
    // Exponent bias for the large powers.
    pub bias: i32,
}

/// Allow indexing of values without bounds checking
impl<M: Mantissa> ModeratePathPowers<M> {
    #[inline]
    pub fn get_small(&self, index: usize) -> ExtendedFloat<M> {
        self.small.get_extended_float(index)
    }

    #[inline]
    pub fn get_large(&self, index: usize) -> ExtendedFloat<M> {
        self.large.get_extended_float(index)
    }

    #[inline]
    pub fn get_small_int(&self, index: usize) -> M {
        self.small_int[index]
    }
}

// CACHED EXTENDED POWERS
// ----------------------

/// Cached powers as a trait for a floating-point type.
pub(crate) trait ModeratePathCache<M: Mantissa> {
    /// Get powers from radix.
    fn get_powers(radix: u32) -> &'static ModeratePathPowers<M>;
}

impl ModeratePathCache<u64> for ExtendedFloat<u64> {
    #[inline]
    fn get_powers(radix: u32) -> &'static ModeratePathPowers<u64> {
        float80::get_powers(radix)
    }
}

#[cfg(feature = "f128")]
impl ModeratePathCache<u128> for ExtendedFloat<u128> {
    #[inline]
    fn get_powers(radix: u32) -> &'static ModeratePathPowers<u128> {
        float160::get_powers(radix)
    }
}
