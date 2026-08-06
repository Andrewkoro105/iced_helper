use iced::{
    Alignment, Element, Event, Length, Rectangle, Size, Vector,
    advanced::{
        Clipboard, Layout, Renderer, Shell, Widget,
        layout::{Limits, Node},
        renderer::Style,
        widget::Tree,
    },
    mouse::{Cursor, ScrollDelta},
};
use indexmap::IndexMap;
use std::{
    debug_assert_matches,
    hash::{DefaultHasher, Hash, Hasher},
};
use tracing::debug;

#[derive(Debug)]
pub struct Pos {
    pub current_element: usize,
    pub offset: f32,
}

struct CashElement<'elem, M, T, R> {
    hash: u64,
    elem: Element<'elem, M, T, R>,
    tree: Tree,
}

struct State<'elem, M, T, R> {
    cash_elements: IndexMap<usize, CashElement<'elem, M, T, R>>,
    cash_limits: Limits,
}

pub struct VirtualizedList<'elem, D, M, T, R, I, F>
where
    D: 'elem,
    R: Renderer,
    I: IntoIterator<Item = &'elem D> + Copy,
    F: Fn(&'elem D) -> Element<'elem, M, T, R>,
{
    pos: Pos,
    db: I,
    get_elem: F,
    is_vertical: bool,
    spacing: f32,
    width: Length,
    height: Length,
    align: Alignment,
    state: State<'elem, M, T, R>,
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
    fn size(&self) -> Size<Length> {
        Size {
            width: self.width,
            height: self.height,
        }
    }

    fn layout(&mut self, _tree: &mut Tree, renderer: &R, limits: &Limits) -> Node {
        debug!("layout: limits: {:?}", limits);
        self.state.cash_limits = *limits;
        let (elements, nodes) = self
            .db
            .into_iter()
            .enumerate()
            .skip(self.pos.current_element)
            .scan(-self.pos.offset, |children_size, (i, data)| {
                debug!("children_size ({i}): {:.2?}", children_size);
                if self.is_vertical {
                    *children_size <= limits.max().height
                } else {
                    *children_size <= limits.max().width
                }
                .then(|| self.get_element_and_node(i, data, children_size, renderer, limits))
            })
            .unzip::<_, _, IndexMap<_, _>, Vec<_>>();

        self.state.cash_elements = elements;

        nodes
            .iter()
            .zip(&self.state.cash_elements)
            .for_each(|(node, (i, _))| debug!("result layout: i: {i}, bounds: {:?}", node.bounds()));

        Node::with_children(
            Size {
                width: limits.max().width,
                height: limits.max().height,
            },
            nodes,
        )
    }

    fn draw(
        &self,
        _tree: &Tree,
        renderer: &mut R,
        theme: &T,
        style: &Style,
        layout: Layout<'_>,
        cursor: Cursor,
        viewport: &Rectangle,
    ) {
        debug!("view_count: {}", self.state.cash_elements.iter().count());
        layout.children()
            .zip(&self.state.cash_elements)
            .for_each(|(layout, (i, _))| debug!("draw: i: {i}, bounds: {:?}", layout.bounds()));
        self.state
            .cash_elements
            .iter()
            .zip(layout.children())
            .for_each(|((_, cash_elem), layout)| {
                cash_elem.elem.as_widget().draw(
                    &cash_elem.tree,
                    renderer,
                    theme,
                    style,
                    layout,
                    cursor,
                    viewport,
                )
            });
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        _layout: Layout<'_>,
        _cursor: Cursor,
        renderer: &R,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, M>,
        _viewport: &Rectangle,
    ) {
        match event {
            Event::Mouse(iced::mouse::Event::WheelScrolled {
                delta: ScrollDelta::Lines { x, y },
            }) => {
                debug!("start pos: {:?}", self.pos);
                let x = x * -60.;
                let y = y * -60.;
                debug!("ScrollDelta::Pixels {{ x: {x}, y: {y} }}");
                self.pos.offset += if self.is_vertical { y } else { x };
                if self.pos.offset < 0. {
                    if self.pos.current_element == 0 {
                        self.pos.offset = 0.;
                    } else {
                        self.db
                            .clone()
                            .into_iter()
                            .enumerate()
                            .rev()
                            .skip(self.db.clone().into_iter().count() - self.pos.current_element)
                            .try_for_each(|(i, data)| {
                                let mut elem = (self.get_elem)(data);
                                let mut tree = Tree::new(elem.as_widget());
                                let widget = elem.as_widget_mut();
                                let size = widget
                                    .layout(
                                        &mut tree,
                                        renderer,
                                        &self.get_limits(i, widget.size(), &self.state.cash_limits),
                                    )
                                    .bounds()
                                    .size();

                                self.pos.offset += if self.is_vertical {
                                    size.height
                                } else {
                                    size.width
                                };
                                self.pos.current_element -= 1;

                                (self.pos.offset < 0.).then_some(())
                            });
                        debug!("self.layout(tree, renderer, &self.state.cash_limits.clone());");
                        self.layout(tree, renderer, &self.state.cash_limits.clone());
                        shell.request_redraw();
                    }
                } else {
                    if self.pos.current_element == (self.db.into_iter().count() - 1) {
                        self.pos.offset = 0.;
                    } else {
                        self.db
                            .clone()
                            .into_iter()
                            .enumerate()
                            .skip(self.pos.current_element)
                            .try_for_each(|(i, data)| {
                                let mut elem = (self.get_elem)(data);
                                let mut tree = Tree::new(elem.as_widget());
                                let widget = elem.as_widget_mut();
                                let size = widget
                                    .layout(
                                        &mut tree,
                                        renderer,
                                        &self.get_limits(i, widget.size(), &self.state.cash_limits),
                                    )
                                    .bounds()
                                    .size();

                                self.pos.offset = if self.is_vertical {
                                    size.height
                                } else {
                                    size.width
                                };
                                self.pos.current_element += 1;

                                (self.pos.offset < 0.).then_some(())
                            });
                        debug!("self.layout(tree, renderer, &self.state.cash_limits.clone());");
                        self.layout(tree, renderer, &self.state.cash_limits.clone());
                        shell.request_redraw();
                    }
                }
                debug!("end pos: {:?}", self.pos);
            }
            _ => {}
        }
    }
}

impl<'elem, D, M, T, R, I, F> VirtualizedList<'elem, D, M, T, R, I, F>
where
    D: Hash + 'elem,
    R: Renderer,
    I: IntoIterator<Item = &'elem D> + Copy,
    F: Fn(&'elem D) -> Element<'elem, M, T, R>,
{
    pub fn new(db: I, get_elem: F) -> Self {
        Self {
            pos: Pos {
                current_element: 5,
                offset: 20.,
            },
            db,
            get_elem,
            is_vertical: true,
            spacing: 10.,
            state: State {
                cash_elements: Default::default(),
                cash_limits: Limits::NONE,
            },
            width: Length::Shrink,
            height: Length::Fill,
            align: Alignment::Start,
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
        children_size: &mut f32,
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

        *children_size += if self.is_vertical {
            node.bounds().height
        } else {
            node.bounds().width
        } + self.spacing;

        node
    }

    fn get_element_and_node(
        &mut self,
        i: usize,
        data: &'elem D,
        children_size: &mut f32,
        renderer: &R,
        limits: &Limits,
    ) -> ((usize, CashElement<'elem, M, T, R>), Node) {
        let hash = {
            let mut hasher = DefaultHasher::new();
            data.hash(&mut hasher);
            hasher.finish()
        };

        if let Some(mut cash_data) = self.state.cash_elements.swap_remove(&i) {
            if cash_data.hash == hash {
                let node = self.get_node(
                    i,
                    cash_data.elem.as_widget_mut(),
                    children_size,
                    &mut cash_data.tree,
                    renderer,
                    limits,
                );
                return ((i, cash_data), node);
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

        ((i, CashElement { hash, elem, tree }), node)
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
