use iced::{Point, Rectangle};

use crate::widgets::virtualized_list::ScrollBarState;

#[derive(Debug)]
pub struct State {
    pub pos: f32,
    pub view: Option<f32>,
    pub offset: Option<f32>,
    pub is_focused: bool,
    pub base_view: f32,
}

impl State {
    pub fn new(base_view: f32) -> Self {
        Self {
            pos: 0.,
            view: None,
            offset: None,
            is_focused: false,
            base_view,
        }
    }

    pub fn get_view(&self) -> f32 {
        self.view.unwrap_or(self.base_view)
    }

    fn get_view_bound(&self, is_vertical: bool, mut base_bounds: Rectangle) -> Rectangle {
        if is_vertical {
            let view = self.get_view() * base_bounds.height;
            base_bounds.y += view / 2.;
            base_bounds.height -= view;
        } else {
            let view = self.get_view() * base_bounds.width;
            base_bounds.x += view / 2.;
            base_bounds.width -= view;
        }
        base_bounds
    }

    pub fn get_scroller_bounds(&self, is_vertical: bool, base_bounds: &Rectangle) -> Rectangle {
        let real_bounds = self.get_view_bound(is_vertical, *base_bounds);
        if is_vertical {
            let real_pos = real_bounds.y + (real_bounds.height * self.pos);
            let scroller_size = base_bounds.height * self.get_view();
            Rectangle {
                x: real_bounds.x,
                y: real_pos - scroller_size / 2.,
                width: real_bounds.width,
                height: scroller_size,
            }
        } else {
            let real_pos = real_bounds.x + (real_bounds.width * self.pos);
            let scroller_size = base_bounds.width * self.get_view();
            Rectangle {
                x: real_pos - scroller_size / 2.,
                y: real_bounds.y,
                width: scroller_size,
                height: real_bounds.height,
            }
        }
    }

    pub fn set_offset(&mut self, is_vertical: bool, base_bounds: &Rectangle, cursor: Point) {
        let base_bounds = self.get_view_bound(is_vertical, *base_bounds);
        self.offset = Some(if is_vertical {
            cursor.y - (base_bounds.y + (base_bounds.height * self.pos))
        } else {
            cursor.x - (base_bounds.x + (base_bounds.width * self.pos))
        });
    }

    pub fn set_pos(&mut self, is_vertical: bool, base_bounds: &Rectangle, cursor: Point) {
        let base_bounds = self.get_view_bound(is_vertical, *base_bounds);
        self.pos = if is_vertical {
            (cursor.y - base_bounds.y - self.offset.unwrap_or(0.)) / base_bounds.height
        } else {
            (cursor.x - base_bounds.x - self.offset.unwrap_or(0.)) / base_bounds.width
        }
        .clamp(0., 1.);
    }
}

impl ScrollBarState for State {
    fn get_pos(&self) -> f32 {
        self.pos
    }

    fn get_base_view(&self) -> f32 {
        self.base_view
    }

    fn set_pos_and_view(&mut self, pos: f32, view: Option<f32>) {
        self.pos = pos;
        self.view = view;
    }
}
