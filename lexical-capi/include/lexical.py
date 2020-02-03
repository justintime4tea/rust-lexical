"""
    lexical
    =======

    Access lexical-capi functionality from Python.

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

from ctypes.util import find_library
from ctypes import *
import contextlib
import enum
import os
import sys

# LOADING
# -------

# Identify the shared lib suffix on the platform.
# Allow the user to specify `SHARED_LIBRARY_SUFFIX` in the shell
# environment to override the default.
if 'SHARED_LIBRARY_SUFFIX' in os.environ:
    SHARED_LIBRARY_SUFFIX = os.environ['SHARED_LIBRARY_SUFFIX']
elif os.name == 'nt':
    SHARED_LIBRARY_SUFFIX = 'dll'
elif sys.platform == 'darwin':
    # Path can be either.
    SHARED_LIBRARY_SUFFIX = 'dylib,so'
else:
    SHARED_LIBRARY_SUFFIX = 'so'

# Wrap the dynlib. Find's the path to an installed lexical-capi library,
# otherwise, assumes it's in the working directory.
# You can modify this code to change how liblexical_capi is loaded for
# your application.
PATH = find_library('lexical_capi')
LIB = None
if PATH is not None:
    LIB = CDLL(PATH)
else:
    for suffix in SHARED_LIBRARY_SUFFIX.split(','):
        path = os.path.join(os.getcwd(), 'liblexical_capi.{}'.format(suffix))
        with contextlib.suppress(OSError):
            LIB = CDLL(path)
if LIB is None:
    raise OSError("Unavailable to find path to the liblexical_capi shared library.")

# HELPERS
# -------

# OPTION TAG

class OptionTag(enum.Enum):
    '''Tag for the option tagged enum.'''

    Some = 0
    NotAny = 1

# OPTION

def _option(cls, name):
    class Option(Structure):
        _fields_ = [
            ("_tag", c_uint),
            ("data", cls)
        ]

        @property
        def tag(self):
            return OptionTag(self._tag)

        @tag.setter
        def tag(self, value):
            if not isinstance(value, OptionTag):
                raise TypeError('Expected OptionTag')
            self._tag = value.value

        def into(self):
            '''Extract value from structure.'''

            if self.tag == OptionTag.NotAny:
                raise RuntimeError('Option was None.')
            return self.data

    Option.__name__ = name
    return Option

# CONVERSIONS

def _to_address(ptr):
    return cast(ptr, c_voidp).value

def _to_u8_ptr(address):
    return cast(address, POINTER(c_ubyte))

def _distance(first, last):
    return _to_address(last) - _to_address(first)

def _to_c_ubyte(char):
    '''Convert single character to c_uint8.'''

    if isinstance(char, str):
        char = char.encode('ascii')
    if isinstance(char, (bytes, bytearray)):
        if len(char) > 1:
            raise ValueError("Must be a single ASCII character.")
        char = char[0]
    if not isinstance(char, c_uint8):
        char = c_uint8(char)
    return char

def _to_bytes(string):
    '''Encode string to bytes when parsing.'''

    if isinstance(string, str):
        string = string.encode('UTF-8')
    return string

def _to_string(string):
    '''Convert string to (ptr, length).'''

    if not isinstance(string, (bytes, bytearray)):
        raise TypeError('String must be a bytes type.')
    return (_to_u8_ptr(string), len(string))

def _to_range(string):
    '''Convert byte string to (first, last) pointer range.'''

    if not isinstance(string, (bytes, bytearray)):
        raise TypeError("Must parse from bytes.")
    first = _to_u8_ptr(string)
    last = _to_u8_ptr(_to_address(first) + len(string))
    return (first, last)

def _to_type(value, type):
    '''Convert generic value to type.'''

    if not isinstance(value, type):
        value = type(value)
    return value

def _from_range(ptr, length):
    '''Convert buffer range to string.'''
    return string_at(ptr, length)

# FEATURES
# --------

HAVE_FORMAT = hasattr(LIB, 'lexical_parse_float_options_builder_format')
HAVE_RADIX = hasattr(LIB, 'lexical_parse_float_options_builder_radix')
HAVE_ROUNDING = hasattr(LIB, 'lexical_parse_float_options_builder_rounding')

class RoundingKind(Structure):
    '''Immutable wrapper around enumerations for float rounding type.'''

    _fields_ = [
        ("_value", c_int32)
    ]

if HAVE_ROUNDING:
    class RoundingKindEnum(enum.Enum):
        '''Rounding type for float-parsing.'''

        NearestTieEven = 0
        NearestTieAwayZero = 1
        TowardPositiveInfinity = 2
        TowardNegativeInfinity = 3
        TowardZero = 4

    # PRE-DEFINED CONSTANTS
    RoundingKind.NearestTieEven = RoundingKind(RoundingKindEnum.NearestTieEven.value)
    RoundingKind.NearestTieAwayZero = RoundingKind(RoundingKindEnum.NearestTieAwayZero.value)
    RoundingKind.TowardPositiveInfinity = RoundingKind(RoundingKindEnum.TowardPositiveInfinity.value)
    RoundingKind.TowardNegativeInfinity = RoundingKind(RoundingKindEnum.TowardNegativeInfinity.value)
    RoundingKind.TowardZero = RoundingKind(RoundingKindEnum.TowardZero.value)

if HAVE_FORMAT:

    def digit_separator_to_flags(digit_separator):
        '''Convert digit separator byte to 32-bit flags.'''
        return ord(digit_separator) << 56

    def digit_separator_from_flags(flags):
        '''Extract digit separator byte from 32-bit flags.'''
        return chr(flags >> 56)


    class NumberFormatFlags(enum.Flag):
        '''Bitflags for a serialized number format.'''

        # NON-DIGIT SEPARATOR FLAGS
        RequiredIntegerDigits               = 0b0000000000000000000000000000000000000000000000000000000000000001
        RequiredFractionDigits              = 0b0000000000000000000000000000000000000000000000000000000000000010
        RequiredExponentDigits              = 0b0000000000000000000000000000000000000000000000000000000000000100
        NoPositiveMantissaSign              = 0b0000000000000000000000000000000000000000000000000000000000001000
        RequiredMantissaSign                = 0b0000000000000000000000000000000000000000000000000000000000010000
        NoExponentNotation                  = 0b0000000000000000000000000000000000000000000000000000000000100000
        NoPositiveExponentSign              = 0b0000000000000000000000000000000000000000000000000000000001000000
        RequiredExponentSign                = 0b0000000000000000000000000000000000000000000000000000000010000000
        NoExponentWithoutFraction           = 0b0000000000000000000000000000000000000000000000000000000100000000
        NoSpecial                           = 0b0000000000000000000000000000000000000000000000000000001000000000
        CaseSensitiveSpecial                = 0b0000000000000000000000000000000000000000000000000000010000000000
        NoIntegerLeadingZeros               = 0b0000000000000000000000000000000000000000000000000000100000000000
        NoFloatLeadingZeros                 = 0b0000000000000000000000000000000000000000000000000001000000000000

        # DIGIT SEPARATOR FLAGS
        IntegerInternalDigitSeparator       = 0b0000000000000000000000000000000100000000000000000000000000000000
        IntegerLeadingDigitSeparator        = 0b0000000000000000000000000000001000000000000000000000000000000000
        IntegerTrailingDigitSeparator       = 0b0000000000000000000000000000010000000000000000000000000000000000
        IntegerConsecutiveDigitSeparator    = 0b0000000000000000000000000000100000000000000000000000000000000000
        FractionInternalDigitSeparator      = 0b0000000000000000000000000001000000000000000000000000000000000000
        FractionLeadingDigitSeparator       = 0b0000000000000000000000000010000000000000000000000000000000000000
        FractionTrailingDigitSeparator      = 0b0000000000000000000000000100000000000000000000000000000000000000
        FractionConsecutiveDigitSeparator   = 0b0000000000000000000000001000000000000000000000000000000000000000
        ExponentInternalDigitSeparator      = 0b0000000000000000000000010000000000000000000000000000000000000000
        ExponentLeadingDigitSeparator       = 0b0000000000000000000000100000000000000000000000000000000000000000
        ExponentTrailingDigitSeparator      = 0b0000000000000000000001000000000000000000000000000000000000000000
        ExponentConsecutiveDigitSeparator   = 0b0000000000000000000010000000000000000000000000000000000000000000
        SpecialDigitSeparator               = 0b0000000000000000000100000000000000000000000000000000000000000000

        # MASKS

        RequiredDigits = (
            RequiredIntegerDigits
            | RequiredFractionDigits
            | RequiredExponentDigits
        )

        InternalDigitSeparator = (
            IntegerInternalDigitSeparator
            | FractionInternalDigitSeparator
            | ExponentInternalDigitSeparator
        )

        LeadingDigitSeparator = (
            IntegerLeadingDigitSeparator
            | FractionLeadingDigitSeparator
            | ExponentLeadingDigitSeparator
        )

        TrailingDigitSeparator = (
            IntegerTrailingDigitSeparator
            | FractionTrailingDigitSeparator
            | ExponentTrailingDigitSeparator
        )

        ConsecutiveDigitSeparator = (
            IntegerConsecutiveDigitSeparator
            | FractionConsecutiveDigitSeparator
            | ExponentConsecutiveDigitSeparator
        )

        DigitSeparatorFlagMask = (
            InternalDigitSeparator
            | LeadingDigitSeparator
            | TrailingDigitSeparator
            | ConsecutiveDigitSeparator
            | SpecialDigitSeparator
        )

        IntegerDigitSeparatorFlagMask = (
            IntegerInternalDigitSeparator
            | IntegerLeadingDigitSeparator
            | IntegerTrailingDigitSeparator
            | IntegerConsecutiveDigitSeparator
        )

        FractionDigitSeparatorFlagMask = (
            FractionInternalDigitSeparator
            | FractionLeadingDigitSeparator
            | FractionTrailingDigitSeparator
            | FractionConsecutiveDigitSeparator
        )

        ExponentDigitSeparatorFlagMask = (
            ExponentInternalDigitSeparator
            | ExponentLeadingDigitSeparator
            | ExponentTrailingDigitSeparator
            | ExponentConsecutiveDigitSeparator
        )

        ExponentFlagMask = (
            RequiredExponentDigits
            | NoPositiveExponentSign
            | RequiredExponentSign
            | NoExponentWithoutFraction
            | ExponentInternalDigitSeparator
            | ExponentLeadingDigitSeparator
            | ExponentTrailingDigitSeparator
            | ExponentConsecutiveDigitSeparator
        )

        FlagMask = (
            RequiredDigits
            | NoPositiveMantissaSign
            | RequiredMantissaSign
            | NoExponentNotation
            | NoPositiveExponentSign
            | RequiredExponentSign
            | NoExponentWithoutFraction
            | NoSpecial
            | CaseSensitiveSpecial
            | NoIntegerLeadingZeros
            | NoFloatLeadingZeros
            | InternalDigitSeparator
            | LeadingDigitSeparator
            | TrailingDigitSeparator
            | ConsecutiveDigitSeparator
            | SpecialDigitSeparator
        )

        # HIDDEN DEFAULTS

        Permissive = 0
        Standard = RequiredExponentDigits
        Ignore = DigitSeparatorFlagMask

class NumberFormat(Structure):
    '''Immutable wrapper around bitflags for a serialized number format.'''

    _fields_ = [
        ("_value", c_uint64)
    ]

    @staticmethod
    def builder():
        '''Get NumberFormat builder.'''
        return NumberFormatBuilder(LIB.lexical_number_format_builder())

    if HAVE_FORMAT:

        @property
        def _digit_separator(self):
            return digit_separator_from_flags(self._value)

        @property
        def _flags(self):
            return NumberFormatFlags(self._value & NumberFormatFlags.FlagMask.value)

        # FUNCTIONS

        @staticmethod
        def permissive():
            '''
            Compile permissive number format.

            The permissive number format does not require any control
            grammar, besides the presence of mantissa digits.
            '''
            return LIB.lexical_number_format_permissive().into()

        @staticmethod
        def standard():
            '''
            Compile standard number format.

            The standard number format is guaranteed to be identical
            to the format expected by Rust's string to number parsers.
            '''
            return LIB.lexical_number_format_standard().into()

        @staticmethod
        def ignore(digit_separator):
            '''
            Compile ignore number format.

            The ignore number format ignores all digit separators,
            and is permissive for all other control grammar, so
            implements a fast parser.

            * `digit_separator`                         - Character to separate digits.
            '''
            return LIB.lexical_number_format_ignore(_to_c_ubyte(digit_separator)).into()

        def intersects(self, flags):
            '''Determine if a flag'''
            return self._value & flags.value != 0

        @property
        def flags(self):
            '''Get the flag bits from the compiled float format.'''
            return self._flags

        @property
        def digit_separator(self):
            '''Get the digit separator from the compiled float format.'''
            return self._digit_separator

        @property
        def required_integer_digits(self):
            '''Get if digits are required before the decimal point.'''
            return self.intersects(NumberFormatFlags.RequiredIntegerDigits)

        @property
        def required_fraction_digits(self):
            '''Get if digits are required after the decimal point.'''
            return self.intersects(NumberFormatFlags.RequiredFractionDigits)

        @property
        def required_exponent_digits(self):
            '''Get if digits are required after the exponent character.'''
            return self.intersects(NumberFormatFlags.RequiredExponentDigits)

        @property
        def required_digits(self):
            '''Get if digits are required before or after the decimal point.'''
            return self.intersects(NumberFormatFlags.RequiredDigits)

        @property
        def no_positive_mantissa_sign(self):
            '''Get if positive sign before the mantissa is not allowed.'''
            return self.intersects(NumberFormatFlags.NoPositiveMantissaSign)

        @property
        def required_mantissa_sign(self):
            '''Get if positive sign before the mantissa is required.'''
            return self.intersects(NumberFormatFlags.RequiredMantissaSign)

        @property
        def no_exponent_notation(self):
            '''Get if exponent notation is not allowed.'''
            return self.intersects(NumberFormatFlags.NoExponentNotation)

        @property
        def no_positive_exponent_sign(self):
            '''Get if positive sign before the exponent is not allowed.'''
            return self.intersects(NumberFormatFlags.NoPositiveExponentSign)

        @property
        def required_exponent_sign(self):
            '''Get if sign before the exponent is required.'''
            return self.intersects(NumberFormatFlags.RequiredExponentSign)

        @property
        def no_exponent_without_fraction(self):
            '''Get if exponent without fraction is not allowed.'''
            return self.intersects(NumberFormatFlags.NoExponentWithoutFraction)

        @property
        def no_special(self):
            '''Get if special (non-finite) values are not allowed.'''
            return self.intersects(NumberFormatFlags.NoSpecial)

        @property
        def case_sensitive_special(self):
            '''Get if special (non-finite) values are case-sensitive.'''
            return self.intersects(NumberFormatFlags.CaseSensitiveSpecial)

        @property
        def no_integer_leading_zeros(self):
            '''Get if leading zeros before an integer are not allowed.'''
            return self.intersects(NumberFormatFlags.NoIntegerLeadingZeros)

        @property
        def no_float_leading_zeros(self):
            '''Get if leading zeros before a float are not allowed.'''
            return self.intersects(NumberFormatFlags.NoFloatLeadingZeros)

        @property
        def integer_internal_digit_separator(self):
            '''Get if digit separators are allowed between integer digits.'''
            return self.intersects(NumberFormatFlags.IntegerInternalDigitSeparator)

        @property
        def fraction_internal_digit_separator(self):
            '''Get if digit separators are allowed between fraction digits.'''
            return self.intersects(NumberFormatFlags.FractionInternalDigitSeparator)

        @property
        def exponent_internal_digit_separator(self):
            '''Get if digit separators are allowed between exponent digits.'''
            return self.intersects(NumberFormatFlags.ExponentInternalDigitSeparator)

        @property
        def internal_digit_separator(self):
            '''Get if digit separators are allowed between digits.'''
            return self.intersects(NumberFormatFlags.InternalDigitSeparator)

        @property
        def integer_leading_digit_separator(self):
            '''Get if a digit separator is allowed before any integer digits.'''
            return self.intersects(NumberFormatFlags.IntegerLeadingDigitSeparator)

        @property
        def fraction_leading_digit_separator(self):
            '''Get if a digit separator is allowed before any fraction digits.'''
            return self.intersects(NumberFormatFlags.FractionLeadingDigitSeparator)

        @property
        def exponent_leading_digit_separator(self):
            '''Get if a digit separator is allowed before any exponent digits.'''
            return self.intersects(NumberFormatFlags.ExponentLeadingDigitSeparator)

        @property
        def leading_digit_separator(self):
            '''Get if a digit separator is allowed before any digits.'''
            return self.intersects(NumberFormatFlags.LeadingDigitSeparator)

        @property
        def integer_trailing_digit_separator(self):
            '''Get if a digit separator is allowed after any integer digits.'''
            return self.intersects(NumberFormatFlags.IntegerTrailingDigitSeparator)

        @property
        def fraction_trailing_digit_separator(self):
            '''Get if a digit separator is allowed after any fraction digits.'''
            return self.intersects(NumberFormatFlags.FractionTrailingDigitSeparator)

        @property
        def exponent_trailing_digit_separator(self):
            '''Get if a digit separator is allowed after any exponent digits.'''
            return self.intersects(NumberFormatFlags.ExponentTrailingDigitSeparator)

        @property
        def trailing_digit_separator(self):
            '''Get if a digit separator is allowed after any digits.'''
            return self.intersects(NumberFormatFlags.TrailingDigitSeparator)

        @property
        def integer_consecutive_digit_separator(self):
            '''Get if multiple consecutive integer digit separators are allowed.'''
            return self.intersects(NumberFormatFlags.IntegerConsecutiveDigitSeparator)

        @property
        def fraction_consecutive_digit_separator(self):
            '''Get if multiple consecutive fraction digit separators are allowed.'''
            return self.intersects(NumberFormatFlags.FractionConsecutiveDigitSeparator)

        @property
        def exponent_consecutive_digit_separator(self):
            '''Get if multiple consecutive exponent digit separators are allowed.'''
            return self.intersects(NumberFormatFlags.ExponentConsecutiveDigitSeparator)

        @property
        def consecutive_digit_separator(self):
            '''Get if multiple consecutive digit separators are allowed.'''
            return self.intersects(NumberFormatFlags.ConsecutiveDigitSeparator)

        @property
        def special_digit_separator(self):
            '''Get if any digit separators are allowed in special (non-finite) values.'''
            return self.intersects(NumberFormatFlags.SpecialDigitSeparator)

if HAVE_FORMAT:
    class _NumberFormatBuilder(Structure):
        _fields_ = [
            ('_digit_separator', c_uint8),
            ('_required_integer_digits', c_bool),
            ('_required_fraction_digits', c_bool),
            ('_required_exponent_digits', c_bool),
            ('_no_positive_mantissa_sign', c_bool),
            ('_required_mantissa_sign', c_bool),
            ('_no_exponent_notation', c_bool),
            ('_no_positive_exponent_sign', c_bool),
            ('_required_exponent_sign', c_bool),
            ('_no_exponent_without_fraction', c_bool),
            ('_no_special', c_bool),
            ('_case_sensitive_special', c_bool),
            ('_no_integer_leading_zeros', c_bool),
            ('_no_float_leading_zeros', c_bool),
            ('_integer_internal_digit_separator', c_bool),
            ('_fraction_internal_digit_separator', c_bool),
            ('_exponent_internal_digit_separator', c_bool),
            ('_integer_leading_digit_separator', c_bool),
            ('_fraction_leading_digit_separator', c_bool),
            ('_exponent_leading_digit_separator', c_bool),
            ('_integer_trailing_digit_separator', c_bool),
            ('_fraction_trailing_digit_separator', c_bool),
            ('_exponent_trailing_digit_separator', c_bool),
            ('_integer_consecutive_digit_separator', c_bool),
            ('_fraction_consecutive_digit_separator', c_bool),
            ('_exponent_consecutive_digit_separator', c_bool),
            ('_special_digit_separator', c_bool),
        ]

        def build(self):
            return LIB.lexical_number_format_build(self)

        def digit_separator(self, digit_separator):
            return LIB.lexical_number_format_builder_digit_separator(self, _to_c_ubyte(digit_separator))

        def required_integer_digits(self, required_integer_digits):
            return LIB.lexical_number_format_builder_required_integer_digits(self, _to_type(required_integer_digits, c_bool))

        def required_fraction_digits(self, required_fraction_digits):
            return LIB.lexical_number_format_builder_required_fraction_digits(self, _to_type(required_fraction_digits, c_bool))

        def required_exponent_digits(self, required_exponent_digits):
            return LIB.lexical_number_format_builder_required_exponent_digits(self, _to_type(required_exponent_digits, c_bool))

        def no_positive_mantissa_sign(self, no_positive_mantissa_sign):
            return LIB.lexical_number_format_builder_no_positive_mantissa_sign(self, _to_type(no_positive_mantissa_sign, c_bool))

        def required_mantissa_sign(self, required_mantissa_sign):
            return LIB.lexical_number_format_builder_required_mantissa_sign(self, _to_type(required_mantissa_sign, c_bool))

        def no_exponent_notation(self, no_exponent_notation):
            return LIB.lexical_number_format_builder_no_exponent_notation(self, _to_type(no_exponent_notation, c_bool))

        def no_positive_exponent_sign(self, no_positive_exponent_sign):
            return LIB.lexical_number_format_builder_no_positive_exponent_sign(self, _to_type(no_positive_exponent_sign, c_bool))

        def required_exponent_sign(self, required_exponent_sign):
            return LIB.lexical_number_format_builder_required_exponent_sign(self, _to_type(required_exponent_sign, c_bool))

        def no_exponent_without_fraction(self, no_exponent_without_fraction):
            return LIB.lexical_number_format_builder_no_exponent_without_fraction(self, _to_type(no_exponent_without_fraction, c_bool))

        def no_special(self, no_special):
            return LIB.lexical_number_format_builder_no_special(self, _to_type(no_special, c_bool))

        def case_sensitive_special(self, case_sensitive_special):
            return LIB.lexical_number_format_builder_case_sensitive_special(self, _to_type(case_sensitive_special, c_bool))

        def no_integer_leading_zeros(self, no_integer_leading_zeros):
            return LIB.lexical_number_format_builder_no_integer_leading_zeros(self, _to_type(no_integer_leading_zeros, c_bool))

        def no_float_leading_zeros(self, no_float_leading_zeros):
            return LIB.lexical_number_format_builder_no_float_leading_zeros(self, _to_type(no_float_leading_zeros, c_bool))

        def integer_internal_digit_separator(self, integer_internal_digit_separator):
            return LIB.lexical_number_format_builder_integer_internal_digit_separator(self, _to_type(integer_internal_digit_separator, c_bool))

        def fraction_internal_digit_separator(self, fraction_internal_digit_separator):
            return LIB.lexical_number_format_builder_fraction_internal_digit_separator(self, _to_type(fraction_internal_digit_separator, c_bool))

        def exponent_internal_digit_separator(self, exponent_internal_digit_separator):
            return LIB.lexical_number_format_builder_exponent_internal_digit_separator(self, _to_type(exponent_internal_digit_separator, c_bool))

        def integer_leading_digit_separator(self, integer_leading_digit_separator):
            return LIB.lexical_number_format_builder_integer_leading_digit_separator(self, _to_type(integer_leading_digit_separator, c_bool))

        def fraction_leading_digit_separator(self, fraction_leading_digit_separator):
            return LIB.lexical_number_format_builder_fraction_leading_digit_separator(self, _to_type(fraction_leading_digit_separator, c_bool))

        def exponent_leading_digit_separator(self, exponent_leading_digit_separator):
            return LIB.lexical_number_format_builder_exponent_leading_digit_separator(self, _to_type(exponent_leading_digit_separator, c_bool))

        def integer_trailing_digit_separator(self, integer_trailing_digit_separator):
            return LIB.lexical_number_format_builder_integer_trailing_digit_separator(self, _to_type(integer_trailing_digit_separator, c_bool))

        def fraction_trailing_digit_separator(self, fraction_trailing_digit_separator):
            return LIB.lexical_number_format_builder_fraction_trailing_digit_separator(self, _to_type(fraction_trailing_digit_separator, c_bool))

        def exponent_trailing_digit_separator(self, exponent_trailing_digit_separator):
            return LIB.lexical_number_format_builder_exponent_trailing_digit_separator(self, _to_type(exponent_trailing_digit_separator, c_bool))

        def integer_consecutive_digit_separator(self, integer_consecutive_digit_separator):
            return LIB.lexical_number_format_builder_integer_consecutive_digit_separator(self, _to_type(integer_consecutive_digit_separator, c_bool))

        def fraction_consecutive_digit_separator(self, fraction_consecutive_digit_separator):
            return LIB.lexical_number_format_builder_fraction_consecutive_digit_separator(self, _to_type(fraction_consecutive_digit_separator, c_bool))

        def exponent_consecutive_digit_separator(self, exponent_consecutive_digit_separator):
            return LIB.lexical_number_format_builder_exponent_consecutive_digit_separator(self, _to_type(exponent_consecutive_digit_separator, c_bool))

        def special_digit_separator(self, special_digit_separator):
            return LIB.lexical_number_format_builder_special_digit_separator(self, _to_type(special_digit_separator, c_bool))

else:
    class _NumberFormatBuilder(Structure):
        _fields_ = [
            ('_dummy', c_bool),
        ]

        def build(self):
            return LIB.lexical_number_format_build(self)


class NumberFormatBuilder:
    '''Wrapper around _NumberFormatBuilder'''

    def __init__(self, builder):
        self.builder = builder

    def build(self):
        return self.builder.build().into()

    if HAVE_FORMAT:
        def digit_separator(self, digit_separator):
            self.builder = self.builder.digit_separator(digit_separator)
            return self

        def required_integer_digits(self, required_integer_digits):
            self.builder = self.builder.required_integer_digits(required_integer_digits)
            return self

        def required_fraction_digits(self, required_fraction_digits):
            self.builder = self.builder.required_fraction_digits(required_fraction_digits)
            return self

        def required_exponent_digits(self, required_exponent_digits):
            self.builder = self.builder.required_exponent_digits(required_exponent_digits)
            return self

        def no_positive_mantissa_sign(self, no_positive_mantissa_sign):
            self.builder = self.builder.no_positive_mantissa_sign(no_positive_mantissa_sign)
            return self

        def required_mantissa_sign(self, required_mantissa_sign):
            self.builder = self.builder.required_mantissa_sign(required_mantissa_sign)
            return self

        def no_exponent_notation(self, no_exponent_notation):
            self.builder = self.builder.no_exponent_notation(no_exponent_notation)
            return self

        def no_positive_exponent_sign(self, no_positive_exponent_sign):
            self.builder = self.builder.no_positive_exponent_sign(no_positive_exponent_sign)
            return self

        def required_exponent_sign(self, required_exponent_sign):
            self.builder = self.builder.required_exponent_sign(required_exponent_sign)
            return self

        def no_exponent_without_fraction(self, no_exponent_without_fraction):
            self.builder = self.builder.no_exponent_without_fraction(no_exponent_without_fraction)
            return self

        def no_special(self, no_special):
            self.builder = self.builder.no_special(no_special)
            return self

        def case_sensitive_special(self, case_sensitive_special):
            self.builder = self.builder.case_sensitive_special(case_sensitive_special)
            return self

        def no_integer_leading_zeros(self, no_integer_leading_zeros):
            self.builder = self.builder.no_integer_leading_zeros(no_integer_leading_zeros)
            return self

        def no_float_leading_zeros(self, no_float_leading_zeros):
            self.builder = self.builder.no_float_leading_zeros(no_float_leading_zeros)
            return self

        def integer_internal_digit_separator(self, integer_internal_digit_separator):
            self.builder = self.builder.integer_internal_digit_separator(integer_internal_digit_separator)
            return self

        def fraction_internal_digit_separator(self, fraction_internal_digit_separator):
            self.builder = self.builder.fraction_internal_digit_separator(fraction_internal_digit_separator)
            return self

        def exponent_internal_digit_separator(self, exponent_internal_digit_separator):
            self.builder = self.builder.exponent_internal_digit_separator(exponent_internal_digit_separator)
            return self

        def integer_leading_digit_separator(self, integer_leading_digit_separator):
            self.builder = self.builder.integer_leading_digit_separator(integer_leading_digit_separator)
            return self

        def fraction_leading_digit_separator(self, fraction_leading_digit_separator):
            self.builder = self.builder.fraction_leading_digit_separator(fraction_leading_digit_separator)
            return self

        def exponent_leading_digit_separator(self, exponent_leading_digit_separator):
            self.builder = self.builder.exponent_leading_digit_separator(exponent_leading_digit_separator)
            return self

        def integer_trailing_digit_separator(self, integer_trailing_digit_separator):
            self.builder = self.builder.integer_trailing_digit_separator(integer_trailing_digit_separator)
            return self

        def fraction_trailing_digit_separator(self, fraction_trailing_digit_separator):
            self.builder = self.builder.fraction_trailing_digit_separator(fraction_trailing_digit_separator)
            return self

        def exponent_trailing_digit_separator(self, exponent_trailing_digit_separator):
            self.builder = self.builder.exponent_trailing_digit_separator(exponent_trailing_digit_separator)
            return self

        def integer_consecutive_digit_separator(self, integer_consecutive_digit_separator):
            self.builder = self.builder.integer_consecutive_digit_separator(integer_consecutive_digit_separator)
            return self

        def fraction_consecutive_digit_separator(self, fraction_consecutive_digit_separator):
            self.builder = self.builder.fraction_consecutive_digit_separator(fraction_consecutive_digit_separator)
            return self

        def exponent_consecutive_digit_separator(self, exponent_consecutive_digit_separator):
            self.builder = self.builder.exponent_consecutive_digit_separator(exponent_consecutive_digit_separator)
            return self

        def special_digit_separator(self, special_digit_separator):
            self.builder = self.builder.special_digit_separator(special_digit_separator)
            return self

_OptionNumberFormat = _option(NumberFormat, "_OptionNumberFormat")
LIB.lexical_number_format_builder.restype = _NumberFormatBuilder
LIB.lexical_number_format_build.restype = _OptionNumberFormat
if HAVE_FORMAT:
    LIB.lexical_number_format_builder_digit_separator.restype = _NumberFormatBuilder
    LIB.lexical_number_format_builder_required_integer_digits.restype = _NumberFormatBuilder
    LIB.lexical_number_format_builder_required_fraction_digits.restype = _NumberFormatBuilder
    LIB.lexical_number_format_builder_required_exponent_digits.restype = _NumberFormatBuilder
    LIB.lexical_number_format_builder_no_positive_mantissa_sign.restype = _NumberFormatBuilder
    LIB.lexical_number_format_builder_required_mantissa_sign.restype = _NumberFormatBuilder
    LIB.lexical_number_format_builder_no_exponent_notation.restype = _NumberFormatBuilder
    LIB.lexical_number_format_builder_no_positive_exponent_sign.restype = _NumberFormatBuilder
    LIB.lexical_number_format_builder_required_exponent_sign.restype = _NumberFormatBuilder
    LIB.lexical_number_format_builder_no_exponent_without_fraction.restype = _NumberFormatBuilder
    LIB.lexical_number_format_builder_no_special.restype = _NumberFormatBuilder
    LIB.lexical_number_format_builder_case_sensitive_special.restype = _NumberFormatBuilder
    LIB.lexical_number_format_builder_no_integer_leading_zeros.restype = _NumberFormatBuilder
    LIB.lexical_number_format_builder_no_float_leading_zeros.restype = _NumberFormatBuilder
    LIB.lexical_number_format_builder_integer_internal_digit_separator.restype = _NumberFormatBuilder
    LIB.lexical_number_format_builder_fraction_internal_digit_separator.restype = _NumberFormatBuilder
    LIB.lexical_number_format_builder_exponent_internal_digit_separator.restype = _NumberFormatBuilder
    LIB.lexical_number_format_builder_integer_leading_digit_separator.restype = _NumberFormatBuilder
    LIB.lexical_number_format_builder_fraction_leading_digit_separator.restype = _NumberFormatBuilder
    LIB.lexical_number_format_builder_exponent_leading_digit_separator.restype = _NumberFormatBuilder
    LIB.lexical_number_format_builder_integer_trailing_digit_separator.restype = _NumberFormatBuilder
    LIB.lexical_number_format_builder_fraction_trailing_digit_separator.restype = _NumberFormatBuilder
    LIB.lexical_number_format_builder_exponent_trailing_digit_separator.restype = _NumberFormatBuilder
    LIB.lexical_number_format_builder_integer_consecutive_digit_separator.restype = _NumberFormatBuilder
    LIB.lexical_number_format_builder_fraction_consecutive_digit_separator.restype = _NumberFormatBuilder
    LIB.lexical_number_format_builder_exponent_consecutive_digit_separator.restype = _NumberFormatBuilder
    LIB.lexical_number_format_builder_special_digit_separator.restype = _NumberFormatBuilder

if HAVE_FORMAT:
    LIB.lexical_number_format_permissive.restype = _OptionNumberFormat
    LIB.lexical_number_format_standard.restype = _OptionNumberFormat
    LIB.lexical_number_format_ignore.restype = _OptionNumberFormat

    # PRE-DEFINED CONSTANTS

    # Float format for a Rust literal floating-point number.
    NumberFormat.RustLiteral = NumberFormat(
        digit_separator_to_flags(b'_')
        | NumberFormatFlags.RequiredDigits.value
        | NumberFormatFlags.NoPositiveMantissaSign.value
        | NumberFormatFlags.NoSpecial.value
        | NumberFormatFlags.InternalDigitSeparator.value
        | NumberFormatFlags.TrailingDigitSeparator.value
        | NumberFormatFlags.ConsecutiveDigitSeparator.value
    )

    # Float format to parse a Rust float from string.
    NumberFormat.RustString = NumberFormat(
        NumberFormatFlags.RequiredExponentDigits.value
    )

    # `RustString`, but enforces strict equality for special values.
    NumberFormat.RustStringStrict = NumberFormat(
        NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.CaseSensitiveSpecial.value
    )

    # Float format for a Python literal floating-point number.
    NumberFormat.PythonLiteral = NumberFormat(
        NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.NoSpecial.value
    )

    # Float format to parse a Python float from string.
    NumberFormat.PythonString = NumberFormat(
        NumberFormatFlags.RequiredExponentDigits.value
    )

    # Float format for a C++17 literal floating-point number.
    NumberFormat.Cxx17Literal = NumberFormat(
        digit_separator_to_flags(b'\'')
        | NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.CaseSensitiveSpecial.value
        | NumberFormatFlags.InternalDigitSeparator.value
    )

    NumberFormat.Cxx17String = NumberFormat(
        NumberFormatFlags.RequiredExponentDigits.value
    )

    # Float format for a C++14 literal floating-point number.
    NumberFormat.Cxx14Literal = NumberFormat(
        digit_separator_to_flags(b'\'')
        | NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.CaseSensitiveSpecial.value
        | NumberFormatFlags.InternalDigitSeparator.value
    )

    # Float format to parse a C++14 float from string.
    NumberFormat.Cxx14String = NumberFormat(
        NumberFormatFlags.RequiredExponentDigits.value
    )

    # Float format for a C++11 literal floating-point number.
    NumberFormat.Cxx11Literal = NumberFormat(
        NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.CaseSensitiveSpecial.value
    )

    # Float format to parse a C++11 float from string.
    NumberFormat.Cxx11String = NumberFormat(
        NumberFormatFlags.RequiredExponentDigits.value
    )

    # Float format for a C++03 literal floating-point number.
    NumberFormat.Cxx03Literal = NumberFormat(
        NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.NoSpecial.value
    )

    # Float format to parse a C++03 float from string.
    NumberFormat.Cxx03String = NumberFormat(
        NumberFormatFlags.RequiredExponentDigits.value
    )

    # Float format for a C++98 literal floating-point number.
    NumberFormat.Cxx98Literal = NumberFormat(
        NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.NoSpecial.value
    )

    # Float format to parse a C++98 float from string.
    NumberFormat.Cxx98String = NumberFormat(
        NumberFormatFlags.RequiredExponentDigits.value
    )

    # Float format for a C18 literal floating-point number.
    NumberFormat.C18Literal = NumberFormat(
        NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.CaseSensitiveSpecial.value
    )

    # Float format to parse a C18 float from string.
    NumberFormat.C18String = NumberFormat(
        NumberFormatFlags.RequiredExponentDigits.value
    )

    # Float format for a C11 literal floating-point number.
    NumberFormat.C11Literal = NumberFormat(
        NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.CaseSensitiveSpecial.value
    )

    # Float format to parse a C11 float from string.
    NumberFormat.C11String = NumberFormat(
        NumberFormatFlags.RequiredExponentDigits.value
    )

    # Float format for a C99 literal floating-point number.
    NumberFormat.C99Literal = NumberFormat(
        NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.CaseSensitiveSpecial.value
    )

    # Float format to parse a C99 float from string.
    NumberFormat.C99String = NumberFormat(
        NumberFormatFlags.RequiredExponentDigits.value
    )

    # Float format for a C90 literal floating-point number.
    NumberFormat.C90Literal = NumberFormat(
        NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.NoSpecial.value
    )

    # Float format to parse a C90 float from string.
    NumberFormat.C90String = NumberFormat(
        NumberFormatFlags.RequiredExponentDigits.value
    )

    # Float format for a C89 literal floating-point number.
    NumberFormat.C89Literal = NumberFormat(
        NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.NoSpecial.value
    )

    # Float format to parse a C89 float from string.
    NumberFormat.C89String = NumberFormat(
        NumberFormatFlags.RequiredExponentDigits.value
    )

    # Float format for a Ruby literal floating-point number.
    NumberFormat.RubyLiteral = NumberFormat(
        digit_separator_to_flags(b'_')
        | NumberFormatFlags.RequiredDigits.value
        | NumberFormatFlags.NoSpecial.value
        | NumberFormatFlags.InternalDigitSeparator.value
    )

    # Float format to parse a Ruby float from string.
    NumberFormat.RubyString = NumberFormat(
        digit_separator_to_flags(b'_')
        | NumberFormatFlags.NoSpecial.value
        | NumberFormatFlags.InternalDigitSeparator.value
    )

    # Float format for a Swift literal floating-point number.
    NumberFormat.SwiftLiteral = NumberFormat(
        digit_separator_to_flags(b'_')
        | NumberFormatFlags.RequiredDigits.value
        | NumberFormatFlags.NoSpecial.value
        | NumberFormatFlags.InternalDigitSeparator.value
        | NumberFormatFlags.TrailingDigitSeparator.value
        | NumberFormatFlags.ConsecutiveDigitSeparator.value
    )

    # Float format to parse a Swift float from string.
    NumberFormat.SwiftString = NumberFormat(
        NumberFormatFlags.RequiredFractionDigits.value
    )

    # Float format for a Golang literal floating-point number.
    NumberFormat.GoLiteral = NumberFormat(
        NumberFormatFlags.RequiredFractionDigits.value
        | NumberFormatFlags.NoSpecial.value
    )

    # Float format to parse a Golang float from string.
    NumberFormat.GoString = NumberFormat(
        NumberFormatFlags.RequiredFractionDigits.value
    )

    # Float format for a Haskell literal floating-point number.
    NumberFormat.HaskellLiteral = NumberFormat(
        NumberFormatFlags.RequiredDigits.value
        | NumberFormatFlags.NoPositiveMantissaSign.value
        | NumberFormatFlags.NoSpecial.value
    )

    # Float format to parse a Haskell float from string.
    NumberFormat.HaskellString = NumberFormat(
        NumberFormatFlags.RequiredDigits.value
        | NumberFormatFlags.NoPositiveMantissaSign.value
        | NumberFormatFlags.CaseSensitiveSpecial.value
    )

    # Float format for a Javascript literal floating-point number.
    NumberFormat.JavascriptLiteral = NumberFormat(
        NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.CaseSensitiveSpecial.value
    )

    # Float format to parse a Javascript float from string.
    NumberFormat.JavascriptString = NumberFormat(
        NumberFormatFlags.CaseSensitiveSpecial.value
    )

    # Float format for a Perl literal floating-point number.
    NumberFormat.PerlLiteral = NumberFormat(
        digit_separator_to_flags(b'_')
        | NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.NoSpecial.value
        | NumberFormatFlags.InternalDigitSeparator.value
        | NumberFormatFlags.FractionLeadingDigitSeparator.value
        | NumberFormatFlags.ExponentLeadingDigitSeparator.value
        | NumberFormatFlags.TrailingDigitSeparator.value
        | NumberFormatFlags.ConsecutiveDigitSeparator.value
    )

    # Float format to parse a Perl float from string.
    NumberFormat.PerlString = NumberFormat(0)

    # Float format for a PHP literal floating-point number.
    NumberFormat.PhpLiteral = NumberFormat(
        NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.CaseSensitiveSpecial.value
    )

    # Float format to parse a PHP float from string.
    NumberFormat.PhpString = NumberFormat(
        NumberFormatFlags.NoSpecial.value
    )

    # Float format for a Java literal floating-point number.
    NumberFormat.JavaLiteral = NumberFormat(
        digit_separator_to_flags(b'_')
        | NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.NoSpecial.value
        | NumberFormatFlags.InternalDigitSeparator.value
        | NumberFormatFlags.ConsecutiveDigitSeparator.value
    )

    # Float format to parse a Java float from string.
    NumberFormat.JavaString = NumberFormat(
        NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.CaseSensitiveSpecial.value
    )

    # Float format for a R literal floating-point number.
    NumberFormat.RLiteral = NumberFormat(
        NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.CaseSensitiveSpecial.value
    )

    # Float format to parse a R float from string.
    NumberFormat.RString = NumberFormat(0)

    # Float format for a Kotlin literal floating-point number.
    NumberFormat.KotlinLiteral = NumberFormat(
        digit_separator_to_flags(b'_')
        | NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.NoSpecial.value
        | NumberFormatFlags.InternalDigitSeparator.value
        | NumberFormatFlags.ConsecutiveDigitSeparator.value
    )

    # Float format to parse a Kotlin float from string.
    NumberFormat.KotlinString = NumberFormat(
        NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.CaseSensitiveSpecial.value
    )

    # Float format for a Julia literal floating-point number.
    NumberFormat.JuliaLiteral = NumberFormat(
        digit_separator_to_flags(b'_')
        | NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.CaseSensitiveSpecial.value
        | NumberFormatFlags.IntegerInternalDigitSeparator.value
        | NumberFormatFlags.FractionInternalDigitSeparator.value
    )

    # Float format to parse a Julia float from string.
    NumberFormat.JuliaString = NumberFormat(
        NumberFormatFlags.RequiredExponentDigits.value
    )

    # Float format for a C#7 literal floating-point number.
    NumberFormat.Csharp7Literal = NumberFormat(
        digit_separator_to_flags(b'_')
        | NumberFormatFlags.RequiredFractionDigits.value
        | NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.NoSpecial.value
        | NumberFormatFlags.InternalDigitSeparator.value
        | NumberFormatFlags.ConsecutiveDigitSeparator.value
    )

    # Float format to parse a C#7 float from string.
    NumberFormat.Csharp7String = NumberFormat(
        NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.CaseSensitiveSpecial.value
    )

    # Float format for a C#6 literal floating-point number.
    NumberFormat.Csharp6Literal = NumberFormat(
        NumberFormatFlags.RequiredFractionDigits.value
        | NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.NoSpecial.value
    )

    # Float format to parse a C#6 float from string.
    NumberFormat.Csharp6String = NumberFormat(
        NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.CaseSensitiveSpecial.value
    )

    # Float format for a C#5 literal floating-point number.
    NumberFormat.Csharp5Literal = NumberFormat(
        NumberFormatFlags.RequiredFractionDigits.value
        | NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.NoSpecial.value
    )

    # Float format to parse a C#5 float from string.
    NumberFormat.Csharp5String = NumberFormat(
        NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.CaseSensitiveSpecial.value
    )

    # Float format for a C#4 literal floating-point number.
    NumberFormat.Csharp4Literal = NumberFormat(
        NumberFormatFlags.RequiredFractionDigits.value
        | NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.NoSpecial.value
    )

    # Float format to parse a C#4 float from string.
    NumberFormat.Csharp4String = NumberFormat(
        NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.CaseSensitiveSpecial.value
    )

    # Float format for a C#3 literal floating-point number.
    NumberFormat.Csharp3Literal = NumberFormat(
        NumberFormatFlags.RequiredFractionDigits.value
        | NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.NoSpecial.value
    )

    # Float format to parse a C#3 float from string.
    NumberFormat.Csharp3String = NumberFormat(
        NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.CaseSensitiveSpecial.value
    )

    # Float format for a C#2 literal floating-point number.
    NumberFormat.Csharp2Literal = NumberFormat(
        NumberFormatFlags.RequiredFractionDigits.value
        | NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.NoSpecial.value
    )

    # Float format to parse a C#2 float from string.
    NumberFormat.Csharp2String = NumberFormat(
        NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.CaseSensitiveSpecial.value
    )

    # Float format for a C#1 literal floating-point number.
    NumberFormat.Csharp1Literal = NumberFormat(
        NumberFormatFlags.RequiredFractionDigits.value
        | NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.NoSpecial.value
    )

    # Float format to parse a C#1 float from string.
    NumberFormat.Csharp1String = NumberFormat(
        NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.CaseSensitiveSpecial.value
    )

    # Float format for a Kawa literal floating-point number.
    NumberFormat.KawaLiteral = NumberFormat(
        NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.NoSpecial.value
    )

    # Float format to parse a Kawa float from string.
    NumberFormat.KawaString = NumberFormat(
        NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.NoSpecial.value
    )

    # Float format for a Gambit-C literal floating-point number.
    NumberFormat.GambitcLiteral = NumberFormat(
        NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.NoSpecial.value
    )

    # Float format to parse a Gambit-C float from string.
    NumberFormat.GambitcString = NumberFormat(
        NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.NoSpecial.value
    )

    # Float format for a Guile literal floating-point number.
    NumberFormat.GuileLiteral = NumberFormat(
        NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.NoSpecial.value
    )

    # Float format to parse a Guile float from string.
    NumberFormat.GuileString = NumberFormat(
        NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.NoSpecial.value
    )

    # Float format for a Clojure literal floating-point number.
    NumberFormat.ClojureLiteral = NumberFormat(
        NumberFormatFlags.RequiredIntegerDigits.value
        | NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.NoSpecial.value
    )

    # Float format to parse a Clojure float from string.
    NumberFormat.ClojureString = NumberFormat(
        NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.CaseSensitiveSpecial.value
    )

    # Float format for an Erlang literal floating-point number.
    NumberFormat.ErlangLiteral = NumberFormat(
        NumberFormatFlags.RequiredDigits.value
        | NumberFormatFlags.NoExponentWithoutFraction.value
        | NumberFormatFlags.CaseSensitiveSpecial.value
    )

    # Float format to parse an Erlang float from string.
    NumberFormat.ErlangString = NumberFormat(
        NumberFormatFlags.RequiredDigits.value
        | NumberFormatFlags.NoExponentWithoutFraction.value
        | NumberFormatFlags.NoSpecial.value
    )

    # Float format for an Elm literal floating-point number.
    NumberFormat.ElmLiteral = NumberFormat(
        NumberFormatFlags.RequiredDigits.value
        | NumberFormatFlags.NoPositiveMantissaSign.value
    )

    # Float format to parse an Elm float from string.
    NumberFormat.ElmString = NumberFormat(
        NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.CaseSensitiveSpecial.value
    )

    # Float format for a Scala literal floating-point number.
    NumberFormat.ScalaLiteral = NumberFormat(
        NumberFormatFlags.RequiredDigits.value
        | NumberFormatFlags.NoSpecial.value
    )

    # Float format to parse a Scala float from string.
    NumberFormat.ScalaString = NumberFormat(
        NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.CaseSensitiveSpecial.value
    )

    # Float format for an Elixir literal floating-point number.
    NumberFormat.ElixirLiteral = NumberFormat(
        digit_separator_to_flags(b'_')
        | NumberFormatFlags.RequiredDigits.value
        | NumberFormatFlags.NoExponentWithoutFraction.value
        | NumberFormatFlags.NoSpecial.value
        | NumberFormatFlags.InternalDigitSeparator.value
    )

    # Float format to parse an Elixir float from string.
    NumberFormat.ElixirString = NumberFormat(
        NumberFormatFlags.RequiredDigits.value
        | NumberFormatFlags.NoExponentWithoutFraction.value
        | NumberFormatFlags.NoSpecial.value
    )

    # Float format for a FORTRAN literal floating-point number.
    NumberFormat.FortranLiteral = NumberFormat(
        NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.NoSpecial.value
    )

    # Float format to parse a FORTRAN float from string.
    NumberFormat.FortranString = NumberFormat(
        NumberFormatFlags.RequiredExponentDigits.value
    )

    # Float format for a D literal floating-point number.
    NumberFormat.DLiteral = NumberFormat(
        digit_separator_to_flags(b'_')
        | NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.NoSpecial.value
        | NumberFormatFlags.InternalDigitSeparator.value
        | NumberFormatFlags.TrailingDigitSeparator.value
        | NumberFormatFlags.ConsecutiveDigitSeparator.value
    )

    # Float format to parse a D float from string.
    NumberFormat.DString = NumberFormat(
        digit_separator_to_flags(b'_')
        | NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.IntegerInternalDigitSeparator.value
        | NumberFormatFlags.FractionInternalDigitSeparator.value
        | NumberFormatFlags.IntegerTrailingDigitSeparator.value
        | NumberFormatFlags.FractionTrailingDigitSeparator.value
    )

    # Float format for a Coffeescript literal floating-point number.
    NumberFormat.CoffeescriptLiteral = NumberFormat(
        NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.CaseSensitiveSpecial.value
    )

    # Float format to parse a Coffeescript float from string.
    NumberFormat.CoffeescriptString = NumberFormat(
        NumberFormatFlags.CaseSensitiveSpecial.value
    )

    # Float format for a Cobol literal floating-point number.
    NumberFormat.CobolLiteral = NumberFormat(
        NumberFormatFlags.RequiredFractionDigits.value
        | NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.NoExponentWithoutFraction.value
        | NumberFormatFlags.NoSpecial.value
    )

    # Float format to parse a Cobol float from string.
    NumberFormat.CobolString = NumberFormat(
        NumberFormatFlags.RequiredExponentSign.value
        | NumberFormatFlags.NoSpecial.value
    )

    # Float format for a F# literal floating-point number.
    NumberFormat.FsharpLiteral = NumberFormat(
        digit_separator_to_flags(b'_')
        | NumberFormatFlags.RequiredIntegerDigits.value
        | NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.CaseSensitiveSpecial.value
        | NumberFormatFlags.InternalDigitSeparator.value
        | NumberFormatFlags.ConsecutiveDigitSeparator.value
    )

    # Float format to parse a F# float from string.
    NumberFormat.FsharpString = NumberFormat(
        digit_separator_to_flags(b'_')
        | NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.InternalDigitSeparator.value
        | NumberFormatFlags.CaseSensitiveSpecial.value
        | NumberFormatFlags.LeadingDigitSeparator.value
        | NumberFormatFlags.TrailingDigitSeparator.value
        | NumberFormatFlags.ConsecutiveDigitSeparator.value
        | NumberFormatFlags.SpecialDigitSeparator.value
    )

    # Float format for a Visual Basic literal floating-point number.
    NumberFormat.VbLiteral = NumberFormat(
        NumberFormatFlags.RequiredFractionDigits.value
        | NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.NoSpecial.value
    )

    # Float format to parse a Visual Basic float from string.
    NumberFormat.VbString = NumberFormat(
        NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.CaseSensitiveSpecial.value
    )

    # Float format for an OCaml literal floating-point number.
    NumberFormat.OcamlLiteral = NumberFormat(
        digit_separator_to_flags(b'_')
        | NumberFormatFlags.RequiredIntegerDigits.value
        | NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.NoPositiveMantissaSign.value
        | NumberFormatFlags.CaseSensitiveSpecial.value
        | NumberFormatFlags.InternalDigitSeparator.value
        | NumberFormatFlags.FractionLeadingDigitSeparator.value
        | NumberFormatFlags.TrailingDigitSeparator.value
        | NumberFormatFlags.ConsecutiveDigitSeparator.value
    )

    # Float format to parse an OCaml float from string.
    NumberFormat.OcamlString = NumberFormat(
        digit_separator_to_flags(b'_')
        | NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.InternalDigitSeparator.value
        | NumberFormatFlags.LeadingDigitSeparator.value
        | NumberFormatFlags.TrailingDigitSeparator.value
        | NumberFormatFlags.ConsecutiveDigitSeparator.value
        | NumberFormatFlags.SpecialDigitSeparator.value
    )

    # Float format for an Objective-C literal floating-point number.
    NumberFormat.ObjectivecLiteral = NumberFormat(
        NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.NoSpecial.value
    )

    # Float format to parse an Objective-C float from string.
    NumberFormat.ObjectivecString = NumberFormat(
        NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.NoSpecial.value
    )

    # Float format for a ReasonML literal floating-point number.
    NumberFormat.ReasonmlLiteral = NumberFormat(
        digit_separator_to_flags(b'_')
        | NumberFormatFlags.RequiredIntegerDigits.value
        | NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.CaseSensitiveSpecial.value
        | NumberFormatFlags.InternalDigitSeparator.value
        | NumberFormatFlags.FractionLeadingDigitSeparator.value
        | NumberFormatFlags.TrailingDigitSeparator.value
        | NumberFormatFlags.ConsecutiveDigitSeparator.value
    )

    # Float format to parse a ReasonML float from string.
    NumberFormat.ReasonmlString = NumberFormat(
        digit_separator_to_flags(b'_')
        | NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.InternalDigitSeparator.value
        | NumberFormatFlags.LeadingDigitSeparator.value
        | NumberFormatFlags.TrailingDigitSeparator.value
        | NumberFormatFlags.ConsecutiveDigitSeparator.value
        | NumberFormatFlags.SpecialDigitSeparator.value
    )

    # Float format for an Octave literal floating-point number.
    NumberFormat.OctaveLiteral = NumberFormat(
        digit_separator_to_flags(b'_')
        | NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.CaseSensitiveSpecial.value
        | NumberFormatFlags.InternalDigitSeparator.value
        | NumberFormatFlags.FractionLeadingDigitSeparator.value
        | NumberFormatFlags.TrailingDigitSeparator.value
        | NumberFormatFlags.ConsecutiveDigitSeparator.value
    )

    # Float format to parse an Octave float from string.
    NumberFormat.OctaveString = NumberFormat(
        digit_separator_to_flags(b',')
        | NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.InternalDigitSeparator.value
        | NumberFormatFlags.LeadingDigitSeparator.value
        | NumberFormatFlags.TrailingDigitSeparator.value
        | NumberFormatFlags.ConsecutiveDigitSeparator.value
    )

    # Float format for an Matlab literal floating-point number.
    NumberFormat.MatlabLiteral = NumberFormat(
        digit_separator_to_flags(b'_')
        | NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.CaseSensitiveSpecial.value
        | NumberFormatFlags.InternalDigitSeparator.value
        | NumberFormatFlags.FractionLeadingDigitSeparator.value
        | NumberFormatFlags.TrailingDigitSeparator.value
        | NumberFormatFlags.ConsecutiveDigitSeparator.value
    )

    # Float format to parse an Matlab float from string.
    NumberFormat.MatlabString = NumberFormat(
        digit_separator_to_flags(b',')
        | NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.InternalDigitSeparator.value
        | NumberFormatFlags.LeadingDigitSeparator.value
        | NumberFormatFlags.TrailingDigitSeparator.value
        | NumberFormatFlags.ConsecutiveDigitSeparator.value
    )

    # Float format for a Zig literal floating-point number.
    NumberFormat.ZigLiteral = NumberFormat(
        NumberFormatFlags.RequiredIntegerDigits.value
        | NumberFormatFlags.NoPositiveMantissaSign.value
        | NumberFormatFlags.NoSpecial.value
    )

    # Float format to parse a Zig float from string.
    NumberFormat.ZigString = NumberFormat(0)

    # Float format for a Sage literal floating-point number.
    NumberFormat.SageLiteral = NumberFormat(
        NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.CaseSensitiveSpecial.value
    )

    # Float format to parse a Sage float from string.
    NumberFormat.SageString = NumberFormat(
        digit_separator_to_flags(b'_')
        | NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.InternalDigitSeparator.value
    )

    # Float format for a JSON literal floating-point number.
    NumberFormat.Json = NumberFormat(
        NumberFormatFlags.RequiredDigits.value
        | NumberFormatFlags.NoPositiveMantissaSign.value
        | NumberFormatFlags.NoSpecial.value
    )

    # Float format for a TOML literal floating-point number.
    NumberFormat.Toml = NumberFormat(
        NumberFormatFlags.RequiredDigits.value
        | NumberFormatFlags.NoSpecial.value
        | NumberFormatFlags.InternalDigitSeparator.value
    )

    # Float format for a YAML literal floating-point number.
    NumberFormat.Yaml = NumberFormat.Json

    # Float format for a XML literal floating-point number.
    NumberFormat.Xml = NumberFormat(
        NumberFormatFlags.CaseSensitiveSpecial.value
    )

    # Float format for a SQLite literal floating-point number.
    NumberFormat.Sqlite = NumberFormat(
        NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.NoSpecial.value
    )

    # Float format for a PostgreSQL literal floating-point number.
    NumberFormat.Postgresql = NumberFormat(
        NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.NoSpecial.value
    )

    # Float format for a MySQL literal floating-point number.
    NumberFormat.Mysql = NumberFormat(
        NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.NoSpecial.value
    )

    # Float format for a MongoDB literal floating-point number.
    NumberFormat.Mongodb = NumberFormat(
        NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.CaseSensitiveSpecial.value
    )

# GLOBALS
# -------

# CONSTANTS
I8_FORMATTED_SIZE = c_size_t.in_dll(LIB, 'LEXICAL_I8_FORMATTED_SIZE').value
I16_FORMATTED_SIZE = c_size_t.in_dll(LIB, 'LEXICAL_I16_FORMATTED_SIZE').value
I32_FORMATTED_SIZE = c_size_t.in_dll(LIB, 'LEXICAL_I32_FORMATTED_SIZE').value
I64_FORMATTED_SIZE = c_size_t.in_dll(LIB, 'LEXICAL_I64_FORMATTED_SIZE').value
I128_FORMATTED_SIZE = c_size_t.in_dll(LIB, 'LEXICAL_I128_FORMATTED_SIZE').value
ISIZE_FORMATTED_SIZE = c_size_t.in_dll(LIB, 'LEXICAL_ISIZE_FORMATTED_SIZE').value
U8_FORMATTED_SIZE = c_size_t.in_dll(LIB, 'LEXICAL_U8_FORMATTED_SIZE').value
U16_FORMATTED_SIZE = c_size_t.in_dll(LIB, 'LEXICAL_U16_FORMATTED_SIZE').value
U32_FORMATTED_SIZE = c_size_t.in_dll(LIB, 'LEXICAL_U32_FORMATTED_SIZE').value
U64_FORMATTED_SIZE = c_size_t.in_dll(LIB, 'LEXICAL_U64_FORMATTED_SIZE').value
U128_FORMATTED_SIZE = c_size_t.in_dll(LIB, 'LEXICAL_U128_FORMATTED_SIZE').value
USIZE_FORMATTED_SIZE = c_size_t.in_dll(LIB, 'LEXICAL_USIZE_FORMATTED_SIZE').value
F32_FORMATTED_SIZE = c_size_t.in_dll(LIB, 'LEXICAL_F32_FORMATTED_SIZE').value
F64_FORMATTED_SIZE = c_size_t.in_dll(LIB, 'LEXICAL_F64_FORMATTED_SIZE').value

I8_FORMATTED_SIZE_DECIMAL = c_size_t.in_dll(LIB, 'LEXICAL_I8_FORMATTED_SIZE_DECIMAL').value
I16_FORMATTED_SIZE_DECIMAL = c_size_t.in_dll(LIB, 'LEXICAL_I16_FORMATTED_SIZE_DECIMAL').value
I32_FORMATTED_SIZE_DECIMAL = c_size_t.in_dll(LIB, 'LEXICAL_I32_FORMATTED_SIZE_DECIMAL').value
I64_FORMATTED_SIZE_DECIMAL = c_size_t.in_dll(LIB, 'LEXICAL_I64_FORMATTED_SIZE_DECIMAL').value
I128_FORMATTED_SIZE_DECIMAL = c_size_t.in_dll(LIB, 'LEXICAL_I128_FORMATTED_SIZE_DECIMAL').value
ISIZE_FORMATTED_SIZE_DECIMAL = c_size_t.in_dll(LIB, 'LEXICAL_ISIZE_FORMATTED_SIZE_DECIMAL').value
U8_FORMATTED_SIZE_DECIMAL = c_size_t.in_dll(LIB, 'LEXICAL_U8_FORMATTED_SIZE_DECIMAL').value
U16_FORMATTED_SIZE_DECIMAL = c_size_t.in_dll(LIB, 'LEXICAL_U16_FORMATTED_SIZE_DECIMAL').value
U32_FORMATTED_SIZE_DECIMAL = c_size_t.in_dll(LIB, 'LEXICAL_U32_FORMATTED_SIZE_DECIMAL').value
U64_FORMATTED_SIZE_DECIMAL = c_size_t.in_dll(LIB, 'LEXICAL_U64_FORMATTED_SIZE_DECIMAL').value
U128_FORMATTED_SIZE_DECIMAL = c_size_t.in_dll(LIB, 'LEXICAL_U128_FORMATTED_SIZE_DECIMAL').value
USIZE_FORMATTED_SIZE_DECIMAL = c_size_t.in_dll(LIB, 'LEXICAL_USIZE_FORMATTED_SIZE_DECIMAL').value
F32_FORMATTED_SIZE_DECIMAL = c_size_t.in_dll(LIB, 'LEXICAL_F32_FORMATTED_SIZE_DECIMAL').value
F64_FORMATTED_SIZE_DECIMAL = c_size_t.in_dll(LIB, 'LEXICAL_F64_FORMATTED_SIZE_DECIMAL').value

BUFFER_SIZE = c_size_t.in_dll(LIB, 'LEXICAL_BUFFER_SIZE').value

# TYPES
# -----

# ERROR

class ErrorCode(enum.Enum):
    '''Error code, indicating failure type.'''

    Overflow = -1
    Underflow = -2
    InvalidDigit = -3
    Empty = -4
    EmptyMantissa = -5
    EmptyExponent = -6
    EmptyInteger = -7
    EmptyFraction = -8
    InvalidPositiveMantissaSign = -9
    MissingMantissaSign = -10
    InvalidExponent = -11
    InvalidPositiveExponentSign = -12
    MissingExponentSign = -13
    ExponentWithoutFraction = -14
    InvalidLeadingZeros = -15

class Error(Structure):
    '''C-compatible error for FFI.'''

    _fields_ = [
        ("_code", c_int),
        ("index", c_size_t)
    ]

    @property
    def code(self):
        return ErrorCode(self._code)

    @code.setter
    def code(self, value):
        if not isinstance(value, ErrorCode):
            raise TypeError('Expected ErrorCode')
        self._code = value.value

    def is_overflow(self):
        return self.code == ErrorCode.Overflow

    def is_underflow(self):
        return self.code == ErrorCode.Underflow

    def is_invalid_digit(self):
        return self.code == ErrorCode.InvalidDigit

    def is_empty(self):
        return self.code == ErrorCode.Empty

    def is_empty_mantissa(self):
        return self.code == ErrorCode.EmptyMantissa

    def is_empty_exponent(self):
        return self.code == ErrorCode.EmptyExponent

    def is_empty_integer(self):
        return self.code == ErrorCode.EmptyInteger

    def is_empty_fraction(self):
        return self.code == ErrorCode.EmptyFraction

    def is_invalid_positive_mantissa_sign(self):
        return self.code == ErrorCode.InvalidPositiveMantissaSign

    def is_missing_mantissa_sign(self):
        return self.code == ErrorCode.MissingMantissaSign

    def is_invalid_exponent(self):
        return self.code == ErrorCode.InvalidExponent

    def is_invalid_positive_exponent_sign(self):
        return self.code == ErrorCode.InvalidPositiveExponentSign

    def is_missing_exponent_sign(self):
        return self.code == ErrorCode.MissingExponentSign

    def is_exponent_without_fraction(self):
        return self.code == ErrorCode.ExponentWithoutFraction

    def is_invalid_leading_zeros(self):
        return self.code == ErrorCode.InvalidLeadingZeros

class LexicalError(Exception):
    '''Python-native exception raised during errors in lexical parsing.'''

    def __init__(self, error):
        self.error = error

    def __repr__(self):
        code = self.error.code
        if code == ErrorCode.Overflow:
            return 'Numeric overflow occurred at index {}'.format(self.error.index)
        elif code == ErrorCode.Underflow:
            return 'Numeric underflow occurred at index {}'.format(self.error.index)
        elif code == ErrorCode.InvalidDigit:
            return 'Invalid digit found at index {}'.format(self.error.index)
        elif code == ErrorCode.Empty:
            return 'Empty input found, starting at index {}'.format(self.error.index)
        elif code == ErrorCode.EmptyMantissa:
            return 'Empty mantissa found, starting at index {}'.format(self.error.index)
        elif code == ErrorCode.EmptyExponent:
            return 'Empty exponent found, starting at index {}'.format(self.error.index)
        elif code == ErrorCode.EmptyInteger:
            return 'Empty integer found, starting at index {}'.format(self.error.index)
        elif code == ErrorCode.EmptyFraction:
            return 'Empty fraction found, starting at index {}'.format(self.error.index)
        elif code == ErrorCode.InvalidPositiveMantissaSign:
            return 'Invalid "+" sign found for mantissa, starting at index {}'.format(self.error.index)
        elif code == ErrorCode.MissingMantissaSign:
            return 'Missing required sign for mantissa, starting at index {}'.format(self.error.index)
        elif code == ErrorCode.InvalidExponent:
            return 'Disallowed exponent was found, starting at index {}'.format(self.error.index)
        elif code == ErrorCode.InvalidPositiveExponentSign:
            return 'Invalid "+" sign found for exponent, starting at index {}'.format(self.error.index)
        elif code == ErrorCode.MissingExponentSign:
            return 'Missing required sign for exponent, starting at index {}'.format(self.error.index)
        elif code == ErrorCode.ExponentWithoutFraction:
            return 'Exponent found without fraction, starting at index {}'.format(self.error.index)
        elif code == ErrorCode.InvalidPositiveExponentSign:
            return 'Number was found with invalid leading zeros at index {}'.format(self.error.index)
        else:
            raise ValueError('Invalid ErrorCode for lexical error.')

# OPTIONS

def _set_exponent_char(builder, exponent_char, callback):
    return callback(builder, _to_c_ubyte(exponent_char))

def _set_format(builder, format, callback):
    return callback(builder, _to_type(format, NumberFormat))

def _set_lossy(builder, lossy, callback):
    return callback(builder, _to_type(lossy, c_bool))

def _set_radix(builder, radix, callback):
    return callback(builder, _to_type(radix, c_uint8))

def _set_rounding(builder, rounding, callback):
    return callback(builder, _to_type(rounding, RoundingKind))

def _set_string(builder, string, callback):
    return callback(builder, *_to_string(string))

def _set_trim_floats(builder, trim_floats, callback):
    return callback(builder, _to_type(trim_floats, c_bool))

# PARSE INTEGER OPTIONS

class _ParseIntegerOptionsBuilder(Structure):
    _fields_ = [
        ('_radix', c_uint8),
        ('_format', NumberFormat),
    ]

    def build(self):
        return LIB.lexical_parse_integer_options_build(self)

    if HAVE_RADIX:
        def radix(self, radix):
            return _set_radix(self, radix, LIB.lexical_parse_integer_options_builder_radix)

    if HAVE_FORMAT:
        def format(self, format):
            return _set_format(self, format, LIB.lexical_parse_integer_options_builder_format)


class ParseIntegerOptionsBuilder:
    '''Wrapper around _ParseIntegerOptionsBuilder'''

    def __init__(self, builder):
        self.builder = builder

    def build(self):
        return self.builder.build().into()

    if HAVE_RADIX:
        def radix(self, radix):
            self.builder = self.builder.radix(radix)
            return self

    if HAVE_FORMAT:
        def format(self, format):
            self.builder = self.builder.format(format)
            return self


class ParseIntegerOptions(Structure):
    '''Options to customize parsing integers.'''

    _fields_ = [
        ('_radix', c_uint32),
        ('_format', NumberFormat),
    ]

    @staticmethod
    def builder():
        return ParseIntegerOptionsBuilder(LIB.lexical_parse_integer_options_builder())

    if HAVE_RADIX:
        @staticmethod
        def binary():
            return ParseIntegerOptions.builder().radix(2).build()

        @staticmethod
        def decimal():
            return ParseIntegerOptions.builder().build()

        @staticmethod
        def hexadecimal():
            return ParseIntegerOptions.builder().radix(16).build()

    @property
    def radix(self):
        return self._radix

    @property
    def format(self):
        return self._format


_OptionParseIntegerOptions = _option(ParseIntegerOptions, '_OptionParseIntegerOptions')
LIB.lexical_parse_integer_options_builder.restype = _ParseIntegerOptionsBuilder
LIB.lexical_parse_integer_options_build.restype = _OptionParseIntegerOptions
if HAVE_RADIX:
    LIB.lexical_parse_integer_options_builder_radix.restype = _ParseIntegerOptionsBuilder
if HAVE_FORMAT:
    LIB.lexical_parse_integer_options_builder_format.restype = _ParseIntegerOptionsBuilder

# PARSE FLOAT OPTIONS

class _ParseFloatOptionsBuilder(Structure):
    _fields_ = [
        ('_lossy', c_bool),
        ('_exponent_char', c_uint8),
        ('_radix', c_uint8),
        ('_format', NumberFormat),
        ('_rounding', RoundingKind),
        ('_nan_string_ptr', POINTER(c_ubyte)),
        ('_nan_string_length', c_size_t),
        ('_inf_string_ptr', POINTER(c_ubyte)),
        ('_inf_string_length', c_size_t),
        ('_infinity_string_ptr', POINTER(c_ubyte)),
        ('_infinity_string_length', c_size_t),
    ]

    def build(self):
        return LIB.lexical_parse_float_options_build(self)

    def lossy(self, lossy):
        return _set_lossy(self, lossy, LIB.lexical_parse_float_options_builder_lossy)

    def exponent_char(self, exponent_char):
        return _set_exponent_char(self, exponent_char, LIB.lexical_parse_float_options_builder_exponent_char)

    def nan_string(self, nan_string):
        return _set_string(self, nan_string, LIB.lexical_parse_float_options_builder_nan_string)

    def inf_string(self, inf_string):
        return _set_string(self, inf_string, LIB.lexical_parse_float_options_builder_inf_string)

    def infinity_string(self, infinity_string):
        return _set_string(self, infinity_string, LIB.lexical_parse_float_options_builder_infinity_string)

    if HAVE_RADIX:
        def radix(self, radix):
            return _set_radix(self, radix, LIB.lexical_parse_float_options_builder_radix)

    if HAVE_FORMAT:
        def format(self, format):
            return _set_format(self, format, LIB.lexical_parse_float_options_builder_format)

    if HAVE_ROUNDING:
        def rounding(self, rounding):
            return _set_rounding(self, rounding, LIB.lexical_parse_float_options_builder_rounding)

class ParseFloatOptionsBuilder:
    '''Wrapper around _ParseFloatOptionsBuilder.'''

    def __init__(self, builder):
        # Store memos to ensure the pointers don't become invalid.
        self.builder = builder
        self._nan_string = None
        self._inf_string = None
        self._infinity_string = None

    def build(self):
        # Copy over memos to ensure pointers don't become invalid.
        result = ParseFloatOptions(self.builder.build().into())
        result._nan_string = self._nan_string
        result._inf_string = self._inf_string
        result._infinity_string = self._infinity_string
        return result

    def lossy(self, lossy):
        self.builder = self.builder.lossy(lossy)
        return self

    def exponent_char(self, exponent_char):
        self.builder = self.builder.exponent_char(exponent_char)
        return self

    def nan_string(self, nan_string):
        if isinstance(nan_string, str):
            nan_string = nan_string.encode('ascii')
        self._nan_string = nan_string
        self.builder = self.builder.nan_string(nan_string)
        return self

    def inf_string(self, inf_string):
        if isinstance(inf_string, str):
            inf_string = inf_string.encode('ascii')
        self._inf_string = inf_string
        self.builder = self.builder.inf_string(inf_string)
        return self

    def infinity_string(self, infinity_string):
        if isinstance(infinity_string, str):
            infinity_string = infinity_string.encode('ascii')
        self._infinity_string = infinity_string
        self.builder = self.builder.infinity_string(infinity_string)
        return self

    if HAVE_RADIX:
        def radix(self, radix):
            self.builder = self.builder.radix(radix)
            return self

    if HAVE_FORMAT:
        def format(self, format):
            self.builder = self.builder.format(format)
            return self

    if HAVE_ROUNDING:
        def rounding(self, rounding):
            self.builder = self.builder.rounding(rounding)
            return self

class _ParseFloatOptions(Structure):
    _fields_ = [
        ('_lossy', c_bool),
        ('_exponent_char', c_uint8),
        ('_radix', c_uint32),
        ('_format', NumberFormat),
        ('_rounding', RoundingKind),
        ('_nan_string_ptr', POINTER(c_ubyte)),
        ('_nan_string_length', c_size_t),
        ('_inf_string_ptr', POINTER(c_ubyte)),
        ('_inf_string_length', c_size_t),
        ('_infinity_string_ptr', POINTER(c_ubyte)),
        ('_infinity_string_length', c_size_t),
    ]

class ParseFloatOptions:
    '''Options to customize parsing floats.'''

    def __init__(self, options):
        # Store memos to ensure the pointers don't become invalid.
        self._options = options
        self._nan_string = None
        self._inf_string = None
        self._infinity_string = None

    @staticmethod
    def builder():
        return ParseFloatOptionsBuilder(LIB.lexical_parse_float_options_builder())

    if HAVE_RADIX:
        @staticmethod
        def binary():
            return ParseFloatOptions.builder().radix(2).build()

        @staticmethod
        def decimal():
            return ParseFloatOptions.builder().build()

        @staticmethod
        def hexadecimal():
            return ParseFloatOptions.builder().radix(16).exponent_char('p').build()

    @property
    def lossy(self):
        return self._options._lossy

    @property
    def exponent_char(self):
        return chr(self._options._exponent_char)

    @property
    def radix(self):
        return self._options._radix

    @property
    def format(self):
        return self._options._format

    @property
    def rounding(self):
        return self._options._rounding

    @property
    def nan_string(self):
        ptr = self._options._nan_string_ptr
        length = self._options._nan_string_length
        return string_at(ptr, length).decode('ascii')

    @property
    def inf_string(self):
        ptr = self._options._inf_string_ptr
        length = self._options._inf_string_length
        return string_at(ptr, length).decode('ascii')

    @property
    def infinity_string(self):
        ptr = self._options._infinity_string_ptr
        length = self._options._infinity_string_length
        return string_at(ptr, length).decode('ascii')

_OptionParseFloatOptions = _option(_ParseFloatOptions, '_OptionParseFloatOptions')
LIB.lexical_parse_float_options_builder.restype = _ParseFloatOptionsBuilder
LIB.lexical_parse_float_options_build.restype = _OptionParseFloatOptions
LIB.lexical_parse_float_options_builder_lossy.restype = _ParseFloatOptionsBuilder
LIB.lexical_parse_float_options_builder_exponent_char.restype = _ParseFloatOptionsBuilder
LIB.lexical_parse_float_options_builder_nan_string.restype = _ParseFloatOptionsBuilder
LIB.lexical_parse_float_options_builder_inf_string.restype = _ParseFloatOptionsBuilder
LIB.lexical_parse_float_options_builder_infinity_string.restype = _ParseFloatOptionsBuilder
if HAVE_RADIX:
    LIB.lexical_parse_float_options_builder_radix.restype = _ParseFloatOptionsBuilder
if HAVE_FORMAT:
    LIB.lexical_parse_float_options_builder_format.restype = _ParseFloatOptionsBuilder
if HAVE_ROUNDING:
    LIB.lexical_parse_float_options_builder_rounding.restype = _ParseFloatOptionsBuilder

# WRITE INTEGER OPTIONS

class _WriteIntegerOptionsBuilder(Structure):
    _fields_ = [
        ('_radix', c_uint8),
    ]

    def build(self):
        return LIB.lexical_write_integer_options_build(self)

    if HAVE_RADIX:
        def radix(self, radix):
            return _set_radix(self, radix, LIB.lexical_write_integer_options_builder_radix)


class WriteIntegerOptionsBuilder:
    '''Wrapper around _WriteIntegerOptionsBuilder'''

    def __init__(self, builder):
        self.builder = builder

    def build(self):
        return self.builder.build().into()

    if HAVE_RADIX:
        def radix(self, radix):
            self.builder = self.builder.radix(radix)
            return self

class WriteIntegerOptions(Structure):
    '''Options to customize writing integers.'''

    _fields_ = [
        ('_radix', c_uint32),
    ]

    @staticmethod
    def builder():
        return WriteIntegerOptionsBuilder(LIB.lexical_write_integer_options_builder())

    if HAVE_RADIX:
        @staticmethod
        def binary():
            return WriteIntegerOptions.builder().radix(2).build()

        @staticmethod
        def decimal():
            return WriteIntegerOptions.builder().build()

        @staticmethod
        def hexadecimal():
            return WriteIntegerOptions.builder().radix(16).build()

    @property
    def radix(self):
        return self._radix

_OptionWriteIntegerOptions = _option(WriteIntegerOptions, '_OptionWriteIntegerOptions')
LIB.lexical_write_integer_options_builder.restype = _WriteIntegerOptionsBuilder
LIB.lexical_write_integer_options_build.restype = _OptionWriteIntegerOptions
if HAVE_RADIX:
    LIB.lexical_write_integer_options_builder_radix.restype = _WriteIntegerOptionsBuilder

# WRITE FLOAT OPTIONS

class _WriteFloatOptionsBuilder(Structure):
    _fields_ = [
        ('_exponent_char', c_uint8),
        ('_radix', c_uint8),
        ('_trim_floats', c_bool),
        ('_nan_string_ptr', POINTER(c_ubyte)),
        ('_nan_string_length', c_size_t),
        ('_inf_string_ptr', POINTER(c_ubyte)),
        ('_inf_string_length', c_size_t),
    ]

    def build(self):
        return LIB.lexical_write_float_options_build(self)

    def exponent_char(self, exponent_char):
        return _set_exponent_char(self, exponent_char, LIB.lexical_write_float_options_builder_exponent_char)

    def trim_floats(self, trim_floats):
        return _set_trim_floats(self, trim_floats, LIB.lexical_write_float_options_builder_trim_floats)

    def nan_string(self, nan_string):
        return _set_string(self, nan_string, LIB.lexical_write_float_options_builder_nan_string)

    def inf_string(self, inf_string):
        return _set_string(self, inf_string, LIB.lexical_write_float_options_builder_inf_string)

    if HAVE_RADIX:
        def radix(self, radix):
            return _set_radix(self, radix, LIB.lexical_write_float_options_builder_radix)

class WriteFloatOptionsBuilder:
    '''Wrapper around _WriteFloatOptionsBuilder.'''

    def __init__(self, builder):
        # Store memos to ensure the pointers don't become invalid.
        self.builder = builder
        self._nan_string = None
        self._inf_string = None

    def build(self):
        # Copy over memos to ensure pointers don't become invalid.
        result = WriteFloatOptions(self.builder.build().into())
        result._nan_string = self._nan_string
        result._inf_string = self._inf_string
        return result

    def exponent_char(self, exponent_char):
        self.builder = self.builder.exponent_char(exponent_char)
        return self

    def trim_floats(self, trim_floats):
        self.builder = self.builder.trim_floats(trim_floats)
        return self

    def nan_string(self, nan_string):
        if isinstance(nan_string, str):
            nan_string = nan_string.encode('ascii')
        self._nan_string = nan_string
        self.builder = self.builder.nan_string(nan_string)
        return self

    def inf_string(self, inf_string):
        if isinstance(inf_string, str):
            inf_string = inf_string.encode('ascii')
        self._inf_string = inf_string
        self.builder = self.builder.inf_string(inf_string)
        return self

    if HAVE_RADIX:
        def radix(self, radix):
            self.builder = self.builder.radix(radix)
            return self

class _WriteFloatOptions(Structure):
    _fields_ = [
        ('_exponent_char', c_uint8),
        ('_radix', c_uint32),
        ('_trim_floats', c_bool),
        ('_nan_string_ptr', POINTER(c_ubyte)),
        ('_nan_string_length', c_size_t),
        ('_inf_string_ptr', POINTER(c_ubyte)),
        ('_inf_string_length', c_size_t),
    ]

class WriteFloatOptions:
    '''Options to customize writing floats.'''

    def __init__(self, options):
        # Store memos to ensure the pointers don't become invalid.
        self._options = options
        self._nan_string = None
        self._inf_string = None

    @staticmethod
    def builder():
        return WriteFloatOptionsBuilder(LIB.lexical_write_float_options_builder())

    if HAVE_RADIX:
        @staticmethod
        def binary():
            return WriteFloatOptions.builder().radix(2).build()

        @staticmethod
        def decimal():
            return WriteFloatOptions.builder().build()

        @staticmethod
        def hexadecimal():
            return WriteFloatOptions.builder().radix(16).exponent_char('p').build()

    @property
    def exponent_char(self):
        return chr(self._options._exponent_char)

    @property
    def radix(self):
        return self._options._radix

    @property
    def trim_floats(self):
        return self._options._trim_floats

    @property
    def nan_string(self):
        ptr = self._options._nan_string_ptr
        length = self._options._nan_string_length
        return string_at(ptr, length).decode('ascii')

    @property
    def inf_string(self):
        ptr = self._options._inf_string_ptr
        length = self._options._inf_string_length
        return string_at(ptr, length).decode('ascii')

_OptionWriteFloatOptions = _option(_WriteFloatOptions, '_OptionWriteFloatOptions')
LIB.lexical_write_float_options_builder.restype = _WriteFloatOptionsBuilder
LIB.lexical_write_float_options_build.restype = _OptionWriteFloatOptions
LIB.lexical_write_float_options_builder_exponent_char.restype = _WriteFloatOptionsBuilder
LIB.lexical_write_float_options_builder_trim_floats.restype = _WriteFloatOptionsBuilder
LIB.lexical_write_float_options_builder_nan_string.restype = _WriteFloatOptionsBuilder
LIB.lexical_write_float_options_builder_inf_string.restype = _WriteFloatOptionsBuilder
if HAVE_RADIX:
    LIB.lexical_write_float_options_builder_radix.restype = _WriteFloatOptionsBuilder

# RESULT TAG

class ResultTag(enum.Enum):
    '''Tag for the result tagged enum.'''

    Ok = 0
    Err = 1

# COMPLETE UNIONS

def _union(cls, name):
    class ResultUnion(Union):
        value_type = cls
        _fields_ = [
            ("value", cls),
            ("error", Error)
        ]

    ResultUnion.__name__ = name
    return ResultUnion

UnionI8 = _union(c_int8, 'UnionI8')
UnionI16 = _union(c_int16, 'UnionI16')
UnionI32 = _union(c_int32, 'UnionI32')
UnionI64 = _union(c_int64, 'UnionI64')
UnionIsize = _union(c_ssize_t, 'UnionIsize')
UnionU8 = _union(c_uint8, 'UnionU8')
UnionU16 = _union(c_uint16, 'UnionU16')
UnionU32 = _union(c_uint32, 'UnionU32')
UnionU64 = _union(c_uint64, 'UnionU64')
UnionUsize = _union(c_size_t, 'UnionUsize')
UnionF32 = _union(c_float, 'UnionF32')
UnionF64 = _union(c_double, 'UnionF64')

# COMPLETE RESULTS

def _result(cls, prefix, name):
    class Result(Structure):
        union_type = cls
        _fields_ = [
            ("_tag", c_uint),
            ("data", cls)
        ]

        @property
        def tag(self):
            return ResultTag(self._tag)

        @tag.setter
        def tag(self, value):
            if not isinstance(value, ResultTag):
                raise TypeError('Expected ResultTag')
            self._tag = value.value

        def into(self):
            '''Extract value from structure.'''

            if self.tag == ResultTag.Err:
                raise LexicalError(self.data.error)
            return self.data.value

    Result.__name__ = name
    return Result

ResultI8 = _result(UnionI8, 'i8', 'ResultI8')
ResultI16 = _result(UnionI16, 'i16', 'ResultI16')
ResultI32 = _result(UnionI32, 'i32', 'ResultI32')
ResultI64 = _result(UnionI64, 'i64', 'ResultI64')
ResultIsize = _result(UnionIsize, 'isize', 'ResultIsize')
ResultU8 = _result(UnionU8, 'u8', 'ResultU8')
ResultU16 = _result(UnionU16, 'u16', 'ResultU16')
ResultU32 = _result(UnionU32, 'u32', 'ResultU32')
ResultU64 = _result(UnionU64, 'u64', 'ResultU64')
ResultUsize = _result(UnionUsize, 'usize', 'ResultUsize')
ResultF32 = _result(UnionF32, 'f32', 'ResultF32')
ResultF64 = _result(UnionF64, 'f64', 'ResultF64')

# PARTIAL TUPLES

def _partial_tuple(cls, name):
    class Tuple(Structure):
        _fields_ = [
            ("x", cls),
            ("y", c_size_t)
        ]

        def into(self):
            '''Extract Python tuple from structure.'''
            return (self.x, self.y)

    Tuple.__name__ = name
    return Tuple

PartialTupleI8 = _partial_tuple(c_int8, 'PartialTupleI8')
PartialTupleI16 = _partial_tuple(c_int16, 'PartialTupleI16')
PartialTupleI32 = _partial_tuple(c_int32, 'PartialTupleI32')
PartialTupleI64 = _partial_tuple(c_int64, 'PartialTupleI64')
PartialTupleIsize = _partial_tuple(c_ssize_t, 'PartialTupleIsize')
PartialTupleU8 = _partial_tuple(c_uint8, 'PartialTupleU8')
PartialTupleU16 = _partial_tuple(c_uint16, 'PartialTupleU16')
PartialTupleU32 = _partial_tuple(c_uint32, 'PartialTupleU32')
PartialTupleU64 = _partial_tuple(c_uint64, 'PartialTupleU64')
PartialTupleUsize = _partial_tuple(c_size_t, 'PartialTupleUsize')
PartialTupleF32 = _partial_tuple(c_float, 'PartialTupleF32')
PartialTupleF64 = _partial_tuple(c_double, 'PartialTupleF64')

# PARTIAL UNIONS

def _partial_union(cls, name):
    return _union(cls, name)

PartialUnionI8 = _partial_union(PartialTupleI8, 'PartialUnionI8')
PartialUnionI16 = _partial_union(PartialTupleI16, 'PartialUnionI16')
PartialUnionI32 = _partial_union(PartialTupleI32, 'PartialUnionI32')
PartialUnionI64 = _partial_union(PartialTupleI64, 'PartialUnionI64')
PartialUnionIsize = _partial_union(PartialTupleIsize, 'PartialUnionIsize')
PartialUnionU8 = _partial_union(PartialTupleU8, 'PartialUnionU8')
PartialUnionU16 = _partial_union(PartialTupleU16, 'PartialUnionU16')
PartialUnionU32 = _partial_union(PartialTupleU32, 'PartialUnionU32')
PartialUnionU64 = _partial_union(PartialTupleU64, 'PartialUnionU64')
PartialUnionUsize = _partial_union(PartialTupleUsize, 'PartialUnionUsize')
PartialUnionF32 = _partial_union(PartialTupleF32, 'PartialUnionF32')
PartialUnionF64 = _partial_union(PartialTupleF64, 'PartialUnionF64')

# PARTIAL RESULTS

def _partial_result(cls, prefix, name):
    class PartialResult(Structure):
        union_type = cls
        _fields_ = [
            ("_tag", c_uint),
            ("data", cls)
        ]

        @property
        def tag(self):
            return ResultTag(self._tag)

        @tag.setter
        def tag(self, value):
            if not isinstance(value, ResultTag):
                raise TypeError('Expected ResultTag')
            self._tag = value.value

        def into(self):
            '''Extract value from structure.'''

            if self.tag == ResultTag.Err:
                raise LexicalError(self.data.error)
            return self.data.value.into()

    PartialResult.__name__ = name
    return PartialResult

PartialResultI8 = _partial_result(PartialUnionI8, 'i8', 'PartialResultI8')
PartialResultI16 = _partial_result(PartialUnionI16, 'i16', 'PartialResultI16')
PartialResultI32 = _partial_result(PartialUnionI32, 'i32', 'PartialResultI32')
PartialResultI64 = _partial_result(PartialUnionI64, 'i64', 'PartialResultI64')
PartialResultIsize = _partial_result(PartialUnionIsize, 'isize', 'PartialResultIsize')
PartialResultU8 = _partial_result(PartialUnionU8, 'u8', 'PartialResultU8')
PartialResultU16 = _partial_result(PartialUnionU16, 'u16', 'PartialResultU16')
PartialResultU32 = _partial_result(PartialUnionU32, 'u32', 'PartialResultU32')
PartialResultU64 = _partial_result(PartialUnionU64, 'u64', 'PartialResultU64')
PartialResultUsize = _partial_result(PartialUnionUsize, 'usize', 'PartialResultUsize')
PartialResultF32 = _partial_result(PartialUnionF32, 'f32', 'PartialResultF32')
PartialResultF64 = _partial_result(PartialUnionF64, 'f64', 'PartialResultF64')

# API
# ---

# WRITE

def _write(name, max_size, type, value):
    # Create buffer to write to.
    buffer_type = c_ubyte * max_size
    buffer = buffer_type()

    # Call C-API.
    cb = getattr(LIB, name)
    value = _to_type(value, type)
    first = _to_u8_ptr(buffer)
    last = _to_u8_ptr(_to_address(first) + len(buffer))
    ptr = cb(value, first, last)

    # Calculate length and decode.
    length = _distance(first, ptr)
    return _from_range(buffer, length).decode('ascii')

LIB.lexical_i8toa.restype = POINTER(c_ubyte)
LIB.lexical_i16toa.restype = POINTER(c_ubyte)
LIB.lexical_i32toa.restype = POINTER(c_ubyte)
LIB.lexical_i64toa.restype = POINTER(c_ubyte)
LIB.lexical_isizetoa.restype = POINTER(c_ubyte)
LIB.lexical_u8toa.restype = POINTER(c_ubyte)
LIB.lexical_u16toa.restype = POINTER(c_ubyte)
LIB.lexical_u32toa.restype = POINTER(c_ubyte)
LIB.lexical_u64toa.restype = POINTER(c_ubyte)
LIB.lexical_usizetoa.restype = POINTER(c_ubyte)
LIB.lexical_f32toa.restype = POINTER(c_ubyte)
LIB.lexical_f64toa.restype = POINTER(c_ubyte)

def i8toa(value):
    '''Format 8-bit signed integer to bytes'''
    return _write('lexical_i8toa', I8_FORMATTED_SIZE_DECIMAL, c_int8, value)

def i16toa(value):
    '''Format 16-bit signed integer to bytes'''
    return _write('lexical_i16toa', I16_FORMATTED_SIZE_DECIMAL, c_int16, value)

def i32toa(value):
    '''Format 32-bit signed integer to bytes'''
    return _write('lexical_i32toa', I32_FORMATTED_SIZE_DECIMAL, c_int32, value)

def i64toa(value):
    '''Format 64-bit signed integer to bytes'''
    return _write('lexical_i64toa', I64_FORMATTED_SIZE_DECIMAL, c_int64, value)

def isizetoa(value):
    '''Format ssize_t to bytes'''
    return _write('lexical_isizetoa', ISIZE_FORMATTED_SIZE_DECIMAL, c_ssize_t, value)

def u8toa(value):
    '''Format 8-bit unsigned integer to bytes'''
    return _write('lexical_u8toa', U8_FORMATTED_SIZE_DECIMAL, c_uint8, value)

def u16toa(value):
    '''Format 16-bit unsigned integer to bytes'''
    return _write('lexical_u16toa', U16_FORMATTED_SIZE_DECIMAL, c_uint16, value)

def u32toa(value):
    '''Format 32-bit unsigned integer to bytes'''
    return _write('lexical_u32toa', U32_FORMATTED_SIZE_DECIMAL, c_uint32, value)

def u64toa(value):
    '''Format 64-bit unsigned integer to bytes'''
    return _write('lexical_u64toa', U64_FORMATTED_SIZE_DECIMAL, c_uint64, value)

def usizetoa(value):
    '''Format size_t to bytes'''
    return _write('lexical_usizetoa', USIZE_FORMATTED_SIZE_DECIMAL, c_size_t, value)

def f32toa(value):
    '''Format 32-bit float to bytes'''
    return _write('lexical_f32toa', F32_FORMATTED_SIZE_DECIMAL, c_float, value)

def f64toa(value):
    '''Format 64-bit float to bytes'''
    return _write('lexical_f64toa', F64_FORMATTED_SIZE_DECIMAL, c_double, value)

# WRITE WITH OPTIONS

def _write_with_options(name, max_size, type, value, options):
    # Create buffer to write to.
    buffer_type = c_ubyte * max_size
    buffer = buffer_type()

    # Call C-API.
    cb = getattr(LIB, name)
    value = _to_type(value, type)
    options = getattr(options, '_options', options)
    first = _to_u8_ptr(buffer)
    last = _to_u8_ptr(_to_address(first) + len(buffer))
    ptr = cb(value, pointer(options), first, last)

    # Calculate length and decode.
    length = _distance(first, ptr)
    return _from_range(buffer, length).decode('ascii')

LIB.lexical_i8toa_with_options.restype = POINTER(c_ubyte)
LIB.lexical_i16toa_with_options.restype = POINTER(c_ubyte)
LIB.lexical_i32toa_with_options.restype = POINTER(c_ubyte)
LIB.lexical_i64toa_with_options.restype = POINTER(c_ubyte)
LIB.lexical_isizetoa_with_options.restype = POINTER(c_ubyte)
LIB.lexical_u8toa_with_options.restype = POINTER(c_ubyte)
LIB.lexical_u16toa_with_options.restype = POINTER(c_ubyte)
LIB.lexical_u32toa_with_options.restype = POINTER(c_ubyte)
LIB.lexical_u64toa_with_options.restype = POINTER(c_ubyte)
LIB.lexical_usizetoa_with_options.restype = POINTER(c_ubyte)
LIB.lexical_f32toa_with_options.restype = POINTER(c_ubyte)
LIB.lexical_f64toa_with_options.restype = POINTER(c_ubyte)

def i8toa_with_options(value, options):
    '''Format 8-bit signed integer to bytes with options.'''
    return _write_with_options('lexical_i8toa_with_options', I8_FORMATTED_SIZE, c_int8, value, options)

def i16toa_with_options(value, options):
    '''Format 16-bit signed integer to bytes with options.'''
    return _write_with_options('lexical_i16toa_with_options', I16_FORMATTED_SIZE, c_int16, value, options)

def i32toa_with_options(value, options):
    '''Format 32-bit signed integer to bytes with options.'''
    return _write_with_options('lexical_i32toa_with_options', I32_FORMATTED_SIZE, c_int32, value, options)

def i64toa_with_options(value, options):
    '''Format 64-bit signed integer to bytes with options.'''
    return _write_with_options('lexical_i64toa_with_options', I64_FORMATTED_SIZE, c_int64, value, options)

def isizetoa_with_options(value, options):
    '''Format ssize_t to bytes with options.'''
    return _write_with_options('lexical_isizetoa_with_options', ISIZE_FORMATTED_SIZE, c_ssize_t, value, options)

def u8toa_with_options(value, options):
    '''Format 8-bit unsigned integer to bytes with options.'''
    return _write_with_options('lexical_u8toa_with_options', U8_FORMATTED_SIZE, c_uint8, value, options)

def u16toa_with_options(value, options):
    '''Format 16-bit unsigned integer to bytes with options.'''
    return _write_with_options('lexical_u16toa_with_options', U16_FORMATTED_SIZE, c_uint16, value, options)

def u32toa_with_options(value, options):
    '''Format 32-bit unsigned integer to bytes with options.'''
    return _write_with_options('lexical_u32toa_with_options', U32_FORMATTED_SIZE, c_uint32, value, options)

def u64toa_with_options(value, options):
    '''Format 64-bit unsigned integer to bytes with options.'''
    return _write_with_options('lexical_u64toa_with_options', U64_FORMATTED_SIZE, c_uint64, value, options)

def usizetoa_with_options(value, options):
    '''Format size_t to bytes with options.'''
    return _write_with_options('lexical_usizetoa_with_options', USIZE_FORMATTED_SIZE, c_size_t, value, options)

def f32toa_with_options(value, options):
    '''Format 32-bit float to bytes with options.'''
    return _write_with_options('lexical_f32toa_with_options', F32_FORMATTED_SIZE, c_float, value, options)

def f64toa_with_options(value, options):
    '''Format 64-bit float to bytes with options.'''
    return _write_with_options('lexical_f64toa_with_options', F64_FORMATTED_SIZE, c_double, value, options)

# PARSE

def _parse(name, data):
    # Call C-API.
    cb = getattr(LIB, name)
    first, last = _to_range(_to_bytes(data))
    result = cb(first, last)
    return result.into()

# COMPLETE PARSE

LIB.lexical_atoi8.restype = ResultI8
LIB.lexical_atoi16.restype = ResultI16
LIB.lexical_atoi32.restype = ResultI32
LIB.lexical_atoi64.restype = ResultI64
LIB.lexical_atoisize.restype = ResultIsize
LIB.lexical_atou8.restype = ResultU8
LIB.lexical_atou16.restype = ResultU16
LIB.lexical_atou32.restype = ResultU32
LIB.lexical_atou64.restype = ResultU64
LIB.lexical_atousize.restype = ResultUsize
LIB.lexical_atof32.restype = ResultF32
LIB.lexical_atof64.restype = ResultF64

def atoi8(data):
    '''Parse 8-bit signed integer from input data.'''
    return _parse('lexical_atoi8', data)

def atoi16(data):
    '''Parse 16-bit signed integer from input data.'''
    return _parse('lexical_atoi16', data)

def atoi32(data):
    '''Parse 32-bit signed integer from input data.'''
    return _parse('lexical_atoi32', data)

def atoi64(data):
    '''Parse 64-bit signed integer from input data.'''
    return _parse('lexical_atoi64', data)

def atoisize(data):
    '''Parse ssize_t from input data.'''
    return _parse('lexical_atoisize', data)

def atou8(data):
    '''Parse 8-bit unsigned integer from input data.'''
    return _parse('lexical_atou8', data)

def atou16(data):
    '''Parse 16-bit unsigned integer from input data.'''
    return _parse('lexical_atou16', data)

def atou32(data):
    '''Parse 32-bit unsigned integer from input data.'''
    return _parse('lexical_atou32', data)

def atou64(data):
    '''Parse 64-bit unsigned integer from input data.'''
    return _parse('lexical_atou64', data)

def atousize(data):
    '''Parse size_t from input data.'''
    return _parse('lexical_atousize', data)

def atof32(data):
    '''Parse 32-bit float from input data.'''
    return _parse('lexical_atof32', data)

def atof64(data):
    '''Parse 64-bit float from input data.'''
    return _parse('lexical_atof64', data)

# PARTIAL PARSE

LIB.lexical_atoi8_partial.restype = PartialResultI8
LIB.lexical_atoi16_partial.restype = PartialResultI16
LIB.lexical_atoi32_partial.restype = PartialResultI32
LIB.lexical_atoi64_partial.restype = PartialResultI64
LIB.lexical_atoisize_partial.restype = PartialResultIsize
LIB.lexical_atou8_partial.restype = PartialResultU8
LIB.lexical_atou16_partial.restype = PartialResultU16
LIB.lexical_atou32_partial.restype = PartialResultU32
LIB.lexical_atou64_partial.restype = PartialResultU64
LIB.lexical_atousize_partial.restype = PartialResultUsize
LIB.lexical_atof32_partial.restype = PartialResultF32
LIB.lexical_atof64_partial.restype = PartialResultF64

def atoi8_partial(data):
    '''Parse 8-bit signed integer and the number of processed bytes from input data.'''
    return _parse('lexical_atoi8_partial', data)

def atoi16_partial(data):
    '''Parse 16-bit signed integer and the number of processed bytes from input data.'''
    return _parse('lexical_atoi16_partial', data)

def atoi32_partial(data):
    '''Parse 32-bit signed integer and the number of processed bytes from input data.'''
    return _parse('lexical_atoi32_partial', data)

def atoi64_partial(data):
    '''Parse 64-bit signed integer and the number of processed bytes from input data.'''
    return _parse('lexical_atoi64_partial', data)

def atoisize_partial(data):
    '''Parse ssize_t and the number of processed bytes from input data.'''
    return _parse('lexical_atoisize_partial', data)

def atou8_partial(data):
    '''Parse 8-bit unsigned integer and the number of processed bytes from input data.'''
    return _parse('lexical_atou8_partial', data)

def atou16_partial(data):
    '''Parse 16-bit unsigned integer and the number of processed bytes from input data.'''
    return _parse('lexical_atou16_partial', data)

def atou32_partial(data):
    '''Parse 32-bit unsigned integer and the number of processed bytes from input data.'''
    return _parse('lexical_atou32_partial', data)

def atou64_partial(data):
    '''Parse 64-bit unsigned integer and the number of processed bytes from input data.'''
    return _parse('lexical_atou64_partial', data)

def atousize_partial(data):
    '''Parse size_t and the number of processed bytes from input data.'''
    return _parse('lexical_atousize_partial', data)

def atof32_partial(data):
    '''Parse 32-bit float and the number of processed bytes from bytes.'''
    return _parse('lexical_atof32_partial', data)

def atof64_partial(data):
    '''Parse 64-bit float and the number of processed bytes from bytes.'''
    return _parse('lexical_atof64_partial', data)

# PARSE WITH OPTIONS

def _parse_with_options(name, data, options):
    if isinstance(data, str):
        data = data.encode('ascii')
    if not isinstance(data, (bytes, bytearray)):
        raise TypeError("Must parse from bytes.")
    cb = getattr(LIB, name)
    options = getattr(options, '_options', options)
    first = _to_u8_ptr(data)
    last = _to_u8_ptr(_to_address(first) + len(data))
    result = cb(first, last, pointer(options))
    return result.into()

# COMPLETE PARSE WITH OPTIONS

LIB.lexical_atoi8_with_options.restype = ResultI8
LIB.lexical_atoi16_with_options.restype = ResultI16
LIB.lexical_atoi32_with_options.restype = ResultI32
LIB.lexical_atoi64_with_options.restype = ResultI64
LIB.lexical_atoisize_with_options.restype = ResultIsize
LIB.lexical_atou8_with_options.restype = ResultU8
LIB.lexical_atou16_with_options.restype = ResultU16
LIB.lexical_atou32_with_options.restype = ResultU32
LIB.lexical_atou64_with_options.restype = ResultU64
LIB.lexical_atousize_with_options.restype = ResultUsize
LIB.lexical_atof32_with_options.restype = ResultF32
LIB.lexical_atof64_with_options.restype = ResultF64

def atoi8_with_options(data, options):
    '''Parse 8-bit signed integer from input data.'''
    return _parse_with_options('lexical_atoi8_with_options', data, options)

def atoi16_with_options(data, options):
    '''Parse 16-bit signed integer from input data.'''
    return _parse_with_options('lexical_atoi16_with_options', data, options)

def atoi32_with_options(data, options):
    '''Parse 32-bit signed integer from input data.'''
    return _parse_with_options('lexical_atoi32_with_options', data, options)

def atoi64_with_options(data, options):
    '''Parse 64-bit signed integer from input data.'''
    return _parse_with_options('lexical_atoi64_with_options', data, options)

def atoisize_with_options(data, options):
    '''Parse ssize_t from input data.'''
    return _parse_with_options('lexical_atoisize_with_options', data, options)

def atou8_with_options(data, options):
    '''Parse 8-bit unsigned integer from input data.'''
    return _parse_with_options('lexical_atou8_with_options', data, options)

def atou16_with_options(data, options):
    '''Parse 16-bit unsigned integer from input data.'''
    return _parse_with_options('lexical_atou16_with_options', data, options)

def atou32_with_options(data, options):
    '''Parse 32-bit unsigned integer from input data.'''
    return _parse_with_options('lexical_atou32_with_options', data, options)

def atou64_with_options(data, options):
    '''Parse 64-bit unsigned integer from input data.'''
    return _parse_with_options('lexical_atou64_with_options', data, options)

def atousize_with_options(data, options):
    '''Parse size_t from input data.'''
    return _parse_with_options('lexical_atousize_with_options', data, options)

def atof32_with_options(data, options):
    '''Parse 32-bit float from input data.'''
    return _parse_with_options('lexical_atof32_with_options', data, options)

def atof64_with_options(data, options):
    '''Parse 64-bit float from input data.'''
    return _parse_with_options('lexical_atof64_with_options', data, options)

# PARTIAL PARSE WITH OPTIONS

LIB.lexical_atoi8_partial_with_options.restype = PartialResultI8
LIB.lexical_atoi16_partial_with_options.restype = PartialResultI16
LIB.lexical_atoi32_partial_with_options.restype = PartialResultI32
LIB.lexical_atoi64_partial_with_options.restype = PartialResultI64
LIB.lexical_atoisize_partial_with_options.restype = PartialResultIsize
LIB.lexical_atou8_partial_with_options.restype = PartialResultU8
LIB.lexical_atou16_partial_with_options.restype = PartialResultU16
LIB.lexical_atou32_partial_with_options.restype = PartialResultU32
LIB.lexical_atou64_partial_with_options.restype = PartialResultU64
LIB.lexical_atousize_partial_with_options.restype = PartialResultUsize
LIB.lexical_atof32_partial_with_options.restype = PartialResultF32
LIB.lexical_atof64_partial_with_options.restype = PartialResultF64

def atoi8_partial_with_options(data, options):
    '''Parse 8-bit signed integer and the number of processed bytes from input data.'''
    return _parse_with_options('lexical_atoi8_partial_with_options', data, options)

def atoi16_partial_with_options(data, options):
    '''Parse 16-bit signed integer and the number of processed bytes from input data.'''
    return _parse_with_options('lexical_atoi16_partial_with_options', data, options)

def atoi32_partial_with_options(data, options):
    '''Parse 32-bit signed integer and the number of processed bytes from input data.'''
    return _parse_with_options('lexical_atoi32_partial_with_options', data, options)

def atoi64_partial_with_options(data, options):
    '''Parse 64-bit signed integer and the number of processed bytes from input data.'''
    return _parse_with_options('lexical_atoi64_partial_with_options', data, options)

def atoisize_partial_with_options(data, options):
    '''Parse ssize_t and the number of processed bytes from input data.'''
    return _parse_with_options('lexical_atoisize_partial_with_options', data, options)

def atou8_partial_with_options(data, options):
    '''Parse 8-bit unsigned integer and the number of processed bytes from input data.'''
    return _parse_with_options('lexical_atou8_partial_with_options', data, options)

def atou16_partial_with_options(data, options):
    '''Parse 16-bit unsigned integer and the number of processed bytes from input data.'''
    return _parse_with_options('lexical_atou16_partial_with_options', data, options)

def atou32_partial_with_options(data, options):
    '''Parse 32-bit unsigned integer and the number of processed bytes from input data.'''
    return _parse_with_options('lexical_atou32_partial_with_options', data, options)

def atou64_partial_with_options(data, options):
    '''Parse 64-bit unsigned integer and the number of processed bytes from input data.'''
    return _parse_with_options('lexical_atou64_partial_with_options', data, options)

def atousize_partial_with_options(data, options):
    '''Parse size_t and the number of processed bytes from input data.'''
    return _parse_with_options('lexical_atousize_partial_with_options', data, options)

def atof32_partial_with_options(data, options):
    '''Parse 32-bit float and the number of processed bytes from bytes.'''
    return _parse_with_options('lexical_atof32_partial_with_options', data, options)

def atof64_partial_with_options(data, options):
    '''Parse 64-bit float and the number of processed bytes from bytes.'''
    return _parse_with_options('lexical_atof64_partial_with_options', data, options)
