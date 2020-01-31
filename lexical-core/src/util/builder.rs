//! Traits that implement the builder pattern.

/// Trait for type builders.
pub trait Builder {
    type Buildable: Buildable;

    /// Consume builder and create type.
    fn build(self) -> Option<Self::Buildable>;
}

/// Trait for types that can be constructed through a builder.
pub trait Buildable {
    type Builder: Builder;

    /// Create builder to instantiate the class.
    fn builder() -> Self::Builder;
}
