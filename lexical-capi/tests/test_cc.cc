/**
 *  Unittests for the C++ API to lexical-core.
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

#include <gtest/gtest.h>
#include "lexical.hpp"

using namespace lexical;

// HELPERS
// -------

template <typename T>
inline result<T> result_ok(T value)
{
    // Initialize the union.
    result_union<T> u;
    u.value = value;

    // Create the tagged union.
    result<T> r;
    r.tag = result_tag::ok;
    r.data = u;

    return r;
}

template <typename T>
inline result<T> result_err(error_code code, size_t index)
{
    // Initialize the error.
    error e;
    e.code = code;
    e.index = index;

    // Initialize the union.
    result_union<T> u;
    u.error = e;

    // Create the tagged union.
    result<T> r;
    r.tag = result_tag::err;
    r.data = u;

    return r;
}

#define lexical_result_error(type)                                              \
    template <typename T>                                                       \
    inline result<T> result_##type(size_t index)                                \
    {                                                                           \
        return result_err<T>(error_code::type, index);                          \
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

template <typename T>
inline partial_result<T> partial_result_ok(T value, size_t index)
{
    // Initialize the tuple.
    partial_result_tuple<T> t;
    t.x = value;
    t.y = index;

    // Initialize the union.
    partial_result_union<T> u;
    u.value = t;

    // Create the tagged union.
    partial_result<T> r;
    r.tag = result_tag::ok;
    r.data = u;

    return r;
}

template <typename T>
inline partial_result<T> partial_result_err(error_code code, size_t index)
{
    // Initialize the error.
    error e;
    e.code = code;
    e.index = index;

    // Initialize the union.
    partial_result_union<T> u;
    u.error = e;

    // Create the tagged union.
    partial_result<T> r;
    r.tag = result_tag::err;
    r.data = u;

    return r;
}

#define lexical_partial_result_error(type)                                      \
    template <typename T>                                                       \
    inline partial_result<T> partial_result_##type(size_t index)                \
    {                                                                           \
        return partial_result_err<T>(error_code::type, index);                  \
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

// CONSTANT TESTS

TEST(test_size, constant_tests)
{
    // Check all the sizes can be used.
    EXPECT_GE(I8_FORMATTED_SIZE, 0);
    EXPECT_GE(I16_FORMATTED_SIZE, 0);
    EXPECT_GE(I32_FORMATTED_SIZE, 0);
    EXPECT_GE(I64_FORMATTED_SIZE, 0);
    EXPECT_GE(ISIZE_FORMATTED_SIZE, 0);
    EXPECT_GE(U8_FORMATTED_SIZE, 0);
    EXPECT_GE(U16_FORMATTED_SIZE, 0);
    EXPECT_GE(U32_FORMATTED_SIZE, 0);
    EXPECT_GE(U64_FORMATTED_SIZE, 0);
    EXPECT_GE(USIZE_FORMATTED_SIZE, 0);
    EXPECT_GE(F32_FORMATTED_SIZE, 0);
    EXPECT_GE(F64_FORMATTED_SIZE, 0);
    EXPECT_GE(I8_FORMATTED_SIZE_DECIMAL, 0);
    EXPECT_GE(I16_FORMATTED_SIZE_DECIMAL, 0);
    EXPECT_GE(I32_FORMATTED_SIZE_DECIMAL, 0);
    EXPECT_GE(I64_FORMATTED_SIZE_DECIMAL, 0);
    EXPECT_GE(ISIZE_FORMATTED_SIZE_DECIMAL, 0);
    EXPECT_GE(U8_FORMATTED_SIZE_DECIMAL, 0);
    EXPECT_GE(U16_FORMATTED_SIZE_DECIMAL, 0);
    EXPECT_GE(U32_FORMATTED_SIZE_DECIMAL, 0);
    EXPECT_GE(U64_FORMATTED_SIZE_DECIMAL, 0);
    EXPECT_GE(USIZE_FORMATTED_SIZE_DECIMAL, 0);
    EXPECT_GE(F32_FORMATTED_SIZE_DECIMAL, 0);
    EXPECT_GE(F64_FORMATTED_SIZE_DECIMAL, 0);
    EXPECT_GE(BUFFER_SIZE, 0);
}

// ERROR TESTS

TEST(test_is_overflow, error_tests)
{
    error overflow = { error_code::overflow, 0 };
    error underflow = { error_code::underflow, 0 };
    EXPECT_TRUE(overflow.is_overflow());
    EXPECT_FALSE(underflow.is_overflow());
}

TEST(test_is_underflow, error_tests)
{
    error overflow = { error_code::overflow, 0 };
    error underflow = { error_code::underflow, 0 };
    EXPECT_FALSE(overflow.is_underflow());
    EXPECT_TRUE(underflow.is_underflow());
}

TEST(test_is_invalid_digit, error_tests)
{
    error overflow = { error_code::overflow, 0 };
    error invalid_digit = { error_code::invalid_digit, 0 };
    EXPECT_FALSE(overflow.is_invalid_digit());
    EXPECT_TRUE(invalid_digit.is_invalid_digit());
}

TEST(test_is_empty, error_tests)
{
    error overflow = { error_code::overflow, 0 };
    error empty = { error_code::empty, 0 };
    EXPECT_FALSE(overflow.is_empty());
    EXPECT_TRUE(empty.is_empty());
}

TEST(test_is_empty_mantissa, error_tests)
{
    error overflow = { error_code::overflow, 0 };
    error empty_mantissa = { error_code::empty_mantissa, 0 };
    EXPECT_FALSE(overflow.is_empty_mantissa());
    EXPECT_TRUE(empty_mantissa.is_empty_mantissa());
}

TEST(test_is_empty_exponent, error_tests)
{
    error overflow = { error_code::overflow, 0 };
    error empty_exponent = { error_code::empty_exponent, 0 };
    EXPECT_FALSE(overflow.is_empty_exponent());
    EXPECT_TRUE(empty_exponent.is_empty_exponent());
}

TEST(test_is_empty_integer, error_tests)
{
    error overflow = { error_code::overflow, 0 };
    error empty_integer = { error_code::empty_integer, 0 };
    EXPECT_FALSE(overflow.is_empty_integer());
    EXPECT_TRUE(empty_integer.is_empty_integer());
}

TEST(test_is_empty_fraction, error_tests)
{
    error overflow = { error_code::overflow, 0 };
    error empty_fraction = { error_code::empty_fraction, 0 };
    EXPECT_FALSE(overflow.is_empty_fraction());
    EXPECT_TRUE(empty_fraction.is_empty_fraction());
}

TEST(test_is_invalid_positive_mantissa_sign, error_tests)
{
    error overflow = { error_code::overflow, 0 };
    error invalid_positive_mantissa_sign = { error_code::invalid_positive_mantissa_sign, 0 };
    EXPECT_FALSE(overflow.is_invalid_positive_mantissa_sign());
    EXPECT_TRUE(invalid_positive_mantissa_sign.is_invalid_positive_mantissa_sign());
}

TEST(test_is_missing_mantissa_sign, error_tests)
{
    error overflow = { error_code::overflow, 0 };
    error missing_mantissa_sign = { error_code::missing_mantissa_sign, 0 };
    EXPECT_FALSE(overflow.is_missing_mantissa_sign());
    EXPECT_TRUE(missing_mantissa_sign.is_missing_mantissa_sign());
}

TEST(test_is_invalid_exponent, error_tests)
{
    error overflow = { error_code::overflow, 0 };
    error invalid_exponent = { error_code::invalid_exponent, 0 };
    EXPECT_FALSE(overflow.is_invalid_exponent());
    EXPECT_TRUE(invalid_exponent.is_invalid_exponent());
}

TEST(test_is_invalid_positive_exponent_sign, error_tests)
{
    error overflow = { error_code::overflow, 0 };
    error invalid_positive_exponent_sign = { error_code::invalid_positive_exponent_sign, 0 };
    EXPECT_FALSE(overflow.is_invalid_positive_exponent_sign());
    EXPECT_TRUE(invalid_positive_exponent_sign.is_invalid_positive_exponent_sign());
}

TEST(test_is_missing_exponent_sign, error_tests)
{
    error overflow = { error_code::overflow, 0 };
    error missing_exponent_sign = { error_code::missing_exponent_sign, 0 };
    EXPECT_FALSE(overflow.is_missing_exponent_sign());
    EXPECT_TRUE(missing_exponent_sign.is_missing_exponent_sign());
}

TEST(test_is_exponent_without_fraction, error_tests)
{
    error overflow = { error_code::overflow, 0 };
    error exponent_without_fraction = { error_code::exponent_without_fraction, 0 };
    EXPECT_FALSE(overflow.is_exponent_without_fraction());
    EXPECT_TRUE(exponent_without_fraction.is_exponent_without_fraction());
}

TEST(test_is_invalid_leading_zeros, error_tests)
{
    error overflow = { error_code::overflow, 0 };
    error invalid_leading_zeros = { error_code::invalid_leading_zeros, 0 };
    EXPECT_FALSE(overflow.is_invalid_leading_zeros());
    EXPECT_TRUE(invalid_leading_zeros.is_invalid_leading_zeros());
}

// NUMBER FORMAT TESTS

TEST(number_format, builder_tests)
{
    auto builder = number_format::builder();
#ifdef HAVE_FORMAT
     builder
        .digit_separator(uint8_t('_'))
        .required_integer_digits(false)
        .required_fraction_digits(false)
        .required_exponent_digits(false)
        .no_positive_mantissa_sign(false)
        .required_mantissa_sign(false)
        .no_exponent_notation(false)
        .no_positive_exponent_sign(false)
        .required_exponent_sign(false)
        .no_exponent_without_fraction(false)
        .no_special(false)
        .case_sensitive_special(false)
        .no_integer_leading_zeros(false)
        .no_float_leading_zeros(false)
        .integer_internal_digit_separator(false)
        .fraction_internal_digit_separator(false)
        .exponent_internal_digit_separator(false)
        .integer_leading_digit_separator(false)
        .fraction_leading_digit_separator(false)
        .exponent_leading_digit_separator(false)
        .integer_trailing_digit_separator(false)
        .fraction_trailing_digit_separator(false)
        .exponent_trailing_digit_separator(false)
        .integer_consecutive_digit_separator(false)
        .fraction_consecutive_digit_separator(false)
        .exponent_consecutive_digit_separator(false)
        .special_digit_separator(false);
#endif  // HAVE_FORMAT

    auto option = builder.build();
    ASSERT_TRUE(option.is_some());

    auto format = option.unwrap();
#ifdef HAVE_FORMAT
    EXPECT_EQ(format.digit_separator(), uint8_t('\x00'));
    EXPECT_EQ(format.flags(), format);
    EXPECT_EQ(format.has_required_integer_digits(), false);
    EXPECT_EQ(format.has_required_fraction_digits(), false);
    EXPECT_EQ(format.has_required_exponent_digits(), false);
    EXPECT_EQ(format.has_no_positive_mantissa_sign(), false);
    EXPECT_EQ(format.has_required_mantissa_sign(), false);
    EXPECT_EQ(format.has_no_exponent_notation(), false);
    EXPECT_EQ(format.has_no_positive_exponent_sign(), false);
    EXPECT_EQ(format.has_required_exponent_sign(), false);
    EXPECT_EQ(format.has_no_exponent_without_fraction(), false);
    EXPECT_EQ(format.has_no_special(), false);
    EXPECT_EQ(format.has_case_sensitive_special(), false);
    EXPECT_EQ(format.has_no_integer_leading_zeros(), false);
    EXPECT_EQ(format.has_no_float_leading_zeros(), false);
    EXPECT_EQ(format.has_integer_internal_digit_separator(), false);
    EXPECT_EQ(format.has_fraction_internal_digit_separator(), false);
    EXPECT_EQ(format.has_exponent_internal_digit_separator(), false);
    EXPECT_EQ(format.has_integer_leading_digit_separator(), false);
    EXPECT_EQ(format.has_fraction_leading_digit_separator(), false);
    EXPECT_EQ(format.has_exponent_leading_digit_separator(), false);
    EXPECT_EQ(format.has_integer_trailing_digit_separator(), false);
    EXPECT_EQ(format.has_fraction_trailing_digit_separator(), false);
    EXPECT_EQ(format.has_exponent_trailing_digit_separator(), false);
    EXPECT_EQ(format.has_integer_consecutive_digit_separator(), false);
    EXPECT_EQ(format.has_fraction_consecutive_digit_separator(), false);
    EXPECT_EQ(format.has_exponent_consecutive_digit_separator(), false);
    EXPECT_EQ(format.has_special_digit_separator(), false);
#endif  // HAVE_FORMAT
}

TEST(number_format, permissive_tests)
{
#ifdef HAVE_FORMAT
    auto format = number_format::permissive().unwrap();
    EXPECT_EQ(format.digit_separator(), uint8_t('\x00'));
    EXPECT_EQ(format.flags(), format);
#endif  // HAVE_FORMAT
}

TEST(number_format, standard_tests)
{
#ifdef HAVE_FORMAT
    auto format = number_format::standard().unwrap();
    EXPECT_EQ(format.digit_separator(), uint8_t('\x00'));
    EXPECT_EQ(format.flags(), number_format::rust_string);
#endif  // HAVE_FORMAT
}

TEST(number_format, ignore_tests)
{
#ifdef HAVE_FORMAT
    auto format = number_format::ignore(uint8_t('_')).unwrap();
    EXPECT_EQ(format.digit_separator(), uint8_t('_'));
    EXPECT_EQ(format.flags(), number_format::digit_separator_flag_mask);
#endif  // HAVE_FORMAT
}

TEST(number_format, constant_tests)
{
#ifdef HAVE_FORMAT
    number_format constants[] = {
        number_format::rust_literal,
        number_format::rust_string,
        number_format::rust_string_strict,
        number_format::python_literal,
        number_format::python_string,
        number_format::cxx17_literal,
        number_format::cxx17_string,
        number_format::cxx14_literal,
        number_format::cxx14_string,
        number_format::cxx11_literal,
        number_format::cxx11_string,
        number_format::cxx03_literal,
        number_format::cxx03_string,
        number_format::cxx98_literal,
        number_format::cxx98_string,
        number_format::c18_literal,
        number_format::c18_string,
        number_format::c11_literal,
        number_format::c11_string,
        number_format::c99_literal,
        number_format::c99_string,
        number_format::c90_literal,
        number_format::c90_string,
        number_format::c89_literal,
        number_format::c89_string,
        number_format::ruby_literal,
        number_format::ruby_string,
        number_format::swift_literal,
        number_format::swift_string,
        number_format::go_literal,
        number_format::go_string,
        number_format::haskell_literal,
        number_format::haskell_string,
        number_format::javascript_literal,
        number_format::javascript_string,
        number_format::perl_literal,
        number_format::perl_string,
        number_format::php_literal,
        number_format::php_string,
        number_format::java_literal,
        number_format::java_string,
        number_format::r_literal,
        number_format::r_string,
        number_format::kotlin_literal,
        number_format::kotlin_string,
        number_format::julia_literal,
        number_format::julia_string,
        number_format::csharp7_literal,
        number_format::csharp7_string,
        number_format::csharp6_literal,
        number_format::csharp6_string,
        number_format::csharp5_literal,
        number_format::csharp5_string,
        number_format::csharp4_literal,
        number_format::csharp4_string,
        number_format::csharp3_literal,
        number_format::csharp3_string,
        number_format::csharp2_literal,
        number_format::csharp2_string,
        number_format::csharp1_literal,
        number_format::csharp1_string,
        number_format::kawa_literal,
        number_format::kawa_string,
        number_format::gambitc_literal,
        number_format::gambitc_string,
        number_format::guile_literal,
        number_format::guile_string,
        number_format::clojure_literal,
        number_format::clojure_string,
        number_format::erlang_literal,
        number_format::erlang_string,
        number_format::elm_literal,
        number_format::elm_string,
        number_format::scala_literal,
        number_format::scala_string,
        number_format::elixir_literal,
        number_format::elixir_string,
        number_format::fortran_literal,
        number_format::fortran_string,
        number_format::d_literal,
        number_format::d_string,
        number_format::coffeescript_literal,
        number_format::coffeescript_string,
        number_format::cobol_literal,
        number_format::cobol_string,
        number_format::fsharp_literal,
        number_format::fsharp_string,
        number_format::vb_literal,
        number_format::vb_string,
        number_format::ocaml_literal,
        number_format::ocaml_string,
        number_format::objectivec_literal,
        number_format::objectivec_string,
        number_format::reasonml_literal,
        number_format::reasonml_string,
        number_format::octave_literal,
        number_format::octave_string,
        number_format::matlab_literal,
        number_format::matlab_string,
        number_format::zig_literal,
        number_format::zig_string,
        number_format::sage_literal,
        number_format::sage_string,
        number_format::json,
        number_format::toml,
        number_format::yaml,
        number_format::xml,
        number_format::sqlite,
        number_format::postgresql,
        number_format::mysql,
        number_format::mongodb
    };
    // Just insure we can compile them.
    for (number_format value: constants);
#endif  // HAVE_FORMAT
}

// PARSE INTEGER OPTIONS TESTS

TEST(parse_integer_options, builder_tests)
{
    auto builder = parse_integer_options::builder();
#ifdef  HAVE_RADIX
    builder.radix(2);
#endif  // HAVE_RADIX
#ifdef  HAVE_FORMAT
    builder.format(number_format::json);
#endif  // HAVE_FORMAT

    auto option = builder.build();
    ASSERT_TRUE(option.is_some());
    auto options = option.unwrap();
    // Ignore the fields, we just want to ensure it compiles.
}

// PARSE FLOAT OPTIONS TESTS

TEST(parse_float_options, builder_tests)
{
    constexpr std::string_view nan_string = "NAN";
    constexpr std::string_view inf_string = "INF";
    constexpr std::string_view infinity_string = "INFINITY";

    auto builder = parse_float_options::builder();
    builder
        .lossy(true)
        .exponent_char(uint8_t('e'))
        .nan_string(nan_string)
        .inf_string(inf_string)
        .infinity_string(infinity_string);
#ifdef  HAVE_RADIX
    builder.radix(2);
#endif  // HAVE_RADIX
#ifdef  HAVE_FORMAT
    builder.format(number_format::json);
#endif  // HAVE_FORMAT
#ifdef  HAVE_ROUNDING
    builder.rounding(rounding_kind::toward_zero);
#endif  // HAVE_ROUNDING

    auto option = builder.build();
    ASSERT_TRUE(option.is_some());
    auto options = option.unwrap();
    EXPECT_EQ(options.lossy(), true);
    EXPECT_EQ(options.exponent_char(), uint8_t('e'));
    EXPECT_EQ(options.nan_string(), nan_string);
    EXPECT_EQ(options.inf_string(), inf_string);
    EXPECT_EQ(options.infinity_string(), infinity_string);
}

// WRITE INTEGER OPTIONS TESTS

TEST(write_integer_options, builder_tests)
{
    auto builder = write_integer_options::builder();
#ifdef  HAVE_RADIX
    builder.radix(2);
#endif  // HAVE_RADIX

    auto option = builder.build();
    ASSERT_TRUE(option.is_some());
    auto options = option.unwrap();
    // Ignore the fields, we just want to ensure it compiles.
}

// WRITE FLOAT OPTIONS TESTS

TEST(write_float_options, builder_tests)
{
    constexpr std::string_view nan_string = "NAN";
    constexpr std::string_view inf_string = "INF";

    auto builder = write_float_options::builder();
    builder
        .exponent_char(uint8_t('e'))
        .trim_floats(true)
        .nan_string(nan_string)
        .inf_string(inf_string);
#ifdef  HAVE_RADIX
    builder.radix(2);
#endif  // HAVE_RADIX

    auto option = builder.build();
    ASSERT_TRUE(option.is_some());
    auto options = option.unwrap();
    EXPECT_EQ(options.exponent_char(), uint8_t('e'));
    EXPECT_EQ(options.trim_floats(), true);
    EXPECT_EQ(options.nan_string(), nan_string);
    EXPECT_EQ(options.inf_string(), inf_string);
}

// RESULT TESTS

TEST(result, result_tests)
{
    auto ok = result_ok<u8>(0);
    auto overflow = result_overflow<u8>(0);
    auto underflow = result_underflow<u8>(0);
    auto invalid_digit = result_invalid_digit<u8>(0);
    auto empty = result_empty<u8>(0);
    auto empty_mantissa = result_empty_mantissa<u8>(0);
    auto empty_exponent = result_empty_exponent<u8>(0);
    auto empty_integer = result_empty_integer<u8>(0);
    auto empty_fraction = result_empty_fraction<u8>(0);
    auto invalid_positive_mantissa_sign = result_invalid_positive_mantissa_sign<u8>(0);
    auto missing_mantissa_sign = result_missing_mantissa_sign<u8>(0);
    auto invalid_exponent = result_invalid_exponent<u8>(0);
    auto invalid_positive_exponent_sign = result_invalid_positive_exponent_sign<u8>(0);
    auto missing_exponent_sign = result_missing_exponent_sign<u8>(0);
    auto exponent_without_fraction = result_exponent_without_fraction<u8>(0);
    auto invalid_leading_zeros = result_invalid_leading_zeros<u8>(0);

    EXPECT_TRUE(ok.is_ok());
    EXPECT_FALSE(ok.is_err());
    EXPECT_TRUE(overflow.is_err());
    EXPECT_TRUE(underflow.is_err());
    EXPECT_TRUE(invalid_digit.is_err());
    EXPECT_TRUE(empty.is_err());
    EXPECT_TRUE(empty_mantissa.is_err());
    EXPECT_TRUE(empty_exponent.is_err());
    EXPECT_TRUE(empty_integer.is_err());
    EXPECT_TRUE(empty_fraction.is_err());
    EXPECT_TRUE(invalid_positive_mantissa_sign.is_err());
    EXPECT_TRUE(missing_mantissa_sign.is_err());
    EXPECT_TRUE(invalid_exponent.is_err());
    EXPECT_TRUE(invalid_positive_exponent_sign.is_err());
    EXPECT_TRUE(missing_exponent_sign.is_err());
    EXPECT_TRUE(exponent_without_fraction.is_err());
    EXPECT_TRUE(invalid_leading_zeros.is_err());

    EXPECT_EQ(ok.ok(), 0);
    EXPECT_TRUE(overflow.err().is_overflow());
    EXPECT_TRUE(underflow.err().is_underflow());
    EXPECT_TRUE(invalid_digit.err().is_invalid_digit());
    EXPECT_TRUE(empty.err().is_empty());
    EXPECT_TRUE(empty_mantissa.err().is_empty_mantissa());
    EXPECT_TRUE(empty_exponent.err().is_empty_exponent());
    EXPECT_TRUE(empty_integer.err().is_empty_integer());
    EXPECT_TRUE(empty_fraction.err().is_empty_fraction());
    EXPECT_TRUE(invalid_positive_mantissa_sign.err().is_invalid_positive_mantissa_sign());
    EXPECT_TRUE(missing_mantissa_sign.err().is_missing_mantissa_sign());
    EXPECT_TRUE(invalid_exponent.err().is_invalid_exponent());
    EXPECT_TRUE(invalid_positive_exponent_sign.err().is_invalid_positive_exponent_sign());
    EXPECT_TRUE(missing_exponent_sign.err().is_missing_exponent_sign());
    EXPECT_TRUE(exponent_without_fraction.err().is_exponent_without_fraction());
    EXPECT_TRUE(invalid_leading_zeros.err().is_invalid_leading_zeros());
}

// PARTIAL RESULT TESTS

TEST(partial_result, partial_result_tests)
{
    auto ok = partial_result_ok<u8>(0, 1);
    auto overflow = partial_result_overflow<u8>(0);
    auto underflow = partial_result_underflow<u8>(0);
    auto invalid_digit = partial_result_invalid_digit<u8>(0);
    auto empty = partial_result_empty<u8>(0);
    auto empty_mantissa = partial_result_empty_mantissa<u8>(0);
    auto empty_exponent = partial_result_empty_exponent<u8>(0);
    auto empty_integer = partial_result_empty_integer<u8>(0);
    auto empty_fraction = partial_result_empty_fraction<u8>(0);
    auto invalid_positive_mantissa_sign = partial_result_invalid_positive_mantissa_sign<u8>(0);
    auto missing_mantissa_sign = partial_result_missing_mantissa_sign<u8>(0);
    auto invalid_exponent = partial_result_invalid_exponent<u8>(0);
    auto invalid_positive_exponent_sign = partial_result_invalid_positive_exponent_sign<u8>(0);
    auto missing_exponent_sign = partial_result_missing_exponent_sign<u8>(0);
    auto exponent_without_fraction = partial_result_exponent_without_fraction<u8>(0);
    auto invalid_leading_zeros = partial_result_invalid_leading_zeros<u8>(0);

    EXPECT_TRUE(ok.is_ok());
    EXPECT_FALSE(ok.is_err());
    EXPECT_TRUE(overflow.is_err());
    EXPECT_TRUE(underflow.is_err());
    EXPECT_TRUE(invalid_digit.is_err());
    EXPECT_TRUE(empty.is_err());
    EXPECT_TRUE(empty_mantissa.is_err());
    EXPECT_TRUE(empty_exponent.is_err());
    EXPECT_TRUE(empty_integer.is_err());
    EXPECT_TRUE(empty_fraction.is_err());
    EXPECT_TRUE(invalid_positive_mantissa_sign.is_err());
    EXPECT_TRUE(missing_mantissa_sign.is_err());
    EXPECT_TRUE(invalid_exponent.is_err());
    EXPECT_TRUE(invalid_positive_exponent_sign.is_err());
    EXPECT_TRUE(missing_exponent_sign.is_err());
    EXPECT_TRUE(exponent_without_fraction.is_err());
    EXPECT_TRUE(invalid_leading_zeros.is_err());

    EXPECT_EQ(ok.ok(), std::make_tuple(0, 1));
    EXPECT_TRUE(overflow.err().is_overflow());
    EXPECT_TRUE(underflow.err().is_underflow());
    EXPECT_TRUE(invalid_digit.err().is_invalid_digit());
    EXPECT_TRUE(empty.err().is_empty());
    EXPECT_TRUE(empty_mantissa.err().is_empty_mantissa());
    EXPECT_TRUE(empty_exponent.err().is_empty_exponent());
    EXPECT_TRUE(empty_integer.err().is_empty_integer());
    EXPECT_TRUE(empty_fraction.err().is_empty_fraction());
    EXPECT_TRUE(invalid_positive_mantissa_sign.err().is_invalid_positive_mantissa_sign());
    EXPECT_TRUE(missing_mantissa_sign.err().is_missing_mantissa_sign());
    EXPECT_TRUE(invalid_exponent.err().is_invalid_exponent());
    EXPECT_TRUE(invalid_positive_exponent_sign.err().is_invalid_positive_exponent_sign());
    EXPECT_TRUE(missing_exponent_sign.err().is_missing_exponent_sign());
    EXPECT_TRUE(exponent_without_fraction.err().is_exponent_without_fraction());
    EXPECT_TRUE(invalid_leading_zeros.err().is_invalid_leading_zeros());
}

// TO STRING TESTS

#define TO_STRING_TEST(t)                                                       \
    EXPECT_EQ("10", to_string<t>(10))

#define TO_STRING_FLOAT_TEST(t)                                                 \
    EXPECT_EQ("10.5", to_string<t>(10.5))

TEST(to_string, api_tests)
{
    TO_STRING_TEST(u8);
    TO_STRING_TEST(u16);
    TO_STRING_TEST(u32);
    TO_STRING_TEST(u64);
    TO_STRING_TEST(usize);
    TO_STRING_TEST(i8);
    TO_STRING_TEST(i16);
    TO_STRING_TEST(i32);
    TO_STRING_TEST(i64);
    TO_STRING_TEST(isize);
    TO_STRING_FLOAT_TEST(f32);
    TO_STRING_FLOAT_TEST(f64);
}

#define TO_STRING_OPTIONS_TEST(t, value, options)                               \
    EXPECT_EQ(value, to_string_with_options<t>(10, options))

#define TO_STRING_OPTIONS_FLOAT_TEST(t, value, options)                         \
    EXPECT_EQ(value, to_string_with_options<t>(10.5, options))

TEST(to_string_with_options, api_tests)
{
    auto integer_options = write_integer_options::builder()
        .build()
        .unwrap();
    TO_STRING_OPTIONS_TEST(u8, "10", integer_options);
    TO_STRING_OPTIONS_TEST(u16, "10", integer_options);
    TO_STRING_OPTIONS_TEST(u32, "10", integer_options);
    TO_STRING_OPTIONS_TEST(u64, "10", integer_options);
    TO_STRING_OPTIONS_TEST(usize, "10", integer_options);
    TO_STRING_OPTIONS_TEST(i8, "10", integer_options);
    TO_STRING_OPTIONS_TEST(i16, "10", integer_options);
    TO_STRING_OPTIONS_TEST(i32, "10", integer_options);
    TO_STRING_OPTIONS_TEST(i64, "10", integer_options);
    TO_STRING_OPTIONS_TEST(isize, "10", integer_options);

    auto float_options = write_float_options::builder()
        .build()
        .unwrap();
    TO_STRING_OPTIONS_FLOAT_TEST(f32, "10.5", float_options);
    TO_STRING_OPTIONS_FLOAT_TEST(f64, "10.5", float_options);

#ifdef HAVE_RADIX
    integer_options = write_integer_options::binary();
    TO_STRING_OPTIONS_TEST(u8, "1010", integer_options);
    TO_STRING_OPTIONS_TEST(u16, "1010", integer_options);
    TO_STRING_OPTIONS_TEST(u32, "1010", integer_options);
    TO_STRING_OPTIONS_TEST(u64, "1010", integer_options);
    TO_STRING_OPTIONS_TEST(usize, "1010", integer_options);
    TO_STRING_OPTIONS_TEST(i8, "1010", integer_options);
    TO_STRING_OPTIONS_TEST(i16, "1010", integer_options);
    TO_STRING_OPTIONS_TEST(i32, "1010", integer_options);
    TO_STRING_OPTIONS_TEST(i64, "1010", integer_options);
    TO_STRING_OPTIONS_TEST(isize, "1010", integer_options);

    integer_options = write_integer_options::decimal();
    TO_STRING_OPTIONS_TEST(u8, "10", integer_options);
    TO_STRING_OPTIONS_TEST(u16, "10", integer_options);
    TO_STRING_OPTIONS_TEST(u32, "10", integer_options);
    TO_STRING_OPTIONS_TEST(u64, "10", integer_options);
    TO_STRING_OPTIONS_TEST(usize, "10", integer_options);
    TO_STRING_OPTIONS_TEST(i8, "10", integer_options);
    TO_STRING_OPTIONS_TEST(i16, "10", integer_options);
    TO_STRING_OPTIONS_TEST(i32, "10", integer_options);
    TO_STRING_OPTIONS_TEST(i64, "10", integer_options);
    TO_STRING_OPTIONS_TEST(isize, "10", integer_options);

    integer_options = write_integer_options::hexadecimal();
    TO_STRING_OPTIONS_TEST(u8, "A", integer_options);
    TO_STRING_OPTIONS_TEST(u16, "A", integer_options);
    TO_STRING_OPTIONS_TEST(u32, "A", integer_options);
    TO_STRING_OPTIONS_TEST(u64, "A", integer_options);
    TO_STRING_OPTIONS_TEST(usize, "A", integer_options);
    TO_STRING_OPTIONS_TEST(i8, "A", integer_options);
    TO_STRING_OPTIONS_TEST(i16, "A", integer_options);
    TO_STRING_OPTIONS_TEST(i32, "A", integer_options);
    TO_STRING_OPTIONS_TEST(i64, "A", integer_options);
    TO_STRING_OPTIONS_TEST(isize, "A", integer_options);

    float_options = write_float_options::binary();
    TO_STRING_OPTIONS_FLOAT_TEST(f32, "1010.1", float_options);
    TO_STRING_OPTIONS_FLOAT_TEST(f64, "1010.1", float_options);

    float_options = write_float_options::decimal();
    TO_STRING_OPTIONS_FLOAT_TEST(f32, "10.5", float_options);
    TO_STRING_OPTIONS_FLOAT_TEST(f64, "10.5", float_options);

    float_options = write_float_options::hexadecimal();
    TO_STRING_OPTIONS_FLOAT_TEST(f32, "A.8", float_options);
    TO_STRING_OPTIONS_FLOAT_TEST(f64, "A.8", float_options);
#endif  // HAVE_RADIX
}

// PARSE TESTS

#define PARSE_TEST(t)                                                           \
    EXPECT_EQ(result_ok<t>(10), parse<t>("10"));                                \
    EXPECT_EQ(result_invalid_digit<t>(2), parse<t>("10a"));                     \
    EXPECT_EQ(result_empty<t>(0), parse<t>(""))

#define PARSE_FLOAT_TEST(t)                                                     \
    PARSE_TEST(t);                                                              \
    EXPECT_EQ(result_ok<t>(10.5), parse<t>("10.5"));                            \
    EXPECT_EQ(result_ok<t>(10e5), parse<t>("10e5"));                            \
    EXPECT_EQ(result_empty_mantissa<t>(0), parse<t>("."));                      \
    EXPECT_EQ(result_empty_mantissa<t>(0), parse<t>("e5"));                     \
    EXPECT_EQ(result_empty_exponent<t>(3), parse<t>("10e+"))

TEST(parse, api_tests)
{
    PARSE_TEST(u8);
    PARSE_TEST(u16);
    PARSE_TEST(u32);
    PARSE_TEST(u64);
    PARSE_TEST(usize);
    PARSE_TEST(i8);
    PARSE_TEST(i16);
    PARSE_TEST(i32);
    PARSE_TEST(i64);
    PARSE_TEST(isize);
    PARSE_FLOAT_TEST(f32);
    PARSE_FLOAT_TEST(f64);
}

#define PARSE_OPTIONS_TEST(t, options, str1, str2)                                  \
    EXPECT_EQ(result_ok<t>(10), parse_with_options<t>(str1, options));              \
    EXPECT_TRUE(parse_with_options<t>(str2, options).is_err());                     \
    EXPECT_EQ(result_empty<t>(0), parse_with_options<t>("", options))

#define PARSE_OPTIONS_FLOAT_TEST(t, options, str1, str2, str3, str4)                \
    PARSE_OPTIONS_TEST(t, options, str1, str2);                                     \
    EXPECT_EQ(result_ok<t>(10.5), parse_with_options<t>(str3, options));            \
    EXPECT_EQ(result_ok<t>(10e5), parse_with_options<t>(str4, options));            \
    EXPECT_EQ(result_empty_mantissa<t>(0), parse_with_options<t>(".", options));

TEST(parse_with_options, api_tests)
{
    auto integer_options = parse_integer_options::builder()
        .build()
        .unwrap();
    PARSE_OPTIONS_TEST(u8, integer_options, "10", "10a");
    PARSE_OPTIONS_TEST(u16, integer_options, "10", "10a");
    PARSE_OPTIONS_TEST(u32, integer_options, "10", "10a");
    PARSE_OPTIONS_TEST(u64, integer_options, "10", "10a");
    PARSE_OPTIONS_TEST(usize, integer_options, "10", "10a");
    PARSE_OPTIONS_TEST(i8, integer_options, "10", "10a");
    PARSE_OPTIONS_TEST(i16, integer_options, "10", "10a");
    PARSE_OPTIONS_TEST(i32, integer_options, "10", "10a");
    PARSE_OPTIONS_TEST(i64, integer_options, "10", "10a");
    PARSE_OPTIONS_TEST(isize, integer_options, "10", "10a");

    auto float_options = parse_float_options::builder()
        .build()
        .unwrap();
    PARSE_OPTIONS_FLOAT_TEST(f32, float_options, "10", "10a", "10.5", "10e5");
    PARSE_OPTIONS_FLOAT_TEST(f64, float_options, "10", "10a", "10.5", "10e5");

#ifdef HAVE_RADIX
    integer_options = parse_integer_options::binary();
    PARSE_OPTIONS_TEST(u8, integer_options, "1010", "10102");
    PARSE_OPTIONS_TEST(u16, integer_options, "1010", "10102");
    PARSE_OPTIONS_TEST(u32, integer_options, "1010", "10102");
    PARSE_OPTIONS_TEST(u64, integer_options, "1010", "10102");
    PARSE_OPTIONS_TEST(usize, integer_options, "1010", "10102");
    PARSE_OPTIONS_TEST(i8, integer_options, "1010", "10102");
    PARSE_OPTIONS_TEST(i16, integer_options, "1010", "10102");
    PARSE_OPTIONS_TEST(i32, integer_options, "1010", "10102");
    PARSE_OPTIONS_TEST(i64, integer_options, "1010", "10102");
    PARSE_OPTIONS_TEST(isize, integer_options, "1010", "10102");

    integer_options = parse_integer_options::decimal();
    PARSE_OPTIONS_TEST(u8, integer_options, "10", "10a");
    PARSE_OPTIONS_TEST(u16, integer_options, "10", "10a");
    PARSE_OPTIONS_TEST(u32, integer_options, "10", "10a");
    PARSE_OPTIONS_TEST(u64, integer_options, "10", "10a");
    PARSE_OPTIONS_TEST(usize, integer_options, "10", "10a");
    PARSE_OPTIONS_TEST(i8, integer_options, "10", "10a");
    PARSE_OPTIONS_TEST(i16, integer_options, "10", "10a");
    PARSE_OPTIONS_TEST(i32, integer_options, "10", "10a");
    PARSE_OPTIONS_TEST(i64, integer_options, "10", "10a");
    PARSE_OPTIONS_TEST(isize, integer_options, "10", "10a");

    integer_options = parse_integer_options::hexadecimal();
    PARSE_OPTIONS_TEST(u8, integer_options, "A", "AG");
    PARSE_OPTIONS_TEST(u16, integer_options, "A", "AG");
    PARSE_OPTIONS_TEST(u32, integer_options, "A", "AG");
    PARSE_OPTIONS_TEST(u64, integer_options, "A", "AG");
    PARSE_OPTIONS_TEST(usize, integer_options, "A", "AG");
    PARSE_OPTIONS_TEST(i8, integer_options, "A", "AG");
    PARSE_OPTIONS_TEST(i16, integer_options, "A", "AG");
    PARSE_OPTIONS_TEST(i32, integer_options, "A", "AG");
    PARSE_OPTIONS_TEST(i64, integer_options, "A", "AG");
    PARSE_OPTIONS_TEST(isize, integer_options, "A", "AG");

    float_options = parse_float_options::binary();
    PARSE_OPTIONS_FLOAT_TEST(f32, float_options, "1010", "10102", "1010.1", "11110100001001000000");
    PARSE_OPTIONS_FLOAT_TEST(f64, float_options, "1010", "10102", "1010.1", "11110100001001000000");

    float_options = parse_float_options::decimal();
    PARSE_OPTIONS_FLOAT_TEST(f32, float_options, "10", "10a", "10.5", "10e5");
    PARSE_OPTIONS_FLOAT_TEST(f64, float_options, "10", "10a", "10.5", "10e5");

    float_options = parse_float_options::hexadecimal();
    PARSE_OPTIONS_FLOAT_TEST(f32, float_options, "A", "AG", "A.8", "f4240");
    PARSE_OPTIONS_FLOAT_TEST(f64, float_options, "A", "AG", "A.8", "f4240");
#endif  // HAVE_RADIX

#ifdef HAVE_FORMAT
    integer_options = parse_integer_options::builder()
        .format(number_format::fsharp_string)
        .build()
        .unwrap();
    PARSE_OPTIONS_TEST(u8, integer_options, "1_0", "1_0a");

    float_options = parse_float_options::builder()
        .format(number_format::fsharp_string)
        .build()
        .unwrap();
    PARSE_OPTIONS_FLOAT_TEST(f32, float_options, "1_0", "1_0a", "1_0.5_0", "1_0e5");
#endif  // HAVE_FORMAT
}

#define PARSE_PARTIAL_TEST(t)                                                   \
    EXPECT_EQ(partial_result_ok<t>(10, 2), parse_partial<t>("10"));             \
    EXPECT_EQ(partial_result_ok<t>(10, 2), parse_partial<t>("10a"));            \
    EXPECT_EQ(partial_result_empty<t>(0), parse_partial<t>(""))

#define PARSE_PARTIAL_FLOAT_TEST(t)                                             \
    PARSE_PARTIAL_TEST(t);                                                      \
    EXPECT_EQ(partial_result_ok<t>(10.5, 4), parse_partial<t>("10.5"));         \
    EXPECT_EQ(partial_result_ok<t>(10e5, 4), parse_partial<t>("10e5"));         \
    EXPECT_EQ(partial_result_empty_mantissa<t>(0), parse_partial<t>("."));      \
    EXPECT_EQ(partial_result_empty_mantissa<t>(0), parse_partial<t>("e5"));     \
    EXPECT_EQ(partial_result_empty_exponent<t>(3), parse_partial<t>("10e+"))

TEST(parse_partial, api_tests)
{
    PARSE_PARTIAL_TEST(u8);
    PARSE_PARTIAL_TEST(u16);
    PARSE_PARTIAL_TEST(u32);
    PARSE_PARTIAL_TEST(u64);
    PARSE_PARTIAL_TEST(usize);
    PARSE_PARTIAL_TEST(i8);
    PARSE_PARTIAL_TEST(i16);
    PARSE_PARTIAL_TEST(i32);
    PARSE_PARTIAL_TEST(i64);
    PARSE_PARTIAL_TEST(isize);
    PARSE_PARTIAL_FLOAT_TEST(f32);
    PARSE_PARTIAL_FLOAT_TEST(f64);
}

#define PARSE_PARTIAL_OPTIONS_TEST(t, options, str1, str2)                                              \
    EXPECT_EQ(partial_result_ok<t>(10, strlen(str1)), parse_partial_with_options<t>(str1, options));    \
    EXPECT_EQ(partial_result_ok<t>(10, strlen(str2)-1), parse_partial_with_options<t>(str2, options));  \
    EXPECT_EQ(partial_result_empty<t>(0), parse_partial_with_options<t>("", options))

#define PARSE_PARTIAL_OPTIONS_FLOAT_TEST(t, options, str1, str2, str3, str4)                            \
    PARSE_PARTIAL_OPTIONS_TEST(t, options, str1, str2);                                                 \
    EXPECT_EQ(partial_result_ok<t>(10.5, strlen(str3)), parse_partial_with_options<t>(str3, options));  \
    EXPECT_EQ(partial_result_ok<t>(10e5, strlen(str4)), parse_partial_with_options<t>(str4, options));  \
    EXPECT_EQ(partial_result_empty_mantissa<t>(0), parse_partial_with_options<t>(".", options));

TEST(parse_partial_with_options, api_tests)
{
    auto integer_options = parse_integer_options::builder()
        .build()
        .unwrap();
    PARSE_PARTIAL_OPTIONS_TEST(u8, integer_options, "10", "10a");
    PARSE_PARTIAL_OPTIONS_TEST(u16, integer_options, "10", "10a");
    PARSE_PARTIAL_OPTIONS_TEST(u32, integer_options, "10", "10a");
    PARSE_PARTIAL_OPTIONS_TEST(u64, integer_options, "10", "10a");
    PARSE_PARTIAL_OPTIONS_TEST(usize, integer_options, "10", "10a");
    PARSE_PARTIAL_OPTIONS_TEST(i8, integer_options, "10", "10a");
    PARSE_PARTIAL_OPTIONS_TEST(i16, integer_options, "10", "10a");
    PARSE_PARTIAL_OPTIONS_TEST(i32, integer_options, "10", "10a");
    PARSE_PARTIAL_OPTIONS_TEST(i64, integer_options, "10", "10a");
    PARSE_PARTIAL_OPTIONS_TEST(isize, integer_options, "10", "10a");

    auto float_options = parse_float_options::builder()
        .build()
        .unwrap();
    PARSE_PARTIAL_OPTIONS_FLOAT_TEST(f32, float_options, "10", "10a", "10.5", "10e5");
    PARSE_PARTIAL_OPTIONS_FLOAT_TEST(f64, float_options, "10", "10a", "10.5", "10e5");

#ifdef HAVE_RADIX
    integer_options = parse_integer_options::binary();
    PARSE_PARTIAL_OPTIONS_TEST(u8, integer_options, "1010", "10102");
    PARSE_PARTIAL_OPTIONS_TEST(u16, integer_options, "1010", "10102");
    PARSE_PARTIAL_OPTIONS_TEST(u32, integer_options, "1010", "10102");
    PARSE_PARTIAL_OPTIONS_TEST(u64, integer_options, "1010", "10102");
    PARSE_PARTIAL_OPTIONS_TEST(usize, integer_options, "1010", "10102");
    PARSE_PARTIAL_OPTIONS_TEST(i8, integer_options, "1010", "10102");
    PARSE_PARTIAL_OPTIONS_TEST(i16, integer_options, "1010", "10102");
    PARSE_PARTIAL_OPTIONS_TEST(i32, integer_options, "1010", "10102");
    PARSE_PARTIAL_OPTIONS_TEST(i64, integer_options, "1010", "10102");
    PARSE_PARTIAL_OPTIONS_TEST(isize, integer_options, "1010", "10102");

    integer_options = parse_integer_options::decimal();
    PARSE_PARTIAL_OPTIONS_TEST(u8, integer_options, "10", "10a");
    PARSE_PARTIAL_OPTIONS_TEST(u16, integer_options, "10", "10a");
    PARSE_PARTIAL_OPTIONS_TEST(u32, integer_options, "10", "10a");
    PARSE_PARTIAL_OPTIONS_TEST(u64, integer_options, "10", "10a");
    PARSE_PARTIAL_OPTIONS_TEST(usize, integer_options, "10", "10a");
    PARSE_PARTIAL_OPTIONS_TEST(i8, integer_options, "10", "10a");
    PARSE_PARTIAL_OPTIONS_TEST(i16, integer_options, "10", "10a");
    PARSE_PARTIAL_OPTIONS_TEST(i32, integer_options, "10", "10a");
    PARSE_PARTIAL_OPTIONS_TEST(i64, integer_options, "10", "10a");
    PARSE_PARTIAL_OPTIONS_TEST(isize, integer_options, "10", "10a");

    integer_options = parse_integer_options::hexadecimal();
    PARSE_PARTIAL_OPTIONS_TEST(u8, integer_options, "A", "AG");
    PARSE_PARTIAL_OPTIONS_TEST(u16, integer_options, "A", "AG");
    PARSE_PARTIAL_OPTIONS_TEST(u32, integer_options, "A", "AG");
    PARSE_PARTIAL_OPTIONS_TEST(u64, integer_options, "A", "AG");
    PARSE_PARTIAL_OPTIONS_TEST(usize, integer_options, "A", "AG");
    PARSE_PARTIAL_OPTIONS_TEST(i8, integer_options, "A", "AG");
    PARSE_PARTIAL_OPTIONS_TEST(i16, integer_options, "A", "AG");
    PARSE_PARTIAL_OPTIONS_TEST(i32, integer_options, "A", "AG");
    PARSE_PARTIAL_OPTIONS_TEST(i64, integer_options, "A", "AG");
    PARSE_PARTIAL_OPTIONS_TEST(isize, integer_options, "A", "AG");

    float_options = parse_float_options::binary();
    PARSE_PARTIAL_OPTIONS_FLOAT_TEST(f32, float_options, "1010", "10102", "1010.1", "11110100001001000000");
    PARSE_PARTIAL_OPTIONS_FLOAT_TEST(f64, float_options, "1010", "10102", "1010.1", "11110100001001000000");

    float_options = parse_float_options::decimal();
    PARSE_PARTIAL_OPTIONS_FLOAT_TEST(f32, float_options, "10", "10a", "10.5", "10e5");
    PARSE_PARTIAL_OPTIONS_FLOAT_TEST(f64, float_options, "10", "10a", "10.5", "10e5");

    float_options = parse_float_options::hexadecimal();
    PARSE_PARTIAL_OPTIONS_FLOAT_TEST(f32, float_options, "A", "AG", "A.8", "f4240");
    PARSE_PARTIAL_OPTIONS_FLOAT_TEST(f64, float_options, "A", "AG", "A.8", "f4240");
#endif  // HAVE_RADIX

#ifdef HAVE_FORMAT
    integer_options = parse_integer_options::builder()
        .format(number_format::fsharp_string)
        .build()
        .unwrap();
    PARSE_PARTIAL_OPTIONS_TEST(u8, integer_options, "1_0", "1_0a");

    float_options = parse_float_options::builder()
        .format(number_format::fsharp_string)
        .build()
        .unwrap();
    PARSE_PARTIAL_OPTIONS_FLOAT_TEST(f32, float_options, "1_0", "1_0a", "1_0.5_0", "1_0e5");
#endif  // HAVE_FORMAT
}
