//! C-compatible option builders.

use lexical_core;
use crate::lib::slice;

// ParseIntegerOptionsBuilder
create_builder!(
    fn lexical_parse_integer_options_builder -> ParseIntegerOptions;
    fn lexical_parse_integer_options_build;

    @field
    #[cfg(feature = "radix")]
    fn lexical_parse_integer_options_builder_radix(radix: u8);

    @field
    #[cfg(feature = "format")]
    fn lexical_parse_integer_options_builder_format(format: lexical_core::NumberFormat);
);

// ParseFloatOptionsBuilder
create_builder!(
    fn lexical_parse_float_options_builder -> ParseFloatOptions;
    fn lexical_parse_float_options_build;

    @field  fn lexical_parse_float_options_builder_lossy(lossy: bool);
    @field  fn lexical_parse_float_options_builder_exponent_char(exponent_char: u8);
    @string fn lexical_parse_float_options_builder_nan_string(nan_string: &'static [u8]);
    @string fn lexical_parse_float_options_builder_inf_string(inf_string: &'static [u8]);
    @string fn lexical_parse_float_options_builder_infinity_string(infinity_string: &'static [u8]);

    @field
    #[cfg(feature = "radix")]
    fn lexical_parse_float_options_builder_radix(radix: u8);

    @field
    #[cfg(feature = "format")]
    fn lexical_parse_float_options_builder_format(format: lexical_core::NumberFormat);

    @field
    #[cfg(feature = "rounding")]
    fn lexical_parse_float_options_builder_rounding(rounding: lexical_core::RoundingKind);
);

// WriteIntegerOptionsBuilder
create_builder!(
    fn lexical_write_integer_options_builder -> WriteIntegerOptions;
    fn lexical_write_integer_options_build;

    @field
    #[cfg(feature = "radix")]
    fn lexical_write_integer_options_builder_radix(radix: u8);
);

// WriteFloatOptionsBuilder
create_builder!(
    fn lexical_write_float_options_builder -> WriteFloatOptions;
    fn lexical_write_float_options_build;

    @field  fn lexical_write_float_options_builder_exponent_char(exponent_char: u8);
    @field  fn lexical_write_float_options_builder_trim_floats(trim_floats: bool);
    @string fn lexical_write_float_options_builder_nan_string(nan_string: &'static [u8]);
    @string fn lexical_write_float_options_builder_inf_string(inf_string: &'static [u8]);

    @field
    #[cfg(feature = "radix")]
    fn lexical_write_float_options_builder_radix(radix: u8);
);
