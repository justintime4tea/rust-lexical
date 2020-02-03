//! Macro to define a shared builder.

/// Create a complete builder from a single definition.
macro_rules! create_builder {
    // All other fields.
    (@field
        $(#[$feature:meta])?
        fn $name:ident ($argname:ident : $argtype:ty) -> $type:ident
    ) => (
        #[doc(hidden)]
        #[no_mangle]
        $(#[$feature])?
        pub unsafe extern fn $name(
            builder: <lexical_core::$type as lexical_core::Buildable>::Builder,
            $argname: $argtype
        )
            -> <lexical_core::$type as lexical_core::Buildable>::Builder
        {
            builder.$argname($argname)
        }
    );

    // String field.
    // &'static [u8]
    (@string
        $(#[$feature:meta])?
        fn $name:ident ($argname:ident : $argtype:ty) -> $type:ident
    ) => (
        #[doc(hidden)]
        #[no_mangle]
        $(#[$feature])?
        pub unsafe extern fn $name(
            builder: <lexical_core::$type as lexical_core::Buildable>::Builder,
            string: *const u8,
            length: usize
        )
            -> <lexical_core::$type as lexical_core::Buildable>::Builder
        {
            let slc = slice::from_raw_parts(string, length);
            builder.$argname(slc)
        }
    );

    // Main API.
    (
        fn $builder_fn:ident -> $type:ident;
        fn $build:ident;
        $(
            @$path:ident
            $(#[$feature:meta])?
            fn $field_fn:ident ($argname:ident : $argtype:ty);
        )*
    ) => (
        // New builder.
        #[doc(hidden)]
        #[no_mangle]
        pub unsafe extern fn $builder_fn()
            -> <lexical_core::$type as lexical_core::Buildable>::Builder
        {
            <lexical_core::$type as lexical_core::Buildable>::builder()
        }

        // Fields.
        $(create_builder!{@$path
            $(#[$feature])?
            fn $field_fn ($argname : $argtype) -> $type
        })*

        // Build type.
        #[doc(hidden)]
        #[no_mangle]
        pub unsafe extern fn $build(builder: <lexical_core::$type as lexical_core::Buildable>::Builder)
            -> $crate::option::Option<lexical_core::$type>
        {
            type Builder = <lexical_core::$type as lexical_core::Buildable>::Builder;
            <Builder as lexical_core::Builder>::build(builder).into()
        }
    );
}
