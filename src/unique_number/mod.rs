#[macro_export]
macro_rules! unique_number {
    () => {{
        use std::any::TypeId;
        struct A {}
        TypeId::of::<A>()
    }};
}
