//! Big integer type definition.

use crate::float::*;
use crate::traits::*;

use super::alias::{FloatType, MantissaType};
use super::math::*;

// BINARY FACTOR
// -------------

/// Calculate the integral ceiling of the binary factor from a basen number.
#[inline]
pub(super) fn integral_binary_factor(radix: u32) -> u32 {
    debug_assert_radix!(radix);

    #[cfg(not(feature = "radix"))]
    {
        4
    }

    #[cfg(feature = "radix")]
    {
        match radix.as_i32() {
            2 => 1,
            3 => 2,
            4 => 2,
            5 => 3,
            6 => 3,
            7 => 3,
            8 => 3,
            9 => 4,
            10 => 4,
            11 => 4,
            12 => 4,
            13 => 4,
            14 => 4,
            15 => 4,
            16 => 4,
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
            28 => 5,
            29 => 5,
            30 => 5,
            31 => 5,
            32 => 5,
            33 => 6,
            34 => 6,
            35 => 6,
            36 => 6,
            // Invalid radix
            _ => unreachable!(),
        }
    }
}

// BIGINT
// ------

/// Storage for a big integer type.
///
/// This is used for the bhcomp::large_atof and bhcomp::small_atof
/// algorithms. Specifically, it stores all the significant digits
/// scaled to the proper exponent, as an integral type,
/// and then directly compares these digits.
///
/// This requires us to store the number of significant bits, plus the
/// number of exponent bits (required) since we scale everything
/// to the same exponent.
/// This therefore only needs the following number of digits to
/// determine the correct representation (the algorithm can be found in
/// `max_digits` in `bhcomp.rs`):
///  * `bfloat16` - 138
///  * `f16`      - 29
///  * `f32`      - 158
///  * `f64`      - 1092
///  * `f128`     - 16530
#[derive(Clone, PartialEq, Eq)]
#[cfg_attr(test, derive(Debug))]
pub(crate) struct Bigint<F: FloatType> {
    /// Internal storage for the Bigint, in little-endian order.
    pub(crate) data: F::BigintStorage,
}

impl<F: FloatType> Bigint<F> {
    /// Get the upper bits of the mantissa, shifted into place.
    #[inline(always)]
    pub(crate) fn himant(&self) -> (F::MantissaType, bool) {
        F::MantissaType::himant(self.data().as_slice())
    }

    /// Create the container from the the mantissa digits.
    #[inline(always)]
    pub(crate) fn from_mant(mant: F::UnsignedType) -> Self {
        Self::from_uint(mant)
    }
}

impl<F: FloatType> Default for Bigint<F> {
    #[inline]
    fn default() -> Self {
        // We want to avoid lower-order
        let mut bigint = Self {
            data: F::BigintStorage::default(),
        };
        bigint.data.reserve(20);
        bigint
    }
}

impl<F: FloatType> SharedOps for Bigint<F> {
    type StorageType = F::BigintStorage;

    #[inline(always)]
    fn data<'a>(&'a self) -> &'a Self::StorageType {
        &self.data
    }

    #[inline(always)]
    fn data_mut<'a>(&'a mut self) -> &'a mut Self::StorageType {
        &mut self.data
    }
}

impl<F: FloatType> SmallOps for Bigint<F> {
}

impl<F: FloatType> LargeOps for Bigint<F> {
}

// BIGFLOAT
// --------

/// Storage for a big floating-point type.
///
/// This is used for the bigcomp::atof algorithm, which crates a
/// representation of `b+h` and the float scaled into the range `[1, 10)`.
/// This therefore only needs the following number of digits to
/// determine the correct representation (the algorithm can be found in
/// `max_digits` in `bhcomp.rs`):
///  * `bfloat16` - 97
///  * `f16`      - 22
///  * `f32`      - 113
///  * `f64`      - 768
///  * `f128`     - 11564
#[derive(Clone, PartialEq, Eq)]
#[cfg_attr(test, derive(Debug))]
pub(crate) struct Bigfloat<F: FloatType> {
    /// Internal storage for the Bigfloat, in little-endian order.
    ///
    /// Enough storage for up to 10^345, which is 2^1146, or more than
    /// the max for f64.
    pub(crate) data: F::BigfloatStorage,
    /// It also makes sense to store an exponent, since this simplifies
    /// normalizing and powers of 2.
    pub(crate) exp: i32,
}

impl<F: FloatType> Default for Bigfloat<F> {
    #[inline]
    fn default() -> Self {
        // We want to avoid lower-order
        let mut bigfloat = Self {
            data: F::BigfloatStorage::default(),
            exp: 0,
        };
        bigfloat.data.reserve(10);
        bigfloat
    }
}

impl<F: FloatType> SharedOps for Bigfloat<F> {
    type StorageType = F::BigfloatStorage;

    #[inline(always)]
    fn data<'a>(&'a self) -> &'a Self::StorageType {
        &self.data
    }

    #[inline(always)]
    fn data_mut<'a>(&'a mut self) -> &'a mut Self::StorageType {
        &mut self.data
    }
}

impl<F: FloatType> SmallOps for Bigfloat<F> {
    #[inline]
    fn imul_pow2(&mut self, n: u32) {
        // Increment exponent to simulate actual multiplication.
        self.exp += n.as_i32();
    }
}

impl<F: FloatType> LargeOps for Bigfloat<F> {
}

// TO BIGFLOAT
// -----------

/// Simple overloads to allow conversions of extended floats to big integers.
pub(crate) trait ToBigfloat<F: FloatType> {
    fn to_bigfloat(&self) -> Bigfloat<F>;
}

#[cfg(feature = "f16")]
impl ToBigfloat<f16> for ExtendedFloat<<f16 as Float>::Unsigned> {
    #[inline]
    fn to_bigfloat(&self) -> Bigfloat<f16> {
        let mut bigfloat = Bigfloat::<f16>::from_u32(self.mant);
        bigfloat.exp = self.exp;
        bigfloat
    }
}

#[cfg(feature = "f16")]
impl ToBigfloat<bf16> for ExtendedFloat<<bf16 as Float>::Unsigned> {
    #[inline]
    fn to_bigfloat(&self) -> Bigfloat<bf16> {
        let mut bigfloat = Bigfloat::<bf16>::from_u32(self.mant);
        bigfloat.exp = self.exp;
        bigfloat
    }
}

impl ToBigfloat<f32> for ExtendedFloat<<f32 as Float>::Unsigned> {
    #[inline]
    fn to_bigfloat(&self) -> Bigfloat<f32> {
        let mut bigfloat = Bigfloat::<f32>::from_u32(self.mant);
        bigfloat.exp = self.exp;
        bigfloat
    }
}

impl ToBigfloat<f64> for ExtendedFloat<<f64 as Float>::Unsigned> {
    #[inline]
    fn to_bigfloat(&self) -> Bigfloat<f64> {
        let mut bigfloat = Bigfloat::<f64>::from_u64(self.mant);
        bigfloat.exp = self.exp;
        bigfloat
    }
}

#[cfg(feature = "f128")]
impl ToBigfloat<u128> for ExtendedFloat<<f128 as Float>::Unsigned> {
    #[inline]
    fn to_bigfloat(&self) -> Bigfloat<f128> {
        let mut bigfloat = Bigfloat::<f128>::from_u128(self.mant);
        bigfloat.exp = self.exp;
        bigfloat
    }
}

// TESTS
// -----

#[cfg(all(test, feature = "radix"))]
mod test {
    use super::*;

    #[test]
    fn integral_binary_factor_test() {
        const TABLE: [u32; 35] = [
            1, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 4, 4, 4, 4, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
            5, 5, 6, 6, 6, 6,
        ];
        for (idx, base) in (2..37).enumerate() {
            assert_eq!(integral_binary_factor(base), TABLE[idx]);
        }
    }
}
