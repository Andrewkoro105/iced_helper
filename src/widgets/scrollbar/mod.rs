pub mod api;
pub mod state;
pub mod style;

use crate::widgets::scrollbar::{state::State, style::Status};
use iced::{
    Element, Length, Rectangle, Size,
    advanced::{
        Layout, Renderer, Widget,
        layout::{Limits, Node},
        renderer::{self, Quad},
        widget::{Tree, tree},
    },
    mouse::Cursor,
};

pub struct ScrollBar<'elem, M, T: style::Catalog> {
    on_scroll: Option<Box<dyn Fn(f32) -> M + 'elem>>,
    mut_on_scroll: Option<Box<dyn FnMut(f32) + 'elem>>,
    style: T::Class<'elem>,
    is_vertical: bool,
    width: f32,
    base_view: f32,
}

impl<'elem, M: 'elem, T: style::Catalog + 'elem, R: Renderer> From<ScrollBar<'elem, M, T>>
    for Element<'elem, M, T, R>
{
    fn from(value: ScrollBar<'elem, M, T>) -> Self {
        Element::new(value)
    }
}

impl<'elem, M, T: style::Catalog, R: Renderer> Widget<M, T, R> for ScrollBar<'elem, M, T> {
    fn state(&self) -> tree::State {
        tree::State::Some(Box::new(State::new(self.base_view)))
    }

    fn size(&self) -> Size<Length> {
        if self.is_vertical {
            Size {
                width: Length::Fixed(self.width),
                height: Length::Fill,
            }
        } else {
            Size {
                width: Length::Fill,
                height: Length::Fixed(self.width),
            }
        }
    }

    fn layout(&mut self, _tree: &mut Tree, _renderer: &R, limits: &Limits) -> Node {
        if self.is_vertical {
            Node::new(Size::new(self.width, limits.max().height))
        } else {
            Node::new(Size::new(limits.max().width, self.width))
        }
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut R,
        theme: &T,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: Cursor,
        _viewport: &Rectangle,
    ) {
        let state = tree.state.downcast_ref::<State>();
        let base_bounds = layout.bounds();
        let scroller_bounds = state.get_scroller_bounds(self.is_vertical, &base_bounds);
        let style = theme.style(
            if state.offset.is_some() {
                Status::Dragged
            } else if state.is_focused {
                Status::Hovered
            } else {
                Status::Active
            },
            &self.style,
        );

        renderer.fill_quad(
            Quad {
                bounds: base_bounds,
                border: style.border,
                shadow: style.shadow,
                snap: style.snap,
            },
            style.background,
        );

        renderer.fill_quad(
            Quad {
                bounds: scroller_bounds,
                border: style.scroller_border,
                shadow: style.scroller_shadow,
                snap: style.scroller_snap,
            },
            style.scroller_background,
        );
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &iced::Event,
        layout: Layout<'_>,
        cursor: iced::advanced::mouse::Cursor,
        _renderer: &R,
        _clipboard: &mut dyn iced::advanced::Clipboard,
        shell: &mut iced::advanced::Shell<'_, M>,
        _viewport: &Rectangle,
    ) {
        let state = tree.state.downcast_mut::<State>();
        let base_bounds = layout.bounds();

        let scroller_bounds = state.get_scroller_bounds(self.is_vertical, &base_bounds);

        match event {
            iced::Event::Mouse(iced::mouse::Event::ButtonPressed(iced::mouse::Button::Left)) => {
                if let Some(position) = cursor.position() {
                    if cursor.is_over(scroller_bounds) {
                        state.set_offset(self.is_vertical, &base_bounds, position);
                    } else if cursor.is_over(base_bounds) {
                        state.set_pos(self.is_vertical, &base_bounds, position);
                        state.offset = Some(0.);
                        state.is_focused = true;
                        if let Some(on_scroll) = self.on_scroll.as_ref() {
                            shell.publish(on_scroll(state.pos));
                        }
                        if let Some(mut_on_scroll) = self.mut_on_scroll.as_mut() {
                            mut_on_scroll(state.pos);
                        }
                    }
                    shell.request_redraw();
                }
            }
            iced::Event::Mouse(iced::mouse::Event::CursorMoved { .. }) => {
                let is_focused = cursor.is_over(scroller_bounds);
                if state.is_focused != is_focused {
                    state.is_focused = is_focused;
                    shell.request_redraw();
                }
                if state.offset.is_some() {
                    if let Some(position) = cursor.position() {
                        state.set_pos(self.is_vertical, &base_bounds, position);
                        if let Some(on_scroll) = self.on_scroll.as_ref() {
                            shell.publish(on_scroll(state.pos));
                        }
                        if let Some(mut_on_scroll) = self.mut_on_scroll.as_mut() {
                            mut_on_scroll(state.pos);
                        }
                        shell.request_redraw();
                    }
                }
            }
            iced::Event::Mouse(iced::mouse::Event::ButtonReleased(iced::mouse::Button::Left)) => {
                state.offset = None;
                shell.request_redraw();
            }
            _ => {}
        }
    }
}
