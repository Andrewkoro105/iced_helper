use crate::widgets::{
    scrollbar,
    virtualized_list::{Pos, ScrollBarState, VirtualizedList},
};
use iced::{Alignment, Element, Length, Pixels, advanced::{Renderer, Widget}, widget::Id};
use std::{hash::Hash, marker::PhantomData};

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

pub fn virtualized_list<'elem, D, M, T, R, I>(
    db: I,
    get_elem: fn(D) -> Element<'elem, M, T, R>,
) -> VirtualizedList<'elem, D, M, T, R, I, scrollbar::ScrollBar<'elem, M, T>, scrollbar::state::State>
where
    D: Hash + 'elem,
    M: 'elem + 'elem,
    R: Renderer + 'elem,
    I: IntoIterator<
            IntoIter: Iterator<Item = D> + DoubleEndedIterator + ExactSizeIterator,
            Item = D,
        > + Clone
        + 'elem,
    T: scrollbar::style::Catalog + 'elem,
{
    VirtualizedList::new(db, get_elem)
}

pub fn new_with_scrollbar<'elem, D, M, T, R, I, S, SBS>(
    db: I,
    get_elem: fn(D) -> Element<'elem, M, T, R>,
    scrollbar: S,
) -> VirtualizedList<'elem, D, M, T, R, I, S, SBS>
where
    D: Hash + 'elem,
    R: Renderer,
    I: IntoIterator<
            IntoIter: Iterator<Item = D> + DoubleEndedIterator + ExactSizeIterator,
            Item = D,
        > + Clone,
    S: Widget<M, T, R>,
    SBS: ScrollBarState,
{
    VirtualizedList::new_with_scrollbar(db, get_elem, scrollbar)
}

impl<'elem, D, M, T, R, I> VirtualizedList<'elem, D, M, T, R, I, scrollbar::ScrollBar<'elem, M, T>, scrollbar::state::State>
where
    D: Hash + 'elem,
    M: 'elem + 'elem,
    R: Renderer + 'elem,
    I: IntoIterator<
            IntoIter: Iterator<Item = D> + DoubleEndedIterator + ExactSizeIterator,
            Item = D,
        > + Clone
        + 'elem,
    T: scrollbar::style::Catalog + 'elem,
{
    pub fn new(db: I, get_elem: fn(D) -> Element<'elem, M, T, R>) -> Self {
        Self {
            id: None,
            db,
            get_elem,
            on_scroll: None,
            scrollbar: scrollbar::api::scrollbar(),
            gap: None,
            is_vertical: true,
            spacing: 0.,
            width: Length::Shrink,
            height: Length::Fill,
            align: Alignment::Start,
            speed_scroll: 60.,
            cash_elem: Default::default(),
            _phantom: PhantomData,
        }
    }
}

impl<'elem, D, M, T, R, I, S, SBS> VirtualizedList<'elem, D, M, T, R, I, S, SBS>
where
    D: Hash + 'elem,
    R: Renderer,
    I: IntoIterator<
            IntoIter: Iterator<Item = D> + DoubleEndedIterator + ExactSizeIterator,
            Item = D,
        > + Clone,
    S: Widget<M, T, R>,
    SBS: ScrollBarState,
{
    pub fn new_with_scrollbar(
        db: I,
        get_elem: fn(D) -> Element<'elem, M, T, R>,
        scrollbar: S,
    ) -> Self {
        Self {
            id: None,
            db,
            get_elem,
            on_scroll: None,
            scrollbar,
            gap: None,
            is_vertical: true,
            spacing: 0.,
            width: Length::Shrink,
            height: Length::Fill,
            align: Alignment::Start,
            speed_scroll: 60.,
            cash_elem: Default::default(),
            _phantom: PhantomData,
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

    pub fn gap(mut self, gap: f32) -> Self {
        self.gap = Some(gap);
        self
    }
}
