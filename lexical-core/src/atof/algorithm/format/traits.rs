//! Traits that provide format-dependent data for floating parsing algorithms.

use crate::result::*;
use crate::util::*;

use super::exponent::*;

/// Private data interface for local utilities.
pub(crate) trait FastDataInterfaceImpl<'a>: Sized {
    /// Get integer component of float.
    fn integer(&self) -> &'a [u8];

    /// Set integer component of float.
    fn set_integer(&mut self, integer: &'a [u8]);

    /// Get fraction component of float.
    fn fraction(&self) -> Option<&'a [u8]>;

    /// Set fraction component of float.
    fn set_fraction(&mut self, fraction: Option<&'a [u8]>);

    /// Get exponent component of float.
    fn exponent(&self) -> Option<&'a [u8]>;

    /// Set exponent component of float.
    fn set_exponent(&mut self, exponent: Option<&'a [u8]>);

    /// Get raw exponent component of float.
    fn raw_exponent(&self) -> i32;

    /// Set raw exponent component of float.
    fn set_raw_exponent(&mut self, raw_exponent: i32);
}

/// Private data interface for local utilities.
pub(crate) trait SlowDataInterfaceImpl<'a>: Sized {
    /// Get integer component of float.
    fn integer(&self) -> &'a [u8];

    /// Set integer component of float.
    fn set_integer(&mut self, integer: &'a [u8]);

    /// Get fraction component of float.
    fn fraction(&self) -> &'a [u8];

    /// Set fraction component of float.
    fn set_fraction(&mut self, fraction: &'a [u8]);

    /// Get raw exponent component of float.
    fn raw_exponent(&self) -> i32;

    /// Set raw exponent component of float.
    fn set_raw_exponent(&mut self, raw_exponent: i32);
}

// Implement FastDataInterfaceImpl for a default structure.
macro_rules! fast_data_interface_impl {
    ($name:ident) => {
        impl<'a> FastDataInterfaceImpl<'a> for $name<'a> {
            #[inline(always)]
            fn integer(&self) -> &'a [u8] {
                self.integer
            }

            #[inline(always)]
            fn set_integer(&mut self, integer: &'a [u8]) {
                self.integer = integer
            }

            #[inline(always)]
            fn fraction(&self) -> Option<&'a [u8]> {
                self.fraction
            }

            #[inline(always)]
            fn set_fraction(&mut self, fraction: Option<&'a [u8]>) {
                self.fraction = fraction
            }

            #[inline(always)]
            fn exponent(&self) -> Option<&'a [u8]> {
                self.exponent
            }

            #[inline(always)]
            fn set_exponent(&mut self, exponent: Option<&'a [u8]>) {
                self.exponent = exponent
            }

            #[inline(always)]
            fn raw_exponent(&self) -> i32 {
                self.raw_exponent
            }

            #[inline(always)]
            fn set_raw_exponent(&mut self, raw_exponent: i32) {
                self.raw_exponent = raw_exponent
            }
        }
    };
}

// Implement SlowDataInterfaceImpl for a default structure.
macro_rules! slow_data_interface_impl {
    ($name:ident) => {
        impl<'a> SlowDataInterfaceImpl<'a> for $name<'a> {
            #[inline(always)]
            fn integer(&self) -> &'a [u8] {
                self.integer
            }

            #[inline(always)]
            fn set_integer(&mut self, integer: &'a [u8]) {
                self.integer = integer
            }

            #[inline(always)]
            fn fraction(&self) -> &'a [u8] {
                self.fraction
            }

            #[inline(always)]
            fn set_fraction(&mut self, fraction: &'a [u8]) {
                self.fraction = fraction
            }

            #[inline(always)]
            fn raw_exponent(&self) -> i32 {
                self.raw_exponent
            }

            #[inline(always)]
            fn set_raw_exponent(&mut self, raw_exponent: i32) {
                self.raw_exponent = raw_exponent
            }
        }
    };
}

// PUBLIC

/// Data interface for fast float parsers.
pub(crate) trait FastDataInterface<'a>: FastDataInterfaceImpl<'a> {
    /// Integer digits iterator type.
    type IntegerIter: ConsumedIterator<Item = &'a u8> + AsPtrIterator<'a, u8>;

    /// Float digits iterator type.
    type FractionIter: ConsumedIterator<Item = &'a u8> + AsPtrIterator<'a, u8>;

    /// Exponent digits iterator type.
    type ExponentIter: ConsumedIterator<Item = &'a u8> + AsPtrIterator<'a, u8>;

    /// Associated slow data type.
    type SlowInterface: SlowDataInterface<'a>;

    /// Create new float data from format specification.
    fn new(format: NumberFormat) -> Self;

    // DATA

    /// Iterate over all integer digits.
    fn integer_iter(&self) -> Self::IntegerIter;

    /// Iterate over all fraction digits
    fn fraction_iter(&self) -> Self::FractionIter;

    /// Iterate over all exponent digits
    fn exponent_iter(&self) -> Self::ExponentIter;

    /// Get the number format.
    fn format(&self) -> NumberFormat;

    /// Get the mantissa exponent from the raw exponent.
    #[inline(always)]
    fn mantissa_exponent(&self, truncated_digits: usize) -> i32 {
        mantissa_exponent(self.raw_exponent(), self.fraction_iter().count(), truncated_digits)
    }

    // EXTRACT

    // Consume integer digits until a non-digit character is found.
    fn consume_integer_digits(&self, bytes: &'a [u8], radix: u32) -> (&'a [u8], &'a [u8]);

    // Consume fraction digits until a non-digit character is found.
    fn consume_fraction_digits(&self, bytes: &'a [u8], radix: u32) -> (&'a [u8], &'a [u8]);

    // Extract the integer substring from the float.
    #[inline(always)]
    fn extract_integer(&mut self, bytes: &'a [u8], radix: u32) -> &'a [u8] {
        let result = self.consume_integer_digits(bytes, radix);
        self.set_integer(result.0);
        result.1
    }

    // Extract the fraction substring from the float.
    //
    //  Preconditions:
    //      `bytes.len()` >= 1 and `bytes[0] == b'.'`.
    #[inline(always)]
    fn extract_fraction(&mut self, bytes: &'a [u8], radix: u32) -> &'a [u8] {
        let digits = &bytes[1..];
        let result = self.consume_fraction_digits(digits, radix);
        self.set_fraction(Some(result.0));
        result.1
    }

    // Extract and parse the exponent substring from the float.
    fn extract_exponent(&mut self, bytes: &'a [u8], radix: u32) -> &'a [u8];

    // Validate the extracted mantissa components.
    fn validate_mantissa(&self) -> ParseResult<()>;

    // Validate the extracted exponent component.
    fn validate_exponent(&self) -> ParseResult<()>;

    // Validate the extracted exponent depending on the fraction component.
    fn validate_exponent_fraction(&self) -> ParseResult<()>;

    // Validate the extracted exponent sign.
    fn validate_exponent_sign(&self) -> ParseResult<()>;

    // Trim leading 0s and digit separators.
    fn ltrim_zero(&self, bytes: &'a [u8]) -> (&'a [u8], usize);

    // Trim leading digit separators.
    fn ltrim_separator(&self, bytes: &'a [u8]) -> (&'a [u8], usize);

    // Trim trailing 0s and digit separators.
    fn rtrim_zero(&self, bytes: &'a [u8]) -> (&'a [u8], usize);

    // Trim trailing digit separators.
    fn rtrim_separator(&self, bytes: &'a [u8]) -> (&'a [u8], usize);

    // Post-process float to trim leading and trailing 0s and digit separators.
    // This is required for accurate results in the slow-path algorithm,
    // otherwise, we may incorrect guess the mantissa or scientific exponent.
    #[inline(always)]
    fn trim(&mut self) {
        self.set_integer(self.ltrim_zero(self.integer()).0);
        self.set_fraction(self.fraction().map(|x| self.rtrim_zero(x).0));
    }

    /// Extract float subcomponents from input bytes.
    #[inline]
    fn extract(&mut self, bytes: &'a [u8], radix: u32) -> ParseResult<*const u8> {
        // Parse the integer, aka, the digits preceding any control characters.
        let mut digits = bytes;
        digits = self.extract_integer(digits, radix);

        // Get the control characters. The exponent will always be
        // in ASCII lowercase, due to how NumberFormat checks it.
        let decimal_point = self.format().decimal_point();
        let exponent = self.format().exponent(radix);

        // Parse and validate a fraction, if present.
        if let Some(&c) = digits.first() {
            if c == decimal_point {
                digits = self.extract_fraction(digits, radix);
            }
        }
        self.validate_mantissa()?;

        // Parse and validate an exponent, if present.
        if let Some(&c) = digits.first() {
            if c.to_ascii_lowercase() == exponent {
                digits = self.extract_exponent(digits, radix);
            }
        }
        self.validate_exponent()?;
        self.validate_exponent_fraction()?;
        self.validate_exponent_sign()?;

        // Trim the remaining digits.
        self.trim();

        Ok(digits.as_ptr())
    }

    // TO SLOW DATA

    // Calculate the digit start from the integer and fraction slices.
    #[inline(always)]
    fn digits_start(&self) -> usize {
        // If there are no returned values in the integer iterator
        // since we've trimmed leading 0s, then we have to trim
        // leading zeros to get to the start of the significant
        // digits in the fraction.
        match self.integer().is_empty() {
            true => self.ltrim_zero(self.fraction().unwrap_or(&[])).1,
            false => 0,
        }
    }

    /// Process float data for moderate/slow float parsers.
    fn to_slow(self, truncated_digits: usize) -> Self::SlowInterface;

    // TESTS

    #[cfg(test)]
    fn clear(&mut self) {
        self.set_integer(&[]);
        self.set_fraction(None);
        self.set_exponent(None);
        self.set_raw_exponent(0);
    }

    /// Check the float state parses the desired data.
    #[cfg(test)]
    fn check_extract(&mut self, digits: &'a [u8], expected: &ParseTestResult<Self>) {
        let expected = expected.as_ref();
        match self.extract(digits, 10) {
            Ok(_) => {
                let expected = expected.unwrap();
                assert_eq!(self.integer(), expected.integer());
                assert_eq!(self.fraction(), expected.fraction());
                assert_eq!(self.exponent(), expected.exponent());
            },
            Err((c, _)) => assert_eq!(c, *expected.err().unwrap()),
        }
    }

    // Run series of tests.
    #[cfg(test)]
    fn run_tests<Iter>(&mut self, tests: Iter)
    where
        Iter: Iterator<Item = &'a (&'a str, ParseTestResult<Self>)>,
        Self: 'a,
    {
        for value in tests {
            self.check_extract(value.0.as_bytes(), &value.1);
            self.clear();
        }
    }
}

/// Shared definition for all fast data interfaces.
macro_rules! fast_data_interface {
    (
        struct $name:ident,
        fields => { $( $field:ident : $type:ty, )* },
        integer_iter => ( $integer_iter:tt, $integer_iter_fn:ident ),
        fraction_iter => ( $fraction_iter:tt, $fraction_iter_fn:ident ),
        exponent_iter => ( $exponent_iter:tt, $exponent_iter_fn:ident ),
        format => $format:expr,
        slow_interface => $slow_interface:tt,
        consume_integer_digits => $consume_integer_digits:expr,
        consume_fraction_digits => $consume_fraction_digits:expr,
        extract_exponent => $extract_exponent:expr,
        validate_mantissa => $validate_mantissa:expr,
        validate_exponent => $validate_exponent:expr,
        validate_exponent_fraction => $validate_exponent_fraction:expr,
        validate_exponent_sign => $validate_exponent_sign:expr,
        ltrim_zero => $ltrim_zero:ident,
        ltrim_separator => $ltrim_separator:ident,
        rtrim_zero => $rtrim_zero:ident,
        rtrim_separator => $rtrim_separator:ident,
        new => fn $newfn:ident($($arg:ident: $argtype:ty),*) -> $rettype:ty $newbody:block
    ) => (
        pub(crate) struct $name<'a> {
            $( $field : $type, )*
            integer: &'a [u8],
            fraction: Option<&'a [u8]>,
            exponent: Option<&'a [u8]>,
            raw_exponent: i32
        }

        fast_data_interface_impl!($name);

        impl<'a> FastDataInterface<'a> for $name<'a> {
            type IntegerIter = $integer_iter<'a>;
            type FractionIter = $fraction_iter<'a>;
            type ExponentIter = $exponent_iter<'a>;

            type SlowInterface = $slow_interface<'a>;

            #[inline(always)]
            #[allow(unused_variables)]
            fn $newfn($($arg: $argtype),*) -> $rettype $newbody

            // DATA

            #[inline(always)]
            fn integer_iter(&self) -> Self::IntegerIter {
                $integer_iter_fn(self.integer, self.format().digit_separator())
            }

            #[inline(always)]
            fn fraction_iter(&self) -> Self::FractionIter {
                let fraction = self.fraction.unwrap_or(&[]);
                $fraction_iter_fn(fraction, self.format().digit_separator())
            }

            #[inline(always)]
            fn exponent_iter(&self) -> Self::ExponentIter {
                let exponent = self.exponent.unwrap_or(&[]);
                $exponent_iter_fn(exponent, self.format().digit_separator())
            }

            #[inline(always)]
            fn format(&self) -> NumberFormat {
                $format(self)
            }

            #[inline(always)]
            fn consume_integer_digits(&self, digits: &'a [u8], radix: u32)
                -> (&'a [u8], &'a [u8])
            {
                $consume_integer_digits(digits, radix, self.format())
            }

            #[inline(always)]
            fn consume_fraction_digits(&self, digits: &'a [u8], radix: u32)
                -> (&'a [u8], &'a [u8])
            {
                $consume_fraction_digits(digits, radix, self.format())
            }

            #[inline(always)]
            fn extract_exponent(&mut self, bytes: &'a [u8], radix: u32) -> &'a [u8]
            {
                $extract_exponent(self, bytes, radix, self.format())
            }

            #[inline(always)]
            fn validate_mantissa(&self) -> ParseResult<()> {
                $validate_mantissa(self)
            }

            #[inline(always)]
            fn validate_exponent(&self) -> ParseResult<()> {
                $validate_exponent(self)
            }

            #[inline(always)]
            fn validate_exponent_fraction(&self) -> ParseResult<()> {
                $validate_exponent_fraction(self)
            }

            #[inline(always)]
            fn validate_exponent_sign(&self) -> ParseResult<()> {
                $validate_exponent_sign(self)
            }

            #[inline(always)]
            fn ltrim_zero(&self, bytes: &'a [u8]) -> (&'a [u8], usize) {
                $ltrim_zero(bytes, self.format().digit_separator())
            }

            #[inline(always)]
            fn ltrim_separator(&self, bytes: &'a [u8]) -> (&'a [u8], usize) {
                $ltrim_separator(bytes, self.format().digit_separator())
            }

            #[inline(always)]
            fn rtrim_zero(&self, bytes: &'a [u8]) -> (&'a [u8], usize) {
                $rtrim_zero(bytes, self.format().digit_separator())
            }

            #[inline(always)]
            fn rtrim_separator(&self, bytes: &'a [u8]) -> (&'a [u8], usize) {
                $rtrim_separator(bytes, self.format().digit_separator())
            }

            // TO SLOW DATA

            #[inline(always)]
            fn to_slow(self, truncated_digits: usize) -> Self::SlowInterface {
                let digits_start = self.digits_start();
                Self::SlowInterface {
                    $( $field: self.$field, )*
                    digits_start,
                    truncated_digits,
                    integer: self.integer,
                    fraction: self.fraction.unwrap_or(&[]),
                    raw_exponent: self.raw_exponent
                }
            }
        }
    );
}

/// Data interface for moderate/slow float parsers.
pub(crate) trait SlowDataInterface<'a>: SlowDataInterfaceImpl<'a> {
    /// Integer digits iterator type.
    type IntegerIter: ConsumedIterator<Item = &'a u8> + AsPtrIterator<'a, u8>;

    /// Float digits iterator type.
    type FractionIter: ConsumedIterator<Item = &'a u8> + AsPtrIterator<'a, u8>;

    /// Iterate over all integer digits.
    fn integer_iter(&self) -> Self::IntegerIter;

    /// Get number of all integer digits.
    #[inline(always)]
    fn integer_digits(&self) -> usize {
        self.integer_iter().count()
    }

    /// Iterate over all fraction digits
    fn fraction_iter(&self) -> Self::FractionIter;

    /// Get number of all fraction digits.
    #[inline(always)]
    fn fraction_digits(&self) -> usize {
        self.fraction_iter().count()
    }

    /// Iterate over significant fraction digits.
    fn significant_fraction_iter(&self) -> Self::FractionIter;

    /// Get number of significant fraction digits.
    #[inline(always)]
    fn significant_fraction_digits(&self) -> usize {
        self.significant_fraction_iter().count()
    }

    /// Get the number of digits in the mantissa.
    /// Cannot overflow, since this is based off a single usize input string.
    #[inline(always)]
    fn mantissa_digits(&self) -> usize {
        self.integer_digits() + self.significant_fraction_digits()
    }

    /// Get the number format.
    fn format(&self) -> NumberFormat;

    /// Get index to start of significant digits in the fraction.
    fn digits_start(&self) -> usize;

    /// Get number of truncated digits.
    fn truncated_digits(&self) -> usize;

    /// Get the mantissa exponent from the raw exponent.
    #[inline(always)]
    fn mantissa_exponent(&self) -> i32 {
        mantissa_exponent(self.raw_exponent(), self.fraction_digits(), self.truncated_digits())
    }

    /// Get the scientific exponent from the raw exponent.
    #[inline(always)]
    fn scientific_exponent(&self) -> i32 {
        scientific_exponent(self.raw_exponent(), self.integer_digits(), self.digits_start())
    }
}

/// Shared definition for all slow data interfaces.
macro_rules! slow_data_interface {
    (
        struct $name:ident,
        fields => { $( $field:ident : $type:ty, )* },
        integer_iter => ( $integer_iter:tt, $integer_iter_fn:ident ),
        fraction_iter => ( $fraction_iter:tt, $fraction_iter_fn:ident ),
        format => $format:expr
    ) => (
        pub(crate) struct $name<'a> {
            $( $field : $type, )*
            integer: &'a [u8],
            fraction: &'a [u8],
            digits_start: usize,
            truncated_digits: usize,
            raw_exponent: i32
        }

        slow_data_interface_impl!($name);

        impl<'a> SlowDataInterface<'a> for $name<'a> {
            type IntegerIter = $integer_iter<'a>;
            type FractionIter = $fraction_iter<'a>;

            // DATA

            #[inline(always)]
            fn integer_iter(&self) -> Self::IntegerIter {
                $integer_iter_fn(self.integer, self.format().digit_separator())
            }

            #[inline(always)]
            fn fraction_iter(&self) -> Self::FractionIter {
                $fraction_iter_fn(self.fraction, self.format().digit_separator())
            }

            #[inline(always)]
            fn significant_fraction_iter(&self) -> Self::FractionIter {
                let fraction = &self.fraction[self.digits_start..];
                $fraction_iter_fn(fraction, self.format().digit_separator())
            }

            #[inline(always)]
            fn format(&self) -> NumberFormat {
                $format(self)
            }

            #[inline(always)]
            fn digits_start(&self) -> usize {
                self.digits_start
            }

            #[inline(always)]
            fn truncated_digits(&self) -> usize {
                self.truncated_digits
            }
        }
    );
}

/// Shared definition for all data interfaces.
macro_rules! data_interface {
    (
        struct $fast:ident,
        struct $slow:ident,
        fields => { $( $field:ident : $type:ty, )* },
        integer_iter => ( $integer_iter:tt, $integer_iter_fn:ident ),
        fraction_iter => ( $fraction_iter:tt, $fraction_iter_fn:ident ),
        exponent_iter => ( $exponent_iter:tt, $exponent_iter_fn:ident ),
        format => $format:expr,
        consume_integer_digits => $consume_integer_digits:expr,
        consume_fraction_digits => $consume_fraction_digits:expr,
        extract_exponent => $extract_exponent:expr,
        validate_mantissa => $validate_mantissa:expr,
        validate_exponent => $validate_exponent:expr,
        validate_exponent_fraction => $validate_exponent_fraction:expr,
        validate_exponent_sign => $validate_exponent_sign:expr,
        ltrim_zero => $ltrim_zero:ident,
        ltrim_separator => $ltrim_separator:ident,
        rtrim_zero => $rtrim_zero:ident,
        rtrim_separator => $rtrim_separator:ident,
        new => fn $newfn:ident($($arg:ident : $argtype:ty),*) -> $rettype:ty $newbody:block
    ) => (
        fast_data_interface!(
            struct $fast,
            fields => { $( $field : $type , )* },
            integer_iter => ($integer_iter, $integer_iter_fn),
            fraction_iter => ($fraction_iter, $fraction_iter_fn),
            exponent_iter => ($exponent_iter, $exponent_iter_fn),
            format => $format,
            slow_interface => $slow,
            consume_integer_digits => $consume_integer_digits,
            consume_fraction_digits => $consume_fraction_digits,
            extract_exponent => $extract_exponent,
            validate_mantissa => $validate_mantissa,
            validate_exponent => $validate_exponent,
            validate_exponent_fraction => $validate_exponent_fraction,
            validate_exponent_sign => $validate_exponent_sign,
            ltrim_zero => $ltrim_zero,
            ltrim_separator => $ltrim_separator,
            rtrim_zero => $rtrim_zero,
            rtrim_separator => $rtrim_separator,
            new => fn $newfn($($arg: $argtype),*) -> $rettype $newbody
        );

        slow_data_interface!(
            struct $slow,
            fields => { $( $field : $type , )* },
            integer_iter => ($integer_iter, $integer_iter_fn),
            fraction_iter => ($fraction_iter, $fraction_iter_fn),
            format => $format
        );
    );
}
