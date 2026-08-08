pub mod api;
pub mod operations;
mod utils;

use iced::{
    Alignment, Element, Event, Length, Rectangle, Size, Vector,
    advanced::{
        Clipboard, Layout, Renderer, Shell, Widget,
        layout::{Limits, Node},
        mouse,
        renderer::Style,
        widget::{Tree, tree},
    },
    mouse::Cursor,
    widget::Id,
};
use indexmap::IndexMap;
use std::hash::Hash;
use tracing::debug;

#[derive(Debug, Default, Clone, Copy)]
pub struct Pos {
    pub current_element: usize,
    pub offset: f32,
}

struct CashDataElement {
    hash: u64,
    tree: Tree,
    node: Node,
}

struct State {
    cash_elements: IndexMap<usize, CashDataElement>,
    cash_limits: Limits,
    pos: Pos,
    user_pos: Option<Pos>,
}

pub struct VirtualizedList<'elem, D, M, T, R, I>
where
    D: 'elem,
    R: Renderer,
    I: IntoIterator<
            IntoIter: Iterator<Item = D> + DoubleEndedIterator + ExactSizeIterator,
            Item = D,
        > + Clone,
{
    id: Option<Id>,
    db: I,
    get_elem: fn(D) -> Element<'elem, M, T, R>,
    on_scroll: Option<Box<dyn Fn(Pos) -> M + 'elem>>,
    is_vertical: bool,
    spacing: f32,
    width: Length,
    height: Length,
    align: Alignment,
    speed_scroll: f32,
    cash_elem: IndexMap<usize, Element<'elem, M, T, R>>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            cash_elements: Default::default(),
            pos: Default::default(),
            cash_limits: Limits::NONE,
            user_pos: Default::default(),
        }
    }
}

impl<'elem, D, M, T, R, I> From<VirtualizedList<'elem, D, M, T, R, I>> for Element<'elem, M, T, R>
where
    D: Hash + 'elem,
    M: 'elem,
    T: 'elem,
    R: Renderer + 'elem,
    I: IntoIterator<
            IntoIter: Iterator<Item = D> + DoubleEndedIterator + ExactSizeIterator,
            Item = D,
        > + Clone
        + 'elem,
{
    fn from(value: VirtualizedList<'elem, D, M, T, R, I>) -> Self {
        Self::new(value)
    }
}

impl<'elem, D, M, T, R, I> Widget<M, T, R> for VirtualizedList<'elem, D, M, T, R, I>
where
    D: Hash + 'elem,
    R: Renderer,
    I: IntoIterator<
            IntoIter: Iterator<Item = D> + DoubleEndedIterator + ExactSizeIterator,
            Item = D,
        > + Clone,
{
    fn state(&self) -> tree::State {
        tree::State::Some(Box::new(State::default()) as _)
    }

    fn size(&self) -> Size<Length> {
        Size {
            width: self.width,
            height: self.height,
        }
    }

    fn layout(&mut self, tree: &mut Tree, renderer: &R, limits: &Limits) -> Node {
        let state = tree.state.downcast_mut::<State>();
        if let Some(mut scroll) = state.user_pos {
            state.user_pos = None;
            let data = self
                .db
                .clone()
                .into_iter()
                .skip(scroll.current_element)
                .next();
            if let Some(data) = data {
                scroll.offset *= self.get_size_element(
                    scroll.current_element,
                    data,
                    renderer,
                    &state.cash_limits,
                );
                state.pos = scroll;
            }
        }
        self.layout_core(state, renderer, limits)
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut R,
        theme: &T,
        style: &Style,
        layout: Layout<'_>,
        cursor: Cursor,
        viewport: &Rectangle,
    ) {
        let state = tree.state.downcast_ref::<State>();
        debug!("view_count: {}", state.cash_elements.iter().len());

        state
            .cash_elements
            .iter()
            .map(|(_, elem)| {
                Layout::with_offset(
                    Vector::new(layout.position().x, layout.position().y),
                    &elem.node,
                )
            })
            .zip(&state.cash_elements)
            .for_each(|(layout, (i, _))| debug!("draw: i: {i}, bounds: {:?}", layout.bounds()));

        renderer.with_layer(layout.bounds(), |renderer| {
            state
                .cash_elements
                .iter()
                .zip(self.cash_elem.iter())
                .for_each(|((_, cash_elem), (_, elem))| {
                    elem.as_widget().draw(
                        &cash_elem.tree,
                        renderer,
                        theme,
                        style,
                        Layout::with_offset(
                            Vector::new(layout.position().x, layout.position().y),
                            &cash_elem.node,
                        ),
                        cursor,
                        viewport,
                    )
                });
        });
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: Cursor,
        renderer: &R,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, M>,
        viewport: &Rectangle,
    ) {
        let state = tree.state.downcast_mut::<State>();
        let updated = self.my_update(
            state, event, layout, cursor, renderer, clipboard, shell, viewport,
        );
        if updated {
            shell.capture_event();
        } else {
            state
                .cash_elements
                .iter_mut()
                .zip(self.cash_elem.iter_mut())
                .for_each(|((_, data), (_, elem))| {
                    let layout = Layout::with_offset(
                        Vector::new(layout.position().x, layout.position().y),
                        &data.node,
                    );
                    elem.as_widget_mut().update(
                        &mut data.tree,
                        event,
                        layout,
                        cursor,
                        renderer,
                        clipboard,
                        shell,
                        viewport,
                    );
                });
        }
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: Cursor,
        viewport: &Rectangle,
        renderer: &R,
    ) -> mouse::Interaction {
        let state = tree.state.downcast_ref::<State>();
        state
            .cash_elements
            .iter()
            .zip(self.cash_elem.iter())
            .map(|((_, data), (_, elem))| {
                elem.as_widget().mouse_interaction(
                    &data.tree,
                    Layout::with_offset(
                        Vector::new(layout.position().x, layout.position().y),
                        &data.node,
                    ),
                    cursor,
                    viewport,
                    renderer,
                )
            })
            .max()
            .unwrap_or_default()
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &R,
        operation: &mut dyn iced::advanced::widget::Operation,
    ) {
        let state = tree.state.downcast_mut::<State>();

        operation.custom(self.id.as_ref(), layout.bounds(), state);
        operation.traverse(&mut |operation| {
            state
                .cash_elements
                .iter_mut()
                .zip(self.cash_elem.iter_mut())
                .for_each(|((_, data), (_, elem))| {
                    elem.as_widget_mut().operate(
                        &mut data.tree,
                        Layout::with_offset(
                            Vector::new(layout.position().x, layout.position().y),
                            &data.node,
                        ),
                        renderer,
                        operation,
                    );
                });
        })
    }
}
