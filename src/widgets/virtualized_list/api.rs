use crate::widgets::virtualized_list::{Pos, VirtualizedList};
use iced::{Alignment, Element, Length, Pixels, advanced::Renderer, widget::Id};
use std::hash::Hash;

impl Pos {
    pub fn new(current_element: usize, offset: f32) -> Self {
        Self {
            current_element,
            offset,
        }
    }

    pub fn from_index(current_element: usize) -> Self {
        Self {
            current_element,
            offset: 0.,
        }
    }
}

impl<'elem, D, M, T, R, I> VirtualizedList<'elem, D, M, T, R, I>
where
    D: Hash + 'elem,
    R: Renderer,
    I: IntoIterator<
            IntoIter: Iterator<Item = D> + DoubleEndedIterator + ExactSizeIterator,
            Item = D,
        > + Clone,
{
    pub fn new(db: I, get_elem: fn(D) -> Element<'elem, M, T, R>) -> Self {
        Self {
            id: None,
            db,
            get_elem,
            on_scroll: None,
            is_vertical: true,
            spacing: 0.,
            width: Length::Shrink,
            height: Length::Fill,
            align: Alignment::Start,
            speed_scroll: 60.,
            cash_elem: Default::default(),
        }
    }

    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    pub fn align(mut self, align: impl Into<Alignment>) -> Self {
        self.align = align.into();
        self
    }

    pub fn spacing(mut self, spacing: impl Into<Pixels>) -> Self {
        self.spacing = spacing.into().0;
        self
    }

    pub fn vertical(mut self) -> Self {
        self.is_vertical = true;
        self
    }

    pub fn horizontal(mut self) -> Self {
        self.is_vertical = false;
        self
    }

    pub fn on_scroll(mut self, on_scroll: impl Fn(Pos) -> M + 'elem) -> Self {
        self.on_scroll = Some(Box::new(on_scroll) as _);
        self
    }

    pub fn set_id(mut self, id: Id) -> Self {
        self.id = Some(id);
        self
    }
}
