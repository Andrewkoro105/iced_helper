use crate::widgets::scrollbar::{
    ScrollBar,
    style::{self, StyleFn},
};

pub fn scrollbar<'elem, M: 'elem, T: style::Catalog + 'elem>() -> ScrollBar<'elem, M, T> {
    ScrollBar::new()
}

impl<'elem, M: 'elem, T: style::Catalog + 'elem> ScrollBar<'elem, M, T> {
    pub fn new() -> Self {
        Self {
            on_scroll: None,
            mut_on_scroll: None,
            class: T::default(),
            is_vertical: true,
            width: 10.,
            base_view: 0.04,
        }
    }

    pub fn vertical(mut self) -> Self {
        self.is_vertical = true;
        self
    }

    pub fn horizontal(mut self) -> Self {
        self.is_vertical = false;
        self
    }

    pub fn base_view(mut self, base_view: f32) -> Self {
        self.base_view = base_view;
        self
    }

    pub fn on_scroll(mut self, on_scroll: impl Fn(f32) -> M + 'elem) -> Self {
        self.on_scroll = Some(Box::new(on_scroll));
        self
    }

    pub fn style(mut self, style: impl Fn(&T, style::Status) -> style::Style + 'elem) -> Self
    where
        <T as style::Catalog>::Class<'elem>: From<StyleFn<'elem, T>>,
    {
        self.class = (Box::new(style) as StyleFn<'elem, T>).into();
        self
    }

    pub fn class(mut self, class: impl Into<T::Class<'elem>>) -> Self
    {
        self.class = class.into();
        self
    }
}
