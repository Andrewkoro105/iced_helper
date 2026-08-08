use std::any::Any;

use iced::{Task, advanced::widget::Operation, widget::Id};
use iced_runtime::{Action, task};

use crate::widgets::virtualized_list::{Pos, State};

pub struct ScrollTo {
    id: Id,
    pos: Pos,
}

pub fn scroll_to<T>(id: Id, pos: Pos) -> Task<T> {
    task::effect(Action::widget(ScrollTo::new(id, pos)))
}

impl ScrollTo {
    pub fn new(id: Id, pos: Pos) -> Self {
        Self { id, pos }
    }
}

impl Operation for ScrollTo {
    fn traverse(&mut self, operate: &mut dyn FnMut(&mut dyn Operation<()>)) {
        operate(self)
    }

    fn custom(
        &mut self,
        id: Option<&iced::widget::Id>,
        _bounds: iced::Rectangle,
        state: &mut dyn Any,
    ) {
        if id.cloned() == Some(self.id.clone()) {
            let state = state.downcast_mut::<State>().unwrap();
            state.user_pos = Some(self.pos);
        }
    }
}
