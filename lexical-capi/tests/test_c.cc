/**
 *  Unittests for the C API to lexical-core.
 *
 *  License
 *  -------
 *
 *  This is free and unencumbered software released into the public domain.
 *
 *  Anyone is free to copy, modify, publish, use, compile, sell, or
 *  distribute this software, either in source code form or as a compiled
 *  binary, for any purpose, commercial or non-commercial, and by any
 *  means.
 *
 *  In jurisdictions that recognize copyright laws, the author or authors
 *  of this software dedicate any and all copyright interest in the
 *  software to the public domain. We make this dedication for the benefit
 *  of the public at large and to the detriment of our heirs and
 *  successors. We intend this dedication to be an overt act of
 *  relinquishment in perpetuity of all present and future rights to this
 *  software under copyright law.
 *
 *  THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
 *  EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
 *  MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
 *  IN NO EVENT SHALL THE AUTHORS BE LIABLE FOR ANY CLAIM, DAMAGES OR
 *  OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 *  ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 *  OTHER DEALINGS IN THE SOFTWARE.
 *
 *  For more information, please refer to <http://unlicense.org/>
 */

#include <stdexcept>
#include <gtest/gtest.h>
#include "lexical.h"

// HELPERS
// -------

inline lexical_i8_result result_ok(lexical_i8 value)
{
    // Initialize the union.
    lexical_i8_result_union u;
    u.value = value;

    // Create the tagged union.
    lexical_i8_result r;
    r.tag = lexical_ok;
    r.data = u;

    return r;
}

inline lexical_i8_result result_err(int32_t code, size_t index)
{
    // Initialize the error.
    lexical_error e;
    e.code = code;
    e.index = index;

    // Initialize the union.
    lexical_i8_result_union u;
    u.error = e;

    // Create the tagged union.
    lexical_i8_result r;
    r.tag = lexical_err;
    r.data = u;

    return r;
}

#define lexical_result_error(type)                                              \
    inline lexical_i8_result result_##type(size_t index)                        \
    {                                                                           \
        return result_err(lexical_##type, index);                               \
    }

lexical_result_error(overflow);
lexical_result_error(underflow);
lexical_result_error(invalid_digit);
lexical_result_error(empty);
lexical_result_error(empty_mantissa);
lexical_result_error(empty_exponent);
lexical_result_error(empty_integer);
lexical_result_error(empty_fraction);
lexical_result_error(invalid_positive_mantissa_sign);
lexical_result_error(missing_mantissa_sign);
lexical_result_error(invalid_exponent);
lexical_result_error(invalid_positive_exponent_sign);
lexical_result_error(missing_exponent_sign);
lexical_result_error(exponent_without_fraction);
lexical_result_error(invalid_leading_zeros);

inline lexical_i8_partial_result partial_result_ok(lexical_i8 value, size_t index)
{
    // Initialize the tuple.
    lexical_i8_partial_result_tuple t;
    t.x = value;
    t.y = index;

    // Initialize the union.
    lexical_i8_partial_result_union u;
    u.value = t;

    // Create the tagged union.
    lexical_i8_partial_result r;
    r.tag = lexical_ok;
    r.data = u;

    return r;
}

inline lexical_i8_partial_result partial_result_err(int32_t code, size_t index)
{
    // Initialize the error.
    lexical_error e;
    e.code = code;
    e.index = index;

    // Initialize the union.
    lexical_i8_partial_result_union u;
    u.error = e;

    // Create the tagged union.
    lexical_i8_partial_result r;
    r.tag = lexical_err;
    r.data = u;

    return r;
}

#define lexical_partial_result_error(type)                                      \
    inline lexical_i8_partial_result partial_result_##type(size_t index)        \
    {                                                                           \
        return partial_result_err(lexical_##type, index);                       \
    }

lexical_partial_result_error(overflow);
lexical_partial_result_error(underflow);
lexical_partial_result_error(invalid_digit);
lexical_partial_result_error(empty);
lexical_partial_result_error(empty_mantissa);
lexical_partial_result_error(empty_exponent);
lexical_partial_result_error(empty_integer);
lexical_partial_result_error(empty_fraction);
lexical_partial_result_error(invalid_positive_mantissa_sign);
lexical_partial_result_error(missing_mantissa_sign);
lexical_partial_result_error(invalid_exponent);
lexical_partial_result_error(invalid_positive_exponent_sign);
lexical_partial_result_error(missing_exponent_sign);
lexical_partial_result_error(exponent_without_fraction);
lexical_partial_result_error(invalid_leading_zeros);

#define lexical_is_error(type)                                                  \
    inline bool is_##type(lexical_error error)                                  \
    {                                                                           \
        return lexical_error_is_##type(&error);                                 \
    }

lexical_is_error(overflow);
lexical_is_error(underflow);
lexical_is_error(invalid_digit);
lexical_is_error(empty);
lexical_is_error(empty_mantissa);
lexical_is_error(empty_exponent);
lexical_is_error(empty_integer);
lexical_is_error(empty_fraction);
lexical_is_error(invalid_positive_mantissa_sign);
lexical_is_error(missing_mantissa_sign);
lexical_is_error(invalid_exponent);
lexical_is_error(invalid_positive_exponent_sign);
lexical_is_error(missing_exponent_sign);
lexical_is_error(exponent_without_fraction);
lexical_is_error(invalid_leading_zeros);

// CONSTANT TESTS

TEST(size, constant_tests)
{
    // Check all the sizes can be used.
    EXPECT_GE(LEXICAL_I8_FORMATTED_SIZE, 0);
    EXPECT_GE(LEXICAL_I16_FORMATTED_SIZE, 0);
    EXPECT_GE(LEXICAL_I32_FORMATTED_SIZE, 0);
    EXPECT_GE(LEXICAL_I64_FORMATTED_SIZE, 0);
    EXPECT_GE(LEXICAL_ISIZE_FORMATTED_SIZE, 0);
    EXPECT_GE(LEXICAL_U8_FORMATTED_SIZE, 0);
    EXPECT_GE(LEXICAL_U16_FORMATTED_SIZE, 0);
    EXPECT_GE(LEXICAL_U32_FORMATTED_SIZE, 0);
    EXPECT_GE(LEXICAL_U64_FORMATTED_SIZE, 0);
    EXPECT_GE(LEXICAL_USIZE_FORMATTED_SIZE, 0);
    EXPECT_GE(LEXICAL_F32_FORMATTED_SIZE, 0);
    EXPECT_GE(LEXICAL_F64_FORMATTED_SIZE, 0);
    EXPECT_GE(LEXICAL_I8_FORMATTED_SIZE_DECIMAL, 0);
    EXPECT_GE(LEXICAL_I16_FORMATTED_SIZE_DECIMAL, 0);
    EXPECT_GE(LEXICAL_I32_FORMATTED_SIZE_DECIMAL, 0);
    EXPECT_GE(LEXICAL_I64_FORMATTED_SIZE_DECIMAL, 0);
    EXPECT_GE(LEXICAL_ISIZE_FORMATTED_SIZE_DECIMAL, 0);
    EXPECT_GE(LEXICAL_U8_FORMATTED_SIZE_DECIMAL, 0);
    EXPECT_GE(LEXICAL_U16_FORMATTED_SIZE_DECIMAL, 0);
    EXPECT_GE(LEXICAL_U32_FORMATTED_SIZE_DECIMAL, 0);
    EXPECT_GE(LEXICAL_U64_FORMATTED_SIZE_DECIMAL, 0);
    EXPECT_GE(LEXICAL_USIZE_FORMATTED_SIZE_DECIMAL, 0);
    EXPECT_GE(LEXICAL_F32_FORMATTED_SIZE_DECIMAL, 0);
    EXPECT_GE(LEXICAL_F64_FORMATTED_SIZE_DECIMAL, 0);
    EXPECT_GE(LEXICAL_BUFFER_SIZE, 0);
}

// ERROR TESTS

TEST(is_overflow, error_tests)
{
    lexical_error overflow = { lexical_overflow, 0 };
    lexical_error underflow = { lexical_underflow, 0 };
    EXPECT_TRUE(lexical_error_is_overflow(&overflow));
    EXPECT_FALSE(lexical_error_is_overflow(&underflow));
}

TEST(is_underflow, error_tests)
{
    lexical_error overflow = { lexical_overflow, 0 };
    lexical_error underflow = { lexical_underflow, 0 };
    EXPECT_FALSE(lexical_error_is_underflow(&overflow));
    EXPECT_TRUE(lexical_error_is_underflow(&underflow));
}

TEST(is_invalid_digit, error_tests)
{
    lexical_error overflow = { lexical_overflow, 0 };
    lexical_error invalid_digit = { lexical_invalid_digit, 0 };
    EXPECT_FALSE(lexical_error_is_invalid_digit(&overflow));
    EXPECT_TRUE(lexical_error_is_invalid_digit(&invalid_digit));
}

TEST(is_empty, error_tests)
{
    lexical_error overflow = { lexical_overflow, 0 };
    lexical_error empty = { lexical_empty, 0 };
    EXPECT_FALSE(lexical_error_is_empty(&overflow));
    EXPECT_TRUE(lexical_error_is_empty(&empty));
}

TEST(is_empty_mantissa, error_tests)
{
    lexical_error overflow = { lexical_overflow, 0 };
    lexical_error empty_mantissa = { lexical_empty_mantissa, 0 };
    EXPECT_FALSE(lexical_error_is_empty_mantissa(&overflow));
    EXPECT_TRUE(lexical_error_is_empty_mantissa(&empty_mantissa));
}

TEST(is_empty_exponent, error_tests)
{
    lexical_error overflow = { lexical_overflow, 0 };
    lexical_error empty_exponent = { lexical_empty_exponent, 0 };
    EXPECT_FALSE(lexical_error_is_empty_exponent(&overflow));
    EXPECT_TRUE(lexical_error_is_empty_exponent(&empty_exponent));
}

TEST(is_empty_integer, error_tests)
{
    lexical_error overflow = { lexical_overflow, 0 };
    lexical_error empty_integer = { lexical_empty_integer, 0 };
    EXPECT_FALSE(lexical_error_is_empty_integer(&overflow));
    EXPECT_TRUE(lexical_error_is_empty_integer(&empty_integer));
}

TEST(is_empty_fraction, error_tests)
{
    lexical_error overflow = { lexical_overflow, 0 };
    lexical_error empty_fraction = { lexical_empty_fraction, 0 };
    EXPECT_FALSE(lexical_error_is_empty_fraction(&overflow));
    EXPECT_TRUE(lexical_error_is_empty_fraction(&empty_fraction));
}

TEST(is_invalid_positive_mantissa_sign, error_tests)
{
    lexical_error overflow = { lexical_overflow, 0 };
    lexical_error invalid_positive_mantissa_sign = { lexical_invalid_positive_mantissa_sign, 0 };
    EXPECT_FALSE(lexical_error_is_invalid_positive_mantissa_sign(&overflow));
    EXPECT_TRUE(lexical_error_is_invalid_positive_mantissa_sign(&invalid_positive_mantissa_sign));
}

TEST(is_missing_mantissa_sign, error_tests)
{
    lexical_error overflow = { lexical_overflow, 0 };
    lexical_error missing_mantissa_sign = { lexical_missing_mantissa_sign, 0 };
    EXPECT_FALSE(lexical_error_is_missing_mantissa_sign(&overflow));
    EXPECT_TRUE(lexical_error_is_missing_mantissa_sign(&missing_mantissa_sign));
}

TEST(is_invalid_exponent, error_tests)
{
    lexical_error overflow = { lexical_overflow, 0 };
    lexical_error invalid_exponent = { lexical_invalid_exponent, 0 };
    EXPECT_FALSE(lexical_error_is_invalid_exponent(&overflow));
    EXPECT_TRUE(lexical_error_is_invalid_exponent(&invalid_exponent));
}

TEST(is_invalid_positive_exponent_sign, error_tests)
{
    lexical_error overflow = { lexical_overflow, 0 };
    lexical_error invalid_positive_exponent_sign = { lexical_invalid_positive_exponent_sign, 0 };
    EXPECT_FALSE(lexical_error_is_invalid_positive_exponent_sign(&overflow));
    EXPECT_TRUE(lexical_error_is_invalid_positive_exponent_sign(&invalid_positive_exponent_sign));
}

TEST(is_missing_exponent_sign, error_tests)
{
    lexical_error overflow = { lexical_overflow, 0 };
    lexical_error missing_exponent_sign = { lexical_missing_exponent_sign, 0 };
    EXPECT_FALSE(lexical_error_is_missing_exponent_sign(&overflow));
    EXPECT_TRUE(lexical_error_is_missing_exponent_sign(&missing_exponent_sign));
}

TEST(is_exponent_without_fraction, error_tests)
{
    lexical_error overflow = { lexical_overflow, 0 };
    lexical_error exponent_without_fraction = { lexical_exponent_without_fraction, 0 };
    EXPECT_FALSE(lexical_error_is_exponent_without_fraction(&overflow));
    EXPECT_TRUE(lexical_error_is_exponent_without_fraction(&exponent_without_fraction));
}

TEST(is_invalid_leading_zeros, error_tests)
{
    lexical_error overflow = { lexical_overflow, 0 };
    lexical_error invalid_leading_zeros = { lexical_invalid_leading_zeros, 0 };
    EXPECT_FALSE(lexical_error_is_invalid_leading_zeros(&overflow));
    EXPECT_TRUE(lexical_error_is_invalid_leading_zeros(&invalid_leading_zeros));
}

// NUMBER FORMAT TESTS

TEST(number_format, builder_tests)
{
    lexical_number_format_builder_t builder = lexical_number_format_builder();
#ifdef HAVE_FORMAT
    builder.digit_separator = uint8_t('_');
    builder.required_integer_digits = false;
    builder.required_fraction_digits = false;
    builder.required_exponent_digits = false;
    builder.no_positive_mantissa_sign = false;
    builder.required_mantissa_sign = false;
    builder.no_exponent_notation = false;
    builder.no_positive_exponent_sign = false;
    builder.required_exponent_sign = false;
    builder.no_exponent_without_fraction = false;
    builder.no_special = false;
    builder.case_sensitive_special = false;
    builder.no_integer_leading_zeros = false;
    builder.no_float_leading_zeros = false;
    builder.integer_internal_digit_separator = false;
    builder.fraction_internal_digit_separator = false;
    builder.exponent_internal_digit_separator = false;
    builder.integer_leading_digit_separator = false;
    builder.fraction_leading_digit_separator = false;
    builder.exponent_leading_digit_separator = false;
    builder.integer_trailing_digit_separator = false;
    builder.fraction_trailing_digit_separator = false;
    builder.exponent_trailing_digit_separator = false;
    builder.integer_consecutive_digit_separator = false;
    builder.fraction_consecutive_digit_separator = false;
    builder.exponent_consecutive_digit_separator = false;
    builder.special_digit_separator = false;
#endif  // HAVE_FORMAT

    lexical_number_format_option option = lexical_number_format_build(builder);
    ASSERT_TRUE(lexical_number_format_option_is_some(&option));

    lexical_number_format format = lexical_number_format_option_unwrap(option);
#ifdef HAVE_FORMAT
    EXPECT_EQ(lexical_number_format_digit_separator(format), uint8_t('\x00'));
    EXPECT_EQ(lexical_number_format_required_integer_digits(format), false);
    EXPECT_EQ(lexical_number_format_required_fraction_digits(format), false);
    EXPECT_EQ(lexical_number_format_required_exponent_digits(format), false);
    EXPECT_EQ(lexical_number_format_no_positive_mantissa_sign(format), false);
    EXPECT_EQ(lexical_number_format_required_mantissa_sign(format), false);
    EXPECT_EQ(lexical_number_format_no_exponent_notation(format), false);
    EXPECT_EQ(lexical_number_format_no_positive_exponent_sign(format), false);
    EXPECT_EQ(lexical_number_format_required_exponent_sign(format), false);
    EXPECT_EQ(lexical_number_format_no_exponent_without_fraction(format), false);
    EXPECT_EQ(lexical_number_format_no_special(format), false);
    EXPECT_EQ(lexical_number_format_case_sensitive_special(format), false);
    EXPECT_EQ(lexical_number_format_no_integer_leading_zeros(format), false);
    EXPECT_EQ(lexical_number_format_no_float_leading_zeros(format), false);
    EXPECT_EQ(lexical_number_format_integer_internal_digit_separator(format), false);
    EXPECT_EQ(lexical_number_format_fraction_internal_digit_separator(format), false);
    EXPECT_EQ(lexical_number_format_exponent_internal_digit_separator(format), false);
    EXPECT_EQ(lexical_number_format_integer_leading_digit_separator(format), false);
    EXPECT_EQ(lexical_number_format_fraction_leading_digit_separator(format), false);
    EXPECT_EQ(lexical_number_format_exponent_leading_digit_separator(format), false);
    EXPECT_EQ(lexical_number_format_integer_trailing_digit_separator(format), false);
    EXPECT_EQ(lexical_number_format_fraction_trailing_digit_separator(format), false);
    EXPECT_EQ(lexical_number_format_exponent_trailing_digit_separator(format), false);
    EXPECT_EQ(lexical_number_format_integer_consecutive_digit_separator(format), false);
    EXPECT_EQ(lexical_number_format_fraction_consecutive_digit_separator(format), false);
    EXPECT_EQ(lexical_number_format_exponent_consecutive_digit_separator(format), false);
    EXPECT_EQ(lexical_number_format_special_digit_separator(format), false);
#endif  // HAVE_FORMAT
}

TEST(number_format, constant_tests)
{
#ifdef HAVE_FORMAT
    lexical_number_format constants[] = {
        lexical_rust_literal,
        lexical_rust_string,
        lexical_rust_string_strict,
        lexical_python_literal,
        lexical_python_string,
        lexical_cxx17_literal,
        lexical_cxx17_string,
        lexical_cxx14_literal,
        lexical_cxx14_string,
        lexical_cxx11_literal,
        lexical_cxx11_string,
        lexical_cxx03_literal,
        lexical_cxx03_string,
        lexical_cxx98_literal,
        lexical_cxx98_string,
        lexical_c18_literal,
        lexical_c18_string,
        lexical_c11_literal,
        lexical_c11_string,
        lexical_c99_literal,
        lexical_c99_string,
        lexical_c90_literal,
        lexical_c90_string,
        lexical_c89_literal,
        lexical_c89_string,
        lexical_ruby_literal,
        lexical_ruby_string,
        lexical_swift_literal,
        lexical_swift_string,
        lexical_go_literal,
        lexical_go_string,
        lexical_haskell_literal,
        lexical_haskell_string,
        lexical_javascript_literal,
        lexical_javascript_string,
        lexical_perl_literal,
        lexical_perl_string,
        lexical_php_literal,
        lexical_php_string,
        lexical_java_literal,
        lexical_java_string,
        lexical_r_literal,
        lexical_r_string,
        lexical_kotlin_literal,
        lexical_kotlin_string,
        lexical_julia_literal,
        lexical_julia_string,
        lexical_csharp7_literal,
        lexical_csharp7_string,
        lexical_csharp6_literal,
        lexical_csharp6_string,
        lexical_csharp5_literal,
        lexical_csharp5_string,
        lexical_csharp4_literal,
        lexical_csharp4_string,
        lexical_csharp3_literal,
        lexical_csharp3_string,
        lexical_csharp2_literal,
        lexical_csharp2_string,
        lexical_csharp1_literal,
        lexical_csharp1_string,
        lexical_kawa_literal,
        lexical_kawa_string,
        lexical_gambitc_literal,
        lexical_gambitc_string,
        lexical_guile_literal,
        lexical_guile_string,
        lexical_clojure_literal,
        lexical_clojure_string,
        lexical_erlang_literal,
        lexical_erlang_string,
        lexical_elm_literal,
        lexical_elm_string,
        lexical_scala_literal,
        lexical_scala_string,
        lexical_elixir_literal,
        lexical_elixir_string,
        lexical_fortran_literal,
        lexical_fortran_string,
        lexical_d_literal,
        lexical_d_string,
        lexical_coffeescript_literal,
        lexical_coffeescript_string,
        lexical_cobol_literal,
        lexical_cobol_string,
        lexical_fsharp_literal,
        lexical_fsharp_string,
        lexical_vb_literal,
        lexical_vb_string,
        lexical_ocaml_literal,
        lexical_ocaml_string,
        lexical_objectivec_literal,
        lexical_objectivec_string,
        lexical_reasonml_literal,
        lexical_reasonml_string,
        lexical_octave_literal,
        lexical_octave_string,
        lexical_matlab_literal,
        lexical_matlab_string,
        lexical_zig_literal,
        lexical_zig_string,
        lexical_sage_literal,
        lexical_sage_string,
        lexical_json,
        lexical_toml,
        lexical_yaml,
        lexical_xml,
        lexical_sqlite,
        lexical_postgresql,
        lexical_mysql,
        lexical_mongodb
    };
    // Just insure we can compile them.
    for (lexical_number_format value: constants);
#endif  // HAVE_FORMAT
}

// PARSE INTEGER OPTIONS TESTS

TEST(parse_integer_options, builder_tests)
{
    lexical_parse_integer_options_builder_t builder = lexical_parse_integer_options_builder();
#ifdef  HAVE_RADIX
    builder.radix = 2;
#endif  // HAVE_RADIX
#ifdef  HAVE_FORMAT
    builder.format = lexical_json;
#endif  // HAVE_FORMAT

    lexical_parse_integer_options_option option = lexical_parse_integer_options_build(builder);
    ASSERT_TRUE(lexical_parse_integer_options_option_is_some(&option));
    lexical_parse_integer_options options = lexical_parse_integer_options_option_unwrap(option);
    // Ignore the fields, we just want to ensure it compiles.
}

// PARSE FLOAT OPTIONS TESTS

TEST(parse_float_options, builder_tests)
{
    constexpr char const* nan_string = "NAN";
    constexpr char const* inf_string = "INF";
    constexpr char const* infinity_string = "INFINITY";

    lexical_parse_float_options_builder_t builder = lexical_parse_float_options_builder();
    builder.lossy = true;
    builder.exponent_char = uint8_t('e');
    builder.nan_string_ptr = reinterpret_cast<uint8_t const*>(nan_string);
    builder.nan_string_length = strlen(nan_string);
    builder.inf_string_ptr = reinterpret_cast<uint8_t const*>(inf_string);
    builder.inf_string_length = strlen(inf_string);
    builder.infinity_string_ptr = reinterpret_cast<uint8_t const*>(infinity_string);
    builder.infinity_string_length = strlen(infinity_string);
#ifdef  HAVE_RADIX
    builder.radix = 2;
#endif  // HAVE_RADIX
#ifdef  HAVE_FORMAT
    builder.format = lexical_json;
#endif  // HAVE_FORMAT
#ifdef  HAVE_ROUNDING
    builder.rounding = lexical_toward_zero;
#endif  // HAVE_ROUNDING

    lexical_parse_float_options_option option = lexical_parse_float_options_build(builder);
    ASSERT_TRUE(lexical_parse_float_options_option_is_some(&option));
    lexical_parse_float_options options = lexical_parse_float_options_option_unwrap(option);
    EXPECT_EQ(options.lossy, true);
    EXPECT_EQ(options.exponent_char, uint8_t('e'));
    EXPECT_EQ(options.nan_string_ptr, reinterpret_cast<uint8_t const*>(nan_string));
    EXPECT_EQ(options.nan_string_length, 3);
    EXPECT_EQ(options.inf_string_ptr, reinterpret_cast<uint8_t const*>(inf_string));
    EXPECT_EQ(options.inf_string_length, 3);
    EXPECT_EQ(options.infinity_string_ptr, reinterpret_cast<uint8_t const*>(infinity_string));
    EXPECT_EQ(options.infinity_string_length, 8);
}

// WRITE INTEGER OPTIONS TESTS

TEST(write_integer_options, builder_tests)
{
    lexical_write_integer_options_builder_t builder = lexical_write_integer_options_builder();
#ifdef  HAVE_RADIX
    builder.radix = 2;
#endif  // HAVE_RADIX

    lexical_write_integer_options_option option = lexical_write_integer_options_build(builder);
    ASSERT_TRUE(lexical_write_integer_options_option_is_some(&option));
    lexical_write_integer_options options = lexical_write_integer_options_option_unwrap(option);
    // Ignore the fields, we just want to ensure it compiles.
}

// WRITE FLOAT OPTIONS TESTS

TEST(write_float_options, builder_tests)
{
    constexpr char const* nan_string = "NAN";
    constexpr char const* inf_string = "INF";

    lexical_write_float_options_builder_t builder = lexical_write_float_options_builder();
    builder.exponent_char = uint8_t('e');
    builder.trim_floats = true;
    builder.nan_string_ptr = reinterpret_cast<uint8_t const*>(nan_string);
    builder.nan_string_length = strlen(nan_string);
    builder.inf_string_ptr = reinterpret_cast<uint8_t const*>(inf_string);
    builder.inf_string_length = strlen(inf_string);
#ifdef  HAVE_RADIX
    builder.radix = 2;
#endif  // HAVE_RADIX

    lexical_write_float_options_option option = lexical_write_float_options_build(builder);
    EXPECT_TRUE(lexical_write_float_options_option_is_some(&option));
    lexical_write_float_options options = lexical_write_float_options_option_unwrap(option);
    EXPECT_EQ(options.exponent_char, uint8_t('e'));
    EXPECT_EQ(options.trim_floats, true);
    EXPECT_EQ(options.nan_string_ptr, reinterpret_cast<uint8_t const*>(nan_string));
    EXPECT_EQ(options.nan_string_length, 3);
    EXPECT_EQ(options.inf_string_ptr, reinterpret_cast<uint8_t const*>(inf_string));
    EXPECT_EQ(options.inf_string_length, 3);
}

// RESULT TESTS

TEST(result, result_tests)
{
    auto ok = result_ok(0);
    auto overflow = result_overflow(0);
    auto underflow = result_underflow(0);
    auto invalid_digit = result_invalid_digit(0);
    auto empty = result_empty(0);
    auto empty_mantissa = result_empty_mantissa(0);
    auto empty_exponent = result_empty_exponent(0);
    auto empty_integer = result_empty_integer(0);
    auto empty_fraction = result_empty_fraction(0);
    auto invalid_positive_mantissa_sign = result_invalid_positive_mantissa_sign(0);
    auto missing_mantissa_sign = result_missing_mantissa_sign(0);
    auto invalid_exponent = result_invalid_exponent(0);
    auto invalid_positive_exponent_sign = result_invalid_positive_exponent_sign(0);
    auto missing_exponent_sign = result_missing_exponent_sign(0);
    auto exponent_without_fraction = result_exponent_without_fraction(0);
    auto invalid_leading_zeros = result_invalid_leading_zeros(0);

    EXPECT_TRUE(lexical_i8_result_is_ok(&ok));
    EXPECT_FALSE(lexical_i8_result_is_err(&ok));
    EXPECT_TRUE(lexical_i8_result_is_err(&overflow));
    EXPECT_TRUE(lexical_i8_result_is_err(&underflow));
    EXPECT_TRUE(lexical_i8_result_is_err(&invalid_digit));
    EXPECT_TRUE(lexical_i8_result_is_err(&empty));
    EXPECT_TRUE(lexical_i8_result_is_err(&empty_mantissa));
    EXPECT_TRUE(lexical_i8_result_is_err(&empty_exponent));
    EXPECT_TRUE(lexical_i8_result_is_err(&empty_integer));
    EXPECT_TRUE(lexical_i8_result_is_err(&empty_fraction));
    EXPECT_TRUE(lexical_i8_result_is_err(&invalid_positive_mantissa_sign));
    EXPECT_TRUE(lexical_i8_result_is_err(&missing_mantissa_sign));
    EXPECT_TRUE(lexical_i8_result_is_err(&invalid_exponent));
    EXPECT_TRUE(lexical_i8_result_is_err(&invalid_positive_exponent_sign));
    EXPECT_TRUE(lexical_i8_result_is_err(&missing_exponent_sign));
    EXPECT_TRUE(lexical_i8_result_is_err(&exponent_without_fraction));
    EXPECT_TRUE(lexical_i8_result_is_err(&invalid_leading_zeros));

    EXPECT_EQ(lexical_i8_result_ok(ok), 0);
    EXPECT_TRUE(is_overflow(lexical_i8_result_err(overflow)));
    EXPECT_TRUE(is_underflow(lexical_i8_result_err(underflow)));
    EXPECT_TRUE(is_invalid_digit(lexical_i8_result_err(invalid_digit)));
    EXPECT_TRUE(is_empty(lexical_i8_result_err(empty)));
    EXPECT_TRUE(is_empty_mantissa(lexical_i8_result_err(empty_mantissa)));
    EXPECT_TRUE(is_empty_exponent(lexical_i8_result_err(empty_exponent)));
    EXPECT_TRUE(is_empty_integer(lexical_i8_result_err(empty_integer)));
    EXPECT_TRUE(is_empty_fraction(lexical_i8_result_err(empty_fraction)));
    EXPECT_TRUE(is_invalid_positive_mantissa_sign(lexical_i8_result_err(invalid_positive_mantissa_sign)));
    EXPECT_TRUE(is_missing_mantissa_sign(lexical_i8_result_err(missing_mantissa_sign)));
    EXPECT_TRUE(is_invalid_exponent(lexical_i8_result_err(invalid_exponent)));
    EXPECT_TRUE(is_invalid_positive_exponent_sign(lexical_i8_result_err(invalid_positive_exponent_sign)));
    EXPECT_TRUE(is_missing_exponent_sign(lexical_i8_result_err(missing_exponent_sign)));
    EXPECT_TRUE(is_exponent_without_fraction(lexical_i8_result_err(exponent_without_fraction)));
    EXPECT_TRUE(is_invalid_leading_zeros(lexical_i8_result_err(invalid_leading_zeros)));
}

// PARTIAL RESULT TESTS

TEST(partial_result, partial_result_tests)
{
    auto ok = partial_result_ok(0, 1);
    auto overflow = partial_result_overflow(0);
    auto underflow = partial_result_underflow(0);
    auto invalid_digit = partial_result_invalid_digit(0);
    auto empty = partial_result_empty(0);
    auto empty_mantissa = partial_result_empty_mantissa(0);
    auto empty_exponent = partial_result_empty_exponent(0);
    auto empty_integer = partial_result_empty_integer(0);
    auto empty_fraction = partial_result_empty_fraction(0);
    auto invalid_positive_mantissa_sign = partial_result_invalid_positive_mantissa_sign(0);
    auto missing_mantissa_sign = partial_result_missing_mantissa_sign(0);
    auto invalid_exponent = partial_result_invalid_exponent(0);
    auto invalid_positive_exponent_sign = partial_result_invalid_positive_exponent_sign(0);
    auto missing_exponent_sign = partial_result_missing_exponent_sign(0);
    auto exponent_without_fraction = partial_result_exponent_without_fraction(0);
    auto invalid_leading_zeros = partial_result_invalid_leading_zeros(0);

    EXPECT_TRUE(lexical_i8_partial_result_is_ok(&ok));
    EXPECT_FALSE(lexical_i8_partial_result_is_err(&ok));
    EXPECT_TRUE(lexical_i8_partial_result_is_err(&overflow));
    EXPECT_TRUE(lexical_i8_partial_result_is_err(&underflow));
    EXPECT_TRUE(lexical_i8_partial_result_is_err(&invalid_digit));
    EXPECT_TRUE(lexical_i8_partial_result_is_err(&empty));
    EXPECT_TRUE(lexical_i8_partial_result_is_err(&empty_mantissa));
    EXPECT_TRUE(lexical_i8_partial_result_is_err(&empty_exponent));
    EXPECT_TRUE(lexical_i8_partial_result_is_err(&empty_integer));
    EXPECT_TRUE(lexical_i8_partial_result_is_err(&empty_fraction));
    EXPECT_TRUE(lexical_i8_partial_result_is_err(&invalid_positive_mantissa_sign));
    EXPECT_TRUE(lexical_i8_partial_result_is_err(&missing_mantissa_sign));
    EXPECT_TRUE(lexical_i8_partial_result_is_err(&invalid_exponent));
    EXPECT_TRUE(lexical_i8_partial_result_is_err(&invalid_positive_exponent_sign));
    EXPECT_TRUE(lexical_i8_partial_result_is_err(&missing_exponent_sign));
    EXPECT_TRUE(lexical_i8_partial_result_is_err(&exponent_without_fraction));
    EXPECT_TRUE(lexical_i8_partial_result_is_err(&invalid_leading_zeros));

    EXPECT_EQ(lexical_i8_partial_result_ok(ok).x, 0);
    EXPECT_TRUE(is_overflow(lexical_i8_partial_result_err(overflow)));
    EXPECT_TRUE(is_underflow(lexical_i8_partial_result_err(underflow)));
    EXPECT_TRUE(is_invalid_digit(lexical_i8_partial_result_err(invalid_digit)));
    EXPECT_TRUE(is_empty(lexical_i8_partial_result_err(empty)));
    EXPECT_TRUE(is_empty_mantissa(lexical_i8_partial_result_err(empty_mantissa)));
    EXPECT_TRUE(is_empty_exponent(lexical_i8_partial_result_err(empty_exponent)));
    EXPECT_TRUE(is_empty_integer(lexical_i8_partial_result_err(empty_integer)));
    EXPECT_TRUE(is_empty_fraction(lexical_i8_partial_result_err(empty_fraction)));
    EXPECT_TRUE(is_invalid_positive_mantissa_sign(lexical_i8_partial_result_err(invalid_positive_mantissa_sign)));
    EXPECT_TRUE(is_missing_mantissa_sign(lexical_i8_partial_result_err(missing_mantissa_sign)));
    EXPECT_TRUE(is_invalid_exponent(lexical_i8_partial_result_err(invalid_exponent)));
    EXPECT_TRUE(is_invalid_positive_exponent_sign(lexical_i8_partial_result_err(invalid_positive_exponent_sign)));
    EXPECT_TRUE(is_missing_exponent_sign(lexical_i8_partial_result_err(missing_exponent_sign)));
    EXPECT_TRUE(is_exponent_without_fraction(lexical_i8_partial_result_err(exponent_without_fraction)));
    EXPECT_TRUE(is_invalid_leading_zeros(lexical_i8_partial_result_err(invalid_leading_zeros)));
}
