use std::{fmt::Debug, marker::PhantomData, str::FromStr};

use iced::{
    advanced::text,
    widget::{TextInput, text_input::Catalog},
};
use num_traits::Zero;

use crate::widgets::num_input::{NumInput, TextInputMessage};

pub fn num_input<'elem, M, BD, T, R>(value: &'elem BD) -> NumInput<'elem, M, BD, BD, T, R>
where
    BD: PartialEq + Debug + Clone + FromStr + ToString + Zero + 'static,
    <BD as FromStr>::Err: Debug,
    T: Catalog + 'static,
    R: text::Renderer,
{
    NumInput::new(value, |a: &BD| a.clone(), |a| a.clone()).default(Some(BD::zero()))
}

impl<'elem, M, BD, DD, T, R> NumInput<'elem, M, BD, DD, T, R>
where
    DD: FromStr + ToString + 'static,
    <DD as FromStr>::Err: Debug,
    BD: PartialEq + Debug + Clone,
    T: Catalog + 'static,
    R: text::Renderer,
{
    pub fn new(
        value: &'elem BD,
        to_base: impl Fn(&DD) -> BD + 'static,
        to_display: impl Fn(&BD) -> DD + 'static,
    ) -> Self {
        let text_input_generate = Box::new(|a| a) as Box<dyn Fn(_) -> _>;
        let text = to_display(value).to_string();
        NumInput {
            placeholder: None,
            value,
            default: None,
            may_be_negative: true,
            input_num_message: None,
            input_text_message: None,
            text_input: super::get_text_input(&text, &text, false, &text_input_generate),
            text_input_generate,
            to_base: Box::new(to_base) as _,
            to_display: Box::new(to_display) as _,
            _phantom: PhantomData::default(),
        }
    }

    pub fn placeholder(mut self, placeholder: &str) -> Self {
        self.placeholder = Some(placeholder.to_owned());
        self
    }

    pub fn may_be_negative(mut self, may_be_negative: bool) -> Self {
        self.may_be_negative = may_be_negative;
        self
    }

    pub fn positive(mut self) -> Self {
        self.may_be_negative = false;
        self
    }

    pub fn default(mut self, default: Option<BD>) -> Self {
        self.default = default;
        self
    }

    pub fn input_num_message(mut self, input_num_message: impl Fn(BD) -> M + 'elem) -> Self {
        self.input_num_message = Some(Box::new(input_num_message) as _);
        self.update_text_input();
        self
    }

    pub fn input_text_message(
        mut self,
        input_text_message: impl Fn(TextInputMessage) -> M + 'elem,
    ) -> Self {
        self.input_text_message = Some(Box::new(input_text_message) as _);
        self.update_text_input();
        self
    }

    pub fn text_input_generate(
        mut self,
        text_input_generate: Box<
            dyn Fn(
                TextInput<'elem, TextInputMessage, T, R>,
            ) -> TextInput<'elem, TextInputMessage, T, R>,
        >,
    ) -> Self {
        self.text_input_generate = text_input_generate;
        self.update_text_input();
        self
    }

    pub fn cast(
        mut self,
        to_base: impl Fn(&DD) -> BD + 'elem,
        to_display: impl Fn(&BD) -> DD + 'elem,
    ) -> Self {
        self.to_base = Box::new(to_base) as _;
        self.to_display = Box::new(to_display) as _;
        self.set_in_text_input(&(self.to_display)(&self.value).to_string());
        self
    }
}
