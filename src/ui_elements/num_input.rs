use iced::Element;
use iced::widget::text_input;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct NumInput<T> {
    value: T,
    base_value: T,
    str: String,
    placeholder: String,
}

impl<V> NumInput<V>
where
    V: FromStr + ToString + Copy + PartialOrd,
{
    pub fn new(value: V, placeholder: impl ToString, base_value: V) -> Self {
        Self {
            value,
            base_value,
            str: value.to_string(),
            placeholder: placeholder.to_string(),
        }
    }

    pub fn set(&mut self, value: V) {
        self.value = value;
        self.str = value.to_string();
    }

    pub fn get(&self) -> V {
        self.value
    }

    pub fn update(&mut self, value_str: &str) -> V {
        if let Ok(value) = V::from_str(value_str) {
            self.value = value;
            self.str = value_str.to_string();
        } else if value_str.is_empty() {
            self.str = "".to_string();
            self.value = self.base_value;
        }
        self.value
    }

    pub fn view<'elem, F, MB, M, T, R>(&self, message: F) -> Element<'elem, M, T, R>
    where
        F: Fn(String) -> MB + Clone + 'elem,
        M: Clone + From<MB> + 'elem,
        T: text_input::Catalog + 'elem,
        R: iced::advanced::text::Renderer + 'elem,
        MB: Clone + 'elem,
    {
        Element::from(text_input(&self.placeholder, self.str.as_str()).on_input(message.clone()))
            .map(Into::into)
    }
}
