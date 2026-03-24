pub mod base_value;
use iced::Element;
use iced::widget::text_input;
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;

use crate::ui_elements::num_input::base_value::BaseValue;

#[derive(Debug, Clone)]
pub struct NumInput<T, BT> {
    value: T,
    _base_value: BT,
    str: String,
}

impl<V, BT> Default for NumInput<V, BT>
where
    BT: BaseValue<V> + Default,
{
    fn default() -> Self {
        Self {
            value: BT::VALUE,
            _base_value: Default::default(),
            str: Default::default(),
        }
    }
}

impl<V, BT> NumInput<V, BT>
where
    V: FromStr + ToString + Copy + PartialOrd,
    BT: BaseValue<V> + Default,
{
    pub fn new(value: V) -> Self {
        Self {
            value,
            _base_value: BT::default(),
            str: value.to_string(),
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
            self.value = BT::VALUE;
        }
        self.value
    }

    pub fn view<'elem, F, MB, M, T, R>(
        &self,
        placeholder: &str,
        message: F,
    ) -> Element<'elem, M, T, R>
    where
        F: Fn(String) -> MB + Clone + 'elem,
        M: Clone + From<MB> + 'elem,
        T: text_input::Catalog + 'elem,
        R: iced::advanced::text::Renderer + 'elem,
        MB: Clone + 'elem,
    {
        Element::from(text_input(placeholder, self.str.as_str()).on_input(message.clone()))
            .map(Into::into)
    }
}

impl<T: Serialize, BT> Serialize for NumInput<T, BT> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        T::serialize(&self.value, serializer)
    }
}

impl<'de, T, BT> Deserialize<'de> for NumInput<T, BT>
where
    T: Deserialize<'de> + ToString,
    BT: Default,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = T::deserialize(deserializer)?;
        Ok(Self {
            str: value.to_string(),
            value,
            _base_value: BT::default(),
        })
    }
}
