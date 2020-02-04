/**
 *  lexical_core
 *  ============
 *
 *  Access lexical-core functionality from C++.
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

#ifndef LEXICAL_HPP_
#define LEXICAL_HPP_

#include "lexical.h"
#include <cassert>
#include <cstdint>
#include <cstring>
#include <iterator>
#include <stdexcept>
#include <string>
#include <string_view>
#include <tuple>
#include <type_traits>

namespace lexical {

// Features must be enabled through the following macro definitions:
//  1. HAVE_FORMAT
//  2. HAVE_RADIX
//  3. HAVE_ROUNDING

// OPTIONS
// -------

// OPTION TAG

// Tag for the option type in the tagged enum.
enum class option_tag: uint32_t {
    some = ::lexical_some,
    none = ::lexical_none,
};

// Option type for parser functions.
template <typename T>
struct option {
    option_tag tag;
    T data;

    // Safely convert from a C-style result to a C++ one.
    // This is to prevent layout differences from causing UB.
    template <typename OptionT>
    static inline option from(OptionT c_opt)
    {
        // Ensure we likely have a similar layout.
        // We're technically invoking UB, since the layout isn't
        // guaranteed to be the same, but it would take a
        // very malicious compiler to do so.
        // Pretty much any approach would option in UB, even the platform-
        // specific bindings, since the structs aren't **guaranteed**
        // to be the same as what we're using.
        static_assert(sizeof(OptionT) == sizeof(option), "Invalid sizes");
        static_assert(std::is_standard_layout<OptionT>::value, "Not std layout");
        static_assert(std::is_standard_layout<option>::value, "Not std layout");

        option cc_opt;
        std::memcpy(&cc_opt, &c_opt, sizeof(option));
        return cc_opt;
    }

    inline bool is_some()
    {
        return tag == option_tag::some;
    }

    inline bool is_none()
    {
        return tag == option_tag::none;
    }

    inline T unwrap()
    {
        assert(is_some());
        return std::move(data);
    }

    inline friend bool operator==(const option& lhs, const option& rhs)
    {
        if (lhs.tag != rhs.tag) {
            return false;
        } else if (lhs.tag == option_tag::some) {
            return lhs.data.value == rhs.data.value;
        } else {
            return true;
        }
    }

    inline friend bool operator!=(const option& lhs, const option& rhs)
    {
        return !(lhs == rhs);
    }
};

// FEATURES
// --------

// Rounding type for float-parsing.
enum class rounding_kind: int32_t {
#ifdef HAVE_ROUNDING
    nearest_tie_even = lexical_nearest_tie_even,
    nearest_tie_away_zero = lexical_nearest_tie_away_zero,
    toward_positive_infinity = lexical_toward_positive_infinity,
    toward_negative_infinity = lexical_toward_negative_infinity,
    toward_zero = lexical_toward_zero,
#endif  // HAVE_ROUNDING
};

// Declare a pseudo-enum variant.
#define lexical_declare_enum(name, type) static const type name;

// Define a pseudo-enum variant.
#define lexical_define_enum(name, type, init) inline const type type::name = type(init);

// Define a relational operator for a type off of tie.
#define lexical_relational_operator(type, op)                                   \
    inline friend bool operator op(const type& lhs, const type& rhs)            \
    {                                                                           \
        return lhs.tie() op rhs.tie();                                          \
    }

// Define equality relational operators for a type.
#define lexical_equality(type)                                                  \
    lexical_relational_operator(type, ==)                                       \
    lexical_relational_operator(type, !=)


// Define total ordering relational operators for a type.
#define lexical_total_ordering(type)                                            \
    lexical_equality(type)                                                      \
    lexical_relational_operator(type, <)                                        \
    lexical_relational_operator(type, <=)                                       \
    lexical_relational_operator(type, >)                                        \
    lexical_relational_operator(type, >=)

// Explicit conversion operator
#define lexical_explicit_conversion(type, to, field)                            \
    explicit operator to()                                                      \
    {                                                                           \
        return to(field);                                                       \
    }

// Define a bitwise operator for a type off of a field.
#define lexical_bitwise_operator(type, op, field)                               \
    inline friend type operator op(const type& lhs, const type& rhs)            \
    {                                                                           \
        return type(lhs.field op rhs.field);                                    \
    }

// Define all bitwise operators for a type.
#define lexical_bitwise(type, field)                                            \
    lexical_bitwise_operator(type, &, field)                                    \
    lexical_bitwise_operator(type, |, field)                                    \
    lexical_bitwise_operator(type, ^, field)

// Utility to get the correct macro when overloading on argument counts.
#define lexical_get_macro(_1, _2, _3, _4, name, ...) name

// Add field to builder without cast.
#define lexical_builder_field_no_cast(cls, type, field)                         \
    inline cls& field(type field)                                               \
    {                                                                           \
        _builder.field = field;                                                 \
        return *this;                                                           \
    }

// Add field to builder with cast.
#define lexical_builder_field_cast(cls, type, field, cast)                      \
    inline cls& field(type field)                                               \
    {                                                                           \
        _builder.field = cast(field);                                           \
        return *this;                                                           \
    }

// Add field to builder.
#define lexical_builder_field(...) lexical_get_macro(__VA_ARGS__, lexical_builder_field_cast, lexical_builder_field_no_cast)(__VA_ARGS__)

// Add string field to builder without cast.
#define lexical_builder_string(cls, field)                                      \
    inline cls& field(std::string_view str)                                     \
    {                                                                           \
        _builder.field##_ptr = reinterpret_cast<uint8_t const*>(str.data());    \
        _builder.field##_length = str.size();                                   \
        return *this;                                                           \
    }

class number_format;

// Number format builder.
class number_format_builder {
public:
    number_format_builder(const number_format_builder&) = default;
    number_format_builder& operator=(const number_format_builder&) = default;

#ifdef HAVE_FORMAT
    lexical_builder_field(number_format_builder, uint8_t, digit_separator);
    lexical_builder_field(number_format_builder, bool, required_integer_digits);
    lexical_builder_field(number_format_builder, bool, required_fraction_digits);
    lexical_builder_field(number_format_builder, bool, required_exponent_digits);
    lexical_builder_field(number_format_builder, bool, no_positive_mantissa_sign);
    lexical_builder_field(number_format_builder, bool, required_mantissa_sign);
    lexical_builder_field(number_format_builder, bool, no_exponent_notation);
    lexical_builder_field(number_format_builder, bool, no_positive_exponent_sign);
    lexical_builder_field(number_format_builder, bool, required_exponent_sign);
    lexical_builder_field(number_format_builder, bool, no_exponent_without_fraction);
    lexical_builder_field(number_format_builder, bool, no_special);
    lexical_builder_field(number_format_builder, bool, case_sensitive_special);
    lexical_builder_field(number_format_builder, bool, no_integer_leading_zeros);
    lexical_builder_field(number_format_builder, bool, no_float_leading_zeros);
    lexical_builder_field(number_format_builder, bool, integer_internal_digit_separator);
    lexical_builder_field(number_format_builder, bool, fraction_internal_digit_separator);
    lexical_builder_field(number_format_builder, bool, exponent_internal_digit_separator);
    lexical_builder_field(number_format_builder, bool, integer_leading_digit_separator);
    lexical_builder_field(number_format_builder, bool, fraction_leading_digit_separator);
    lexical_builder_field(number_format_builder, bool, exponent_leading_digit_separator);
    lexical_builder_field(number_format_builder, bool, integer_trailing_digit_separator);
    lexical_builder_field(number_format_builder, bool, fraction_trailing_digit_separator);
    lexical_builder_field(number_format_builder, bool, exponent_trailing_digit_separator);
    lexical_builder_field(number_format_builder, bool, integer_consecutive_digit_separator);
    lexical_builder_field(number_format_builder, bool, fraction_consecutive_digit_separator);
    lexical_builder_field(number_format_builder, bool, exponent_consecutive_digit_separator);
    lexical_builder_field(number_format_builder, bool, special_digit_separator);
#endif  // HAVE_FORMAT

    // Build options.
    option<number_format> build() const;

private:
    friend class number_format;
    ::lexical_number_format_builder_t _builder;

    number_format_builder(::lexical_number_format_builder_t builder):
        _builder(builder)
    {}
};

// Bitflags for a serialized number format.
class number_format {
public:
    number_format() = default;
    number_format(const number_format&) = default;
    number_format& operator=(const number_format&) = default;

    number_format(::lexical_number_format value):
        _value(value)
    {}

    lexical_total_ordering(number_format);
    lexical_bitwise(number_format, _value);
    lexical_explicit_conversion(number_format, bool, _value);
    lexical_explicit_conversion(number_format, uint64_t, _value);

    // Create number format builder.
    static inline number_format_builder builder()
    {
        return number_format_builder(::lexical_number_format_builder());
    }

private:
    ::lexical_number_format _value;

    constexpr std::tuple<::lexical_number_format> tie() const
    {
        return std::tie(_value);
    }

public:
#ifdef HAVE_FORMAT
    // FLAGS
    lexical_declare_enum(required_integer_digits, number_format);
    lexical_declare_enum(required_fraction_digits, number_format);
    lexical_declare_enum(required_exponent_digits, number_format);
    lexical_declare_enum(no_positive_mantissa_sign, number_format);
    lexical_declare_enum(required_mantissa_sign, number_format);
    lexical_declare_enum(no_exponent_notation, number_format);
    lexical_declare_enum(no_positive_exponent_sign, number_format);
    lexical_declare_enum(required_exponent_sign, number_format);
    lexical_declare_enum(no_exponent_without_fraction, number_format);
    lexical_declare_enum(no_special, number_format);
    lexical_declare_enum(case_sensitive_special, number_format);
    lexical_declare_enum(no_integer_leading_zeros, number_format);
    lexical_declare_enum(no_float_leading_zeros, number_format);
    lexical_declare_enum(integer_internal_digit_separator, number_format);
    lexical_declare_enum(fraction_internal_digit_separator, number_format);
    lexical_declare_enum(exponent_internal_digit_separator, number_format);
    lexical_declare_enum(integer_leading_digit_separator, number_format);
    lexical_declare_enum(fraction_leading_digit_separator, number_format);
    lexical_declare_enum(exponent_leading_digit_separator, number_format);
    lexical_declare_enum(integer_trailing_digit_separator, number_format);
    lexical_declare_enum(fraction_trailing_digit_separator, number_format);
    lexical_declare_enum(exponent_trailing_digit_separator, number_format);
    lexical_declare_enum(integer_consecutive_digit_separator, number_format);
    lexical_declare_enum(fraction_consecutive_digit_separator, number_format);
    lexical_declare_enum(exponent_consecutive_digit_separator, number_format);
    lexical_declare_enum(special_digit_separator, number_format);
    lexical_declare_enum(required_digits, number_format);
    lexical_declare_enum(internal_digit_separator, number_format);
    lexical_declare_enum(leading_digit_separator, number_format);
    lexical_declare_enum(trailing_digit_separator, number_format);
    lexical_declare_enum(consecutive_digit_separator, number_format);

    // MASKS
    lexical_declare_enum(digit_separator_flag_mask, number_format);
    lexical_declare_enum(integer_digit_separator_flag_mask, number_format);
    lexical_declare_enum(fraction_digit_separator_flag_mask, number_format);
    lexical_declare_enum(exponent_digit_separator_flag_mask, number_format);
    lexical_declare_enum(exponent_flag_mask, number_format);
    lexical_declare_enum(flag_mask, number_format);

    // PRE-DEFINED
    // Note:
    //  The pre-defined enum definitions are the public API for
    //  lexical_number_format.
    lexical_declare_enum(rust_literal, number_format);
    lexical_declare_enum(rust_string, number_format);
    lexical_declare_enum(rust_string_strict, number_format);
    lexical_declare_enum(python_literal, number_format);
    lexical_declare_enum(python_string, number_format);
    lexical_declare_enum(cxx17_literal, number_format);
    lexical_declare_enum(cxx17_string, number_format);
    lexical_declare_enum(cxx14_literal, number_format);
    lexical_declare_enum(cxx14_string, number_format);
    lexical_declare_enum(cxx11_literal, number_format);
    lexical_declare_enum(cxx11_string, number_format);
    lexical_declare_enum(cxx03_literal, number_format);
    lexical_declare_enum(cxx03_string, number_format);
    lexical_declare_enum(cxx98_literal, number_format);
    lexical_declare_enum(cxx98_string, number_format);
    lexical_declare_enum(c18_literal, number_format);
    lexical_declare_enum(c18_string, number_format);
    lexical_declare_enum(c11_literal, number_format);
    lexical_declare_enum(c11_string, number_format);
    lexical_declare_enum(c99_literal, number_format);
    lexical_declare_enum(c99_string, number_format);
    lexical_declare_enum(c90_literal, number_format);
    lexical_declare_enum(c90_string, number_format);
    lexical_declare_enum(c89_literal, number_format);
    lexical_declare_enum(c89_string, number_format);
    lexical_declare_enum(ruby_literal, number_format);
    lexical_declare_enum(ruby_string, number_format);
    lexical_declare_enum(swift_literal, number_format);
    lexical_declare_enum(swift_string, number_format);
    lexical_declare_enum(go_literal, number_format);
    lexical_declare_enum(go_string, number_format);
    lexical_declare_enum(haskell_literal, number_format);
    lexical_declare_enum(haskell_string, number_format);
    lexical_declare_enum(javascript_literal, number_format);
    lexical_declare_enum(javascript_string, number_format);
    lexical_declare_enum(perl_literal, number_format);
    lexical_declare_enum(perl_string, number_format);
    lexical_declare_enum(php_literal, number_format);
    lexical_declare_enum(php_string, number_format);
    lexical_declare_enum(java_literal, number_format);
    lexical_declare_enum(java_string, number_format);
    lexical_declare_enum(r_literal, number_format);
    lexical_declare_enum(r_string, number_format);
    lexical_declare_enum(kotlin_literal, number_format);
    lexical_declare_enum(kotlin_string, number_format);
    lexical_declare_enum(julia_literal, number_format);
    lexical_declare_enum(julia_string, number_format);
    lexical_declare_enum(csharp7_literal, number_format);
    lexical_declare_enum(csharp7_string, number_format);
    lexical_declare_enum(csharp6_literal, number_format);
    lexical_declare_enum(csharp6_string, number_format);
    lexical_declare_enum(csharp5_literal, number_format);
    lexical_declare_enum(csharp5_string, number_format);
    lexical_declare_enum(csharp4_literal, number_format);
    lexical_declare_enum(csharp4_string, number_format);
    lexical_declare_enum(csharp3_literal, number_format);
    lexical_declare_enum(csharp3_string, number_format);
    lexical_declare_enum(csharp2_literal, number_format);
    lexical_declare_enum(csharp2_string, number_format);
    lexical_declare_enum(csharp1_literal, number_format);
    lexical_declare_enum(csharp1_string, number_format);
    lexical_declare_enum(kawa_literal, number_format);
    lexical_declare_enum(kawa_string, number_format);
    lexical_declare_enum(gambitc_literal, number_format);
    lexical_declare_enum(gambitc_string, number_format);
    lexical_declare_enum(guile_literal, number_format);
    lexical_declare_enum(guile_string, number_format);
    lexical_declare_enum(clojure_literal, number_format);
    lexical_declare_enum(clojure_string, number_format);
    lexical_declare_enum(erlang_literal, number_format);
    lexical_declare_enum(erlang_string, number_format);
    lexical_declare_enum(elm_literal, number_format);
    lexical_declare_enum(elm_string, number_format);
    lexical_declare_enum(scala_literal, number_format);
    lexical_declare_enum(scala_string, number_format);
    lexical_declare_enum(elixir_literal, number_format);
    lexical_declare_enum(elixir_string, number_format);
    lexical_declare_enum(fortran_literal, number_format);
    lexical_declare_enum(fortran_string, number_format);
    lexical_declare_enum(d_literal, number_format);
    lexical_declare_enum(d_string, number_format);
    lexical_declare_enum(coffeescript_literal, number_format);
    lexical_declare_enum(coffeescript_string, number_format);
    lexical_declare_enum(cobol_literal, number_format);
    lexical_declare_enum(cobol_string, number_format);
    lexical_declare_enum(fsharp_literal, number_format);
    lexical_declare_enum(fsharp_string, number_format);
    lexical_declare_enum(vb_literal, number_format);
    lexical_declare_enum(vb_string, number_format);
    lexical_declare_enum(ocaml_literal, number_format);
    lexical_declare_enum(ocaml_string, number_format);
    lexical_declare_enum(objectivec_literal, number_format);
    lexical_declare_enum(objectivec_string, number_format);
    lexical_declare_enum(reasonml_literal, number_format);
    lexical_declare_enum(reasonml_string, number_format);
    lexical_declare_enum(octave_literal, number_format);
    lexical_declare_enum(octave_string, number_format);
    lexical_declare_enum(matlab_literal, number_format);
    lexical_declare_enum(matlab_string, number_format);
    lexical_declare_enum(zig_literal, number_format);
    lexical_declare_enum(zig_string, number_format);
    lexical_declare_enum(sage_literal, number_format);
    lexical_declare_enum(sage_string, number_format);
    lexical_declare_enum(json, number_format);
    lexical_declare_enum(toml, number_format);
    lexical_declare_enum(yaml, number_format);
    lexical_declare_enum(xml, number_format);
    lexical_declare_enum(sqlite, number_format);
    lexical_declare_enum(postgresql, number_format);
    lexical_declare_enum(mysql, number_format);
    lexical_declare_enum(mongodb, number_format);

    // Compile permissive number format.
    //
    // The permissive number format does not require any control
    // grammar, besides the presence of mantissa digits.
    static inline option<number_format> permissive()
    {
        return option<number_format>::from(::lexical_number_format_permissive());
    }

    // Compile standard number format.
    //
    // The standard number format is guaranteed to be identical
    // to the format expected by Rust's string to number parsers.
    static inline option<number_format> standard()
    {
        return option<number_format>::from(::lexical_number_format_standard());
    }

    // Compile ignore number format.
    //
    // The ignore number format ignores all digit separators,
    // and is permissive for all other control grammar, so
    // implements a fast parser.
    //
    // * `digit_separator`                         - Character to separate digits.
    static inline option<number_format> ignore(uint8_t digit_separator)
    {
        return option<number_format>::from(::lexical_number_format_ignore(digit_separator));
    }

    // Get the digit separator from the compiled number format.
    inline uint8_t digit_separator() const
    {
        return uint8_t(_value >> 56);
    }

    // Get the flag bits from the compiled number format.
    inline number_format flags() const
    {
        return number_format(::lexical_number_format_flags(_value));
    }

    // Get if digits are required before the decimal point.
    inline bool has_required_integer_digits() const
    {
        return ::lexical_number_format_required_integer_digits(_value);
    }

    // Get if digits are required after the decimal point.
    inline bool has_required_fraction_digits() const
    {
        return ::lexical_number_format_required_fraction_digits(_value);
    }

    // Get if digits are required after the exponent character.
    inline bool has_required_exponent_digits() const
    {
        return ::lexical_number_format_required_exponent_digits(_value);
    }

    // Get if digits are required before or after the decimal point.
    inline bool has_required_digits() const
    {
        return ::lexical_number_format_required_digits(_value);
    }

    // Get if positive sign before the mantissa is not allowed.
    inline bool has_no_positive_mantissa_sign() const
    {
        return ::lexical_number_format_no_positive_mantissa_sign(_value);
    }

    // Get if positive sign before the mantissa is required.
    inline bool has_required_mantissa_sign() const
    {
        return ::lexical_number_format_required_mantissa_sign(_value);
    }

    // Get if exponent notation is not allowed.
    inline bool has_no_exponent_notation() const
    {
        return ::lexical_number_format_no_exponent_notation(_value);
    }

    // Get if positive sign before the exponent is not allowed.
    inline bool has_no_positive_exponent_sign() const
    {
        return ::lexical_number_format_no_positive_exponent_sign(_value);
    }

    // Get if sign before the exponent is required.
    inline bool has_required_exponent_sign() const
    {
        return ::lexical_number_format_required_exponent_sign(_value);
    }

    // Get if exponent without fraction is not allowed.
    inline bool has_no_exponent_without_fraction() const
    {
        return ::lexical_number_format_no_exponent_without_fraction(_value);
    }

    // Get if special (non-finite) values are not allowed.
    inline bool has_no_special() const
    {
        return ::lexical_number_format_no_special(_value);
    }

    // Get if special (non-finite) values are case-sensitive.
    inline bool has_case_sensitive_special() const
    {
        return ::lexical_number_format_case_sensitive_special(_value);
    }

    // Get if leading zeros before an integer are not allowed.
    inline bool has_no_integer_leading_zeros() const
    {
        return ::lexical_number_format_no_integer_leading_zeros(_value);
    }

    // Get if leading zeros before a float are not allowed.
    inline bool has_no_float_leading_zeros() const
    {
        return ::lexical_number_format_no_float_leading_zeros(_value);
    }

    // Get if digit separators are allowed between integer digits.
    inline bool has_integer_internal_digit_separator() const
    {
        return ::lexical_number_format_integer_internal_digit_separator(_value);
    }

    // Get if digit separators are allowed between fraction digits.
    inline bool has_fraction_internal_digit_separator() const
    {
        return ::lexical_number_format_fraction_internal_digit_separator(_value);
    }

    // Get if digit separators are allowed between exponent digits.
    inline bool has_exponent_internal_digit_separator() const
    {
        return ::lexical_number_format_exponent_internal_digit_separator(_value);
    }

    // Get if digit separators are allowed between digits.
    inline bool has_internal_digit_separator() const
    {
        return ::lexical_number_format_internal_digit_separator(_value);
    }

    // Get if a digit separator is allowed before any integer digits.
    inline bool has_integer_leading_digit_separator() const
    {
        return ::lexical_number_format_integer_leading_digit_separator(_value);
    }

    // Get if a digit separator is allowed before any fraction digits.
    inline bool has_fraction_leading_digit_separator() const
    {
        return ::lexical_number_format_fraction_leading_digit_separator(_value);
    }

    // Get if a digit separator is allowed before any exponent digits.
    inline bool has_exponent_leading_digit_separator() const
    {
        return ::lexical_number_format_exponent_leading_digit_separator(_value);
    }

    // Get if a digit separator is allowed before any digits.
    inline bool has_leading_digit_separator() const
    {
        return ::lexical_number_format_leading_digit_separator(_value);
    }

    // Get if a digit separator is allowed after any integer digits.
    inline bool has_integer_trailing_digit_separator() const
    {
        return ::lexical_number_format_integer_trailing_digit_separator(_value);
    }

    // Get if a digit separator is allowed after any fraction digits.
    inline bool has_fraction_trailing_digit_separator() const
    {
        return ::lexical_number_format_fraction_trailing_digit_separator(_value);
    }

    // Get if a digit separator is allowed after any exponent digits.
    inline bool has_exponent_trailing_digit_separator() const
    {
        return ::lexical_number_format_exponent_trailing_digit_separator(_value);
    }

    // Get if a digit separator is allowed after any digits.
    inline bool has_trailing_digit_separator() const
    {
        return ::lexical_number_format_trailing_digit_separator(_value);
    }

    // Get if multiple consecutive integer digit separators are allowed.
    inline bool has_integer_consecutive_digit_separator() const
    {
        return ::lexical_number_format_integer_consecutive_digit_separator(_value);
    }

    // Get if multiple consecutive fraction digit separators are allowed.
    inline bool has_fraction_consecutive_digit_separator() const
    {
        return ::lexical_number_format_fraction_consecutive_digit_separator(_value);
    }

    // Get if multiple consecutive exponent digit separators are allowed.
    inline bool has_exponent_consecutive_digit_separator() const
    {
        return ::lexical_number_format_exponent_consecutive_digit_separator(_value);
    }

    // Get if multiple consecutive digit separators are allowed.
    inline bool has_consecutive_digit_separator() const
    {
        return ::lexical_number_format_consecutive_digit_separator(_value);
    }

    // Get if any digit separators are allowed in special (non-finite) values.
    inline bool has_special_digit_separator() const
    {
        return ::lexical_number_format_special_digit_separator(_value);
    }
#endif  // HAVE_FORMAT
};

#ifdef HAVE_FORMAT
    // FLAGS
    lexical_define_enum(required_integer_digits, number_format, lexical_required_integer_digits);
    lexical_define_enum(required_exponent_digits, number_format, lexical_required_exponent_digits);
    lexical_define_enum(no_positive_mantissa_sign, number_format, lexical_no_positive_mantissa_sign);
    lexical_define_enum(required_mantissa_sign, number_format, lexical_required_mantissa_sign);
    lexical_define_enum(no_exponent_notation, number_format, lexical_no_exponent_notation);
    lexical_define_enum(no_positive_exponent_sign, number_format, lexical_no_positive_exponent_sign);
    lexical_define_enum(required_exponent_sign, number_format, lexical_required_exponent_sign);
    lexical_define_enum(no_exponent_without_fraction, number_format, lexical_no_exponent_without_fraction);
    lexical_define_enum(no_special, number_format, lexical_no_special);
    lexical_define_enum(case_sensitive_special, number_format, lexical_case_sensitive_special);
    lexical_define_enum(no_integer_leading_zeros, number_format, lexical_no_integer_leading_zeros);
    lexical_define_enum(no_float_leading_zeros, number_format, lexical_no_float_leading_zeros);
    lexical_define_enum(integer_internal_digit_separator, number_format, lexical_integer_internal_digit_separator);
    lexical_define_enum(fraction_internal_digit_separator, number_format, lexical_fraction_internal_digit_separator);
    lexical_define_enum(exponent_internal_digit_separator, number_format, lexical_exponent_internal_digit_separator);
    lexical_define_enum(integer_leading_digit_separator, number_format, lexical_integer_leading_digit_separator);
    lexical_define_enum(fraction_leading_digit_separator, number_format, lexical_fraction_leading_digit_separator);
    lexical_define_enum(exponent_leading_digit_separator, number_format, lexical_exponent_leading_digit_separator);
    lexical_define_enum(integer_trailing_digit_separator, number_format, lexical_integer_trailing_digit_separator);
    lexical_define_enum(fraction_trailing_digit_separator, number_format, lexical_fraction_trailing_digit_separator);
    lexical_define_enum(exponent_trailing_digit_separator, number_format, lexical_exponent_trailing_digit_separator);
    lexical_define_enum(integer_consecutive_digit_separator, number_format, lexical_integer_consecutive_digit_separator);
    lexical_define_enum(fraction_consecutive_digit_separator, number_format, lexical_fraction_consecutive_digit_separator);
    lexical_define_enum(exponent_consecutive_digit_separator, number_format, lexical_exponent_consecutive_digit_separator);
    lexical_define_enum(special_digit_separator, number_format, lexical_special_digit_separator);
    lexical_define_enum(required_digits, number_format, lexical_required_digits);
    lexical_define_enum(internal_digit_separator, number_format, lexical_internal_digit_separator);
    lexical_define_enum(leading_digit_separator, number_format, lexical_leading_digit_separator);
    lexical_define_enum(trailing_digit_separator, number_format, lexical_trailing_digit_separator);
    lexical_define_enum(consecutive_digit_separator, number_format, lexical_consecutive_digit_separator);

    // MASKS
    lexical_define_enum(digit_separator_flag_mask, number_format, lexical_digit_separator_flag_mask);
    lexical_define_enum(integer_digit_separator_flag_mask, number_format, lexical_integer_digit_separator_flag_mask);
    lexical_define_enum(fraction_digit_separator_flag_mask, number_format, lexical_fraction_digit_separator_flag_mask);
    lexical_define_enum(exponent_digit_separator_flag_mask, number_format, lexical_exponent_digit_separator_flag_mask);
    lexical_define_enum(exponent_flag_mask, number_format, lexical_exponent_flag_mask);
    lexical_define_enum(flag_mask, number_format, lexical_flag_mask);

    // PRE-DEFINED
    // Note:
    //  The pre-defined enum definitions are the public API for
    //  lexical_number_format.
    lexical_define_enum(rust_literal, number_format, lexical_rust_literal);
    lexical_define_enum(rust_string, number_format, lexical_rust_string);
    lexical_define_enum(rust_string_strict, number_format, lexical_rust_string_strict);
    lexical_define_enum(python_literal, number_format, lexical_python_literal);
    lexical_define_enum(python_string, number_format, lexical_python_string);
    lexical_define_enum(cxx17_literal, number_format, lexical_cxx17_literal);
    lexical_define_enum(cxx17_string, number_format, lexical_cxx17_string);
    lexical_define_enum(cxx14_literal, number_format, lexical_cxx14_literal);
    lexical_define_enum(cxx14_string, number_format, lexical_cxx14_string);
    lexical_define_enum(cxx11_literal, number_format, lexical_cxx11_literal);
    lexical_define_enum(cxx11_string, number_format, lexical_cxx11_string);
    lexical_define_enum(cxx03_literal, number_format, lexical_cxx03_literal);
    lexical_define_enum(cxx03_string, number_format, lexical_cxx03_string);
    lexical_define_enum(cxx98_literal, number_format, lexical_cxx98_literal);
    lexical_define_enum(cxx98_string, number_format, lexical_cxx98_string);
    lexical_define_enum(c18_literal, number_format, lexical_c18_literal);
    lexical_define_enum(c18_string, number_format, lexical_c18_string);
    lexical_define_enum(c11_literal, number_format, lexical_c11_literal);
    lexical_define_enum(c11_string, number_format, lexical_c11_string);
    lexical_define_enum(c99_literal, number_format, lexical_c99_literal);
    lexical_define_enum(c99_string, number_format, lexical_c99_string);
    lexical_define_enum(c90_literal, number_format, lexical_c90_literal);
    lexical_define_enum(c90_string, number_format, lexical_c90_string);
    lexical_define_enum(c89_literal, number_format, lexical_c89_literal);
    lexical_define_enum(c89_string, number_format, lexical_c89_string);
    lexical_define_enum(ruby_literal, number_format, lexical_ruby_literal);
    lexical_define_enum(ruby_string, number_format, lexical_ruby_string);
    lexical_define_enum(swift_literal, number_format, lexical_swift_literal);
    lexical_define_enum(swift_string, number_format, lexical_swift_string);
    lexical_define_enum(go_literal, number_format, lexical_go_literal);
    lexical_define_enum(go_string, number_format, lexical_go_string);
    lexical_define_enum(haskell_literal, number_format, lexical_haskell_literal);
    lexical_define_enum(haskell_string, number_format, lexical_haskell_string);
    lexical_define_enum(javascript_literal, number_format, lexical_javascript_literal);
    lexical_define_enum(javascript_string, number_format, lexical_javascript_string);
    lexical_define_enum(perl_literal, number_format, lexical_perl_literal);
    lexical_define_enum(perl_string, number_format, lexical_perl_string);
    lexical_define_enum(php_literal, number_format, lexical_php_literal);
    lexical_define_enum(php_string, number_format, lexical_php_string);
    lexical_define_enum(java_literal, number_format, lexical_java_literal);
    lexical_define_enum(java_string, number_format, lexical_java_string);
    lexical_define_enum(r_literal, number_format, lexical_r_literal);
    lexical_define_enum(r_string, number_format, lexical_r_string);
    lexical_define_enum(kotlin_literal, number_format, lexical_kotlin_literal);
    lexical_define_enum(kotlin_string, number_format, lexical_kotlin_string);
    lexical_define_enum(julia_literal, number_format, lexical_julia_literal);
    lexical_define_enum(julia_string, number_format, lexical_julia_string);
    lexical_define_enum(csharp7_literal, number_format, lexical_csharp7_literal);
    lexical_define_enum(csharp7_string, number_format, lexical_csharp7_string);
    lexical_define_enum(csharp6_literal, number_format, lexical_csharp6_literal);
    lexical_define_enum(csharp6_string, number_format, lexical_csharp6_string);
    lexical_define_enum(csharp5_literal, number_format, lexical_csharp5_literal);
    lexical_define_enum(csharp5_string, number_format, lexical_csharp5_string);
    lexical_define_enum(csharp4_literal, number_format, lexical_csharp4_literal);
    lexical_define_enum(csharp4_string, number_format, lexical_csharp4_string);
    lexical_define_enum(csharp3_literal, number_format, lexical_csharp3_literal);
    lexical_define_enum(csharp3_string, number_format, lexical_csharp3_string);
    lexical_define_enum(csharp2_literal, number_format, lexical_csharp2_literal);
    lexical_define_enum(csharp2_string, number_format, lexical_csharp2_string);
    lexical_define_enum(csharp1_literal, number_format, lexical_csharp1_literal);
    lexical_define_enum(csharp1_string, number_format, lexical_csharp1_string);
    lexical_define_enum(kawa_literal, number_format, lexical_kawa_literal);
    lexical_define_enum(kawa_string, number_format, lexical_kawa_string);
    lexical_define_enum(gambitc_literal, number_format, lexical_gambitc_literal);
    lexical_define_enum(gambitc_string, number_format, lexical_gambitc_string);
    lexical_define_enum(guile_literal, number_format, lexical_guile_literal);
    lexical_define_enum(guile_string, number_format, lexical_guile_string);
    lexical_define_enum(clojure_literal, number_format, lexical_clojure_literal);
    lexical_define_enum(clojure_string, number_format, lexical_clojure_string);
    lexical_define_enum(erlang_literal, number_format, lexical_erlang_literal);
    lexical_define_enum(erlang_string, number_format, lexical_erlang_string);
    lexical_define_enum(elm_literal, number_format, lexical_elm_literal);
    lexical_define_enum(elm_string, number_format, lexical_elm_string);
    lexical_define_enum(scala_literal, number_format, lexical_scala_literal);
    lexical_define_enum(scala_string, number_format, lexical_scala_string);
    lexical_define_enum(elixir_literal, number_format, lexical_elixir_literal);
    lexical_define_enum(elixir_string, number_format, lexical_elixir_string);
    lexical_define_enum(fortran_literal, number_format, lexical_fortran_literal);
    lexical_define_enum(fortran_string, number_format, lexical_fortran_string);
    lexical_define_enum(d_literal, number_format, lexical_d_literal);
    lexical_define_enum(d_string, number_format, lexical_d_string);
    lexical_define_enum(coffeescript_literal, number_format, lexical_coffeescript_literal);
    lexical_define_enum(coffeescript_string, number_format, lexical_coffeescript_string);
    lexical_define_enum(cobol_literal, number_format, lexical_cobol_literal);
    lexical_define_enum(cobol_string, number_format, lexical_cobol_string);
    lexical_define_enum(fsharp_literal, number_format, lexical_fsharp_literal);
    lexical_define_enum(fsharp_string, number_format, lexical_fsharp_string);
    lexical_define_enum(vb_literal, number_format, lexical_vb_literal);
    lexical_define_enum(vb_string, number_format, lexical_vb_string);
    lexical_define_enum(ocaml_literal, number_format, lexical_ocaml_literal);
    lexical_define_enum(ocaml_string, number_format, lexical_ocaml_string);
    lexical_define_enum(objectivec_literal, number_format, lexical_objectivec_literal);
    lexical_define_enum(objectivec_string, number_format, lexical_objectivec_string);
    lexical_define_enum(reasonml_literal, number_format, lexical_reasonml_literal);
    lexical_define_enum(reasonml_string, number_format, lexical_reasonml_string);
    lexical_define_enum(octave_literal, number_format, lexical_octave_literal);
    lexical_define_enum(octave_string, number_format, lexical_octave_string);
    lexical_define_enum(matlab_literal, number_format, lexical_matlab_literal);
    lexical_define_enum(matlab_string, number_format, lexical_matlab_string);
    lexical_define_enum(zig_literal, number_format, lexical_zig_literal);
    lexical_define_enum(zig_string, number_format, lexical_zig_string);
    lexical_define_enum(sage_literal, number_format, lexical_sage_literal);
    lexical_define_enum(sage_string, number_format, lexical_sage_string);
    lexical_define_enum(json, number_format, lexical_json);
    lexical_define_enum(toml, number_format, lexical_toml);
    lexical_define_enum(yaml, number_format, lexical_yaml);
    lexical_define_enum(xml, number_format, lexical_xml);
    lexical_define_enum(sqlite, number_format, lexical_sqlite);
    lexical_define_enum(postgresql, number_format, lexical_postgresql);
    lexical_define_enum(mysql, number_format, lexical_mysql);
    lexical_define_enum(mongodb, number_format, lexical_mongodb);
#endif  // HAVE_FORMAT

// Define build here to avoid forward-declaration issues.
inline option<number_format> number_format_builder::build() const
{
    return option<number_format>::from(::lexical_number_format_build(_builder));
}

// CONSTANTS
// ---------

static const size_t I8_FORMATTED_SIZE = ::LEXICAL_I8_FORMATTED_SIZE;
static const size_t I16_FORMATTED_SIZE = ::LEXICAL_I16_FORMATTED_SIZE;
static const size_t I32_FORMATTED_SIZE = ::LEXICAL_I32_FORMATTED_SIZE;
static const size_t I64_FORMATTED_SIZE = ::LEXICAL_I64_FORMATTED_SIZE;
static const size_t ISIZE_FORMATTED_SIZE = ::LEXICAL_ISIZE_FORMATTED_SIZE;
static const size_t U8_FORMATTED_SIZE = ::LEXICAL_U8_FORMATTED_SIZE;
static const size_t U16_FORMATTED_SIZE = ::LEXICAL_U16_FORMATTED_SIZE;
static const size_t U32_FORMATTED_SIZE = ::LEXICAL_U32_FORMATTED_SIZE;
static const size_t U64_FORMATTED_SIZE = ::LEXICAL_U64_FORMATTED_SIZE;
static const size_t USIZE_FORMATTED_SIZE = ::LEXICAL_USIZE_FORMATTED_SIZE;
static const size_t F32_FORMATTED_SIZE = ::LEXICAL_F32_FORMATTED_SIZE;
static const size_t F64_FORMATTED_SIZE = ::LEXICAL_F64_FORMATTED_SIZE;

static const size_t I8_FORMATTED_SIZE_DECIMAL = ::LEXICAL_I8_FORMATTED_SIZE_DECIMAL;
static const size_t I16_FORMATTED_SIZE_DECIMAL = ::LEXICAL_I16_FORMATTED_SIZE_DECIMAL;
static const size_t I32_FORMATTED_SIZE_DECIMAL = ::LEXICAL_I32_FORMATTED_SIZE_DECIMAL;
static const size_t I64_FORMATTED_SIZE_DECIMAL = ::LEXICAL_I64_FORMATTED_SIZE_DECIMAL;
static const size_t ISIZE_FORMATTED_SIZE_DECIMAL = ::LEXICAL_ISIZE_FORMATTED_SIZE_DECIMAL;
static const size_t U8_FORMATTED_SIZE_DECIMAL = ::LEXICAL_U8_FORMATTED_SIZE_DECIMAL;
static const size_t U16_FORMATTED_SIZE_DECIMAL = ::LEXICAL_U16_FORMATTED_SIZE_DECIMAL;
static const size_t U32_FORMATTED_SIZE_DECIMAL = ::LEXICAL_U32_FORMATTED_SIZE_DECIMAL;
static const size_t U64_FORMATTED_SIZE_DECIMAL = ::LEXICAL_U64_FORMATTED_SIZE_DECIMAL;
static const size_t USIZE_FORMATTED_SIZE_DECIMAL = ::LEXICAL_USIZE_FORMATTED_SIZE_DECIMAL;
static const size_t F32_FORMATTED_SIZE_DECIMAL = ::LEXICAL_F32_FORMATTED_SIZE_DECIMAL;
static const size_t F64_FORMATTED_SIZE_DECIMAL = ::LEXICAL_F64_FORMATTED_SIZE_DECIMAL;

static const size_t BUFFER_SIZE = ::LEXICAL_BUFFER_SIZE;

// Buffer size used internally for the to_string implementations.
// Avoid malloc whenever possible.
static constexpr size_t WRITE_SIZE = 256;

// TYPES
// -----

// ALIAS
using u8 = ::lexical_u8;
using u16 = ::lexical_u16;
using u32 = ::lexical_u32;
using u64 = ::lexical_u64;
using usize = ::lexical_usize;

using i8 = ::lexical_i8;
using i16 = ::lexical_i16;
using i32 = ::lexical_i32;
using i64 = ::lexical_i64;
using isize = ::lexical_isize;

using f32 = ::lexical_f32;
using f64 = ::lexical_f64;

// Internal typedef for the parsers.
using string_type = std::string_view;

// ERROR

// Error code, indicating failure type.
enum class error_code: int32_t {
    overflow = ::lexical_overflow,
    underflow = ::lexical_underflow,
    invalid_digit = ::lexical_invalid_digit,
    empty = ::lexical_empty,
    empty_mantissa = ::lexical_empty_mantissa,
    empty_exponent = ::lexical_empty_exponent,
    empty_integer = ::lexical_empty_integer,
    empty_fraction = ::lexical_empty_fraction,
    invalid_positive_mantissa_sign = ::lexical_invalid_positive_mantissa_sign,
    missing_mantissa_sign = ::lexical_missing_mantissa_sign,
    invalid_exponent = ::lexical_invalid_exponent,
    invalid_positive_exponent_sign = ::lexical_invalid_positive_exponent_sign,
    missing_exponent_sign = ::lexical_missing_exponent_sign,
    exponent_without_fraction = ::lexical_exponent_without_fraction,
    invalid_leading_zeros = ::lexical_invalid_leading_zeros,
};

// Determine if an error code matches the desired code.
#define lexical_is_error(type)                                                  \
    inline bool is_##type()                                                     \
    {                                                                           \
        return code == error_code::type;                                        \
    }

// C-compatible error type.
struct error
{
    error_code code;
    size_t index;

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

    inline friend bool operator==(const error& lhs, const error& rhs)
    {
        return std::make_tuple(lhs.code, lhs.index) == std::make_tuple(rhs.code, rhs.index);
    }

    inline friend bool operator!=(const error& lhs, const error& rhs)
    {
        return !(lhs == rhs);
    }
};

// PARSE INTEGER OPTIONS

class parse_integer_options;

// Parse integer options builder.
class parse_integer_options_builder {
public:
    parse_integer_options_builder() = delete;
    parse_integer_options_builder(const parse_integer_options_builder&) = default;
    parse_integer_options_builder& operator=(const parse_integer_options_builder&) = default;

#ifdef HAVE_RADIX
    lexical_builder_field(parse_integer_options_builder, uint8_t, radix);
#endif  // HAVE_RADIX
#ifdef HAVE_FORMAT
    lexical_builder_field(parse_integer_options_builder, number_format, format, uint64_t);
#endif  // HAVE_FORMAT

    // Build options.
    option<parse_integer_options> build() const;

private:
    friend class parse_integer_options;
    ::lexical_parse_integer_options_builder_t _builder;

    parse_integer_options_builder(::lexical_parse_integer_options_builder_t builder):
        _builder(builder)
    {}
};

// Options to customize parsing integers.
class parse_integer_options {
public:
    // CONSTRUCTORS

    parse_integer_options(const parse_integer_options&) = default;
    parse_integer_options& operator=(const parse_integer_options&) = default;

    // Create parse integer options builder.
    static inline parse_integer_options_builder builder()
    {
        return parse_integer_options_builder(::lexical_parse_integer_options_builder());
    }

#ifdef HAVE_RADIX
    static inline parse_integer_options binary()
    {
        return parse_integer_options::builder()
            .radix(2)
            .build()
            .unwrap();
    }

    static inline parse_integer_options decimal()
    {
        return parse_integer_options::builder()
            .build()
            .unwrap();
    }

    static inline parse_integer_options hexadecimal()
    {
        return parse_integer_options::builder()
            .radix(16)
            .build()
            .unwrap();
    }
#endif  // HAVE_RADIX

    // PROPERTIES

    // Get the radix property.
    uint32_t radix() const
    {
        return _options.radix;
    }

    // Get the format property.
    number_format format() const
    {
        return number_format(_options.format);
    }

    // Get underlying options as pointer for the C-API.
    ::lexical_parse_integer_options const* as_ptr() const
    {
        return &_options;
    }

private:
    friend class parse_integer_options_builder;
    friend class option<parse_integer_options>;
    ::lexical_parse_integer_options _options;

    parse_integer_options() = default;
    parse_integer_options(::lexical_parse_integer_options options):
        _options(options)
    {}
};

// Define build here to avoid forward-declaration issues.
inline option<parse_integer_options> parse_integer_options_builder::build() const
{
    return option<parse_integer_options>::from(::lexical_parse_integer_options_build(_builder));
}

// PARSE FLOAT OPTIONS

class parse_float_options;

// Parse float options builder.
class parse_float_options_builder {
public:
    parse_float_options_builder() = delete;
    parse_float_options_builder(const parse_float_options_builder&) = default;
    parse_float_options_builder& operator=(const parse_float_options_builder&) = default;

    lexical_builder_field(parse_float_options_builder, bool, lossy);
    lexical_builder_field(parse_float_options_builder, uint8_t, exponent_char);
    lexical_builder_string(parse_float_options_builder, nan_string);
    lexical_builder_string(parse_float_options_builder, inf_string);
    lexical_builder_string(parse_float_options_builder, infinity_string);
#ifdef HAVE_RADIX
    lexical_builder_field(parse_float_options_builder, uint8_t, radix);
#endif  // HAVE_RADIX
#ifdef HAVE_FORMAT
    lexical_builder_field(parse_float_options_builder, number_format, format, uint64_t);
#endif  // HAVE_FORMAT
#ifdef HAVE_ROUNDING
    lexical_builder_field(parse_float_options_builder, rounding_kind, rounding, int32_t);
#endif  // HAVE_ROUNDING

    // Build options.
    option<parse_float_options> build() const;

private:
    friend class parse_float_options;
    ::lexical_parse_float_options_builder_t _builder;

    parse_float_options_builder(::lexical_parse_float_options_builder_t builder):
        _builder(builder)
    {}
};

// Options to customize parsing floats.
class parse_float_options {
public:
    // CONSTRUCTORS

    parse_float_options(const parse_float_options&) = default;
    parse_float_options& operator=(const parse_float_options&) = default;

    // Create parse float options builder.
    static inline parse_float_options_builder builder()
    {
        return parse_float_options_builder(::lexical_parse_float_options_builder());
    }

#ifdef HAVE_RADIX
    static inline parse_float_options binary()
    {
        return parse_float_options::builder()
            .radix(2)
            .build()
            .unwrap();
    }

    static inline parse_float_options decimal()
    {
        return parse_float_options::builder()
            .build()
            .unwrap();
    }

    static inline parse_float_options hexadecimal()
    {
        return parse_float_options::builder()
            .radix(16)
            .exponent_char(uint8_t('p'))
            .build()
            .unwrap();
    }
#endif  // HAVE_RADIX

    // PROPERTIES

    // Get the lossy property.
    bool lossy() const
    {
        return _options.lossy;
    }

    // Get the exponent character property.
    uint8_t exponent_char() const
    {
        return _options.exponent_char;
    }

    // Get the radix property.
    uint32_t radix() const
    {
        return _options.radix;
    }

    // Get the format property.
    number_format format() const
    {
        return number_format(_options.format);
    }

    // Get the rounding property.
    rounding_kind rounding() const
    {
        return rounding_kind(_options.rounding);
    }

    // Get the nan string property.
    std::string_view nan_string() const
    {
        auto ptr = reinterpret_cast<char const*>(_options.nan_string_ptr);
        auto length = _options.nan_string_length;
        return std::string_view(ptr, length);
    }

    // Get the inf string property.
    std::string_view inf_string() const
    {
        auto ptr = reinterpret_cast<char const*>(_options.inf_string_ptr);
        auto length = _options.inf_string_length;
        return std::string_view(ptr, length);
    }

    // Get the infinity string property.
    std::string_view infinity_string() const
    {
        auto ptr = reinterpret_cast<char const*>(_options.infinity_string_ptr);
        auto length = _options.infinity_string_length;
        return std::string_view(ptr, length);
    }

    // Get underlying options as pointer for the C-API.
    ::lexical_parse_float_options const* as_ptr() const
    {
        return &_options;
    }

private:
    friend class parse_float_options_builder;
    friend class option<parse_float_options>;
    ::lexical_parse_float_options _options;

    parse_float_options() = default;
    parse_float_options(::lexical_parse_float_options options):
        _options(options)
    {}
};

// Define build here to avoid forward-declaration issues.
inline option<parse_float_options> parse_float_options_builder::build() const
{
    return option<parse_float_options>::from(::lexical_parse_float_options_build(_builder));
}

// WRITE INTEGER OPTIONS

class write_integer_options;

// Write integer options builder.
class write_integer_options_builder {
public:
    write_integer_options_builder() = delete;
    write_integer_options_builder(const write_integer_options_builder&) = default;
    write_integer_options_builder& operator=(const write_integer_options_builder&) = default;

#ifdef HAVE_RADIX
    lexical_builder_field(write_integer_options_builder, uint8_t, radix);
#endif  // HAVE_RADIX

    // Build options.
    option<write_integer_options> build() const;

private:
    friend class write_integer_options;
    ::lexical_write_integer_options_builder_t _builder;

    write_integer_options_builder(::lexical_write_integer_options_builder_t builder):
        _builder(builder)
    {}
};

// Options to customize writing integers.
class write_integer_options {
public:
    // CONSTRUCTORS

    write_integer_options(const write_integer_options&) = default;
    write_integer_options& operator=(const write_integer_options&) = default;

    // Create write integer options builder.
    static inline write_integer_options_builder builder()
    {
        return write_integer_options_builder(::lexical_write_integer_options_builder());
    }

#ifdef HAVE_RADIX
    static inline write_integer_options binary()
    {
        return write_integer_options::builder()
            .radix(2)
            .build()
            .unwrap();
    }

    static inline write_integer_options decimal()
    {
        return write_integer_options::builder()
            .build()
            .unwrap();
    }

    static inline write_integer_options hexadecimal()
    {
        return write_integer_options::builder()
            .radix(16)
            .build()
            .unwrap();
    }
#endif  // HAVE_RADIX

    // PROPERTIES

    // Get the radix property.
    uint32_t radix() const
    {
        return _options.radix;
    }

    // Get underlying options as pointer for the C-API.
    ::lexical_write_integer_options const* as_ptr() const
    {
        return &_options;
    }

private:
    friend class write_integer_options_builder;
    friend class option<write_integer_options>;
    ::lexical_write_integer_options _options;

    write_integer_options() = default;
    write_integer_options(::lexical_write_integer_options options):
        _options(options)
    {}
};

// Define build here to avoid forward-declaration issues.
inline option<write_integer_options> write_integer_options_builder::build() const
{
    return option<write_integer_options>::from(::lexical_write_integer_options_build(_builder));
}

// WRITE FLOAT OPTIONS

class write_float_options;

// Write float options builder.
class write_float_options_builder {
public:
    write_float_options_builder() = delete;
    write_float_options_builder(const write_float_options_builder&) = default;
    write_float_options_builder& operator=(const write_float_options_builder&) = default;

    lexical_builder_field(write_float_options_builder, uint8_t, exponent_char);
    lexical_builder_field(write_float_options_builder, bool, trim_floats);
    lexical_builder_string(write_float_options_builder, nan_string);
    lexical_builder_string(write_float_options_builder, inf_string);
#ifdef HAVE_RADIX
    lexical_builder_field(write_float_options_builder, uint8_t, radix);
#endif  // HAVE_RADIX

    // Build options.
    option<write_float_options> build() const;

private:
    friend class write_float_options;
    ::lexical_write_float_options_builder_t _builder;

    write_float_options_builder(::lexical_write_float_options_builder_t builder):
        _builder(builder)
    {}
};

// Options to customize writing floats.
class write_float_options {
public:
    // CONSTRUCTORS

    write_float_options(const write_float_options&) = default;
    write_float_options& operator=(const write_float_options&) = default;

    // Create write float options builder.
    static inline write_float_options_builder builder()
    {
        return write_float_options_builder(::lexical_write_float_options_builder());
    }

#ifdef HAVE_RADIX
    static inline write_float_options binary()
    {
        return write_float_options::builder()
            .radix(2)
            .build()
            .unwrap();
    }

    static inline write_float_options decimal()
    {
        return write_float_options::builder()
            .build()
            .unwrap();
    }

    static inline write_float_options hexadecimal()
    {
        return write_float_options::builder()
            .radix(16)
            .exponent_char(uint8_t('p'))
            .build()
            .unwrap();
    }
#endif  // HAVE_RADIX

    // PROPERTIES

    // Get the exponent character property.
    uint8_t exponent_char() const
    {
        return _options.exponent_char;
    }

    // Get the radix property.
    uint32_t radix() const
    {
        return _options.radix;
    }

    // Get the trim floats property.
    bool trim_floats() const
    {
        return _options.trim_floats;
    }

    // Get the nan string property.
    std::string_view nan_string() const
    {
        auto ptr = reinterpret_cast<char const*>(_options.nan_string_ptr);
        auto length = _options.nan_string_length;
        return std::string_view(ptr, length);
    }

    // Get the inf string property.
    std::string_view inf_string() const
    {
        auto ptr = reinterpret_cast<char const*>(_options.inf_string_ptr);
        auto length = _options.inf_string_length;
        return std::string_view(ptr, length);
    }

    // Get underlying options as pointer for the C-API.
    ::lexical_write_float_options const* as_ptr() const
    {
        return &_options;
    }

private:
    friend class write_float_options_builder;
    friend class option<write_float_options>;
    ::lexical_write_float_options _options;

    write_float_options() = default;
    write_float_options(::lexical_write_float_options options):
        _options(options)
    {}
};

// Define build here to avoid forward-declaration issues.
inline option<write_float_options> write_float_options_builder::build() const
{
    return option<write_float_options>::from(::lexical_write_float_options_build(_builder));
}

// RESULT TAG

// Tag for the result type in the tagged enum.
enum class result_tag: uint32_t {
    ok = ::lexical_ok,
    err = ::lexical_err,
};

// COMPLETE UNIONS

// Union for the lexical result type.
template <typename T>
union result_union {
    T value;
    struct error error;
};

// COMPLETE RESULTS

// Result type for parser functions.
template <typename T>
struct result {
    result_tag tag;
    result_union<T> data;

    // Safely convert from a C-style result to a C++ one.
    // This is to prevent layout differences from causing UB.
    template <typename ResultT>
    static inline result from(ResultT c_res)
    {
        // Ensure we likely have a similar layout.
        // We're technically invoking UB, since the layout isn't
        // guaranteed to be the same, but it would take a
        // very malicious compiler to do so.
        // Pretty much any approach would result in UB, even the platform-
        // specific bindings, since the structs aren't **guaranteed**
        // to be the same as what we're using.
        static_assert(sizeof(ResultT) == sizeof(result), "Invalid sizes");
        static_assert(std::is_standard_layout<ResultT>::value, "Not std layout");
        static_assert(std::is_standard_layout<result>::value, "Not std layout");

        result cc_res;
        std::memcpy(&cc_res, &c_res, sizeof(result));
        return cc_res;
    }

    inline bool is_ok()
    {
        return tag == result_tag::ok;
    }

    inline bool is_err()
    {
        return tag == result_tag::err;
    }

    inline T ok()
    {
        assert(is_ok());
        return std::move(data.value);
    }

    inline error err()
    {
        assert(is_err());
        return std::move(data.error);
    }

    inline friend bool operator==(const result& lhs, const result& rhs)
    {
        if (lhs.tag != rhs.tag) {
            return false;
        } else if (lhs.tag == result_tag::ok) {
            return lhs.data.value == rhs.data.value;
        } else {
            return lhs.data.error == rhs.data.error;
        }
    }

    inline friend bool operator!=(const result& lhs, const result& rhs)
    {
        return !(lhs == rhs);
    }
};

// PARTIAL RESULT TUPLES

// Result value type for the partial parsers.
template <typename T>
struct partial_result_tuple {
    T x;
    size_t y;

    inline friend bool operator==(const partial_result_tuple& lhs, const partial_result_tuple& rhs)
    {
        return std::make_tuple(lhs.x, lhs.y) == std::make_tuple(rhs.x, rhs.y);
    }

    inline friend bool operator!=(const partial_result_tuple& lhs, const partial_result_tuple& rhs)
    {
        return !(lhs == rhs);
    }
};

// PARTIAL RESULT UNIONS

// Union for the lexical partial result type.
template <typename T>
union partial_result_union {
    partial_result_tuple<T> value;
    struct error error;
};

// PARTIAL RESULTS

// Result type for partial parser functions.
template <typename T>
struct partial_result {
    result_tag tag;
    partial_result_union<T> data;

    // Safely convert from a C-style result to a C++ one.
    // This is to prevent layout differences from causing UB.
    template <typename ResultT>
    static inline partial_result from(ResultT c_res)
    {
        // Ensure we likely have a similar layout.
        // We're technically invoking UB, since the layout isn't
        // guaranteed to be the same, but it would take a
        // very malicious compiler to do so.
        // Pretty much any approach would result in UB, even the platform-
        // specific bindings, since the structs aren't **guaranteed**
        // to be the same as what we're using.
        static_assert(sizeof(ResultT) == sizeof(partial_result), "Invalid sizes");
        static_assert(std::is_standard_layout<ResultT>::value, "Not std layout");
        static_assert(std::is_standard_layout<partial_result>::value, "Not std layout");

        partial_result cc_res;
        std::memcpy(&cc_res, &c_res, sizeof(partial_result));
        return cc_res;
    }

    inline bool is_ok()
    {
        return tag == result_tag::ok;
    }

    inline bool is_err()
    {
        return tag == result_tag::err;
    }

    inline std::tuple<T, size_t> ok()
    {
        assert(is_ok());
        auto value = std::move(data.value);
        return std::make_tuple(std::move(value.x), std::move(value.y));
    }

    inline error err()
    {
        assert(is_err());
        return std::move(data.error);
    }

    inline friend bool operator==(const partial_result& lhs, const partial_result& rhs)
    {
        if (lhs.tag != rhs.tag) {
            return false;
        } else if (lhs.tag == result_tag::ok) {
            return lhs.data.value == rhs.data.value;
        } else {
            return lhs.data.error == rhs.data.error;
        }
    }

    inline friend bool operator!=(const partial_result& lhs, const partial_result& rhs)
    {
        return !(lhs == rhs);
    }
};

// API
// ---

// DISPATCHER

// Dispatch function for to_lexical.
#define lexical_to_lexical(type)                                                \
    inline static                                                               \
    uint8_t*                                                                    \
    to_lexical(                                                                 \
        type value,                                                             \
        uint8_t* first,                                                         \
        uint8_t* last                                                           \
    )                                                                           \
    {                                                                           \
        return ::lexical_##type##toa(value, first, last);                       \
    }

// Dispatch function for to_lexical_with_options.
#define lexical_to_lexical_with_options(type, options_t)                        \
    inline static                                                               \
    uint8_t*                                                                    \
    to_lexical_with_options(                                                    \
        type value,                                                             \
        options_t const& options,                                               \
        uint8_t* first,                                                         \
        uint8_t* last                                                           \
    )                                                                           \
    {                                                                           \
        return ::lexical_##type##toa_with_options(                              \
            value, options.as_ptr(), first, last                                \
        );                                                                      \
    }

// Dispatch function for from_lexical.
#define lexical_from_lexical(type)                                              \
    inline static                                                               \
    result<type>                                                                \
    from_lexical(                                                               \
        uint8_t const* first,                                                   \
        uint8_t const* last                                                     \
    )                                                                           \
    {                                                                           \
        using result_type = result<type>;                                       \
        auto r = ::lexical_ato##type(first, last);                              \
        return result_type::from(r);                                            \
    }

// Dispatch function for from_lexical_partial.
#define lexical_from_lexical_partial(type)                                      \
    inline static                                                               \
    partial_result<type>                                                        \
    from_lexical_partial(                                                       \
        uint8_t const* first,                                                   \
        uint8_t const* last                                                     \
    )                                                                           \
    {                                                                           \
        using partial_result_type = partial_result<type>;                       \
        auto r = ::lexical_ato##type##_partial(first, last);                    \
        return partial_result_type::from(r);                                    \
    }

// Dispatch function for from_lexical_with_options.
#define lexical_from_lexical_with_options(type, options_t)                      \
    inline static                                                               \
    result<type>                                                                \
    from_lexical_with_options(                                                  \
        uint8_t const* first,                                                   \
        uint8_t const* last,                                                    \
        options_t const& options                                                \
    )                                                                           \
    {                                                                           \
        using result_type = result<type>;                                       \
        auto r = ::lexical_ato##type##_with_options(                            \
            first, last, options.as_ptr()                                       \
        );                                                                      \
        return result_type::from(r);                                            \
    }

// Dispatch function for from_lexical_partial_with_options.
#define lexical_from_lexical_partial_with_options(type, options_t)              \
    inline static                                                               \
    partial_result<type>                                                        \
    from_lexical_partial_with_options(                                          \
        uint8_t const* first,                                                   \
        uint8_t const* last,                                                    \
        options_t const& options                                                \
    )                                                                           \
    {                                                                           \
        using partial_result_type = partial_result<type>;                       \
        auto r = ::lexical_ato##type##_partial_with_options(                    \
            first, last, options.as_ptr()                                       \
        );                                                                      \
        return partial_result_type::from(r);                                    \
    }

// Get type name for lexical dispatcher
#define lexical_dispatcher_type(type) type##_dispatcher

// Define a dispatcher for a given type. This
// allows us to use std::conditional to get the proper
// type (a type) from the type. Every single function will
// be static.
#define lexical_dispatcher(type, write_options, parse_options)                  \
    struct lexical_dispatcher_type(type)                                        \
    {                                                                           \
        lexical_to_lexical(type)                                                \
        lexical_to_lexical_with_options(type, write_options)                    \
        lexical_from_lexical(type)                                              \
        lexical_from_lexical_partial(type)                                      \
        lexical_from_lexical_with_options(type, parse_options)                  \
        lexical_from_lexical_partial_with_options(type, parse_options)          \
    }

lexical_dispatcher(i8, write_integer_options, parse_integer_options);
lexical_dispatcher(i16, write_integer_options, parse_integer_options);
lexical_dispatcher(i32, write_integer_options, parse_integer_options);
lexical_dispatcher(i64, write_integer_options, parse_integer_options);
lexical_dispatcher(isize, write_integer_options, parse_integer_options);

lexical_dispatcher(u8, write_integer_options, parse_integer_options);
lexical_dispatcher(u16, write_integer_options, parse_integer_options);
lexical_dispatcher(u32, write_integer_options, parse_integer_options);
lexical_dispatcher(u64, write_integer_options, parse_integer_options);
lexical_dispatcher(usize, write_integer_options, parse_integer_options);

lexical_dispatcher(f32, write_float_options, parse_float_options);
lexical_dispatcher(f64, write_float_options, parse_float_options);

// GET DISPATCHER

// Check if value is same as type parameter.
#define lexical_is_same(type) std::is_same<T, type>::value

// Conditional to simplify long recursive statements.
// This is to prevent a lot of super ugly code from being written
// (not that it's very pretty regardless).
#define lexical_conditional(name, fallback)                                     \
    typename std::conditional<                                                  \
        lexical_is_same(name),                                                  \
        lexical_dispatcher_type(name),                                          \
        fallback                                                                \
    >::type

// Create a single template that resolves to our dispatcher **or**
// evaluates to void.
template <typename T>
using dispatcher = lexical_conditional(
    i8, lexical_conditional(i16, lexical_conditional(i32,
        lexical_conditional(i64, lexical_conditional(isize,
            lexical_conditional(u8, lexical_conditional(u16,
                lexical_conditional(u32, lexical_conditional(u64,
                    lexical_conditional(usize, lexical_conditional(f32,
                        lexical_conditional(f64, void)
                    ))
                ))
            ))
        ))
    ))
);

// TO STRING

// Write directly to a pointer range and
// return a pointer to the last written element.
template <typename T>
inline uint8_t* write(T value, uint8_t* first, uint8_t* last)
{
    using disp = dispatcher<T>;
    static_assert(!std::is_void<disp>::value, "Invalid type passed to write.");
    return disp::to_lexical(value, first, last);
}

// High-level function to serialize a value to a string.
template <typename T>
inline std::string to_string(T value)
{
    assert(WRITE_SIZE >= BUFFER_SIZE);

    uint8_t array[WRITE_SIZE];
    uint8_t* first = array;
    uint8_t* last = first + WRITE_SIZE;
    auto ptr = write(value, first, last);
    return std::string(reinterpret_cast<char*>(first), std::distance(first, ptr));
}

// Write directly to a pointer range and
// return a pointer to the last written element.
template <typename T, typename Options>
inline uint8_t* write_with_options(T value, const Options& options, uint8_t* first, uint8_t* last)
{
    using disp = dispatcher<T>;
    static_assert(!std::is_void<disp>::value, "Invalid type passed to write_with_options.");
    return disp::to_lexical_with_options(value, options, first, last);
}

// High-level function to serialize a value to a string with custom options.
template <typename T, typename Options>
inline std::string to_string_with_options(T value, const Options& options)
{
    assert(WRITE_SIZE >= BUFFER_SIZE);

    uint8_t array[WRITE_SIZE];
    uint8_t* first = array;
    uint8_t* last = first + WRITE_SIZE;
    auto ptr = write_with_options(value, options, first, last);
    return std::string(reinterpret_cast<char*>(first), std::distance(first, ptr));
}

// PARSE

// High-level function to parse a value from string.
template <typename T>
inline result<T> parse(std::string_view string)
{
    using disp = dispatcher<T>;
    static_assert(!std::is_void<disp>::value, "Invalid type passed to parse.");

    auto* first = reinterpret_cast<uint8_t const*>(string.data());
    auto* last = first + string.length();
    return disp::from_lexical(first, last);
}

// High-level function to partially parse a value from string.
template <typename T>
inline partial_result<T> parse_partial(std::string_view string)
{
    using disp = dispatcher<T>;
    static_assert(!std::is_void<disp>::value, "Invalid type passed to parse_partial.");

    auto* first = reinterpret_cast<uint8_t const*>(string.data());
    auto* last = first + string.length();
    return disp::from_lexical_partial(first, last);
}

// High-level function to parse a value with options from string.
template <typename T, typename Options>
inline result<T> parse_with_options(std::string_view string, const Options& options)
{
    using disp = dispatcher<T>;
    static_assert(!std::is_void<disp>::value, "Invalid type passed to parse_with_options.");

    auto* first = reinterpret_cast<uint8_t const*>(string.data());
    auto* last = first + string.length();
    return disp::from_lexical_with_options(first, last, options);
}

// High-level function to partially parse a value with options from string.
template <typename T, typename Options>
inline partial_result<T> parse_partial_with_options(std::string_view string, const Options& options)
{
    using disp = dispatcher<T>;
    static_assert(!std::is_void<disp>::value, "Invalid type passed to parse_partial_lossy.");

    auto* first = reinterpret_cast<uint8_t const*>(string.data());
    auto* last = first + string.length();
    return disp::from_lexical_partial_with_options(first, last, options);
}

// CLEANUP
// -------

#undef lexical_declare_enum
#undef lexical_define_enum
#undef lexical_relational_operator
#undef lexical_equality
#undef lexical_total_ordering
#undef lexical_explicit_conversion
#undef lexical_bitwise_operator
#undef lexical_bitwise
#undef lexical_get_macro
#undef lexical_builder_field_no_cast
#undef lexical_builder_field_cast
#undef lexical_builder_field
#undef lexical_builder_string
#undef lexical_is_error
#undef lexical_to_lexical
#undef lexical_to_lexical_with_options
#undef lexical_from_lexical
#undef lexical_from_lexical_partial
#undef lexical_from_lexical_with_options
#undef lexical_from_lexical_partial_with_options
#undef lexical_dispatcher_type
#undef lexical_dispatcher
#undef lexical_is_same
#undef lexical_conditional

}   // lexical

#endif  /* !LEXICAL_HPP_ */
