use iced::{Point, Rectangle};

#[derive(Default, Debug)]
pub struct State {
    pub pos: f32,
    pub view: Option<f32>,
    pub offset: Option<f32>,
    pub is_focused: bool,
}

impl State {
    pub fn get_view(&self) -> f32 {
        self.view.unwrap_or(0.1)
    }

    pub fn get_scroller_bounds(&self, is_vertical: bool, base_bounds: &Rectangle) -> Rectangle {
        if is_vertical {
            let real_pos = base_bounds.y + (base_bounds.height * self.pos);
            let scroller_size = base_bounds.height * self.get_view();
            Rectangle {
                x: base_bounds.x,
                y: if (base_bounds.y + base_bounds.height) - real_pos < scroller_size / 2. {
                    (base_bounds.y + base_bounds.height) - scroller_size
                } else if base_bounds.height * self.pos < scroller_size / 2. {
                    base_bounds.y
                } else {
                    real_pos - scroller_size / 2.
                },
                width: base_bounds.width,
                height: scroller_size,
            }
        } else {
            let real_pos = base_bounds.x + (base_bounds.width * self.pos);
            let scroller_size = base_bounds.width * self.get_view();
            Rectangle {
                x: if (base_bounds.x + base_bounds.width) - real_pos < scroller_size / 2. {
                    (base_bounds.x + base_bounds.width) - scroller_size
                } else if base_bounds.width * self.pos < scroller_size / 2. {
                    base_bounds.x
                } else {
                    real_pos - scroller_size / 2.
                },
                y: base_bounds.y,
                width: scroller_size,
                height: base_bounds.height,
            }
        }
    }

    pub fn set_offset(&mut self, is_vertical: bool, base_bounds: &Rectangle, cursor: Point) {
        self.offset = Some(if is_vertical {
            cursor.y - (base_bounds.y + (base_bounds.height * self.pos))
        } else {
            cursor.x - (base_bounds.x + (base_bounds.width * self.pos))
        });
    }

    pub fn set_pos(&mut self, is_vertical: bool, base_bounds: &Rectangle, cursor: Point) {
        self.pos = if is_vertical {
            (cursor.y - base_bounds.y - self.offset.unwrap_or(0.)) / base_bounds.height
        } else {
            (cursor.x - base_bounds.x - self.offset.unwrap_or(0.)) / base_bounds.width
        }
        .clamp(0., 1.)
    }
}
