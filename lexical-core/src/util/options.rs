//! Configuration options for parsing and formatting numbers.

use super::builder::*;
use super::config::*;
use super::format::NumberFormat;
use super::rounding::RoundingKind;

// CONSTANTS
// ---------

// TODO(ahuszagh) Restore later.
// Constants to dictate default values for options.
//pub(crate) const DEFAULT_EXPONENT_CHAR: u8 = b'e';
pub(crate) const DEFAULT_FORMAT: NumberFormat = NumberFormat::STANDARD;
//pub(crate) const DEFAULT_INF_STRING: &'static [u8] = b"inf";
//pub(crate) const DEFAULT_INFINITY_STRING: &'static [u8] = b"infinity";
pub(crate) const DEFAULT_LOSSY: bool = false;
//pub(crate) const DEFAULT_NAN_STRING: &'static [u8] = b"NaN";
pub(crate) const DEFAULT_RADIX: u8 = 10;
//pub(crate) const DEFAULT_ROUNDING: RoundingKind = RoundingKind::NearestTieEven;
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

/// Builder for `ParseIntegerOptions`.
#[repr(C)]
#[derive(Debug)]
pub struct ParseIntegerOptionsBuilder {
    radix: u8,
    format: NumberFormat
}

impl ParseIntegerOptionsBuilder {
    #[inline(always)]
    fn new() -> ParseIntegerOptionsBuilder {
        ParseIntegerOptionsBuilder {
            radix: DEFAULT_RADIX,
            format: DEFAULT_FORMAT
        }
    }

    #[inline(always)]
    #[cfg(feature = "radix")]
    pub fn radix(mut self, radix: u8) -> Self {
        self.radix = radix;
        self
    }

    #[inline(always)]
    #[cfg(feature = "radix")]
    #[deprecated(
        since = "0.8.0",
        note = "Will be removed with 1.0. For backwards compatibility in radix API calls."
    )]
    pub(crate) fn exponent_char(self, _: u8) -> Self {
        self
    }

    #[inline(always)]
    #[cfg(feature = "format")]
    pub fn format(mut self, format: NumberFormat) -> Self {
        self.format = format;
        self
    }
}

impl Builder for ParseIntegerOptionsBuilder {
    type Buildable = ParseIntegerOptions;

    #[inline(always)]
    fn build(self) -> Option<Self::Buildable> {
        let radix = to_radix(self.radix)?;
        let format = to_format_integer(self.format, radix)?;
        Some(Self::Buildable { radix, format })
    }
}

/// Options to customize parsing integers.
///
/// # Examples
///
/// ```rust
/// # extern crate lexical_core;
/// use lexical_core::{ParseIntegerOptions, Builder, Buildable};
///
/// # pub fn main() {
/// let options = ParseIntegerOptions::builder()
///     .build()
///     .unwrap();
/// # }
/// ```
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct ParseIntegerOptions {
    /// Radix for integer string.
    radix: u32,

    /// Number format.
    format: NumberFormat
}

impl ParseIntegerOptions {
    // PRE-DEFINED CONSTANTS

    /// Create new options to parse the default binary format.
    #[inline(always)]
    #[cfg(feature = "radix")]
    pub fn binary() -> ParseIntegerOptions {
        ParseIntegerOptions::builder()
            .radix(2)
            .build()
            .unwrap()
    }

    /// Create new options to parse the default decimal format.
    #[inline(always)]
    pub fn decimal() -> ParseIntegerOptions {
        ParseIntegerOptions::builder()
            .build()
            .unwrap()
    }

    /// Create new options to parse the default hexadecimal format.
    #[inline(always)]
    #[cfg(feature = "radix")]
    pub fn hexadecimal() -> ParseIntegerOptions {
        ParseIntegerOptions::builder()
            .radix(16)
            .build()
            .unwrap()
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

impl Buildable for ParseIntegerOptions {
    type Builder = ParseIntegerOptionsBuilder;

    #[inline(always)]
    fn builder() -> Self::Builder {
        Self::Builder::new()
    }
}

impl Default for ParseIntegerOptions {
    #[inline]
    fn default() -> Self {
        Self::builder().build().unwrap()
    }
}

// PARSE FLOAT
// -----------

/// Builder for `ParseFloatOptions`.
#[repr(C)]
#[derive(Debug)]
pub struct ParseFloatOptionsBuilder {
    lossy: bool,
    exponent_char: u8,
    radix: u8,
    format: NumberFormat,
    rounding: RoundingKind,
    nan_string: &'static [u8],
    inf_string: &'static [u8],
    infinity_string: &'static [u8]
}

#[allow(deprecated)]    // TODO(ahuszagh) Remove with 1.0.
impl ParseFloatOptionsBuilder {
    #[inline(always)]
    fn new() -> ParseFloatOptionsBuilder {
        ParseFloatOptionsBuilder {
            lossy: DEFAULT_LOSSY,
            exponent_char: exponent_notation_char(DEFAULT_RADIX as u32),
            radix: DEFAULT_RADIX,
            format: DEFAULT_FORMAT,
            rounding: get_float_rounding(),
            nan_string: get_nan_string(),
            inf_string: get_inf_string(),
            infinity_string: get_infinity_string()
        }
    }

    #[inline(always)]
    pub fn lossy(mut self, lossy: bool) -> Self {
        self.lossy = lossy;
        self
    }

    #[inline(always)]
    pub fn exponent_char(mut self, exponent_char: u8) -> Self {
        self.exponent_char = exponent_char;
        self
    }

    #[inline(always)]
    #[cfg(feature = "radix")]
    pub fn radix(mut self, radix: u8) -> Self {
        self.radix = radix;
        self
    }

    #[inline(always)]
    #[cfg(feature = "format")]
    pub fn format(mut self, format: NumberFormat) -> Self {
        self.format = format;
        self
    }

    #[inline(always)]
    #[cfg(feature = "rounding")]
    pub fn rounding(mut self, rounding: RoundingKind) -> Self {
        self.rounding = rounding;
        self
    }

    #[inline(always)]
    pub fn nan_string(mut self, nan_string: &'static [u8]) -> Self {
        self.nan_string = nan_string;
        self
    }

    #[inline(always)]
    pub fn inf_string(mut self, inf_string: &'static [u8]) -> Self {
        self.inf_string = inf_string;
        self
    }

    #[inline(always)]
    pub fn infinity_string(mut self, infinity_string: &'static [u8]) -> Self {
        self.infinity_string = infinity_string;
        self
    }
}

impl Builder for ParseFloatOptionsBuilder {
    type Buildable = ParseFloatOptions;

    #[inline(always)]
    fn build(self) -> Option<Self::Buildable> {
        let radix = to_radix(self.radix)?;
        let exponent_char = to_exponent_char(self.exponent_char, radix)?;
        let format = to_format_float(self.format, radix, exponent_char)?;
        let rounding = to_rounding(self.rounding)?;
        let nan_string = to_nan_string(self.nan_string)?;
        let inf_string = to_inf_string(self.inf_string)?;
        let infinity_string = to_infinity_string(self.infinity_string, inf_string)?;
        Some(Self::Buildable {
            lossy: self.lossy,
            exponent_char: exponent_char,
            radix: radix,
            format: format,
            rounding: rounding,
            nan_string: nan_string,
            inf_string: inf_string,
            infinity_string: infinity_string
        })
    }
}

/// Options to customize parsing floats.
///
/// # Examples
///
/// ```rust
/// # extern crate lexical_core;
/// use lexical_core::{ParseFloatOptions, Builder, Buildable};
///
/// # pub fn main() {
/// let options = ParseFloatOptions::builder()
///     .lossy(true)
///     .exponent_char(b'e')
///     .nan_string(b"NaN")
///     .inf_string(b"Inf")
///     .infinity_string(b"Infinity")
///     .build()
///     .unwrap();
/// # }
/// ```
#[repr(C)]
#[derive(Clone, Copy, Debug)]
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
    infinity_string: &'static [u8]
}

impl ParseFloatOptions {
    // PRE-DEFINED CONSTANTS

    /// Create new options to parse the default binary format.
    #[inline(always)]
    #[cfg(feature = "radix")]
    pub fn binary() -> ParseFloatOptions {
        ParseFloatOptions::builder()
            .radix(2)
            .build()
            .unwrap()
    }

    /// Create new options to parse the default decimal format.
    #[inline(always)]
    pub fn decimal() -> ParseFloatOptions {
        ParseFloatOptions::builder()
            .build()
            .unwrap()
    }

    /// Create new options to parse the default hexadecimal format.
    #[inline(always)]
    #[cfg(feature = "radix")]
    pub fn hexadecimal() -> ParseFloatOptions {
        ParseFloatOptions::builder()
            .radix(16)
            .exponent_char(b'p')
            .build()
            .unwrap()
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

impl Buildable for ParseFloatOptions {
    type Builder = ParseFloatOptionsBuilder;

    #[inline(always)]
    fn builder() -> Self::Builder {
        Self::Builder::new()
    }
}

impl Default for ParseFloatOptions {
    #[inline]
    fn default() -> Self {
        Self::builder().build().unwrap()
    }
}

// WRITE INTEGER
// -------------

#[repr(C)]
#[derive(Debug)]
pub struct WriteIntegerOptionsBuilder {
    radix: u8,
}

impl WriteIntegerOptionsBuilder {
    #[inline(always)]
    fn new() -> WriteIntegerOptionsBuilder {
        WriteIntegerOptionsBuilder {
            radix: DEFAULT_RADIX,
        }
    }

    #[inline(always)]
    #[cfg(feature = "radix")]
    pub fn radix(mut self, radix: u8) -> Self {
        self.radix = radix;
        self
    }

    #[inline(always)]
    #[cfg(feature = "radix")]
    #[deprecated(
        since = "0.8.0",
        note = "Will be removed with 1.0. For backwards compatibility in radix API calls."
    )]
    pub(crate) fn exponent_char(self, _: u8) -> Self {
        self
    }
}

impl Builder for WriteIntegerOptionsBuilder {
    type Buildable = WriteIntegerOptions;

    #[inline(always)]
    fn build(self) -> Option<Self::Buildable> {
        let radix = to_radix(self.radix)?;
        Some(Self::Buildable { radix })
    }
}

/// Immutable options to customize writing integers.
///
/// # Examples
///
/// ```rust
/// # extern crate lexical_core;
/// use lexical_core::{WriteIntegerOptions, Builder, Buildable};
///
/// # pub fn main() {
/// let options = WriteIntegerOptions::builder()
///     .build()
///     .unwrap();
/// # }
/// ```
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct WriteIntegerOptions {
    /// Radix for integer string.
    radix: u32,
}

impl WriteIntegerOptions {
    // PRE-DEFINED CONSTANTS

    /// Create new options to parse the default binary format.
    #[inline(always)]
    #[cfg(feature = "radix")]
    pub fn binary() -> WriteIntegerOptions {
        WriteIntegerOptions::builder()
            .radix(2)
            .build()
            .unwrap()
    }

    /// Create new options to parse the default decimal format.
    #[inline(always)]
    pub fn decimal() -> WriteIntegerOptions {
        WriteIntegerOptions::builder()
            .build()
            .unwrap()
    }

    /// Create new options to parse the default hexadecimal format.
    #[inline(always)]
    #[cfg(feature = "radix")]
    pub fn hexadecimal() -> WriteIntegerOptions {
        WriteIntegerOptions::builder()
            .radix(16)
            .build()
            .unwrap()
    }

    // GETTERS

    /// Get the radix.
    #[inline(always)]
    pub const fn radix(&self) -> u32 {
        self.radix
    }
}

impl Buildable for WriteIntegerOptions {
    type Builder = WriteIntegerOptionsBuilder;

    #[inline(always)]
    fn builder() -> Self::Builder {
        Self::Builder::new()
    }
}

impl Default for WriteIntegerOptions {
    #[inline]
    fn default() -> Self {
        Self::builder().build().unwrap()
    }
}

// WRITE FLOAT
// -----------

#[repr(C)]
#[derive(Debug)]
pub struct WriteFloatOptionsBuilder {
    exponent_char: u8,
    radix: u8,
    trim_floats: bool,
    nan_string: &'static [u8],
    inf_string: &'static [u8],
}

#[allow(deprecated)]    // TODO(ahuszagh) Remove with 1.0.
impl WriteFloatOptionsBuilder {
    #[inline(always)]
    fn new() -> WriteFloatOptionsBuilder {
        WriteFloatOptionsBuilder {
            exponent_char: exponent_notation_char(DEFAULT_RADIX as u32),
            radix: DEFAULT_RADIX,
            trim_floats: DEFAULT_TRIM_FLOATS,
            nan_string: get_nan_string(),
            inf_string: get_inf_string()
        }
    }

    #[inline(always)]
    pub fn exponent_char(mut self, exponent_char: u8) -> Self {
        self.exponent_char = exponent_char;
        self
    }

    #[inline(always)]
    #[cfg(feature = "radix")]
    pub fn radix(mut self, radix: u8) -> Self {
        self.radix = radix;
        self
    }

    #[inline(always)]
    pub fn trim_floats(mut self, trim_floats: bool) -> Self {
        self.trim_floats = trim_floats;
        self
    }

    #[inline(always)]
    pub fn nan_string(mut self, nan_string: &'static [u8]) -> Self {
        self.nan_string = nan_string;
        self
    }

    #[inline(always)]
    pub fn inf_string(mut self, inf_string: &'static [u8]) -> Self {
        self.inf_string = inf_string;
        self
    }
}

impl Builder for WriteFloatOptionsBuilder {
    type Buildable = WriteFloatOptions;

    #[inline(always)]
    fn build(self) -> Option<Self::Buildable> {
        let radix = to_radix(self.radix)?;
        let exponent_char = to_exponent_char(self.exponent_char, radix)?;
        let nan_string = to_nan_string(self.nan_string)?;
        let inf_string = to_inf_string(self.inf_string)?;
        Some(Self::Buildable {
            exponent_char: exponent_char,
            radix: radix,
            trim_floats: self.trim_floats,
            nan_string: nan_string,
            inf_string: inf_string
        })
    }
}

/// Options to customize writing floats.
///
/// # Examples
///
/// ```rust
/// # extern crate lexical_core;
/// use lexical_core::{WriteFloatOptions, Builder, Buildable};
///
/// # pub fn main() {
/// let options = WriteFloatOptions::builder()
///     .exponent_char(b'e')
///     .trim_floats(true)
///     .nan_string(b"NaN")
///     .inf_string(b"Inf")
///     .build()
///     .unwrap();
/// # }
/// ```
#[repr(C)]
#[derive(Clone, Copy, Debug)]
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

impl WriteFloatOptions {
    // PRE-DEFINED CONSTANTS

    /// Create new options to parse the default binary format.
    #[inline(always)]
    #[cfg(feature = "radix")]
    pub fn binary() -> WriteFloatOptions {
        WriteFloatOptions::builder()
            .radix(2)
            .build()
            .unwrap()
    }

    /// Create new options to parse the default decimal format.
    #[inline(always)]
    pub fn decimal() -> WriteFloatOptions {
        WriteFloatOptions::builder()
            .build()
            .unwrap()
    }

    /// Create new options to parse the default hexadecimal format.
    #[inline(always)]
    #[cfg(feature = "radix")]
    pub fn hexadecimal() -> WriteFloatOptions {
        WriteFloatOptions::builder()
            .radix(2)
            .exponent_char(b'p')
            .build()
            .unwrap()
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

impl Buildable for WriteFloatOptions {
    type Builder = WriteFloatOptionsBuilder;

    #[inline(always)]
    fn builder() -> Self::Builder {
        Self::Builder::new()
    }
}

impl Default for WriteFloatOptions {
    #[inline]
    fn default() -> Self {
        Self::builder().build().unwrap()
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
        let options = ParseIntegerOptions::builder()
            .format(format)
            .build();
        assert!(options.is_none());

        let format = NumberFormat::ignore(b'9').unwrap();
        let options = ParseIntegerOptions::builder()
            .format(format)
            .build();
        assert!(options.is_none());

        let format = NumberFormat::ignore(b'A').unwrap();
        let options = ParseIntegerOptions::builder()
            .format(format)
            .build();
        assert!(options.is_some());
    }
}
