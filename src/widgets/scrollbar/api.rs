use crate::widgets::scrollbar::{ScrollBar, style};

pub fn scrollbar<'elem, M: 'elem, T: style::Catalog + 'elem>() -> ScrollBar<'elem, M, T> {
    ScrollBar::new()
}

impl<'elem, M: 'elem, T: style::Catalog + 'elem> ScrollBar<'elem, M, T> {
    pub fn new() -> Self {
        Self {
            on_scroll: None,
            mut_on_scroll: None,
            style: T::default(),
            is_vertical: true,
            width: 10.,
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

    pub fn on_scroll(mut self, on_scroll: impl Fn(f32) -> M + 'elem) -> Self {
        self.on_scroll = Some(Box::new(on_scroll));
        self
    }
}
