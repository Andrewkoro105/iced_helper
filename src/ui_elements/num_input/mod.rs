pub mod base_value;
pub mod modification;
use iced::Element;
use iced::widget::text_input;
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;

use crate::ui_elements::num_input::{base_value::BaseValue, modification::Modification};

#[derive(Debug, Clone)]
pub struct NumInput<V, BT: BaseValue<V>, M: Modification<V>> {
    value: V,
    _modification: M,
    _base_value: BT,
    str: String,
}

impl<V, BT, M> Default for NumInput<V, BT, M>
where
    BT: BaseValue<V> + Default,
    M: Modification<V> + Default,
{
    fn default() -> Self {
        Self {
            value: BT::VALUE,
            _modification: Default::default(),
            _base_value: Default::default(),
            str: Default::default(),
        }
    }
}

impl<V, BT, Mod> NumInput<V, BT, Mod>
where
    V: FromStr + ToString + Copy + PartialOrd,
    BT: BaseValue<V> + Default,
    Mod: Modification<V> + Default,
{
    pub fn new(value: V) -> Self {
        Self {
            value,
            _modification: Default::default(),
            _base_value: Default::default(),
            str: Mod::to_str(&value).to_string(),
        }
    }

    pub fn set(&mut self, value: V) {
        self.value = value;
        self.str = Mod::to_str(&value).to_string();
    }

    pub fn get(&self) -> V {
        self.value
    }

    pub fn update(&mut self, value_str: &str) -> V {
        if let Ok(value) = V::from_str(value_str) {
            self.value = Mod::to_num(&value);
            self.str = value.to_string();
        } else if value_str == "-" || value_str.is_empty() {
            self.str = value_str.to_string();
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

impl<V, BT, M> Serialize for NumInput<V, BT, M>
where
    V: Serialize,
    BT: BaseValue<V> + Default,
    M: Modification<V> + Default,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        V::serialize(&self.value, serializer)
    }
}

impl<'de, V, BT, M> Deserialize<'de> for NumInput<V, BT, M>
where
    V: Deserialize<'de> + ToString,
    BT: BaseValue<V> + Default,
    M: Modification<V> + Default,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = V::deserialize(deserializer)?;
        Ok(Self {
            str: M::to_str(&value).to_string(),
            value: value,
            _modification: Default::default(),
            _base_value: Default::default(),
        })
    }
}
