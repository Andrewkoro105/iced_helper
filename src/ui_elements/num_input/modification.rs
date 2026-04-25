/// Provides functions for converting values for display and back again
pub trait Modification<T> {
    fn to_base(value: &T) -> T;
    fn to_display(value: &T) -> T;
}

/// Implementation of `Modification<T>` without modifying values (possible only for types that implement `Copy`)
#[derive(Default, Debug, Clone)]
pub struct NullCast;

impl<T: Copy> Modification<T> for NullCast {
    fn to_base(value: &T) -> T {
        *value
    }

    fn to_display(value: &T) -> T {
        *value
    }
}

/// Implementation of `Modification<T>` without modifying values but by copying them (possible only for types that implement `Clone` but not `Copy`)
#[derive(Default, Debug, Clone)]
pub struct CloneNullCast;

impl<T: Clone> Modification<T> for CloneNullCast {
    fn to_base(value: &T) -> T {
        value.clone()
    }

    fn to_display(value: &T) -> T {
        value.clone()
    }
}

/// An implementation of `Modification<T>` with a base value ranging from 0 to 1 and a displayed value ranging from 0 to 255 (rounded to the nearest tenth)
/// 
/// ```
/// use iced_helper::ui_elements::num_input::modification::ColorCast;
/// use crate::iced_helper::ui_elements::num_input::modification::Modification;
/// 
/// assert_eq!(0., ColorCast::to_base(&0.));
/// assert_eq!(1., ColorCast::to_base(&255.));
/// assert_eq!(0.5, ColorCast::to_base(&127.5));
/// 
/// assert_eq!(0., ColorCast::to_display(&0.));
/// assert_eq!(255., ColorCast::to_display(&1.));
/// assert_eq!(89.3, ColorCast::to_display(&0.35));
/// ```
#[derive(Default, Debug, Clone)]
pub struct ColorCast;

impl Modification<f32> for ColorCast {
    fn to_base(value: &f32) -> f32 {
        let result = *value / 255.;
        result
    }

    fn to_display(value: &f32) -> f32 {
        let result = (*value * 255. * 10.0).round() / 10.0;
        result
    }
}