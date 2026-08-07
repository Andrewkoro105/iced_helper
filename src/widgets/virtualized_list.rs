use iced::{
    Alignment, Element, Event, Length, Rectangle, Size, Vector,
    advanced::{
        Clipboard, Layout, Renderer, Shell, Widget,
        layout::{Limits, Node},
        mouse,
        renderer::Style,
        widget::{Tree, tree},
    },
    mouse::{Cursor, ScrollDelta},
};
use indexmap::IndexMap;
use std::{
    debug_assert_matches,
    hash::{DefaultHasher, Hash, Hasher},
};
use tracing::debug;

#[derive(Debug, Default)]
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
}

pub struct VirtualizedList<'elem, D, M, T, R, I, F>
where
    D: 'elem,
    R: Renderer,
    I: IntoIterator<Item = &'elem D> + Copy,
    F: Fn(&'elem D) -> Element<'elem, M, T, R>,
{
    db: I,
    get_elem: F,
    is_vertical: bool,
    spacing: f32,
    width: Length,
    height: Length,
    align: Alignment,
    cash_elem: IndexMap<usize, Element<'elem, M, T, R>>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            cash_elements: Default::default(),
            pos: Default::default(),
            cash_limits: Limits::NONE,
        }
    }
}

impl<'elem, D, M, T, R, I, F> VirtualizedList<'elem, D, M, T, R, I, F>
where
    D: Hash + 'elem,
    R: Renderer,
    I: IntoIterator<
            IntoIter: Iterator<Item = &'elem D> + DoubleEndedIterator + ExactSizeIterator,
            Item = &'elem D,
        > + Copy,
    F: Fn(&'elem D) -> Element<'elem, M, T, R>,
{
    pub fn new(db: I, get_elem: F) -> Self {
        Self {
            db,
            get_elem,
            is_vertical: true,
            spacing: 10.,
            width: Length::Shrink,
            height: Length::Fill,
            align: Alignment::Start,
            cash_elem: Default::default(),
        }
    }

    fn get_limits(&self, i: usize, size: Size<Length>, limits: &Limits) -> Limits {
        let ((min_height, max_height), (min_width, max_width)) = if self.is_vertical {
            debug_assert_matches!(
                size.height,
                Length::Shrink | Length::Fixed(_),
                "In vertical mode, elements passed to VirtualizedList can only have a vertical Length::Shrink | Length::Fixed(_) setting, not {:?} as in the case of element {i}.",
                size.height
            );
            debug_assert_matches!(
                size.width,
                Length::Shrink | Length::Fixed(_) | Length::Fill,
                "In vertical mode, elements passed to VirtualizedList can only have a horizontal Length::Shrink | Length::Fixed(_) | Length::Fill setting, not {:?} as in the case of element {i}.",
                size.width
            );

            (
                match size.height {
                    Length::Fixed(size) => (size, size),
                    _ => (0., f32::INFINITY),
                },
                match size.width {
                    Length::Fixed(size) => (size, size),
                    Length::Fill => (limits.max().width, limits.max().width),
                    _ => (limits.min().width, limits.max().width),
                },
            )
        } else {
            debug_assert_matches!(
                size.height,
                Length::Shrink | Length::Fixed(_) | Length::Fill,
                "In horizontal mode, elements passed to VirtualizedList can only have a vertical Length::Shrink | Length::Fixed(_) | Length::Fill setting, not {:?} as in the case of element {i}.",
                size.height
            );
            debug_assert_matches!(
                size.width,
                Length::Shrink | Length::Fixed(_),
                "In horizontal mode, elements passed to VirtualizedList can only have a horizontal Length::Shrink | Length::Fixed(_) setting, not {:?} as in the case of element {i}.",
                size.width
            );

            (
                match size.height {
                    Length::Fixed(size) => (size, size),
                    Length::Fill => (limits.max().height, limits.max().height),
                    _ => (limits.min().height, limits.max().height),
                },
                match size.width {
                    Length::Fixed(size) => (size, size),
                    _ => (0., f32::INFINITY),
                },
            )
        };
        Limits::new(
            Size::new(min_width, min_height),
            Size::new(max_width, max_height),
        )
    }

    fn get_node(
        &self,
        i: usize,
        widget: &mut dyn Widget<M, T, R>,
        children_size: &f32,
        tree: &mut Tree,
        renderer: &R,
        limits: &Limits,
    ) -> Node {
        let node = widget
            .layout(tree, renderer, &self.get_limits(i, widget.size(), limits))
            .align(
                if self.is_vertical {
                    self.align
                } else {
                    Alignment::Start
                },
                if self.is_vertical {
                    Alignment::Start
                } else {
                    self.align
                },
                limits.max(),
            )
            .translate(Vector {
                x: if self.is_vertical { 0. } else { *children_size },
                y: if self.is_vertical { *children_size } else { 0. },
            });
        node
    }

    fn get_size(&self, rectangle: Rectangle) -> f32 {
        (if self.is_vertical {
            rectangle.height
        } else {
            rectangle.width
        }) + self.spacing
    }

    fn get_element_and_node(
        &mut self,
        state: &mut State,
        i: usize,
        data: &'elem D,
        children_size: &mut f32,
        renderer: &R,
        limits: &Limits,
    ) -> (usize, CashDataElement, Element<'elem, M, T, R>) {
        let hash = {
            let mut hasher = DefaultHasher::new();
            data.hash(&mut hasher);
            hasher.finish()
        };

        if let Some((mut cash_data, mut elem)) = state
            .cash_elements
            .swap_remove(&i)
            .zip(self.cash_elem.swap_remove(&i))
        {
            if cash_data.hash == hash {
                cash_data.node = self.get_node(
                    i,
                    elem.as_widget_mut(),
                    children_size,
                    &mut cash_data.tree,
                    renderer,
                    limits,
                );
                *children_size += self.get_size(cash_data.node.bounds());
                return (i, cash_data, elem);
            }
        }

        let mut elem = (self.get_elem)(data);
        let mut tree = Tree::new(elem.as_widget_mut());
        let node = self.get_node(
            i,
            elem.as_widget_mut(),
            children_size,
            &mut tree,
            renderer,
            limits,
        );

        *children_size += self.get_size(node.bounds());

        (i, CashDataElement { hash, tree, node }, elem)
    }

    fn layout_core(&mut self, state: &mut State, renderer: &R, limits: &Limits) -> Node {
        debug!("layout: limits: {:?}", limits);
        state.cash_limits = *limits;
        (state.cash_elements, self.cash_elem) = self
            .db
            .into_iter()
            .enumerate()
            .skip(state.pos.current_element)
            .scan(-state.pos.offset, |children_size, (i, data)| {
                debug!("children_size ({i}): {:.2?}", children_size);
                if self.is_vertical {
                    *children_size <= limits.max().height
                } else {
                    *children_size <= limits.max().width
                }
                .then(|| {
                    let (i, cash_data_elem, cash_elem) =
                        self.get_element_and_node(state, i, data, children_size, renderer, limits);
                    ((i, cash_data_elem), (i, cash_elem))
                })
            })
            .unzip::<_, _, IndexMap<_, _>, IndexMap<_, _>>();

        debug!("result layout: count: {}", state.cash_elements.len());

        state.cash_elements.iter().for_each(|(i, elem)| {
            debug!("result layout: i: {i}, bounds: {:?}", elem.node.bounds())
        });

        Node::with_children(
            Size {
                width: limits.max().width,
                height: limits.max().height,
            },
            vec![],
        )
    }

    fn my_update(
        &mut self,
        state: &mut State,
        event: &Event,
        _layout: Layout<'_>,
        _cursor: Cursor,
        renderer: &R,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, M>,
        _viewport: &Rectangle,
    ) -> bool {
        let mut result = false;
        match event {
            Event::Mouse(iced::mouse::Event::WheelScrolled {
                delta: ScrollDelta::Lines { x, y },
            }) => {
                debug!("[=============================[update]===============================]");
                debug!("start pos: {:?}", state.pos);
                let x = x * -60.;
                let y = y * -60.;
                debug!("ScrollDelta::Pixels {{ x: {x}, y: {y} }}");
                state.pos.offset += if self.is_vertical { y } else { x };
                debug!("pre pos: {:?}", state.pos);
                if state.pos.offset < 0. {
                    if state.pos.current_element == 0 {
                        state.pos.offset = 0.;
                        self.layout_core(state, renderer, &state.cash_limits.clone());
                        shell.request_redraw();
                    } else {
                        self.db
                            .clone()
                            .into_iter()
                            .enumerate()
                            .rev()
                            .skip(self.db.clone().into_iter().len() - state.pos.current_element)
                            .try_for_each(|(i, data)| {
                                let mut elem = (self.get_elem)(data);
                                let mut tree = Tree::new(elem.as_widget());
                                let widget = elem.as_widget_mut();
                                let size = widget
                                    .layout(
                                        &mut tree,
                                        renderer,
                                        &self.get_limits(i, widget.size(), &state.cash_limits),
                                    )
                                    .bounds()
                                    .size();

                                state.pos.offset += (if self.is_vertical {
                                    size.height
                                } else {
                                    size.width
                                }) + self.spacing;
                                state.pos.current_element -= 1;

                                (state.pos.offset < 0.).then_some(())
                            });
                        self.layout_core(state, renderer, &state.cash_limits.clone());
                        shell.request_redraw();
                        result = true;
                    }
                } else {
                    if state.pos.current_element == (self.db.into_iter().len() - 1) {
                        state.pos.offset = 0.;
                        self.layout_core(state, renderer, &state.cash_limits.clone());
                        shell.request_redraw();
                    } else {
                        self.db
                            .clone()
                            .into_iter()
                            .enumerate()
                            .skip(state.pos.current_element)
                            .try_for_each(|(i, data)| {
                                let mut elem = (self.get_elem)(data);
                                let mut tree = Tree::new(elem.as_widget());
                                let widget = elem.as_widget_mut();
                                let size = widget
                                    .layout(
                                        &mut tree,
                                        renderer,
                                        &self.get_limits(i, widget.size(), &state.cash_limits),
                                    )
                                    .bounds()
                                    .size();

                                let size = (if self.is_vertical {
                                    size.height
                                } else {
                                    size.width
                                }) + self.spacing;

                                if state.pos.offset <= size {
                                    None
                                } else {
                                    state.pos.offset -= size;
                                    state.pos.current_element += 1;
                                    Some(())
                                }
                            });
                        self.layout_core(state, renderer, &state.cash_limits.clone());
                        shell.request_redraw();
                        result = true;
                    }
                }
                debug!("end pos: {:?}", state.pos);
            }
            _ => {}
        }
        result
    }
}

impl<'elem, D, M, T, R, I, F> From<VirtualizedList<'elem, D, M, T, R, I, F>>
    for Element<'elem, M, T, R>
where
    D: Hash + 'elem,
    M: 'elem,
    T: 'elem,
    R: Renderer + 'elem,
    I: IntoIterator<
            IntoIter: Iterator<Item = &'elem D> + DoubleEndedIterator + ExactSizeIterator,
            Item = &'elem D,
        > + Copy
        + 'elem,
    F: Fn(&'elem D) -> Element<'elem, M, T, R> + 'elem,
{
    fn from(value: VirtualizedList<'elem, D, M, T, R, I, F>) -> Self {
        Self::new(value)
    }
}

impl<'elem, D, M, T, R, I, F> Widget<M, T, R> for VirtualizedList<'elem, D, M, T, R, I, F>
where
    D: Hash + 'elem,
    R: Renderer,
    I: IntoIterator<
            IntoIter: Iterator<Item = &'elem D> + DoubleEndedIterator + ExactSizeIterator,
            Item = &'elem D,
        > + Copy,
    F: Fn(&'elem D) -> Element<'elem, M, T, R>,
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
        if !updated {
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
    }
}
