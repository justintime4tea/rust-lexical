//! Configuration options for parsing and formatting numbers.

use super::config::*;
use super::format::NumberFormat;
use super::rounding::RoundingKind;

// CONSTANTS
// ---------

// TODO(ahuszagh)
//  Should also have predefined constants, like the NumberFormat.
//  Should simplify things.

// Constants to dictate default values for options.
pub(crate) const DEFAULT_EXPONENT_CHAR: u8 = b'e';
pub(crate) const DEFAULT_FORMAT: NumberFormat = NumberFormat::STANDARD;
pub(crate) const DEFAULT_INF_STRING: &'static [u8] = b"inf";
pub(crate) const DEFAULT_INFINITY_STRING: &'static [u8] = b"infinity";
pub(crate) const DEFAULT_LOSSY: bool = false;
pub(crate) const DEFAULT_NAN_STRING: &'static [u8] = b"NaN";
pub(crate) const DEFAULT_RADIX: u8 = 10;
pub(crate) const DEFAULT_ROUNDING: RoundingKind = RoundingKind::NearestTieEven;
pub(crate) const DEFAULT_TRIM_FLOATS: bool = false;

// HELPERS
// -------

// Check if byte array starts with case-insensitive N.
#[inline]
fn starts_with_n(bytes: &[u8]) -> bool {
    match bytes.get(0) {
        Some(&b'N') => true,
        Some(&b'n') => true,
        _           => false,
    }
}

// Check if byte array starts with case-insensitive I.
#[inline]
fn starts_with_i(bytes: &[u8]) -> bool {
    match bytes.get(0) {
        Some(&b'I') => true,
        Some(&b'i') => true,
        _           => false,
    }
}

/// Get radix if radix is valid.
#[cfg(feature = "radix")]
#[inline(always)]
fn to_radix(radix: u8) -> Option<u32> {
    let radix = radix as u32;
    match radix >= 2 && radix <= 36 {
        false   => None,
        true    => Some(radix)
    }
}

/// Get radix if radix is valid.
#[cfg(not(feature = "radix"))]
#[inline(always)]
fn to_radix(radix: u8) -> Option<u32> {
    let radix = radix as u32;
    match radix == 10 {
        false   => None,
        true    => Some(radix)
    }
}

/// Get exponent character if character is valid.
#[inline(always)]
fn to_exponent_char(exponent_char: u8, radix: u32) -> Option<u8> {
    match (exponent_char as char).to_digit(radix) {
        None => Some(exponent_char),
        _    => None
    }
}

/// Get number format if format is valid.
#[inline(always)]
fn to_format_integer(format: NumberFormat, radix: u32) -> Option<NumberFormat> {
    let digit_separator = format.digit_separator();
    match (digit_separator as char).to_digit(radix) {
        None => Some(format),
        _    => None
    }
}

/// Get number format if format is valid.
#[inline(always)]
fn to_format_float(format: NumberFormat, radix: u32, exponent_char: u8) -> Option<NumberFormat> {
    let digit_separator = format.digit_separator();
    let is_valid_digit = (digit_separator as char).to_digit(radix).is_none();
    match is_valid_digit && digit_separator != exponent_char {
        true    => Some(format),
        false   => None
    }
}

/// Get rounding if rounding is valid.
#[cfg(feature = "rounding")]
#[inline(always)]
fn to_rounding(rounding: RoundingKind) -> Option<RoundingKind> {
    Some(rounding)
}

/// Get rounding if rounding is valid.
#[cfg(not(feature = "rounding"))]
#[inline(always)]
fn to_rounding(rounding: RoundingKind) -> Option<RoundingKind> {
    match rounding == RoundingKind::NearestTieEven {
        false   => None,
        true    => Some(rounding)
    }
}

/// Get nan string if string is valid.
#[inline(always)]
fn to_nan_string(nan_string: &'static [u8]) -> Option<&'static [u8]> {
    match starts_with_n(nan_string) {
        true    => Some(nan_string),
        false   => None
    }
}

/// Get inf string if string is valid.
#[inline(always)]
fn to_inf_string(inf_string: &'static [u8]) -> Option<&'static [u8]> {
    match starts_with_i(inf_string) {
        true    => Some(inf_string),
        false   => None
    }
}

/// Get infinity string if string is valid.
#[inline(always)]
fn to_infinity_string(infinity_string: &'static [u8], inf_string: &'static [u8])
    -> Option<&'static [u8]>
{
    let longer = infinity_string.len() >= inf_string.len();
    let starts_with = starts_with_i(infinity_string);
    match longer && starts_with {
        true    => Some(infinity_string),
        false   => None
    }
}

// PARSE INTEGER
// -------------

/// Options to customize parsing integers.
#[derive(Clone, Debug)]
pub struct ParseIntegerOptions {
    /// Radix for integer string.
    radix: u32,

    /// Number format.
    format: NumberFormat
}

impl ParseIntegerOptions {
    // CONSTRUCTORS

    /// Create new options using default values.
    // TODO(ahuszagh) Make const fn when we deprecate the older methods.
    #[inline(always)]
    pub fn new() -> ParseIntegerOptions {
        ParseIntegerOptions {
            radix: DEFAULT_RADIX as u32,
            format: DEFAULT_FORMAT
        }
    }

    /// Create new options from radix and default values.
    #[cfg(feature = "radix")]
    #[inline(always)]
    #[deprecated(since = "0.8.0", note = "Will be removed with 1.0.")]
    pub(crate) fn from_radix(radix: u8) -> Option<ParseIntegerOptions> {
        let radix = to_radix(radix)?;
        Some(ParseIntegerOptions {
            radix: radix,
            format: DEFAULT_FORMAT
        })
    }

    /// Create new options from format and default values.
    #[cfg(feature = "format")]
    #[inline(always)]
    #[deprecated(since = "0.8.0", note = "Will be removed with 1.0.")]
    pub(crate) fn from_format(format: NumberFormat) -> Option<ParseIntegerOptions> {
        let radix = DEFAULT_RADIX as u32;
        let format = to_format_integer(format, radix)?;
        Some(ParseIntegerOptions {
            radix: radix,
            format: format
        })
    }

    /// Create new options from format and radix.
    #[cfg(all(feature = "format", feature = "radix"))]
    #[inline(always)]
    #[deprecated(since = "0.8.0", note = "Will be removed with 1.0.")]
    pub(crate) fn from_format_and_radix(format: NumberFormat, radix: u8) -> Option<ParseIntegerOptions> {
        Self::create(radix, format)
    }

    /// Create new options from fields.
    ///
    /// * `radix`   - Radix for the number parsing.
    /// * `format`  - Numerical format.
    pub fn create(radix: u8, format: NumberFormat) -> Option<ParseIntegerOptions> {
        let radix = to_radix(radix)?;
        let format = to_format_integer(format, radix)?;
        Some(ParseIntegerOptions { radix, format })
    }

    // PRE-DEFINED CONSTANTS

    /// Create new options to parse the default binary format.
    #[inline(always)]
    #[cfg(feature = "radix")]
    pub fn binary() -> ParseIntegerOptions {
        ParseIntegerOptions {
            radix: 2,
            format: DEFAULT_FORMAT
        }
    }

    /// Create new options to parse the default decimal format.
    #[inline(always)]
    pub fn decimal() -> ParseIntegerOptions {
        ParseIntegerOptions {
            radix: 10,
            format: DEFAULT_FORMAT
        }
    }

    /// Create new options to parse the default hexadecimal format.
    #[inline(always)]
    #[cfg(feature = "radix")]
    pub fn hexadecimal() -> ParseIntegerOptions {
        ParseIntegerOptions {
            radix: 16,
            format: DEFAULT_FORMAT
        }
    }

    // GETTERS

    /// Get the radix.
    #[inline(always)]
    pub const fn radix(&self) -> u32 {
        self.radix
    }

    /// Get the number format.
    #[inline(always)]
    pub const fn format(&self) -> NumberFormat {
        self.format
    }
}

impl Default for ParseIntegerOptions {
    fn default() -> ParseIntegerOptions {
        ParseIntegerOptions::new()
    }
}

// PARSE FLOAT
// -----------

/// Options to customize parsing floats.
#[derive(Clone, Debug)]
pub struct ParseFloatOptions {
    /// Use the lossy, fast parser.
    lossy: bool,

    /// Character to designate exponent component.
    exponent_char: u8,

    /// Radix for float string.
    radix: u32,

    /// Number format.
    format: NumberFormat,

    /// Rounding kind for float.
    rounding: RoundingKind,

    /// String representation of Not A Number.
    nan_string: &'static [u8],

    /// String representation of short infinity.
    inf_string: &'static [u8],

    /// String representation of long infinity.
    infinity_string: &'static [u8],
}

#[allow(deprecated)]    // TODO(ahuszagh) Remove with 1.0.
impl ParseFloatOptions {
    // CONSTRUCTORS

    /// Create new options using default values.
    // TODO(ahuszagh) Make const fn when we deprecate the older methods.
    #[inline(always)]
    pub fn new() -> ParseFloatOptions {
        let radix = DEFAULT_RADIX as u32;
        ParseFloatOptions {
            lossy: DEFAULT_LOSSY,
            exponent_char: exponent_notation_char(radix),
            radix: radix,
            format: DEFAULT_FORMAT,
            rounding: get_float_rounding(),
            nan_string: get_nan_string(),
            inf_string: get_inf_string(),
            infinity_string: get_infinity_string()
        }
    }

    /// Create new options from lossy and default values.
    #[inline(always)]
    #[deprecated(since = "0.8.0", note = "Will be removed with 1.0.")]
    pub(crate) fn from_lossy(lossy: bool) -> Option<ParseFloatOptions> {
        let radix = DEFAULT_RADIX as u32;
        Some(ParseFloatOptions {
            lossy: lossy,
            exponent_char: exponent_notation_char(radix),
            radix: radix,
            format: DEFAULT_FORMAT,
            rounding: get_float_rounding(),
            nan_string: get_nan_string(),
            inf_string: get_inf_string(),
            infinity_string: get_infinity_string()
        })
    }

    /// Create new options from radix and default values.
    #[inline(always)]
    #[cfg(feature = "radix")]
    #[deprecated(since = "0.8.0", note = "Will be removed with 1.0.")]
    pub(crate) fn from_radix(radix: u8) -> Option<ParseFloatOptions> {
        let radix = to_radix(radix)?;
        Some(ParseFloatOptions {
            lossy: DEFAULT_LOSSY,
            exponent_char: exponent_notation_char(radix),
            radix: radix,
            format: DEFAULT_FORMAT,
            rounding: get_float_rounding(),
            nan_string: get_nan_string(),
            inf_string: get_inf_string(),
            infinity_string: get_infinity_string()
        })
    }

    /// Create new options from format and default values.
    #[inline(always)]
    #[cfg(feature = "format")]
    #[deprecated(since = "0.8.0", note = "Will be removed with 1.0.")]
    pub(crate) fn from_format(format: NumberFormat) -> Option<ParseFloatOptions> {
        let radix = DEFAULT_RADIX as u32;
        let exponent_char = exponent_notation_char(radix);
        let format = to_format_float(format, radix, exponent_char)?;
        Some(ParseFloatOptions {
            lossy: DEFAULT_LOSSY,
            exponent_char: exponent_notation_char(radix),
            radix: radix,
            format: format,
            rounding: get_float_rounding(),
            nan_string: get_nan_string(),
            inf_string: get_inf_string(),
            infinity_string: get_infinity_string()
        })
    }

    /// Create new options from format and radix.
    #[cfg(all(feature = "format", feature = "radix"))]
    #[inline(always)]
    #[deprecated(since = "0.8.0", note = "Will be removed with 1.0.")]
    pub(crate) fn from_format_and_radix(format: NumberFormat, radix: u8) -> Option<ParseFloatOptions> {
        let radix = to_radix(radix)?;
        let exponent_char = exponent_notation_char(radix);
        let format = to_format_float(format, radix, exponent_char)?;
        Some(ParseFloatOptions {
            lossy: DEFAULT_LOSSY,
            exponent_char: exponent_char,
            radix: radix,
            format: format,
            rounding: get_float_rounding(),
            nan_string: get_nan_string(),
            inf_string: get_inf_string(),
            infinity_string: get_infinity_string()
        })
    }

    /// Create new options from lossy, radix and default values.
    #[inline(always)]
    #[cfg(feature = "radix")]
    #[deprecated(since = "0.8.0", note = "Will be removed with 1.0.")]
    pub(crate) fn from_lossy_and_radix(lossy: bool, radix: u8) -> Option<ParseFloatOptions> {
        let radix = to_radix(radix)?;
        Some(ParseFloatOptions {
            lossy: lossy,
            exponent_char: exponent_notation_char(radix),
            radix: radix,
            format: DEFAULT_FORMAT,
            rounding: get_float_rounding(),
            nan_string: get_nan_string(),
            inf_string: get_inf_string(),
            infinity_string: get_infinity_string()
        })
    }

    /// Create new options from lossy, format and default values.
    #[inline(always)]
    #[cfg(feature = "format")]
    #[deprecated(since = "0.8.0", note = "Will be removed with 1.0.")]
    pub(crate) fn from_lossy_and_format(lossy: bool, format: NumberFormat) -> Option<ParseFloatOptions> {
        let radix = DEFAULT_RADIX as u32;
        let exponent_char = exponent_notation_char(radix);
        let format = to_format_float(format, radix, exponent_char)?;
        Some(ParseFloatOptions {
            lossy: lossy,
            exponent_char: exponent_notation_char(radix),
            radix: radix,
            format: format,
            rounding: get_float_rounding(),
            nan_string: get_nan_string(),
            inf_string: get_inf_string(),
            infinity_string: get_infinity_string()
        })
    }

    /// Create new options from lossy, format and radix.
    #[cfg(all(feature = "format", feature = "radix"))]
    #[inline(always)]
    #[deprecated(since = "0.8.0", note = "Will be removed with 1.0.")]
    pub(crate) fn from_lossy_and_format_and_radix(lossy: bool, format: NumberFormat, radix: u8)
        -> Option<ParseFloatOptions>
    {
        let radix = to_radix(radix)?;
        let exponent_char = exponent_notation_char(radix);
        let format = to_format_float(format, radix, exponent_char)?;
        Some(ParseFloatOptions {
            lossy: lossy,
            exponent_char: exponent_char,
            radix: radix,
            format: format,
            rounding: get_float_rounding(),
            nan_string: get_nan_string(),
            inf_string: get_inf_string(),
            infinity_string: get_infinity_string()
        })
    }

    /// Create new options from fields.
    ///
    /// * `lossy`           - Use the lossy, fast parser.
    /// * `exponent_char`   - Character to designate exponent component.
    /// * `radix`           - Radix for the number parsing.
    /// * `format`          - Numerical format.
    /// * `rounding`        - IEEE754 Rounding kind for float.
    /// * `nan_string`      - String representation of Not A Number.
    /// * `inf_string`      - String representation of short infinity.
    /// * `infinity_string` - String representation of long infinity.
    #[inline(always)]
    pub fn create(
        lossy: bool,
        exponent_char: u8,
        radix: u8,
        format: NumberFormat,
        rounding: RoundingKind,
        nan_string: &'static [u8],
        inf_string: &'static [u8],
        infinity_string: &'static [u8]
    ) -> Option<ParseFloatOptions> {
        let radix = to_radix(radix)?;
        let exponent_char = to_exponent_char(exponent_char, radix)?;
        let format = to_format_float(format, radix, exponent_char)?;
        let rounding = to_rounding(rounding)?;
        let nan_string = to_nan_string(nan_string)?;
        let inf_string = to_inf_string(inf_string)?;
        let infinity_string = to_infinity_string(infinity_string, inf_string)?;
        Some(ParseFloatOptions {
            lossy: lossy,
            exponent_char: exponent_char,
            radix: radix,
            format: format,
            rounding: rounding,
            nan_string: nan_string,
            inf_string: inf_string,
            infinity_string: infinity_string
        })
    }

    // PRE-DEFINED CONSTANTS

    /// Create new options to parse the default binary format.
    #[inline(always)]
    #[cfg(feature = "radix")]
    pub fn binary() -> ParseFloatOptions {
        ParseFloatOptions {
            lossy: false,
            exponent_char: DEFAULT_EXPONENT_CHAR,
            radix: 2,
            format: DEFAULT_FORMAT,
            rounding: DEFAULT_ROUNDING,
            nan_string: DEFAULT_NAN_STRING,
            inf_string: DEFAULT_INF_STRING,
            infinity_string: DEFAULT_INFINITY_STRING
        }
    }

    /// Create new options to parse the default decimal format.
    #[inline(always)]
    pub fn decimal() -> ParseFloatOptions {
        ParseFloatOptions {
            lossy: false,
            exponent_char: DEFAULT_EXPONENT_CHAR,
            radix: 10,
            format: DEFAULT_FORMAT,
            rounding: DEFAULT_ROUNDING,
            nan_string: DEFAULT_NAN_STRING,
            inf_string: DEFAULT_INF_STRING,
            infinity_string: DEFAULT_INFINITY_STRING
        }
    }

    /// Create new options to parse the default hexadecimal format.
    #[inline(always)]
    #[cfg(feature = "radix")]
    pub fn hexadecimal() -> ParseFloatOptions {
        ParseFloatOptions {
            lossy: false,
            exponent_char: b'p',
            radix: 16,
            format: DEFAULT_FORMAT,
            rounding: DEFAULT_ROUNDING,
            nan_string: DEFAULT_NAN_STRING,
            inf_string: DEFAULT_INF_STRING,
            infinity_string: DEFAULT_INFINITY_STRING
        }
    }

    // GETTERS

    /// Get if we're using the lossy parser.
    #[inline(always)]
    pub const fn lossy(&self) -> bool {
        self.lossy
    }

    /// Get the exponent char.
    #[inline(always)]
    pub const fn exponent_char(&self) -> u8 {
        self.exponent_char
    }

    /// Get the string to represent NaN.
    #[inline(always)]
    pub const fn nan_string(&self) -> &'static [u8] {
        self.nan_string
    }

    /// Get the string to represent short infinity.
    #[inline(always)]
    pub const fn inf_string(&self) -> &'static [u8] {
        self.inf_string
    }

    /// Get the string to represent long infinity.
    #[inline(always)]
    pub const fn infinity_string(&self) -> &'static [u8] {
        self.infinity_string
    }

    /// Get the radix.
    #[inline(always)]
    pub const fn radix(&self) -> u32 {
        self.radix
    }

    /// Get the number format.
    #[inline(always)]
    pub const fn format(&self) -> NumberFormat {
        self.format
    }

    /// Get the rounding kind for float.
    #[inline(always)]
    pub const fn rounding(&self) -> RoundingKind {
        self.rounding
    }
}

// WRITE INTEGER
// -------------

/// Immutable options to customize writing integers.
#[derive(Clone, Debug)]
pub struct WriteIntegerOptions {
    /// Radix for integer string.
    radix: u32,
}

impl WriteIntegerOptions {
    // CONSTRUCTORS

    /// Create new options using default values.
    // TODO(ahuszagh) Make const fn when we deprecate the older methods.
    #[inline(always)]
    pub fn new() -> WriteIntegerOptions {
        WriteIntegerOptions {
            radix: DEFAULT_RADIX as u32
        }
    }

    /// Create new options from radix and default values.
    #[inline(always)]
    #[cfg(feature = "radix")]
    #[deprecated(since = "0.8.0", note = "Will be removed with 1.0.")]
    pub(crate) fn from_radix(radix: u8) -> Option<WriteIntegerOptions> {
        let radix = to_radix(radix)?;
        Some(WriteIntegerOptions { radix })
    }

    /// Create new options from fields.
    ///
    /// * `radix`   - Radix for the number parsing.
    pub fn create(radix: u8) -> Option<WriteIntegerOptions> {
        let radix = to_radix(radix)?;
        Some(WriteIntegerOptions { radix })
    }

    // GETTERS

    /// Get the radix.
    #[inline(always)]
    pub const fn radix(&self) -> u32 {
        self.radix
    }
}

// WRITE FLOAT
// -----------

/// Options to customize writing floats.
#[derive(Clone, Debug)]
pub struct WriteFloatOptions {
    /// Character to designate exponent component.
    /// Warning: This is currently ignored if the radix is 10.
    exponent_char: u8,

    /// Radix for float string.
    radix: u32,

    /// Trim the trailing ".0" from integral float strings.
    trim_floats: bool,

    /// String representation of Not A Number as a byte string.
    nan_string: &'static [u8],

    /// String representation of short infinity as a byte string.
    inf_string: &'static [u8],
}

#[allow(deprecated)]    // TODO(ahuszagh) Remove with 1.0.
impl WriteFloatOptions {
    // CONSTRUCTORS

    /// Create new options using default values.
    // TODO(ahuszagh) Make const fn when we deprecate the older methods.
    #[inline(always)]
    pub fn new() -> WriteFloatOptions {
        let radix = DEFAULT_RADIX as u32;
        WriteFloatOptions {
            exponent_char: exponent_notation_char(radix),
            radix: radix,
            trim_floats: DEFAULT_TRIM_FLOATS,
            nan_string: get_nan_string(),
            inf_string: get_inf_string()
        }
    }

    /// Create new options from radix and default values.
    #[inline(always)]
    #[cfg(feature = "radix")]
    #[deprecated(since = "0.8.0", note = "Will be removed with 1.0.")]
    pub(crate) fn from_radix(radix: u8) -> Option<WriteFloatOptions> {
        let radix = to_radix(radix)?;
        Some(WriteFloatOptions {
            exponent_char: exponent_notation_char(radix),
            radix: radix,
            trim_floats: DEFAULT_TRIM_FLOATS,
            nan_string: get_nan_string(),
            inf_string: get_inf_string()
        })
    }

    /// Create new options from fields.
    ///
    /// * `exponent_char`   - Character to designate exponent component.
    /// * `radix`           - Radix for the number parsing.
    /// * `trim_floats`     - Trim the trailing ".0" from integral float strings.
    /// * `nan_string`      - String representation of Not A Number.
    /// * `inf_string`      - String representation of short infinity.
    #[inline(always)]
    pub fn create(
        exponent_char: u8,
        radix: u8,
        trim_floats: bool,
        nan_string: &'static [u8],
        inf_string: &'static [u8],
    ) -> Option<WriteFloatOptions> {
        let radix = to_radix(radix)?;
        let exponent_char = to_exponent_char(exponent_char, radix)?;
        let nan_string = to_nan_string(nan_string)?;
        let inf_string = to_inf_string(inf_string)?;
        Some(WriteFloatOptions {
            exponent_char: exponent_char,
            radix: radix,
            trim_floats: trim_floats,
            nan_string: nan_string,
            inf_string: inf_string
        })
    }

    // PRE-DEFINED CONSTANTS

    /// Create new options to parse the default binary format.
    #[inline(always)]
    #[cfg(feature = "radix")]
    pub fn binary() -> WriteFloatOptions {
        WriteFloatOptions {
            exponent_char: DEFAULT_EXPONENT_CHAR,
            radix: 2,
            trim_floats: false,
            nan_string: DEFAULT_NAN_STRING,
            inf_string: DEFAULT_INF_STRING
        }
    }

    /// Create new options to parse the default decimal format.
    #[inline(always)]
    pub fn decimal() -> WriteFloatOptions {
        WriteFloatOptions {
            exponent_char: DEFAULT_EXPONENT_CHAR,
            radix: 10,
            trim_floats: false,
            nan_string: DEFAULT_NAN_STRING,
            inf_string: DEFAULT_INF_STRING
        }
    }

    /// Create new options to parse the default hexadecimal format.
    #[inline(always)]
    #[cfg(feature = "radix")]
    pub fn hexadecimal() -> WriteFloatOptions {
        WriteFloatOptions {
            exponent_char: b'p',
            radix: 16,
            trim_floats: false,
            nan_string: DEFAULT_NAN_STRING,
            inf_string: DEFAULT_INF_STRING
        }
    }

    // GETTERS

    /// Get the exponent char.
    #[inline(always)]
    pub const fn exponent_char(&self) -> u8 {
        self.exponent_char
    }

    /// Get the radix.
    #[inline(always)]
    pub const fn radix(&self) -> u32 {
        self.radix
    }

    /// Get if trailing ".0" is trimmed from integral float strings.
    #[inline(always)]
    pub const fn trim_floats(&self) -> bool {
        self.trim_floats
    }

    /// Get the string to represent NaN.
    #[inline(always)]
    pub const fn nan_string(&self) -> &'static [u8] {
        self.nan_string
    }

    /// Get the string to represent short infinity.
    #[inline(always)]
    pub const fn inf_string(&self) -> &'static [u8] {
        self.inf_string
    }
}

// TESTS
// -----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "radix")]
    fn to_radix_test() {
        assert_eq!(to_radix(1), None);
        assert_eq!(to_radix(2), Some(2));
        assert_eq!(to_radix(10), Some(10));
        assert_eq!(to_radix(36), Some(36));
        assert_eq!(to_radix(37), None);
    }

    #[test]
    #[cfg(not(feature = "radix"))]
    fn to_radix_test() {
        assert_eq!(to_radix(1), None);
        assert_eq!(to_radix(2), None);
        assert_eq!(to_radix(10), Some(10));
        assert_eq!(to_radix(36), None);
        assert_eq!(to_radix(37), None);
    }

    #[test]
    fn to_exponent_char_test() {
        assert_eq!(to_exponent_char(b'2', 2), Some(b'2'));
        assert_eq!(to_exponent_char(b'e', 2), Some(b'e'));
        assert_eq!(to_exponent_char(b'p', 2), Some(b'p'));
        assert_eq!(to_exponent_char(b'z', 2), Some(b'z'));

        assert_eq!(to_exponent_char(b'2', 10), None);
        assert_eq!(to_exponent_char(b'e', 10), Some(b'e'));
        assert_eq!(to_exponent_char(b'p', 10), Some(b'p'));
        assert_eq!(to_exponent_char(b'z', 10), Some(b'z'));

        assert_eq!(to_exponent_char(b'2', 16), None);
        assert_eq!(to_exponent_char(b'e', 16), None);
        assert_eq!(to_exponent_char(b'p', 16), Some(b'p'));
        assert_eq!(to_exponent_char(b'z', 16), Some(b'z'));

        assert_eq!(to_exponent_char(b'2', 36), None);
        assert_eq!(to_exponent_char(b'e', 36), None);
        assert_eq!(to_exponent_char(b'p', 36), None);
        assert_eq!(to_exponent_char(b'z', 36), None);
    }

    #[test]
    #[cfg(feature = "format")]
    fn to_format_integer_test() {
        let f1 = NumberFormat::ignore(b'0').unwrap();
        let f2 = NumberFormat::ignore(b'2').unwrap();
        let f3 = NumberFormat::ignore(b'e').unwrap();
        let f4 = NumberFormat::ignore(b'_').unwrap();

        assert_eq!(to_format_integer(f1, 2), None);
        assert_eq!(to_format_integer(f2, 2), Some(f2));
        assert_eq!(to_format_integer(f3, 2), Some(f3));
        assert_eq!(to_format_integer(f4, 2), Some(f4));

        assert_eq!(to_format_integer(f1, 10), None);
        assert_eq!(to_format_integer(f2, 10), None);
        assert_eq!(to_format_integer(f3, 10), Some(f3));
        assert_eq!(to_format_integer(f4, 10), Some(f4));

        assert_eq!(to_format_integer(f1, 16), None);
        assert_eq!(to_format_integer(f2, 16), None);
        assert_eq!(to_format_integer(f3, 16), None);
        assert_eq!(to_format_integer(f4, 16), Some(f4));
    }

    #[test]
    #[cfg(feature = "format")]
    fn to_format_float_test() {
        let f1 = NumberFormat::ignore(b'0').unwrap();
        let f2 = NumberFormat::ignore(b'2').unwrap();
        let f3 = NumberFormat::ignore(b'e').unwrap();
        let f4 = NumberFormat::ignore(b'_').unwrap();

        assert_eq!(to_format_float(f1, 2, b'e'), None);
        assert_eq!(to_format_float(f2, 2, b'e'), Some(f2));
        assert_eq!(to_format_float(f3, 2, b'e'), None);
        assert_eq!(to_format_float(f4, 2, b'e'), Some(f4));

        assert_eq!(to_format_float(f1, 10, b'e'), None);
        assert_eq!(to_format_float(f2, 10, b'e'), None);
        assert_eq!(to_format_float(f3, 10, b'e'), None);
        assert_eq!(to_format_float(f4, 10, b'e'), Some(f4));

        assert_eq!(to_format_float(f1, 16, b'p'), None);
        assert_eq!(to_format_float(f2, 16, b'p'), None);
        assert_eq!(to_format_float(f3, 16, b'p'), None);
        assert_eq!(to_format_float(f4, 16, b'p'), Some(f4));
    }

    #[test]
    fn to_nan_string_test() {
        assert_eq!(to_nan_string(b!("inf")), None);
        assert_eq!(to_nan_string(b!("nan")), Some(b!("nan")));
        assert_eq!(to_nan_string(b!("NAN")), Some(b!("NAN")));
    }

    #[test]
    fn to_inf_string_test() {
        assert_eq!(to_inf_string(b!("nan")), None);
        assert_eq!(to_inf_string(b!("inf")), Some(b!("inf")));
        assert_eq!(to_inf_string(b!("INF")), Some(b!("INF")));
    }

    #[test]
    fn to_infinity_string_test() {
        assert_eq!(to_infinity_string(b!("nan"), b!("inf")), None);
        assert_eq!(to_infinity_string(b!("in"), b!("inf")), None);
        assert_eq!(to_infinity_string(b!("IN"), b!("inf")), None);
        assert_eq!(to_infinity_string(b!("na"), b!("inf")), None);
        assert_eq!(to_infinity_string(b!("infinity"), b!("inf")), Some(b!("infinity")));
        assert_eq!(to_infinity_string(b!("INFINITY"), b!("inf")), Some(b!("INFINITY")));
    }

    #[test]
    #[cfg(feature = "format")]
    fn parse_integer_options_invalid_digit_separator_test() {
        let format = NumberFormat::ignore(b'0').unwrap();
        let options = ParseIntegerOptions::create(10, format);
        assert!(options.is_none());

        let format = NumberFormat::ignore(b'9').unwrap();
        let options = ParseIntegerOptions::create(10, format);
        assert!(options.is_none());

        let format = NumberFormat::ignore(b'A').unwrap();
        let options = ParseIntegerOptions::create(10, format);
        assert!(options.is_some());
    }
}
