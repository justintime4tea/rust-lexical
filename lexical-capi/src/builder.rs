//! Macro to define a shared builder.

/// Create a complete builder from a single definition.
macro_rules! create_builder {
    // Main API.
    (
        fn $builder_fn:ident -> $type:ident;
        fn $build:ident;
    ) => (
        // New builder.
        #[doc(hidden)]
        #[no_mangle]
        pub unsafe extern fn $builder_fn()
            -> <lexical_core::$type as lexical_core::Buildable>::Builder
        {
            <lexical_core::$type as lexical_core::Buildable>::builder()
        }

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
