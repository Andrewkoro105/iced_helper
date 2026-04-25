//! Creates a guaranteed unique value at every point of use during the compilation phase

/// Creates a guaranteed unique value at every point of use during the compilation phase
#[macro_export]
macro_rules! unique_number {
    () => {{
        use std::any::TypeId;
        struct A {}
        TypeId::of::<A>()
    }};
}
