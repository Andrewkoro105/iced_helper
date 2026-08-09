use crate::widgets::virtualized_list::{
    CashDataElement, ScrollBar, ScrollBarState, State, VirtualizedList,
};
use iced::{
    Alignment, Element, Event, Length, Rectangle, Size, Vector,
    advanced::{
        Clipboard, Layout, Renderer, Shell, Widget,
        layout::{Limits, Node},
        widget::Tree,
    },
    mouse::{Cursor, ScrollDelta},
};
use indexmap::IndexMap;
use std::debug_assert_matches;
use std::hash::{DefaultHasher, Hash, Hasher};
use tracing::debug;

impl<'elem, D, M, T, R, I, S, SBS> VirtualizedList<'elem, D, M, T, R, I, S, SBS>
where
    D: Hash + 'elem,
    R: Renderer,
    I: IntoIterator<
            IntoIter: Iterator<Item = D> + DoubleEndedIterator + ExactSizeIterator,
            Item = D,
        > + Clone,
    S: ScrollBar<'elem, M, T, R>,
    SBS: ScrollBarState,
{
    fn scroll_publish(
        &self,
        size: Option<f32>,
        renderer: &R,
        shell: &mut Shell<M>,
        state: &State,
        scrollbar_state: &mut SBS,
    ) {
        let mut pos = state.pos;
        pos.offset /= size.unwrap_or_else(|| {
            self.get_size_element(
                pos.current_element,
                self.db
                    .clone()
                    .into_iter()
                    .skip(pos.current_element)
                    .next()
                    .unwrap(),
                renderer,
                &state.cash_limits,
            )
        });
        let one_len = 1. / self.db.clone().into_iter().len() as f32;
        let view_len = self.cash_elem.len();
        let end_offset = state.end_offset
            / size.unwrap_or_else(|| {
                self.get_size_element(
                    pos.current_element,
                    self.db
                        .clone()
                        .into_iter()
                        .skip(pos.current_element)
                        .next()
                        .unwrap(),
                    renderer,
                    &state.cash_limits,
                )
            });
        scrollbar_state.set_pos_and_view(
            (pos.current_element as f32 * one_len) + (pos.offset * one_len),
            if view_len > 2 {
                ((view_len - 2) as f32 + pos.offset + end_offset) * one_len
            } else {
                0.02
            },
        );
        if let Some(on_scroll) = self.on_scroll.as_ref() {
            shell.publish((on_scroll)(pos))
        }
    }

    pub(super) fn get_limits(&self, i: usize, size: Size<Length>, limits: &Limits) -> Limits {
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

    pub(super) fn get_node(
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

    pub(super) fn get_size(&self, size: Size) -> f32 {
        (if self.is_vertical {
            size.height
        } else {
            size.width
        }) + self.spacing
    }

    pub(super) fn get_size_element(&self, i: usize, data: D, renderer: &R, limits: &Limits) -> f32 {
        let mut elem = (self.get_elem)(data);
        let mut tree = Tree::new(elem.as_widget());
        let widget = elem.as_widget_mut();
        self.get_size(
            widget
                .layout(
                    &mut tree,
                    renderer,
                    &self.get_limits(i, widget.size(), limits),
                )
                .bounds()
                .size(),
        )
    }

    fn get_element_and_node(
        &mut self,
        state: &mut State,
        i: usize,
        data: D,
        children_size: &mut f32,
        renderer: &R,
        limits: &Limits,
    ) -> (usize, CashDataElement, Element<'elem, M, T, R>) {
        let hash = {
            let mut hasher = DefaultHasher::new();
            data.hash(&mut hasher);
            self.get_elem.hash(&mut hasher);
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
                *children_size += self.get_size(cash_data.node.bounds().size());
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

        *children_size += self.get_size(node.bounds().size());

        (i, CashDataElement { hash, tree, node }, elem)
    }

    pub(super) fn layout_scrollbar(
        &mut self,
        tree: &mut Tree,
        renderer: &R,
        limits: &Limits,
    ) -> Node {
        let mut node = self.scrollbar.layout(tree, renderer, limits);
        if self.is_vertical {
            node.translate_mut(Vector::new(limits.max().width - node.bounds().width, 0.));
        } else {
            node.translate_mut(Vector::new(0., limits.max().height - node.bounds().height));
        }
        node
    }

    pub(super) fn layout_core(&mut self, state: &mut State, renderer: &R, limits: &Limits) -> Size {
        debug!("layout: limits: {:?}", limits);

        state.cash_limits = *limits;
        (state.cash_elements, self.cash_elem) = self
            .db
            .clone()
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
                    let old_children_size = *children_size;
                    let (i, cash_data_elem, cash_elem) =
                        self.get_element_and_node(state, i, data, children_size, renderer, limits);
                    state.end_offset = *children_size - old_children_size;
                    ((i, cash_data_elem), (i, cash_elem))
                })
            })
            .unzip::<_, _, IndexMap<_, _>, IndexMap<_, _>>();

        debug!("result layout: count: {}", state.cash_elements.len());

        state.cash_elements.iter().for_each(|(i, elem)| {
            debug!("result layout: i: {i}, bounds: {:?}", elem.node.bounds())
        });

        Size {
            width: limits.max().width,
            height: limits.max().height,
        }
    }

    pub(super) fn my_update(
        &mut self,
        state: &mut State,
        scrollbar_state: &mut SBS,
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
                let db_end = self.db.clone().into_iter().len() - 1;
                let x = x * self.speed_scroll;
                let y = y * self.speed_scroll;
                let scroll = -if self.is_vertical { y } else { x };
                debug!("ScrollDelta::Pixels: {scroll}");
                state.pos.offset += scroll;
                debug!("pre pos: {:?}", state.pos);
                if state.pos.offset < 0. {
                    if !(state.pos.current_element == 0 && state.pos.offset - scroll == 0.) {
                        if state.pos.current_element == 0 {
                            state.pos.offset = 0.;
                            self.scroll_publish(None, renderer, shell, state, scrollbar_state);
                        } else {
                            self.db
                                .clone()
                                .into_iter()
                                .enumerate()
                                .rev()
                                .skip(self.db.clone().into_iter().len() - state.pos.current_element)
                                .try_for_each(|(i, data)| {
                                    let size = self.get_size_element(
                                        i,
                                        data,
                                        renderer,
                                        &state.cash_limits,
                                    );
                                    state.pos.offset += size;
                                    state.pos.current_element -= 1;

                                    self.scroll_publish(
                                        Some(size),
                                        renderer,
                                        shell,
                                        state,
                                        scrollbar_state,
                                    );

                                    (state.pos.offset < 0.).then_some(())
                                });
                        }
                        result = true;
                    } else {
                        state.pos.offset -= scroll;
                    }
                } else {
                    if !(state.pos.current_element == db_end && state.pos.offset - scroll == 0.) {
                        if state.pos.current_element >= db_end {
                            state.pos.current_element = db_end;
                            state.pos.offset = 0.;
                            self.scroll_publish(None, renderer, shell, state, scrollbar_state);
                        } else {
                            self.db
                                .clone()
                                .into_iter()
                                .enumerate()
                                .skip(state.pos.current_element)
                                .try_for_each(|(i, data)| {
                                    let size = self.get_size_element(
                                        i,
                                        data,
                                        renderer,
                                        &state.cash_limits,
                                    );

                                    if state.pos.offset <= size {
                                        None
                                    } else {
                                        state.pos.current_element += 1;
                                        if state.pos.current_element == db_end {
                                            state.pos.offset = 0.;
                                        } else {
                                            state.pos.offset -= size;
                                        }
                                        self.scroll_publish(
                                            Some(size),
                                            renderer,
                                            shell,
                                            state,
                                            scrollbar_state,
                                        );
                                        Some(())
                                    }
                                });
                        }
                        result = true;
                    } else {
                        state.pos.offset -= scroll;
                    }
                }
                debug!("end pos: {:?}", state.pos);
            }
            _ => {}
        }

        if result {
            self.layout_core(state, renderer, &state.cash_limits.clone());
            shell.request_redraw();
            debug!("update result: {result}");
        }

        result
    }
}
