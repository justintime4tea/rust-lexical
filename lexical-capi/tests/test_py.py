"""
    Unittests for the Python API to lexical-core.

    License
    -------

    This is free and unencumbered software released into the public domain.

    Anyone is free to copy, modify, publish, use, compile, sell, or
    distribute this software, either in source code form or as a compiled
    binary, for any purpose, commercial or non-commercial, and by any
    means.

    In jurisdictions that recognize copyright laws, the author or authors
    of this software dedicate any and all copyright interest in the
    software to the public domain. We make this dedication for the benefit
    of the public at large and to the detriment of our heirs and
    successors. We intend this dedication to be an overt act of
    relinquishment in perpetuity of all present and future rights to this
    software under copyright law.

    THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
    EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
    MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
    IN NO EVENT SHALL THE AUTHORS BE LIABLE FOR ANY CLAIM, DAMAGES OR
    OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
    ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
    OTHER DEALINGS IN THE SOFTWARE.

    For more information, please refer to <http://unlicense.org/>
"""

import contextlib
import ctypes
import os
import sys
import unittest

# Get path to DLL and Python wrapper.
PROJECT_DIR = os.path.dirname(os.path.dirname(os.path.realpath(__file__)))
INCLUDE_DIR = os.path.join(PROJECT_DIR, "include")
RELEASE_DIR = os.path.join(PROJECT_DIR, "target", "release")

# Change our working directory to the release directory, and our include
# path to include the Python source file.
sys.path.insert(0, INCLUDE_DIR)
os.chdir(RELEASE_DIR)
import lexical


class GlobalTests(unittest.TestCase):
    '''Test the global config variables.'''

    def test_max_size(self):
        self.assertIsInstance(lexical.I8_FORMATTED_SIZE, int)
        self.assertIsInstance(lexical.I16_FORMATTED_SIZE, int)
        self.assertIsInstance(lexical.I32_FORMATTED_SIZE, int)
        self.assertIsInstance(lexical.I64_FORMATTED_SIZE, int)
        self.assertIsInstance(lexical.I128_FORMATTED_SIZE, int)
        self.assertIsInstance(lexical.ISIZE_FORMATTED_SIZE, int)
        self.assertIsInstance(lexical.U8_FORMATTED_SIZE, int)
        self.assertIsInstance(lexical.U16_FORMATTED_SIZE, int)
        self.assertIsInstance(lexical.U32_FORMATTED_SIZE, int)
        self.assertIsInstance(lexical.U64_FORMATTED_SIZE, int)
        self.assertIsInstance(lexical.U128_FORMATTED_SIZE, int)
        self.assertIsInstance(lexical.USIZE_FORMATTED_SIZE, int)
        self.assertIsInstance(lexical.F32_FORMATTED_SIZE, int)
        self.assertIsInstance(lexical.F64_FORMATTED_SIZE, int)
        self.assertIsInstance(lexical.I8_FORMATTED_SIZE_DECIMAL, int)
        self.assertIsInstance(lexical.I16_FORMATTED_SIZE_DECIMAL, int)
        self.assertIsInstance(lexical.I32_FORMATTED_SIZE_DECIMAL, int)
        self.assertIsInstance(lexical.I64_FORMATTED_SIZE_DECIMAL, int)
        self.assertIsInstance(lexical.I128_FORMATTED_SIZE_DECIMAL, int)
        self.assertIsInstance(lexical.ISIZE_FORMATTED_SIZE_DECIMAL, int)
        self.assertIsInstance(lexical.U8_FORMATTED_SIZE_DECIMAL, int)
        self.assertIsInstance(lexical.U16_FORMATTED_SIZE_DECIMAL, int)
        self.assertIsInstance(lexical.U32_FORMATTED_SIZE_DECIMAL, int)
        self.assertIsInstance(lexical.U64_FORMATTED_SIZE_DECIMAL, int)
        self.assertIsInstance(lexical.U128_FORMATTED_SIZE_DECIMAL, int)
        self.assertIsInstance(lexical.USIZE_FORMATTED_SIZE_DECIMAL, int)
        self.assertIsInstance(lexical.F32_FORMATTED_SIZE_DECIMAL, int)
        self.assertIsInstance(lexical.F64_FORMATTED_SIZE_DECIMAL, int)
        self.assertIsInstance(lexical.BUFFER_SIZE, int)

class ErrorTests(unittest.TestCase):
    '''Test ErrorCode and Error structures.'''

    def setUp(self):
        self.overflow = lexical.Error(lexical.ErrorCode.Overflow.value, 0)
        self.underflow = lexical.Error(lexical.ErrorCode.Underflow.value, 0)
        self.invalid_digit = lexical.Error(lexical.ErrorCode.InvalidDigit.value, 0)
        self.empty = lexical.Error(lexical.ErrorCode.Empty.value, 0)
        self.empty_mantissa = lexical.Error(lexical.ErrorCode.EmptyMantissa.value, 0)
        self.empty_exponent = lexical.Error(lexical.ErrorCode.EmptyExponent.value, 0)
        self.empty_integer = lexical.Error(lexical.ErrorCode.EmptyInteger.value, 0)
        self.empty_fraction = lexical.Error(lexical.ErrorCode.EmptyFraction.value, 0)
        self.invalid_positive_mantissa_sign = lexical.Error(lexical.ErrorCode.InvalidPositiveMantissaSign.value, 0)
        self.missing_mantissa_sign = lexical.Error(lexical.ErrorCode.MissingMantissaSign.value, 0)
        self.invalid_exponent = lexical.Error(lexical.ErrorCode.InvalidExponent.value, 0)
        self.invalid_positive_exponent_sign = lexical.Error(lexical.ErrorCode.InvalidPositiveExponentSign.value, 0)
        self.missing_exponent_sign = lexical.Error(lexical.ErrorCode.MissingExponentSign.value, 0)
        self.exponent_without_fraction = lexical.Error(lexical.ErrorCode.ExponentWithoutFraction.value, 0)
        self.invalid_leading_zeros = lexical.Error(lexical.ErrorCode.InvalidLeadingZeros.value, 0)

    def test_is_overflow(self):
        self.assertTrue(self.overflow.is_overflow())
        self.assertFalse(self.underflow.is_overflow())

    def test_is_underflow(self):
        self.assertFalse(self.overflow.is_underflow())
        self.assertTrue(self.underflow.is_underflow())

    def test_is_invalid_digit(self):
        self.assertFalse(self.overflow.is_invalid_digit())
        self.assertTrue(self.invalid_digit.is_invalid_digit())

    def test_is_empty(self):
        self.assertFalse(self.overflow.is_empty())
        self.assertTrue(self.empty.is_empty())

    def test_is_empty_mantissa(self):
        self.assertFalse(self.overflow.is_empty_mantissa())
        self.assertTrue(self.empty_mantissa.is_empty_mantissa())

    def test_is_empty_exponent(self):
        self.assertFalse(self.overflow.is_empty_exponent())
        self.assertTrue(self.empty_exponent.is_empty_exponent())

    def test_is_empty_integer(self):
        self.assertFalse(self.overflow.is_empty_integer())
        self.assertTrue(self.empty_integer.is_empty_integer())

    def test_is_empty_fraction(self):
        self.assertFalse(self.overflow.is_empty_fraction())
        self.assertTrue(self.empty_fraction.is_empty_fraction())

    def test_is_invalid_positive_mantissa_sign(self):
        self.assertFalse(self.overflow.is_invalid_positive_mantissa_sign())
        self.assertTrue(self.invalid_positive_mantissa_sign.is_invalid_positive_mantissa_sign())

    def test_is_missing_mantissa_sign(self):
        self.assertFalse(self.overflow.is_missing_mantissa_sign())
        self.assertTrue(self.missing_mantissa_sign.is_missing_mantissa_sign())

    def test_is_invalid_exponent(self):
        self.assertFalse(self.overflow.is_invalid_exponent())
        self.assertTrue(self.invalid_exponent.is_invalid_exponent())

    def test_is_invalid_positive_exponent_sign(self):
        self.assertFalse(self.overflow.is_invalid_positive_exponent_sign())
        self.assertTrue(self.invalid_positive_exponent_sign.is_invalid_positive_exponent_sign())

    def test_is_missing_exponent_sign(self):
        self.assertFalse(self.overflow.is_missing_exponent_sign())
        self.assertTrue(self.missing_exponent_sign.is_missing_exponent_sign())

    def test_is_exponent_without_fraction(self):
        self.assertFalse(self.overflow.is_exponent_without_fraction())
        self.assertTrue(self.exponent_without_fraction.is_exponent_without_fraction())

    def test_is_invalid_leading_zeros(self):
        self.assertFalse(self.overflow.is_invalid_leading_zeros())
        self.assertTrue(self.invalid_leading_zeros.is_invalid_leading_zeros())


class NumberFormatTests(unittest.TestCase):
    '''Test NumberFormat and related structures.'''

    def test_builder(self):
        builder = lexical.NumberFormat.builder()
        if lexical.HAVE_FORMAT:
            builder = builder.digit_separator('_')
            builder = builder.required_integer_digits(False)
            builder = builder.required_fraction_digits(False)
            builder = builder.required_exponent_digits(False)
            builder = builder.no_positive_mantissa_sign(False)
            builder = builder.required_mantissa_sign(False)
            builder = builder.no_exponent_notation(False)
            builder = builder.no_positive_exponent_sign(False)
            builder = builder.required_exponent_sign(False)
            builder = builder.no_exponent_without_fraction(False)
            builder = builder.no_special(False)
            builder = builder.case_sensitive_special(False)
            builder = builder.no_integer_leading_zeros(False)
            builder = builder.no_float_leading_zeros(False)
            builder = builder.integer_internal_digit_separator(False)
            builder = builder.fraction_internal_digit_separator(False)
            builder = builder.exponent_internal_digit_separator(False)
            builder = builder.integer_leading_digit_separator(False)
            builder = builder.fraction_leading_digit_separator(False)
            builder = builder.exponent_leading_digit_separator(False)
            builder = builder.integer_trailing_digit_separator(False)
            builder = builder.fraction_trailing_digit_separator(False)
            builder = builder.exponent_trailing_digit_separator(False)
            builder = builder.integer_consecutive_digit_separator(False)
            builder = builder.fraction_consecutive_digit_separator(False)
            builder = builder.exponent_consecutive_digit_separator(False)
            builder = builder.special_digit_separator(False)
        format = builder.build()
        if lexical.HAVE_FORMAT:
            self.assertEqual(format.digit_separator, '\x00')
            self.assertEqual(format.required_integer_digits, False)
            self.assertEqual(format.required_fraction_digits, False)
            self.assertEqual(format.required_exponent_digits, False)
            self.assertEqual(format.no_positive_mantissa_sign, False)
            self.assertEqual(format.required_mantissa_sign, False)
            self.assertEqual(format.no_exponent_notation, False)
            self.assertEqual(format.no_positive_exponent_sign, False)
            self.assertEqual(format.required_exponent_sign, False)
            self.assertEqual(format.no_exponent_without_fraction, False)
            self.assertEqual(format.no_special, False)
            self.assertEqual(format.case_sensitive_special, False)
            self.assertEqual(format.no_integer_leading_zeros, False)
            self.assertEqual(format.no_float_leading_zeros, False)
            self.assertEqual(format.integer_internal_digit_separator, False)
            self.assertEqual(format.fraction_internal_digit_separator, False)
            self.assertEqual(format.exponent_internal_digit_separator, False)
            self.assertEqual(format.integer_leading_digit_separator, False)
            self.assertEqual(format.fraction_leading_digit_separator, False)
            self.assertEqual(format.exponent_leading_digit_separator, False)
            self.assertEqual(format.integer_trailing_digit_separator, False)
            self.assertEqual(format.fraction_trailing_digit_separator, False)
            self.assertEqual(format.exponent_trailing_digit_separator, False)
            self.assertEqual(format.integer_consecutive_digit_separator, False)
            self.assertEqual(format.fraction_consecutive_digit_separator, False)
            self.assertEqual(format.exponent_consecutive_digit_separator, False)
            self.assertEqual(format.special_digit_separator, False)

    def test_constants(self):
        if lexical.HAVE_FORMAT:
            constants = [
                lexical.NumberFormat.RustLiteral,
                lexical.NumberFormat.RustString,
                lexical.NumberFormat.RustStringStrict,
                lexical.NumberFormat.PythonLiteral,
                lexical.NumberFormat.PythonString,
                lexical.NumberFormat.Cxx17Literal,
                lexical.NumberFormat.Cxx17String,
                lexical.NumberFormat.Cxx14Literal,
                lexical.NumberFormat.Cxx14String,
                lexical.NumberFormat.Cxx11Literal,
                lexical.NumberFormat.Cxx11String,
                lexical.NumberFormat.Cxx03Literal,
                lexical.NumberFormat.Cxx03String,
                lexical.NumberFormat.Cxx98Literal,
                lexical.NumberFormat.Cxx98String,
                lexical.NumberFormat.C18Literal,
                lexical.NumberFormat.C18String,
                lexical.NumberFormat.C11Literal,
                lexical.NumberFormat.C11String,
                lexical.NumberFormat.C99Literal,
                lexical.NumberFormat.C99String,
                lexical.NumberFormat.C90Literal,
                lexical.NumberFormat.C90String,
                lexical.NumberFormat.C89Literal,
                lexical.NumberFormat.C89String,
                lexical.NumberFormat.RubyLiteral,
                lexical.NumberFormat.RubyString,
                lexical.NumberFormat.SwiftLiteral,
                lexical.NumberFormat.SwiftString,
                lexical.NumberFormat.GoLiteral,
                lexical.NumberFormat.GoString,
                lexical.NumberFormat.HaskellLiteral,
                lexical.NumberFormat.HaskellString,
                lexical.NumberFormat.JavascriptLiteral,
                lexical.NumberFormat.JavascriptString,
                lexical.NumberFormat.PerlLiteral,
                lexical.NumberFormat.PerlString,
                lexical.NumberFormat.PhpLiteral,
                lexical.NumberFormat.PhpString,
                lexical.NumberFormat.JavaLiteral,
                lexical.NumberFormat.JavaString,
                lexical.NumberFormat.RLiteral,
                lexical.NumberFormat.RString,
                lexical.NumberFormat.KotlinLiteral,
                lexical.NumberFormat.KotlinString,
                lexical.NumberFormat.JuliaLiteral,
                lexical.NumberFormat.JuliaString,
                lexical.NumberFormat.Csharp7Literal,
                lexical.NumberFormat.Csharp7String,
                lexical.NumberFormat.Csharp6Literal,
                lexical.NumberFormat.Csharp6String,
                lexical.NumberFormat.Csharp5Literal,
                lexical.NumberFormat.Csharp5String,
                lexical.NumberFormat.Csharp4Literal,
                lexical.NumberFormat.Csharp4String,
                lexical.NumberFormat.Csharp3Literal,
                lexical.NumberFormat.Csharp3String,
                lexical.NumberFormat.Csharp2Literal,
                lexical.NumberFormat.Csharp2String,
                lexical.NumberFormat.Csharp1Literal,
                lexical.NumberFormat.Csharp1String,
                lexical.NumberFormat.KawaLiteral,
                lexical.NumberFormat.KawaString,
                lexical.NumberFormat.GambitcLiteral,
                lexical.NumberFormat.GambitcString,
                lexical.NumberFormat.GuileLiteral,
                lexical.NumberFormat.GuileString,
                lexical.NumberFormat.ClojureLiteral,
                lexical.NumberFormat.ClojureString,
                lexical.NumberFormat.ErlangLiteral,
                lexical.NumberFormat.ErlangString,
                lexical.NumberFormat.ElmLiteral,
                lexical.NumberFormat.ElmString,
                lexical.NumberFormat.ScalaLiteral,
                lexical.NumberFormat.ScalaString,
                lexical.NumberFormat.ElixirLiteral,
                lexical.NumberFormat.ElixirString,
                lexical.NumberFormat.FortranLiteral,
                lexical.NumberFormat.FortranString,
                lexical.NumberFormat.DLiteral,
                lexical.NumberFormat.DString,
                lexical.NumberFormat.CoffeescriptLiteral,
                lexical.NumberFormat.CoffeescriptString,
                lexical.NumberFormat.CobolLiteral,
                lexical.NumberFormat.CobolString,
                lexical.NumberFormat.FsharpLiteral,
                lexical.NumberFormat.FsharpString,
                lexical.NumberFormat.VbLiteral,
                lexical.NumberFormat.VbString,
                lexical.NumberFormat.OcamlLiteral,
                lexical.NumberFormat.OcamlString,
                lexical.NumberFormat.ObjectivecLiteral,
                lexical.NumberFormat.ObjectivecString,
                lexical.NumberFormat.ReasonmlLiteral,
                lexical.NumberFormat.ReasonmlString,
                lexical.NumberFormat.OctaveLiteral,
                lexical.NumberFormat.OctaveString,
                lexical.NumberFormat.MatlabLiteral,
                lexical.NumberFormat.MatlabString,
                lexical.NumberFormat.ZigLiteral,
                lexical.NumberFormat.ZigString,
                lexical.NumberFormat.SageLiteral,
                lexical.NumberFormat.SageString,
                lexical.NumberFormat.Json,
                lexical.NumberFormat.Toml,
                lexical.NumberFormat.Yaml,
                lexical.NumberFormat.Xml,
                lexical.NumberFormat.Sqlite,
                lexical.NumberFormat.Postgresql,
                lexical.NumberFormat.Mysql,
                lexical.NumberFormat.Mongodb
            ]
            for constant in constants:
                self.assertIsInstance(constant, lexical.NumberFormat)


class ParseIntegerOptionsTests(unittest.TestCase):
    '''Test ParseIntegerOptions and related structures.'''

    def test_builder(self):
        builder = lexical.ParseIntegerOptions.builder()
        if lexical.HAVE_RADIX:
            builder = builder.radix(2)
        if lexical.HAVE_FORMAT:
            builder = builder.format(lexical.NumberFormat.Json)
        options = builder.build()
        self.assertIsInstance(options.radix, int)
        self.assertIsInstance(options.format, lexical.NumberFormat)


class ParseFloatOptionsTests(unittest.TestCase):
    '''Test ParseFloatOptions and related structures.'''

    def test_builder(self):
        builder = lexical.ParseFloatOptions.builder()
        builder = builder.lossy(True)
        builder = builder.exponent_char('e')
        builder = builder.nan_string('NAN')
        builder = builder.inf_string('INF')
        builder = builder.infinity_string('INFINITY')
        if lexical.HAVE_RADIX:
            builder = builder.radix(2)
        if lexical.HAVE_FORMAT:
            builder = builder.format(lexical.NumberFormat.Json)
        if lexical.HAVE_ROUNDING:
            builder = builder.rounding(lexical.RoundingKind.TowardZero)
        options = builder.build()
        self.assertEqual(options.lossy, True)
        self.assertEqual(options.exponent_char, 'e')
        self.assertEqual(options.nan_string, 'NAN')
        self.assertEqual(options.inf_string, 'INF')
        self.assertEqual(options.infinity_string, 'INFINITY')
        self.assertIsInstance(options.radix, int)
        self.assertIsInstance(options.format, lexical.NumberFormat)
        self.assertIsInstance(options.rounding, lexical.RoundingKind)


class WriteIntegerOptionsTests(unittest.TestCase):
    '''Test WriteIntegerOptions and related structures.'''

    def test_builder(self):
        builder = lexical.WriteIntegerOptions.builder()
        if lexical.HAVE_RADIX:
            builder = builder.radix(2)
        options = builder.build()
        self.assertIsInstance(options.radix, int)


class WriteFloatOptionsTests(unittest.TestCase):
    '''Test WriteFloatOptions and related structures.'''

    def test_builder(self):
        builder = lexical.WriteFloatOptions.builder()
        builder = builder.exponent_char('e')
        builder = builder.trim_floats(True)
        builder = builder.nan_string('NAN')
        builder = builder.inf_string('INF')
        if lexical.HAVE_RADIX:
            builder = builder.radix(2)
        options = builder.build()
        self.assertEqual(options.exponent_char, 'e')
        self.assertEqual(options.trim_floats, True)
        self.assertEqual(options.nan_string, 'NAN')
        self.assertEqual(options.inf_string, 'INF')
        self.assertIsInstance(options.radix, int)


class ResultTests(unittest.TestCase):
    '''Test complete and partial result types.'''

    def setUp(self):
        self.error = lexical.Error(lexical.ErrorCode.Overflow.value, 0)

    def _complete_test(self, cls, value_type):
        success_union = cls.union_type(value=value_type(1))
        error_union = cls.union_type(error=self.error)
        success = cls(lexical.ResultTag.Ok.value, success_union)
        error = cls(lexical.ResultTag.Err.value, error_union)
        self.assertEqual(success.into(), value_type(1))
        with self.assertRaises(lexical.LexicalError):
            error.into()

    def _partial_test(self, cls, value_type):
        tuple_type = cls.union_type.value_type
        success_union = cls.union_type(value=tuple_type(value_type(1), 0))
        error_union = cls.union_type(error=self.error)
        success = cls(lexical.ResultTag.Ok.value, success_union)
        error = cls(lexical.ResultTag.Err.value, error_union)
        self.assertEqual(success.into(), (value_type(1), 0))
        with self.assertRaises(lexical.LexicalError):
            error.into()

    def test_result_i8(self):
        self._complete_test(lexical.ResultI8, int)

    def test_result_i16(self):
        self._complete_test(lexical.ResultI16, int)

    def test_result_i32(self):
        self._complete_test(lexical.ResultI32, int)

    def test_result_i64(self):
        self._complete_test(lexical.ResultI64, int)

    def test_result_isize(self):
        self._complete_test(lexical.ResultIsize, int)

    def test_result_u8(self):
        self._complete_test(lexical.ResultU8, int)

    def test_result_u16(self):
        self._complete_test(lexical.ResultU16, int)

    def test_result_u32(self):
        self._complete_test(lexical.ResultU32, int)

    def test_result_u64(self):
        self._complete_test(lexical.ResultU64, int)

    def test_result_usize(self):
        self._complete_test(lexical.ResultUsize, int)

    def test_result_f32(self):
        self._complete_test(lexical.ResultF32, float)

    def test_result_f64(self):
        self._complete_test(lexical.ResultF64, float)

    def test_partial_result_i8(self):
        self._partial_test(lexical.PartialResultI8, int)

    def test_partial_result_i16(self):
        self._partial_test(lexical.PartialResultI16, int)

    def test_partial_result_i32(self):
        self._partial_test(lexical.PartialResultI32, int)

    def test_partial_result_i64(self):
        self._partial_test(lexical.PartialResultI64, int)

    def test_partial_result_isize(self):
        self._partial_test(lexical.PartialResultIsize, int)

    def test_partial_result_u8(self):
        self._partial_test(lexical.PartialResultU8, int)

    def test_partial_result_u16(self):
        self._partial_test(lexical.PartialResultU16, int)

    def test_partial_result_u32(self):
        self._partial_test(lexical.PartialResultU32, int)

    def test_partial_result_u64(self):
        self._partial_test(lexical.PartialResultU64, int)

    def test_partial_result_usize(self):
        self._partial_test(lexical.PartialResultUsize, int)

    def test_partial_result_f32(self):
        self._partial_test(lexical.PartialResultF32, float)

    def test_partial_result_f64(self):
        self._partial_test(lexical.PartialResultF64, float)


class ToStringTests(unittest.TestCase):
    '''Test number-to-string conversion routines.'''

    def _test_integer(self, cb):
        self.assertEqual(cb(10), '10')

    def _test_integer_options(self, cb):
        options = lexical.WriteIntegerOptions.builder().build()
        self.assertEqual(cb(10, options), '10')

        if lexical.HAVE_RADIX:
            options = lexical.WriteIntegerOptions.binary()
            self.assertEqual(cb(10, options), '1010')

            options = lexical.WriteIntegerOptions.decimal()
            self.assertEqual(cb(10, options), '10')

            options = lexical.WriteIntegerOptions.hexadecimal()
            self.assertEqual(cb(10, options), 'A')

    def _test_float(self, cb):
        self.assertEqual(cb(10.5), '10.5')

    def _test_float_options(self, cb):
        options = lexical.WriteFloatOptions.builder().build()
        self.assertEqual(cb(10.0, options), '10.0')
        self.assertEqual(cb(10.5, options), '10.5')

        options = lexical.WriteFloatOptions.builder().trim_floats(True).build()
        self.assertEqual(cb(10.0, options), '10')
        self.assertEqual(cb(10.5, options), '10.5')

        if lexical.HAVE_RADIX:
            options = lexical.WriteFloatOptions.binary()
            self.assertEqual(cb(10.5, options), '1010.1')

            options = lexical.WriteFloatOptions.decimal()
            self.assertEqual(cb(10.5, options), '10.5')

            options = lexical.WriteFloatOptions.hexadecimal()
            self.assertEqual(cb(10.5, options), 'A.8')

    def test_i8toa(self):
        self._test_integer(lexical.i8toa)

    def test_i16toa(self):
        self._test_integer(lexical.i16toa)

    def test_i32toa(self):
        self._test_integer(lexical.i32toa)

    def test_i64toa(self):
        self._test_integer(lexical.i64toa)

    def test_isizetoa(self):
        self._test_integer(lexical.isizetoa)

    def test_u8toa(self):
        self._test_integer(lexical.u8toa)

    def test_u16toa(self):
        self._test_integer(lexical.u16toa)

    def test_u32toa(self):
        self._test_integer(lexical.u32toa)

    def test_u64toa(self):
        self._test_integer(lexical.u64toa)

    def test_usizetoa(self):
        self._test_integer(lexical.usizetoa)

    def test_f32toa(self):
        self._test_float(lexical.f32toa)

    def test_f64toa(self):
        self._test_float(lexical.f64toa)

    def test_i8toa_with_options(self):
        self._test_integer_options(lexical.i8toa_with_options)

    def test_i16toa_with_options(self):
        self._test_integer_options(lexical.i16toa_with_options)

    def test_i32toa_with_options(self):
        self._test_integer_options(lexical.i32toa_with_options)

    def test_i64toa_with_options(self):
        self._test_integer_options(lexical.i64toa_with_options)

    def test_isizetoa_with_options(self):
        self._test_integer_options(lexical.isizetoa_with_options)

    def test_u8toa_with_options(self):
        self._test_integer_options(lexical.u8toa_with_options)

    def test_u16toa_with_options(self):
        self._test_integer_options(lexical.u16toa_with_options)

    def test_u32toa_with_options(self):
        self._test_integer_options(lexical.u32toa_with_options)

    def test_u64toa_with_options(self):
        self._test_integer_options(lexical.u64toa_with_options)

    def test_usizetoa_with_options(self):
        self._test_integer_options(lexical.usizetoa_with_options)

    def test_f32toa_with_options(self):
        self._test_float_options(lexical.f32toa_with_options)

    def test_f64toa_with_options(self):
        self._test_float_options(lexical.f64toa_with_options)


class ParseTests(unittest.TestCase):
    '''Test string-to-number conversion routines.'''

    def _complete_test(self, callback, value_type, *args):
        self.assertEqual(callback('10', *args), value_type(10))
        with self.assertRaises(lexical.LexicalError):
            callback('10a', *args)
        with self.assertRaises(lexical.LexicalError):
            callback('', *args)

        if issubclass(value_type, float):
            # Specialized tests for floats.
            self.assertEqual(callback('10.5', *args), value_type(10.5))
            self.assertEqual(callback('10e5', *args), value_type(10e5))
            with self.assertRaises(lexical.LexicalError):
                callback('.', *args)
            with self.assertRaises(lexical.LexicalError):
                callback('e5', *args)
            with self.assertRaises(lexical.LexicalError):
                callback('10e+', *args)

    def _complete_with_options_test(self, callback, value_type, options_type):
        options = options_type.builder().build()
        with self.assertRaises(lexical.LexicalError):
            callback('', options)

        self.assertEqual(callback('10', options), value_type(10))
        with self.assertRaises(lexical.LexicalError):
            callback('10a', options)

        if lexical.HAVE_RADIX:
            options = options_type.binary()
            self.assertEqual(callback('1010', options), value_type(10))
            with self.assertRaises(lexical.LexicalError):
                callback('10102', options)

            options = options_type.decimal()
            self.assertEqual(callback('10', options), value_type(10))
            with self.assertRaises(lexical.LexicalError):
                callback('10a', options)

            options = options_type.hexadecimal()
            self.assertEqual(callback('A', options), value_type(10))
            with self.assertRaises(lexical.LexicalError):
                callback('AG', options)

        if lexical.HAVE_FORMAT:
            options = options_type.builder().format(lexical.NumberFormat.FsharpString).build()
            self.assertEqual(callback('1_0', options), value_type(10))
            with self.assertRaises(lexical.LexicalError):
                callback('1_0a', options)

        if issubclass(value_type, float):
            # Specialized tests for floats.
            options = options_type.builder().build()
            with self.assertRaises(lexical.LexicalError):
                callback('.', options)

            with self.assertRaises(lexical.LexicalError):
                callback('e5', options)

            with self.assertRaises(lexical.LexicalError):
                callback('10e+', options)

            self.assertEqual(callback('10.5', options), value_type(10.5))
            self.assertEqual(callback('10e5', options), value_type(10e5))

            if lexical.HAVE_RADIX:
                options = options_type.binary()
                self.assertEqual(callback('1010.1', options), value_type(10.5))

                options = options_type.decimal()
                self.assertEqual(callback('10.5', options), value_type(10.5))

                options = options_type.hexadecimal()
                self.assertEqual(callback('A.8', options), value_type(10.5))

            if lexical.HAVE_FORMAT:
                options = options_type.builder().format(lexical.NumberFormat.FsharpString).build()
                self.assertEqual(callback('1_0.7_5', options), value_type(10.75))

    def _partial_test(self, callback, value_type, *args):
        self.assertEqual(callback('10', *args), (value_type(10), 2))
        self.assertEqual(callback('10a', *args), (value_type(10), 2))
        with self.assertRaises(lexical.LexicalError):
            callback('', *args)

        if issubclass(value_type, float):
            # Specialized tests for floats.
            self.assertEqual(callback('10.5', *args), (value_type(10.5), 4))
            self.assertEqual(callback('10e5', *args), (value_type(10e5), 4))
            with self.assertRaises(lexical.LexicalError):
                callback('.', *args)
            with self.assertRaises(lexical.LexicalError):
                callback('e5', *args)
            with self.assertRaises(lexical.LexicalError):
                callback('10e+', *args)

    def _partial_with_options_test(self, callback, value_type, options_type):
        options = options_type.builder().build()
        with self.assertRaises(lexical.LexicalError):
            callback('', options)

        self.assertEqual(callback('10', options), (value_type(10), 2))
        self.assertEqual(callback('10a', options), (value_type(10), 2))

        if lexical.HAVE_RADIX:
            options = options_type.binary()
            self.assertEqual(callback('1010', options), (value_type(10), 4))
            self.assertEqual(callback('10102', options), (value_type(10), 4))

            options = options_type.decimal()
            self.assertEqual(callback('10', options), (value_type(10), 2))
            self.assertEqual(callback('10a', options), (value_type(10), 2))

            options = options_type.hexadecimal()
            self.assertEqual(callback('A', options), (value_type(10), 1))
            self.assertEqual(callback('AG', options), (value_type(10), 1))

        if lexical.HAVE_FORMAT:
            options = options_type.builder().format(lexical.NumberFormat.FsharpString).build()
            self.assertEqual(callback('1_0', options), (value_type(10), 3))
            self.assertEqual(callback('1_0a', options), (value_type(10), 3))

        if issubclass(value_type, float):
            # Specialized tests for floats.
            options = options_type.builder().build()
            with self.assertRaises(lexical.LexicalError):
                callback('.', options)

            with self.assertRaises(lexical.LexicalError):
                callback('e5', options)

            with self.assertRaises(lexical.LexicalError):
                callback('10e+', options)

            self.assertEqual(callback('10.5', options), (value_type(10.5), 4))
            self.assertEqual(callback('10e5', options), (value_type(10e5), 4))

            if lexical.HAVE_RADIX:
                options = options_type.binary()
                self.assertEqual(callback('1010.1', options), (value_type(10.5), 6))

                options = options_type.decimal()
                self.assertEqual(callback('10.5', options), (value_type(10.5), 4))

                options = options_type.hexadecimal()
                self.assertEqual(callback('A.8', options), (value_type(10.5), 3))

            if lexical.HAVE_FORMAT:
                options = options_type.builder().format(lexical.NumberFormat.FsharpString).build()
                self.assertEqual(callback('1_0.7_5', options), (value_type(10.75), 7))

    def test_atoi8(self):
        self._complete_test(lexical.atoi8, int)

    def test_atoi16(self):
        self._complete_test(lexical.atoi16, int)

    def test_atoi32(self):
        self._complete_test(lexical.atoi32, int)

    def test_atoi64(self):
        self._complete_test(lexical.atoi64, int)

    def test_atoisize(self):
        self._complete_test(lexical.atoisize, int)

    def test_atou8(self):
        self._complete_test(lexical.atou8, int)

    def test_atou16(self):
        self._complete_test(lexical.atou16, int)

    def test_atou32(self):
        self._complete_test(lexical.atou32, int)

    def test_atou64(self):
        self._complete_test(lexical.atou64, int)

    def test_atousize(self):
        self._complete_test(lexical.atousize, int)

    def test_atof32(self):
        self._complete_test(lexical.atof32, float)

    def test_atof64(self):
        self._complete_test(lexical.atof64, float)

    def test_atoi8_partial(self):
        self._partial_test(lexical.atoi8_partial, int)

    def test_atoi16_partial(self):
        self._partial_test(lexical.atoi16_partial, int)

    def test_atoi32_partial(self):
        self._partial_test(lexical.atoi32_partial, int)

    def test_atoi64_partial(self):
        self._partial_test(lexical.atoi64_partial, int)

    def test_atoisize_partial(self):
        self._partial_test(lexical.atoisize_partial, int)

    def test_atou8_partial(self):
        self._partial_test(lexical.atou8_partial, int)

    def test_atou16_partial(self):
        self._partial_test(lexical.atou16_partial, int)

    def test_atou32_partial(self):
        self._partial_test(lexical.atou32_partial, int)

    def test_atou64_partial(self):
        self._partial_test(lexical.atou64_partial, int)

    def test_atousize_partial(self):
        self._partial_test(lexical.atousize_partial, int)

    def test_atof32_partial(self):
        self._partial_test(lexical.atof32_partial, float)

    def test_atof64_partial(self):
        self._partial_test(lexical.atof64_partial, float)

    def test_atoi8_with_options(self):
        self._complete_with_options_test(lexical.atoi8_with_options, int, lexical.ParseIntegerOptions)

    def test_atoi16_with_options(self):
        self._complete_with_options_test(lexical.atoi16_with_options, int, lexical.ParseIntegerOptions)

    def test_atoi32_with_options(self):
        self._complete_with_options_test(lexical.atoi32_with_options, int, lexical.ParseIntegerOptions)

    def test_atoi64_with_options(self):
        self._complete_with_options_test(lexical.atoi64_with_options, int, lexical.ParseIntegerOptions)

    def test_atoisize_with_options(self):
        self._complete_with_options_test(lexical.atoisize_with_options, int, lexical.ParseIntegerOptions)

    def test_atou8_with_options(self):
        self._complete_with_options_test(lexical.atou8_with_options, int, lexical.ParseIntegerOptions)

    def test_atou16_with_options(self):
        self._complete_with_options_test(lexical.atou16_with_options, int, lexical.ParseIntegerOptions)

    def test_atou32_with_options(self):
        self._complete_with_options_test(lexical.atou32_with_options, int, lexical.ParseIntegerOptions)

    def test_atou64_with_options(self):
        self._complete_with_options_test(lexical.atou64_with_options, int, lexical.ParseIntegerOptions)

    def test_atousize_with_options(self):
        self._complete_with_options_test(lexical.atousize_with_options, int, lexical.ParseIntegerOptions)

    def test_atof32_with_options(self):
        self._complete_with_options_test(lexical.atof32_with_options, float, lexical.ParseFloatOptions)

    def test_atof64_with_options(self):
        self._complete_with_options_test(lexical.atof64_with_options, float, lexical.ParseFloatOptions)

    def test_atoi8_partial_with_options(self):
        self._partial_with_options_test(lexical.atoi8_partial_with_options, int, lexical.ParseIntegerOptions)

    def test_atoi16_partial_with_options(self):
        self._partial_with_options_test(lexical.atoi16_partial_with_options, int, lexical.ParseIntegerOptions)

    def test_atoi32_partial_with_options(self):
        self._partial_with_options_test(lexical.atoi32_partial_with_options, int, lexical.ParseIntegerOptions)

    def test_atoi64_partial_with_options(self):
        self._partial_with_options_test(lexical.atoi64_partial_with_options, int, lexical.ParseIntegerOptions)

    def test_atoisize_partial_with_options(self):
        self._partial_with_options_test(lexical.atoisize_partial_with_options, int, lexical.ParseIntegerOptions)

    def test_atou8_partial_with_options(self):
        self._partial_with_options_test(lexical.atou8_partial_with_options, int, lexical.ParseIntegerOptions)

    def test_atou16_partial_with_options(self):
        self._partial_with_options_test(lexical.atou16_partial_with_options, int, lexical.ParseIntegerOptions)

    def test_atou32_partial_with_options(self):
        self._partial_with_options_test(lexical.atou32_partial_with_options, int, lexical.ParseIntegerOptions)

    def test_atou64_partial_with_options(self):
        self._partial_with_options_test(lexical.atou64_partial_with_options, int, lexical.ParseIntegerOptions)

    def test_atousize_partial_with_options(self):
        self._partial_with_options_test(lexical.atousize_partial_with_options, int, lexical.ParseIntegerOptions)

    def test_atof32_partial_with_options(self):
        self._partial_with_options_test(lexical.atof32_partial_with_options, float, lexical.ParseFloatOptions)

    def test_atof64_partial_with_options(self):
        self._partial_with_options_test(lexical.atof64_partial_with_options, float, lexical.ParseFloatOptions)


if __name__ == '__main__':
   unittest.main()
