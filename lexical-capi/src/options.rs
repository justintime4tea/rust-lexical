//! C-compatible option builders.

use lexical_core;

// TODO(ahuszagh) Restore later.
//  Going to need a builder trait.
///// Define function to create a new builder.
//macro_rules! new_builder {
//    (
//        fn $name:ident -> $builder:ident
//    ) => (
//        #[doc(hidden)]
//        #[no_mangle]
//        pub unsafe extern fn $name() -> lexical_core::$builder
//        {
//            lexical_core::ParseIntegerOptions::builder()
//        }
//    );
//}
//
///// Generate the parse integer options builder.
//#[doc(hidden)]
//#[no_mangle]
//pub unsafe extern fn parse_integer_options_builder()
//    -> lexical_core::ParseIntegerOptionsBuilder
//{
//    lexical_core::ParseIntegerOptions::builder()
//}
//
///// Add a radix to the parse integer options builder.
//#[doc(hidden)]
//#[no_mangle]
//#[cfg(feature = "radix")]
//pub unsafe extern fn parse_integer_options_builder_radix(
//    builder: lexical_core::ParseIntegerOptionsBuilder,
//    radix: u8
//)
//    -> lexical_core::ParseIntegerOptionsBuilder
//{
//    builder.radix(radix)
//}
//
///// Generate the parse float options builder.
//#[doc(hidden)]
//#[no_mangle]
//pub unsafe extern fn parse_float_options_builder()
//    -> lexical_core::ParseFloatOptionsBuilder
//{
//    lexical_core::ParseFloatOptions::builder()
//}
//
///// Generate the write integer options builder.
//#[doc(hidden)]
//#[no_mangle]
//pub unsafe extern fn write_integer_options_builder()
//    -> lexical_core::WriteIntegerOptionsBuilder
//{
//    lexical_core::WriteIntegerOptions::builder()
//}
//
///// Generate the write float options builder.
//#[doc(hidden)]
//#[no_mangle]
//pub unsafe extern fn write_float_options_builder()
//    -> lexical_core::WriteFloatOptionsBuilder
//{
//    lexical_core::WriteFloatOptions::builder()
//}


// TODO(ahuszagh) need to add radix, etc....
