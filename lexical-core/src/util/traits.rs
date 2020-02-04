//! Wrap the low-level API into idiomatic serializers.

use super::format::NumberFormat;
use super::num::Number;
use super::result::Result;

// HELPERS

/// Map partial result to complete result.
macro_rules! to_complete {
    ($cb:expr, $bytes:expr $(,$args:expr)*) => {
        match $cb($bytes $(,$args)*) {
            Err(e)                  => Err(e),
            Ok((value, processed))  => if processed == $bytes.len() {
                Ok(value)
            } else{
                Err((ErrorCode::InvalidDigit, processed).into())
            }
        }
    };
}

// FROM LEXICAL

/// Trait for numerical types that can be parsed from bytes.
pub trait FromLexical: Number {
    type Options;

    /// Checked parser for a string-to-number conversion.
    ///
    /// This method parses the entire string, returning an error if
    /// any invalid digits are found during parsing.
    ///
    /// Returns a `Result` containing either the parsed value,
    /// or an error containing any errors that occurred during parsing.
    ///
    /// Numeric overflow takes precedence over the presence of an invalid
    /// digit, and therefore may mask an invalid digit error.
    ///
    /// * `bytes`   - Slice containing a numeric string.
    fn from_lexical(bytes: &[u8]) -> Result<Self>;

    /// Checked parser for a custom string-to-number conversion.
    ///
    /// This method parses the entire string, returning an error if
    /// any invalid digits are found during parsing.
    ///
    /// Returns a `Result` containing either the parsed value,
    /// or an error containing any errors that occurred during parsing.
    ///
    /// Numeric overflow takes precedence over the presence of an invalid
    /// digit, and therefore may mask an invalid digit error.
    ///
    /// * `bytes`   - Slice containing a numeric string.
    /// * `options` - Options to customize parsing the number.
    fn from_lexical_with_options(bytes: &[u8], options: &Self::Options) -> Result<Self>;

    /// Checked parser for a string-to-number conversion.
    ///
    /// This method parses until an invalid digit is found (or the end
    /// of the string), returning the number of processed digits
    /// and the parsed value until that point.
    ///
    /// Returns a `Result` containing either the parsed value
    /// and the number of processed digits, or an error containing
    /// any errors that occurred during parsing.
    ///
    /// * `bytes`   - Slice containing a numeric string.
    fn from_lexical_partial(bytes: &[u8]) -> Result<(Self, usize)>;

    /// Checked parser for a custom string-to-number conversion.
    ///
    /// This method parses until an invalid digit is found (or the end
    /// of the string), returning the number of processed digits
    /// and the parsed value until that point.
    ///
    /// Returns a `Result` containing either the parsed value
    /// and the number of processed digits, or an error containing
    /// any errors that occurred during parsing.
    ///
    /// * `bytes`   - Slice containing a numeric string.
    /// * `options` - Options to customize parsing the number.
    fn from_lexical_partial_with_options(bytes: &[u8], options: &Self::Options) -> Result<(Self, usize)>;

    /// Checked parser for a string-to-number conversion.
    ///
    /// This method parses the entire string, returning an error if
    /// any invalid digits are found during parsing.
    ///
    /// Returns a `Result` containing either the parsed value,
    /// or an error containing any errors that occurred during parsing.
    ///
    /// Numeric overflow takes precedence over the presence of an invalid
    /// digit, and therefore may mask an invalid digit error.
    ///
    /// * `bytes`   - Slice containing a numeric string.
    /// * `radix`   - Radix for the number parsing.
    ///
    /// # Panics
    ///
    /// Panics if the radix is not in the range `[2, 36]`.
    #[cfg(feature = "radix")]
    #[deprecated(
        since = "0.8.0",
        note = "Will be removed with 1.0. Use from_lexical_with_options with Self::Options."
    )]
    fn from_lexical_radix(bytes: &[u8], radix: u8) -> Result<Self>;

    /// Checked parser for a string-to-number conversion.
    ///
    /// This method parses until an invalid digit is found (or the end
    /// of the string), returning the number of processed digits
    /// and the parsed value until that point.
    ///
    /// Returns a `Result` containing either the parsed value
    /// and the number of processed digits, or an error containing
    /// any errors that occurred during parsing.
    ///
    /// * `bytes`   - Slice containing a numeric string.
    /// * `radix`   - Radix for the number parsing.
    ///
    /// # Panics
    ///
    /// Panics if the radix is not in the range `[2, 36]`.
    #[cfg(feature = "radix")]
    #[deprecated(
        since = "0.8.0",
        note = "Will be removed with 1.0. Use from_lexical_partial_with_options with Self::Options."
    )]
    fn from_lexical_partial_radix(bytes: &[u8], radix: u8) -> Result<(Self, usize)>;
}

// TODO(ahuszagh) Need to add exponent_char when applicable for the deprecated
// methods. (Lmao, fuck).

// Implement FromLexical for numeric type.
macro_rules! from_lexical {
    (
        type => $type:ty,
        options => $options:ty,
        parse => $parse:expr,
        parse_with_options => $parse_with_options:expr
    ) => (
        #[allow(deprecated)]        // TODO(ahuszagh) Remove with 1.0
        impl FromLexical for $type {
            type Options = $options;

            #[inline]
            fn from_lexical(bytes: &[u8]) -> Result<$type>
            {
                to_complete!($parse, bytes)
            }

            #[inline]
            fn from_lexical_with_options(bytes: &[u8], options: &$options) -> Result<$type>
            {
                to_complete!($parse_with_options, bytes, options)
            }

            #[inline]
            fn from_lexical_partial(bytes: &[u8]) -> Result<($type, usize)>
            {
                $parse(bytes)
            }

            #[inline]
            fn from_lexical_partial_with_options(bytes: &[u8], options: &$options) -> Result<($type, usize)>
            {
                $parse_with_options(bytes, options)
            }

            #[inline]
            #[cfg(feature = "radix")]
            fn from_lexical_radix(bytes: &[u8], radix: u8) -> Result<$type>
            {
                let options = <$options>::builder()
                    .radix(radix)
                    .exponent_char(exponent_notation_char(radix as u32))
                    .build()
                    .expect("Numerical base must be from 2-36.");
                to_complete!($parse_with_options, bytes, &options)
            }

            #[inline]
            #[cfg(feature = "radix")]
            fn from_lexical_partial_radix(bytes: &[u8], radix: u8) -> Result<($type, usize)>
            {
                let options = <$options>::builder()
                    .radix(radix)
                    .exponent_char(exponent_notation_char(radix as u32))
                    .build()
                    .expect("Numerical base must be from 2-36.");
                $parse_with_options(bytes, &options)
            }
        }

        #[allow(deprecated)]        // TODO(ahuszagh) Remove with 1.0
        #[cfg(feature = "format")]
        impl FromLexicalFormat for $type {
            #[inline]
            fn from_lexical_format(bytes: &[u8], format: NumberFormat) -> Result<$type>
            {
                let options = <$options>::builder()
                    .format(format)
                    .build()
                    .expect("Invalid NumberFormat");
                to_complete!($parse_with_options, bytes, &options)
            }

            #[inline]
            fn from_lexical_partial_format(bytes: &[u8], format: NumberFormat) -> Result<($type, usize)>
            {
                let options = <$options>::builder()
                    .format(format)
                    .build()
                    .expect("Invalid NumberFormat");
                $parse_with_options(bytes, &options)
            }

            #[cfg(feature = "radix")]
            #[inline]
            fn from_lexical_format_radix(bytes: &[u8], radix: u8, format: NumberFormat) -> Result<$type>
            {
                let options = <$options>::builder()
                    .format(format)
                    .radix(radix)
                    .exponent_char(exponent_notation_char(radix as u32))
                    .build()
                    .expect("Invalid NumberFormat or radix");
                to_complete!($parse_with_options, bytes, &options)
            }

            #[cfg(feature = "radix")]
            #[inline]
            fn from_lexical_partial_format_radix(bytes: &[u8], radix: u8, format: NumberFormat) -> Result<($type, usize)>
            {
                let options = <$options>::builder()
                    .format(format)
                    .radix(radix)
                    .exponent_char(exponent_notation_char(radix as u32))
                    .build()
                    .expect("Invalid NumberFormat or radix");
                $parse_with_options(bytes, &options)
            }
        }
    )
}

// FROM LEXICAL LOSSY

/// Trait for floating-point types that can be parsed using lossy algorithms from bytes.
#[deprecated(
    since = "0.8.0",
    note = "Will be removed with 1.0. Use FromLexical with Self::Options."
)]
pub trait FromLexicalLossy: FromLexical {
    /// Lossy, checked parser for a string-to-number conversion.
    ///
    /// This method parses the entire string, returning an error if
    /// any invalid digits are found during parsing. This parser is
    /// lossy, so numerical rounding may occur during parsing.
    ///
    /// Returns a `Result` containing either the parsed value,
    /// or an error containing any errors that occurred during parsing.
    ///
    /// * `bytes`   - Slice containing a numeric string.
    #[deprecated(
        since = "0.8.0",
        note = "Will be removed with 1.0. Use from_lexical_with_options with Self::Options."
    )]
    fn from_lexical_lossy(bytes: &[u8]) -> Result<Self>;

    /// Lossy, checked parser for a string-to-number conversion.
    ///
    /// This method parses until an invalid digit is found (or the end
    /// of the string), returning the number of processed digits
    /// and the parsed value until that point. This parser is
    /// lossy, so numerical rounding may occur during parsing.
    ///
    /// Returns a `Result` containing either the parsed value
    /// and the number of processed digits, or an error containing
    /// any errors that occurred during parsing.
    ///
    /// * `bytes`   - Slice containing a numeric string.
    #[deprecated(
        since = "0.8.0",
        note = "Will be removed with 1.0. Use from_lexical_partial_with_options with Self::Options."
    )]
    fn from_lexical_partial_lossy(bytes: &[u8]) -> Result<(Self, usize)>;

    /// Lossy, checked parser for a string-to-number conversion.
    ///
    /// This method parses the entire string, returning an error if
    /// any invalid digits are found during parsing. This parser is
    /// lossy, so numerical rounding may occur during parsing.
    ///
    /// Returns a `Result` containing either the parsed value,
    /// or an error containing any errors that occurred during parsing.
    ///
    /// * `bytes`   - Slice containing a numeric string.
    /// * `radix`   - Radix for the number parsing.
    ///
    /// # Panics
    ///
    /// Panics if the radix is not in the range `[2, 36]`.
    #[cfg(feature = "radix")]
    #[deprecated(
        since = "0.8.0",
        note = "Will be removed with 1.0. Use from_lexical_with_options with Self::Options."
    )]
    fn from_lexical_lossy_radix(bytes: &[u8], radix: u8) -> Result<Self>;

    /// Lossy, checked parser for a string-to-number conversion.
    ///
    /// This method parses until an invalid digit is found (or the end
    /// of the string), returning the number of processed digits
    /// and the parsed value until that point. This parser is
    /// lossy, so numerical rounding may occur during parsing.
    ///
    /// Returns a `Result` containing either the parsed value
    /// and the number of processed digits, or an error containing
    /// any errors that occurred during parsing.
    ///
    /// * `bytes`   - Slice containing a numeric string.
    /// * `radix`   - Radix for the number parsing.
    ///
    /// # Panics
    ///
    /// Panics if the radix is not in the range `[2, 36]`.
    #[cfg(feature = "radix")]
    #[deprecated(
        since = "0.8.0",
        note = "Will be removed with 1.0. Use from_lexical_partial_with_options with Self::Options."
    )]
    fn from_lexical_partial_lossy_radix(bytes: &[u8], radix: u8) -> Result<(Self, usize)>;
}

// Implement FromLexicalLossy for numeric type.
macro_rules! from_lexical_lossy {
    (
        type => $type:ty,
        options => $options:ty,
        parse_with_options => $parse_with_options:expr
    ) => (
        #[allow(deprecated)]        // TODO(ahuszagh) Remove with 1.0
        impl FromLexicalLossy for $type {
            #[inline]
            fn from_lexical_lossy(bytes: &[u8]) -> Result<$type>
            {
                let options = <$options>::builder()
                    .lossy(true)
                    .build()
                    .expect("Invalid use of lossy parser.");
                to_complete!($parse_with_options, bytes, &options)
            }

            #[inline]
            fn from_lexical_partial_lossy(bytes: &[u8]) -> Result<($type, usize)>
            {
                let options = <$options>::builder()
                    .lossy(true)
                    .build()
                    .expect("Invalid use of lossy parser.");
                $parse_with_options(bytes, &options)
            }

            #[inline]
            #[cfg(feature = "radix")]
            fn from_lexical_lossy_radix(bytes: &[u8], radix: u8) -> Result<$type>
            {
                let options = <$options>::builder()
                    .lossy(true)
                    .radix(radix)
                    .exponent_char(exponent_notation_char(radix as u32))
                    .build()
                    .expect("Numerical base must be from 2-36.");
                to_complete!($parse_with_options, bytes, &options)
            }

            #[inline]
            #[cfg(feature = "radix")]
            fn from_lexical_partial_lossy_radix(bytes: &[u8], radix: u8) -> Result<($type, usize)>
            {
                let options = <$options>::builder()
                    .lossy(true)
                    .radix(radix)
                    .exponent_char(exponent_notation_char(radix as u32))
                    .build()
                    .expect("Numerical base must be from 2-36.");
                $parse_with_options(bytes, &options)
            }
        }

        #[cfg(feature = "format")]
        #[allow(deprecated)]        // TODO(ahuszagh) Remove with 1.0
        impl FromLexicalLossyFormat for $type {
            #[inline]
            fn from_lexical_lossy_format(bytes: &[u8], format: NumberFormat) -> Result<$type>
            {
                let options = <$options>::builder()
                    .lossy(true)
                    .format(format)
                    .build()
                    .expect("Invalid NumberFormat.");
                to_complete!($parse_with_options, bytes, &options)
            }

            #[inline]
            fn from_lexical_partial_lossy_format(bytes: &[u8], format: NumberFormat) -> Result<($type, usize)>
            {
                let options = <$options>::builder()
                    .lossy(true)
                    .format(format)
                    .build()
                    .expect("Invalid NumberFormat.");
                $parse_with_options(bytes, &options)
            }

            #[cfg(feature = "radix")]
            #[inline]
            fn from_lexical_lossy_format_radix(bytes: &[u8], radix: u8, format: NumberFormat) -> Result<$type>
            {
                let options = <$options>::builder()
                    .lossy(true)
                    .format(format)
                    .radix(radix)
                    .exponent_char(exponent_notation_char(radix as u32))
                    .build()
                    .expect("Invalid NumberFormat or radix.");
                to_complete!($parse_with_options, bytes, &options)
            }

            #[cfg(feature = "radix")]
            #[inline]
            fn from_lexical_partial_lossy_format_radix(bytes: &[u8], radix: u8, format: NumberFormat) -> Result<($type, usize)>
            {
                let options = <$options>::builder()
                    .lossy(true)
                    .format(format)
                    .radix(radix)
                    .exponent_char(exponent_notation_char(radix as u32))
                    .build()
                    .expect("Invalid NumberFormat or radix.");
                $parse_with_options(bytes, &options)
            }
        }
    )
}

// FROM LEXICAL FORMAT

/// Trait for number that can be parsed using a custom format specification.
#[cfg(feature = "format")]
#[deprecated(
    since = "0.8.0",
    note = "Will be removed with 1.0. Use FromLexical with Self::Options."
)]
pub trait FromLexicalFormat: FromLexical {
    /// Checked parser for a string-to-number conversion.
    ///
    /// This method parses the entire string, returning an error if
    /// any invalid digits are found during parsing. The numerical format
    /// is specified by the format bitflags, which customize the required
    /// components, digit separators, and other parameters of the number.
    ///
    /// Returns a `Result` containing either the parsed value,
    /// or an error containing any errors that occurred during parsing.
    ///
    /// * `bytes`   - Slice containing a numeric string.
    /// * `format`  - Numerical format.
    #[deprecated(
        since = "0.8.0",
        note = "Will be removed with 1.0. Use from_lexical_with_options with Self::Options."
    )]
    fn from_lexical_format(bytes: &[u8], format: NumberFormat) -> Result<Self>;

    /// Checked parser for a string-to-number conversion.
    ///
    /// This method parses until an invalid digit is found (or the end
    /// of the string), returning the number of processed digits
    /// and the parsed value until that point. The numerical format
    /// is specified by the format bitflags, which customize the required
    /// components, digit separators, and other parameters of the number.
    ///
    /// Returns a `Result` containing either the parsed value
    /// and the number of processed digits, or an error containing
    /// any errors that occurred during parsing.
    ///
    /// * `bytes`   - Slice containing a numeric string.
    /// * `format`  - Numerical format.
    #[deprecated(
        since = "0.8.0",
        note = "Will be removed with 1.0. Use from_lexical_partial_with_options with Self::Options."
    )]
    fn from_lexical_partial_format(bytes: &[u8], format: NumberFormat) -> Result<(Self, usize)>;

    /// Checked parser for a string-to-number conversion.
    ///
    /// This method parses the entire string, returning an error if
    /// any invalid digits are found during parsing. The numerical format
    /// is specified by the format bitflags, which customize the required
    /// components, digit separators, and other parameters of the number.
    ///
    /// Returns a `Result` containing either the parsed value,
    /// or an error containing any errors that occurred during parsing.
    ///
    /// * `bytes`   - Slice containing a numeric string.
    /// * `radix`   - Radix for the number parsing.
    /// * `format`  - Numerical format.
    ///
    /// # Panics
    ///
    /// Panics if the radix is not in the range `[2, 36]`.
    #[cfg(feature = "radix")]
    #[deprecated(
        since = "0.8.0",
        note = "Will be removed with 1.0. Use from_lexical_with_options with Self::Options."
    )]
    fn from_lexical_format_radix(bytes: &[u8], radix: u8, format: NumberFormat) -> Result<Self>;

    /// Checked parser for a string-to-number conversion.
    ///
    /// This method parses until an invalid digit is found (or the end
    /// of the string), returning the number of processed digits
    /// and the parsed value until that point. The numerical format
    /// is specified by the format bitflags, which customize the required
    /// components, digit separators, and other parameters of the number.
    ///
    /// Returns a `Result` containing either the parsed value
    /// and the number of processed digits, or an error containing
    /// any errors that occurred during parsing.
    ///
    /// * `bytes`   - Slice containing a numeric string.
    /// * `radix`   - Radix for the number parsing.
    /// * `format`  - Numerical format.
    ///
    /// # Panics
    ///
    /// Panics if the radix is not in the range `[2, 36]`.
    #[cfg(feature = "radix")]
    #[deprecated(
        since = "0.8.0",
        note = "Will be removed with 1.0. Use from_lexical_partial_with_options with Self::Options."
    )]
    fn from_lexical_partial_format_radix(bytes: &[u8], radix: u8, format: NumberFormat) -> Result<(Self, usize)>;
}

// FROM LEXICAL LOSSY

/// Trait for floating-point types that can be parsed using lossy algorithms with a custom format specification.
#[cfg(feature = "format")]
#[deprecated(
    since = "0.8.0",
    note = "Will be removed with 1.0. Use FromLexical with Self::Options."
)]
pub trait FromLexicalLossyFormat: FromLexical {
    /// Lossy, checked parser for a string-to-number conversion.
    ///
    /// This method parses the entire string, returning an error if
    /// any invalid digits are found during parsing. This parser is
    /// lossy, so numerical rounding may occur during parsing. The
    /// numerical format is specified by the format bitflags, which
    /// customize the required components, digit separators, and other
    /// parameters of the number.
    ///
    /// Returns a `Result` containing either the parsed value,
    /// or an error containing any errors that occurred during parsing.
    ///
    /// * `bytes`   - Slice containing a numeric string.
    /// * `format`  - Numerical format.
    #[deprecated(
        since = "0.8.0",
        note = "Will be removed with 1.0. Use from_lexical_with_options with Self::Options."
    )]
    fn from_lexical_lossy_format(bytes: &[u8], format: NumberFormat) -> Result<Self>;

    /// Lossy, checked parser for a string-to-number conversion.
    ///
    /// This method parses until an invalid digit is found (or the end
    /// of the string), returning the number of processed digits
    /// and the parsed value until that point. This parser is
    /// lossy, so numerical rounding may occur during parsing. The
    /// numerical format is specified by the format bitflags, which
    /// customize the required components, digit separators, and other
    /// parameters of the number.
    ///
    /// Returns a `Result` containing either the parsed value
    /// and the number of processed digits, or an error containing
    /// any errors that occurred during parsing.
    ///
    /// * `bytes`   - Slice containing a numeric string.
    /// * `format`  - Numerical format.
    #[deprecated(
        since = "0.8.0",
        note = "Will be removed with 1.0. Use from_lexical_partial_with_options with Self::Options."
    )]
    fn from_lexical_partial_lossy_format(bytes: &[u8], format: NumberFormat) -> Result<(Self, usize)>;

    /// Lossy, checked parser for a string-to-number conversion.
    ///
    /// This method parses the entire string, returning an error if
    /// any invalid digits are found during parsing. This parser is
    /// lossy, so numerical rounding may occur during parsing. The
    /// numerical format is specified by the format bitflags, which
    /// customize the required components, digit separators, and other
    /// parameters of the number.
    ///
    /// Returns a `Result` containing either the parsed value,
    /// or an error containing any errors that occurred during parsing.
    ///
    /// * `bytes`   - Slice containing a numeric string.
    /// * `radix`   - Radix for the number parsing.
    /// * `format`  - Numerical format.
    ///
    /// # Panics
    ///
    /// Panics if the radix is not in the range `[2, 36]`.
    #[cfg(feature = "radix")]
    #[deprecated(
        since = "0.8.0",
        note = "Will be removed with 1.0. Use from_lexical_with_options with Self::Options."
    )]
    fn from_lexical_lossy_format_radix(bytes: &[u8], radix: u8, format: NumberFormat) -> Result<Self>;

    /// Lossy, checked parser for a string-to-number conversion.
    ///
    /// This method parses until an invalid digit is found (or the end
    /// of the string), returning the number of processed digits
    /// and the parsed value until that point. This parser is
    /// lossy, so numerical rounding may occur during parsing. The
    /// numerical format is specified by the format bitflags, which
    /// customize the required components, digit separators, and other
    /// parameters of the number.
    ///
    /// Returns a `Result` containing either the parsed value
    /// and the number of processed digits, or an error containing
    /// any errors that occurred during parsing.
    ///
    /// * `bytes`   - Slice containing a numeric string.
    /// * `radix`   - Radix for the number parsing.
    /// * `format`  - Numerical format.
    ///
    /// # Panics
    ///
    /// Panics if the radix is not in the range `[2, 36]`.
    #[cfg(feature = "radix")]
    #[deprecated(
        since = "0.8.0",
        note = "Will be removed with 1.0. Use from_lexical_partial_with_options with Self::Options."
    )]
    fn from_lexical_partial_lossy_format_radix(bytes: &[u8], radix: u8, format: NumberFormat) -> Result<(Self, usize)>;
}

// TO LEXICAL

/// Trait for numerical types that can be serialized to bytes.
///
/// To determine the number of bytes required to serialize a value to
/// string, check the associated constants from a required trait:
/// - [`FORMATTED_SIZE`]
/// - [`FORMATTED_SIZE_DECIMAL`]
///
/// [`FORMATTED_SIZE`]: trait.Number.html#associatedconstant.FORMATTED_SIZE
/// [`FORMATTED_SIZE_DECIMAL`]: trait.Number.html#associatedconstant.FORMATTED_SIZE_DECIMAL
pub trait ToLexical: Number {
    /// Associated options type.
    type Options;

    /// Writer for a number-to-string conversion.
    ///
    /// Returns a subslice of the input buffer containing the written bytes,
    /// starting from the same address in memory as the input slice.
    ///
    /// * `value`   - Number to serialize.
    /// * `bytes`   - Slice containing a numeric string.
    ///
    /// # Panics
    ///
    /// Panics if the buffer is not of sufficient size. The caller
    /// must provide a slice of sufficient size. In order to ensure
    /// the function will not panic, ensure the buffer has at least
    /// [`FORMATTED_SIZE_DECIMAL`] elements.
    ///
    /// [`FORMATTED_SIZE_DECIMAL`]: trait.Number.html#associatedconstant.FORMATTED_SIZE_DECIMAL
    fn to_lexical<'a>(self, bytes: &'a mut [u8]) -> &'a mut [u8];

    /// Writer for a custom number-to-string conversion.
    ///
    /// Returns a subslice of the input buffer containing the written bytes,
    /// starting from the same address in memory as the input slice.
    ///
    /// * `value`   - Number to serialize.
    /// * `bytes`   - Slice containing a numeric string.
    /// * `options` - Options to specialize writing the number.
    ///
    /// # Panics
    ///
    /// Panics if the buffer is not of sufficient size. The caller
    /// must provide a slice of sufficient size. In order to ensure
    /// the function will not panic, ensure the buffer has at least
    /// [`FORMATTED_SIZE`] elements.
    ///
    /// [`FORMATTED_SIZE`]: trait.Number.html#associatedconstant.FORMATTED_SIZE
    fn to_lexical_with_options<'a>(self, bytes: &'a mut [u8], options: &Self::Options) -> &'a mut [u8];

    /// Writer for a number-to-string conversion.
    ///
    /// Returns a subslice of the input buffer containing the written bytes,
    /// starting from the same address in memory as the input slice.
    ///
    /// * `value`   - Number to serialize.
    /// * `radix`   - Radix for number encoding.
    /// * `bytes`   - Slice containing a numeric string.
    ///
    /// # Panics
    ///
    /// Panics if the radix is not in the range `[2, 36]`.
    ///
    /// Also panics if the buffer is not of sufficient size. The caller
    /// must provide a slice of sufficient size. In order to ensure
    /// the function will not panic, ensure the buffer has at least
    /// [`FORMATTED_SIZE`] elements.
    ///
    /// [`FORMATTED_SIZE`]: trait.Number.html#associatedconstant.FORMATTED_SIZE
    #[cfg(feature = "radix")]
    #[deprecated(
        since = "0.8.0",
        note = "Will be removed with 1.0. Use to_lexical_with_options with Self::Options."
    )]
    fn to_lexical_radix<'a>(self, radix: u8, bytes: &'a mut [u8]) -> &'a mut [u8];
}

// Implement ToLexical for numeric type.
macro_rules! to_lexical {
    (
        type => $type:ty,
        options => $options:ty,
        write => $write:expr,
        write_with_options => $write_with_options:expr
    ) => (
        #[allow(deprecated)]        // TODO(ahuszagh) Remove with 1.0
        impl ToLexical for $type {
            type Options = $options;

            #[inline]
            fn to_lexical<'a>(self, bytes: &'a mut [u8])
                -> &'a mut [u8]
            {
                assert_buffer!(10, bytes, $type);
                let len = $write(self, bytes);
                &mut index_mut!(bytes[..len])
            }

            #[inline]
            fn to_lexical_with_options<'a>(self, bytes: &'a mut [u8], options: &$options)
                -> &'a mut [u8]
            {
                assert_buffer!(options.radix(), bytes, $type);
                let len = $write_with_options(self, bytes, options);
                &mut index_mut!(bytes[..len])
            }

            #[inline]
            #[cfg(feature = "radix")]
            fn to_lexical_radix<'a>(self, radix: u8, bytes: &'a mut [u8])
                -> &'a mut [u8]
            {
                let options = <$options>::builder()
                    .radix(radix)
                    .exponent_char(exponent_notation_char(radix as u32))
                    .build()
                    .expect("Numerical base must be from 2-36.");
                assert_buffer!(radix, bytes, $type);
                let len = $write_with_options(self, bytes, &options);
                &mut index_mut!(bytes[..len])
            }
        }
    );
}
