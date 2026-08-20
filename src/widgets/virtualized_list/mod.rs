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
use std::{hash::Hash, marker::PhantomData};

pub trait ScrollBarState {
    fn get_pos(&self) -> f32;
    fn set_pos_and_view(&mut self, pos: f32, view: Option<f32>);
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Pos {
    pub current_element: usize,
    pub offset: f32,
}

struct CashDataElement {
    hash: u64,
    tree: Tree,
    node: Node,
}

pub struct State {
    cash_elements: IndexMap<usize, CashDataElement>,
    cash_limits: Limits,
    pos: Pos,
    end_offset: f32,
    user_pos: Option<Pos>,
}

pub struct VirtualizedList<'elem, D, M, T, R, I, SB, SBS>
where
    D: 'elem,
    R: Renderer,
    I: IntoIterator<
            IntoIter: Iterator<Item = D> + DoubleEndedIterator + ExactSizeIterator,
            Item = D,
        > + Clone,
    SB: Widget<M, T, R>,
    SBS: ScrollBarState,
{
    id: Option<Id>,
    db: I,
    get_elem: fn(D) -> Element<'elem, M, T, R>,
    on_scroll: Option<Box<dyn Fn(Pos) -> M + 'elem>>,
    scrollbar: SB,
    gap: Option<f32>,
    is_vertical: bool,
    spacing: f32,
    width: Length,
    height: Length,
    align: Alignment,
    speed_scroll: f32,
    cash_elem: IndexMap<usize, Element<'elem, M, T, R>>,
    _phantom: PhantomData<SBS>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            cash_elements: Default::default(),
            pos: Default::default(),
            end_offset: Default::default(),
            cash_limits: Limits::NONE,
            user_pos: Default::default(),
        }
    }
}

impl<'elem, D, M, T, R, I, S, SBS> From<VirtualizedList<'elem, D, M, T, R, I, S, SBS>>
    for Element<'elem, M, T, R>
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
    S: Widget<M, T, R> + 'elem,
    SBS: ScrollBarState + 'static,
{
    fn from(value: VirtualizedList<'elem, D, M, T, R, I, S, SBS>) -> Self {
        Self::new(value)
    }
}

impl<'elem, D, M, T, R, I, S, SBS> Widget<M, T, R> for VirtualizedList<'elem, D, M, T, R, I, S, SBS>
where
    D: Hash + 'elem,
    R: Renderer,
    I: IntoIterator<
            IntoIter: Iterator<Item = D> + DoubleEndedIterator + ExactSizeIterator,
            Item = D,
        > + Clone,
    S: Widget<M, T, R>,
    SBS: ScrollBarState + 'static,
{
    fn diff(&self, tree: &mut Tree) {
        self.scrollbar.diff(&mut tree.children[0]);
    }

    fn state(&self) -> tree::State {
        tree::State::Some(Box::new(State::default()) as _)
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.scrollbar as &dyn Widget<M, T, R>)]
    }

    fn size(&self) -> Size<Length> {
        Size {
            width: self.width,
            height: self.height,
        }
    }

    fn layout(&mut self, tree: &mut Tree, renderer: &R, limits: &Limits) -> Node {
        let scrollbar_node = self.layout_scrollbar(&mut tree.children[0], renderer, limits);
        let scrollbar_state = tree.children[0].state.downcast_mut::<SBS>();
        let mut limits = limits.clone();

        if let Some(gap) = self.gap {
            limits = if self.is_vertical {
                limits.shrink(Size::new(gap + scrollbar_node.bounds().width, 0.))
            } else {
                limits.shrink(Size::new(0., gap + scrollbar_node.bounds().height))
            };
        }

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

        Node::with_children(
            self.layout_core(state, scrollbar_state, renderer, &limits),
            vec![scrollbar_node],
        )
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
        self.scrollbar.draw(
            &tree.children[0],
            renderer,
            theme,
            style,
            layout.child(0),
            cursor,
            viewport,
        );

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
        let scrollbar_state = tree.children[0].state.downcast_mut::<SBS>();
        let updated = self.my_update(
            state,
            scrollbar_state,
            event,
            layout,
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        );

        self.scrollbar.update(
            &mut tree.children[0],
            event,
            layout.child(0),
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        );
        let scrollbar_state = tree.children[0].state.downcast_mut::<SBS>();
        let current_element =
            scrollbar_state.get_pos() * ((self.db.clone().into_iter().len() - 1) as f32);
        let old_pos = state.pos;
        state.pos.current_element = current_element.trunc() as _;
        state.pos.offset = current_element.fract()
            * self.get_size_element(
                state.pos.current_element,
                self.db
                    .clone()
                    .into_iter()
                    .skip(state.pos.current_element)
                    .next()
                    .unwrap(),
                renderer,
                &state.cash_limits,
            );
        if old_pos != state.pos {
            self.layout_core(state, scrollbar_state, renderer, &state.cash_limits.clone());
            shell.request_redraw();
        }

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
        });
    }
}
