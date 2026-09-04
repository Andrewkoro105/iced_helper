use iced::{Color, Theme};

pub trait Catalog: Sized {
    type Class<'a>;

    fn default<'a>() -> Self::Class<'a>;

    fn style(&self, item: &Self::Class<'_>) -> Style;
}

pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme) -> Style + 'a>;

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(|theme|
            Style {
                color: theme.extended_palette().primary.base.color,
            },
        )
    }

    fn style(&self, class: &Self::Class<'_>) -> Style {
        class(self)
    }
}

pub struct Style {
    pub color: Color,
}
