//! C-compatible option builders.

use lexical_core;

// ParseIntegerOptionsBuilder
create_builder!(
    fn lexical_parse_integer_options_builder -> ParseIntegerOptions;
    fn lexical_parse_integer_options_build;
);

// ParseFloatOptionsBuilder
create_builder!(
    fn lexical_parse_float_options_builder -> ParseFloatOptions;
    fn lexical_parse_float_options_build;
);

// WriteIntegerOptionsBuilder
create_builder!(
    fn lexical_write_integer_options_builder -> WriteIntegerOptions;
    fn lexical_write_integer_options_build;
);

// WriteFloatOptionsBuilder
create_builder!(
    fn lexical_write_float_options_builder -> WriteFloatOptions;
    fn lexical_write_float_options_build;
);
