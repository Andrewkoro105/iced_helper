pub trait Modification<T> {
    fn to_num(value: &T) -> T;
    fn to_str(value: &T) -> T;
}

#[derive(Default, Debug, Clone)]
pub struct NullCast;

impl<T: Copy> Modification<T> for NullCast {
    fn to_num(value: &T) -> T {
        *value
    }

    fn to_str(value: &T) -> T {
        *value
    }
}

#[derive(Default, Debug, Clone)]
pub struct CloneNullCast;

impl<T: Clone> Modification<T> for CloneNullCast {
    fn to_num(value: &T) -> T {
        value.clone()
    }

    fn to_str(value: &T) -> T {
        value.clone()
    }
}

#[derive(Default, Debug, Clone)]
pub struct ColorCast;

impl Modification<f32> for ColorCast {
    fn to_num(value: &f32) -> f32 {
        let result = *value / 255.;
        result
    }

    fn to_str(value: &f32) -> f32 {
        let result = (*value * 255. * 10.0).round() / 10.0;
        result
    }
}