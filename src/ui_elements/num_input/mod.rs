//! Data storage structure for a numeric input field

pub mod base_value;
pub mod modification;
use iced::Element;
use iced::widget::text_input;
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;

use crate::ui_elements::num_input::{base_value::BaseValue, modification::Modification};

/// Data storage structure for a numeric input field
/// 
/// # Generic arguments
/// 
/// - `V` - Number type
/// - `BT` - The default value that will be used if an empty string or a string consisting solely of “-” is passed (the associative type VALUE in the BaseValue trait is used to specify the constant). This value is also used to implement `Default`
/// - `M` - A modifier that changes how a number appears in a text field
///  
/// ```
/// use iced_helper::ui_elements::num_input::{
///     base_value::ConstF32,
///     modification::ColorCast,
///     NumInput
/// };
/// use iced::Element;
/// 
/// #[derive(Clone)]
/// enum Message {
///     SetNum(String),
/// }
/// 
/// struct State {
///     num: NumInput<f32, ConstF32<0>, ColorCast>,
/// }
/// 
/// impl State {
///     
///     fn update(&mut self, message: Message) {
///         match message {
///             Message::SetNum(num_str) => println!("new value: {}", self.num.update(&num_str))
///         }
///     }
/// 
///     fn view(&self) -> Element<'_, Message> {
///         self.num.view("num_placeholder", Message::SetNum)
///     }
/// }
/// 
/// ```
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
            str: Mod::to_display(&value).to_string(),
        }
    }

    pub fn set(&mut self, value: V) {
        self.value = value;
        self.str = Mod::to_display(&value).to_string();
    }

    pub fn get(&self) -> V {
        self.value
    }

    /// Updates the value based on the string provided
    pub fn update(&mut self, value_str: &str) -> V {
        if let Ok(value) = V::from_str(value_str) {
            self.value = Mod::to_base(&value);
            self.str = value.to_string();
        } else if value_str == "-" || value_str.is_empty() {
            self.str = value_str.to_string();
            self.value = BT::VALUE;
        }
        self.value
    }

    /// Returns an `iced::Element` for displaying the numeric input field
    /// 
    /// # Argument
    /// - `placeholder` - Placeholder, analogous to the corresponding parameter in `iced::widget::text_input()`
    /// - `message` - A message displayed when the text is changed 
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
            str: M::to_display(&value).to_string(),
            value: value,
            _modification: Default::default(),
            _base_value: Default::default(),
        })
    }
}
