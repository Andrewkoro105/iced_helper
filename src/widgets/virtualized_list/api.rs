use crate::widgets::{
    scrollbar,
    virtualized_list::{Pos, ScrollBarState, VirtualizedList},
};
use iced::{
    Alignment, Element, Length, Pixels,
    advanced::{Renderer, Widget},
    widget::Id,
};
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
) -> VirtualizedList<
    'elem,
    D,
    (),
    (),
    M,
    T,
    R,
    I,
    scrollbar::ScrollBar<'elem, M, T>,
    scrollbar::state::State,
>
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
    VirtualizedList::new(db, (), (), panic_get_elem)
}

fn panic_get_elem<D, S, C, E>(_: D, _: S, _: C) -> E {
    panic!("No function has been specified for converting data into widgets")
}

impl<'elem, D, S, C, M, T, R, I>
    VirtualizedList<
        'elem,
        D,
        S,
        C,
        M,
        T,
        R,
        I,
        scrollbar::ScrollBar<'elem, M, T>,
        scrollbar::state::State,
    >
where
    D: Hash + 'elem,
    S: Hash + Copy +'elem,
    C: Copy + 'elem,
    M: 'elem + 'elem,
    R: Renderer + 'elem,
    I: IntoIterator<
            IntoIter: Iterator<Item = D> + DoubleEndedIterator + ExactSizeIterator,
            Item = D,
        > + Clone
        + 'elem,
    T: scrollbar::style::Catalog + 'elem,
{
    pub fn new(
        db: I,
        state: S,
        context: C,
        get_elem: fn(D, S, C) -> Element<'elem, M, T, R>,
    ) -> Self {
        Self {
            id: None,
            db,
            state,
            context,
            get_elem,
            on_scroll: None,
            scrollbar: scrollbar::api::scrollbar(),
            max_scroller_size: 0.8,
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

impl<'elem, D, S, C, M, T, R, I, SB, SBS> VirtualizedList<'elem, D, S, C, M, T, R, I, SB, SBS>
where
    D: Hash + 'elem,
    S: Hash + Copy +'elem,
    C: Copy + 'elem,
    R: Renderer,
    I: IntoIterator<
            IntoIter: Iterator<Item = D> + DoubleEndedIterator + ExactSizeIterator,
            Item = D,
        > + Clone,
    SB: Widget<M, T, R>,
    SBS: ScrollBarState,
{
    pub fn scrollbar<NSB: Widget<M, T, R>, NSBS: ScrollBarState>(
        self,
        scrollbar: NSB,
    ) -> VirtualizedList<'elem, D, S, C, M, T, R, I, NSB, NSBS> {
        VirtualizedList {
            id: self.id,
            db: self.db,
            state: self.state,
            context: self.context,
            get_elem: self.get_elem,
            on_scroll: self.on_scroll,
            scrollbar,
            max_scroller_size: self.max_scroller_size,
            gap: self.gap,
            is_vertical: self.is_vertical,
            spacing: self.spacing,
            width: self.width,
            height: self.height,
            align: self.align,
            speed_scroll: self.speed_scroll,
            cash_elem: self.cash_elem,
            _phantom: PhantomData,
        }
    }

    pub fn state<NS: Hash + Copy + 'elem>(
        self,
        state: NS,
    ) -> VirtualizedList<'elem, D, NS, C, M, T, R, I, SB, SBS> {
        debug_assert!(panic_get_elem::<D, S, C, Element<'elem, M, T, R>> as *const () == self.get_elem as *const (), "You cannot use `VirtualizedList::state()` if you have already specified a function to convert data into widgets.");
        VirtualizedList {
            id: self.id,
            db: self.db,
            state: state,
            context: self.context,
            get_elem: panic_get_elem,
            on_scroll: self.on_scroll,
            scrollbar: self.scrollbar,
            max_scroller_size: self.max_scroller_size,
            gap: self.gap,
            is_vertical: self.is_vertical,
            spacing: self.spacing,
            width: self.width,
            height: self.height,
            align: self.align,
            speed_scroll: self.speed_scroll,
            cash_elem: self.cash_elem,
            _phantom: PhantomData,
        }
    }

    pub fn context<NC: Copy + 'elem>(
        self,
        context: NC,
    ) -> VirtualizedList<'elem, D, S, NC, M, T, R, I, SB, SBS> {
        debug_assert!(panic_get_elem::<D, S, C, Element<'elem, M, T, R>> as *const () == self.get_elem as *const (), "You cannot use `VirtualizedList::context()` if you have already specified a function to convert data into widgets.");
        VirtualizedList {
            id: self.id,
            db: self.db,
            state: self.state,
            context: context,
            get_elem: panic_get_elem,
            on_scroll: self.on_scroll,
            scrollbar: self.scrollbar,
            max_scroller_size: self.max_scroller_size,
            gap: self.gap,
            is_vertical: self.is_vertical,
            spacing: self.spacing,
            width: self.width,
            height: self.height,
            align: self.align,
            speed_scroll: self.speed_scroll,
            cash_elem: self.cash_elem,
            _phantom: PhantomData,
        }
    }
    
    pub fn get_elem(mut self, get_elem: fn(D, S, C) -> Element<'elem, M, T, R>) -> Self {
        self.get_elem = get_elem;
        self
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

    pub fn gap(mut self, gap: impl Into<Pixels>) -> Self {
        self.gap = Some(gap.into().0);
        self
    }

    pub fn max_scroller_size(mut self, max_scroller_size: impl Into<Pixels>) -> Self {
        self.max_scroller_size = max_scroller_size.into().0;
        self
    }
}
