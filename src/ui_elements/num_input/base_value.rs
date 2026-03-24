pub trait BaseValue<T> {
    const VALUE: T;
}

#[derive(Default, Debug, Clone)]
pub struct ConstF32<const VALUE: i32>;

impl<const VALUE: i32> BaseValue<f32> for ConstF32<VALUE> {
    const VALUE: f32 = VALUE as _;
}

#[derive(Default, Debug, Clone)]
pub struct ConstF64<const VALUE: i64>;

impl<const VALUE: i64> BaseValue<f64> for ConstF64<VALUE> {
    const VALUE: f64 = VALUE as _;
}

#[derive(Default, Debug, Clone)]
pub struct ConstI32<const VALUE: i32>;

impl<const VALUE: i32> BaseValue<i32> for ConstI32<VALUE> {
    const VALUE: i32 = VALUE;
}

#[derive(Default, Debug, Clone)]
pub struct ConstI64<const VALUE: i64>;

impl<const VALUE: i64> BaseValue<i64> for ConstI64<VALUE> {
    const VALUE: i64 = VALUE;
}

#[derive(Default, Debug, Clone)]
pub struct ConstU32<const VALUE: u32>;

impl<const VALUE: u32> BaseValue<u32> for ConstU32<VALUE> {
    const VALUE: u32 = VALUE;
}

#[derive(Default, Debug, Clone)]
pub struct ConstU64<const VALUE: u64>;

impl<const VALUE: u64> BaseValue<u64> for ConstU64<VALUE> {
    const VALUE: u64 = VALUE;
}

#[derive(Default, Debug, Clone)]
pub struct ConstUSize<const VALUE: usize>;

impl<const VALUE: usize> BaseValue<usize> for ConstUSize<VALUE> {
    const VALUE: usize = VALUE;
}