pub mod api;

pub use api::*;
use tracing::{debug, trace};

use crate::shell_merge::shell_merge_without_message;
use iced::{
    Element, Event, Length, Rectangle, Renderer, Size, Theme, Vector,
    advanced::{
        Clipboard, Layout, Shell, Widget, layout, mouse, renderer, text,
        widget::{Operation, Tree, tree},
    },
    overlay,
    widget::{TextInput, text_input, text_input::Catalog},
};
use std::{fmt::Debug, marker::PhantomData, str::FromStr};

#[derive(Clone)]
pub enum TextInputMessage {
    OnInput(String),
    OnPaste(String),
    OnSubmit,
}

pub struct NumInput<'elem, M, BD, DD, T = Theme, R = Renderer>
where
    DD: FromStr + ToString,
    <DD as FromStr>::Err: Debug,
    BD: PartialEq + Debug + Clone,
    T: Catalog,
    R: text::Renderer,
{
    placeholder: Option<String>,
    value: &'elem BD,
    default: Option<BD>,
    may_be_negative: bool,
    input_num_message: Option<Box<dyn Fn(BD) -> M>>,
    input_text_message: Option<Box<dyn Fn(TextInputMessage) -> M>>,
    text_input_generate: Box<
        dyn Fn(
            TextInput<'elem, TextInputMessage, T, R>,
        ) -> TextInput<'elem, TextInputMessage, T, R>,
    >,
    text_input: TextInput<'elem, TextInputMessage, T, R>,
    to_base: Box<dyn Fn(&DD) -> BD>,
    to_display: Box<dyn Fn(&BD) -> DD>,
    _phantom: PhantomData<DD>,
}

pub struct State {
    text: String,
}

impl<'elem, M, BD, DD, T, R> From<NumInput<'elem, M, BD, DD, T, R>> for Element<'elem, M, T, R>
where
    M: 'static,
    DD: FromStr + ToString + 'static,
    <DD as FromStr>::Err: Debug,
    BD: PartialEq + Debug + Clone + 'static,
    T: Catalog + 'static,
    R: text::Renderer + 'static,
{
    fn from(value: NumInput<'elem, M, BD, DD, T, R>) -> Self {
        Self::new(value)
    }
}

impl<'elem, M, BD, DD, T, R> Widget<M, T, R> for NumInput<'elem, M, BD, DD, T, R>
where
    DD: FromStr + ToString + 'static,
    <DD as FromStr>::Err: Debug,
    BD: PartialEq + Debug + Clone,
    T: Catalog + 'static,
    R: text::Renderer,
{
    fn size_hint(&self) -> Size<Length> {
        self.text_input.size_hint()
    }

    fn size(&self) -> Size<Length> {
        <TextInput<'_, _, _, _> as Widget<_, _, _>>::size(&self.text_input)
    }

    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        trace!("state");
        tree::State::new(State {
            text: (self.to_display)(&self.value).to_string(),
        })
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(
            &self.text_input as &dyn Widget<TextInputMessage, T, R>,
        )]
    }

    fn diff(&self, tree: &mut Tree) {
        if tree.tag == self.tag() {
            if tree.children.is_empty() {
                tree.children = self.children();
            }
            self.text_input.diff(&mut tree.children[0]);
        } else {
            *tree = Tree::new(self as &dyn Widget<M, T, R>)
        }
    }

    fn layout(&mut self, tree: &mut Tree, renderer: &R, limits: &layout::Limits) -> layout::Node {
        let state = tree.state.downcast_mut::<State>();
        self.set_in_text_input(&state.text);
        <TextInput<'_, _, _, _> as Widget<_, _, _>>::layout(
            &mut self.text_input,
            &mut tree.children[0],
            renderer,
            limits,
        )
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut R,
        theme: &T,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        <TextInput<'_, _, _, _> as Widget<_, _, _>>::draw(
            &self.text_input,
            &tree.children[0],
            renderer,
            theme,
            style,
            layout,
            cursor,
            viewport,
        )
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &R,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, M>,
        viewport: &Rectangle,
    ) {
        let state = tree.state.downcast_mut::<State>();
        let mut messages = vec![];
        let mut text_input_shell = Shell::new(&mut messages);
        self.text_input.update(
            &mut tree.children[0],
            event,
            layout,
            cursor,
            renderer,
            clipboard,
            &mut text_input_shell,
            viewport,
        );

        shell_merge_without_message(shell, &text_input_shell);

        messages.iter().for_each(|message| {
            if let TextInputMessage::OnInput(value_str) = &message {
                if let Some(input_num_message) = self.input_num_message.as_ref() {
                    if let Ok(value) = DD::from_str(&value_str) {
                        let value = (self.to_base)(&value);
                        state.text = (self.to_display)(&value).to_string();
                        if value_str.chars().last() == Some('.') {
                            state.text.push('.');
                        }
                        shell.publish((input_num_message)(value));
                        debug!("num: {}", state.text)
                    } else if (value_str == "-" && self.may_be_negative) || value_str.is_empty() {
                        if let Some(default) = self.default.as_ref() {
                            shell.publish((input_num_message)(default.clone()));
                        }
                        state.text = value_str.clone();
                    } else {
                        shell.invalidate_layout();
                    };

                    self.set_in_text_input(&state.text);
                    shell.request_redraw();
                }
            }
            if let Some(input_text_message) = self.input_text_message.as_ref() {
                shell.publish((input_text_message)(message.clone()));
            }
        });
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &R,
    ) -> mouse::Interaction {
        self.text_input
            .mouse_interaction(&tree.children[0], layout, cursor, viewport, renderer)
    }

    fn overlay<'a>(
        &'a mut self,
        _tree: &'a mut Tree,
        _layout: Layout<'a>,
        _renderer: &R,
        _viewport: &Rectangle,
        _translation: Vector,
    ) -> Option<overlay::Element<'a, M, T, R>> {
        None
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &R,
        operation: &mut dyn Operation,
    ) {
        self.text_input
            .operate(&mut tree.children[0], layout, renderer, operation);
    }
}

impl<'elem, M, BD, DD, T, R> NumInput<'elem, M, BD, DD, T, R>
where
    DD: FromStr + ToString + 'static,
    <DD as FromStr>::Err: Debug,
    BD: PartialEq + Debug + Clone,
    T: Catalog + 'static,
    R: text::Renderer,
{
    fn set_in_text_input(&mut self, value: &str) {
        let placeholder = self.placeholder.clone().unwrap_or_else(|| {
            (self.to_display)(self.default.as_ref().unwrap_or(&self.value)).to_string()
        });

        self.text_input = get_text_input(
            &placeholder,
            value,
            self.input_num_message.is_some() || self.input_text_message.is_some(),
            &self.text_input_generate,
        );
    }

    fn update_text_input(&mut self) {
        let dummy = text_input("placeholder", "value");

        let owned_input = std::mem::replace(&mut self.text_input, dummy);

        self.text_input = setting_text_input(
            owned_input,
            self.input_num_message.is_some() || self.input_text_message.is_some(),
            &self.text_input_generate,
        );
    }
}

pub fn get_text_input<'elem, T, R>(
    placeholder: &str,
    value: &str,
    is_active: bool,
    text_input_generate: &Box<
        dyn Fn(
            TextInput<'elem, TextInputMessage, T, R>,
        ) -> TextInput<'elem, TextInputMessage, T, R>,
    >,
) -> TextInput<'elem, TextInputMessage, T, R>
where
    T: Catalog + 'static,
    R: text::Renderer,
{
    setting_text_input(
        text_input(placeholder, value),
        is_active,
        text_input_generate,
    )
}

pub fn setting_text_input<'elem, T, R>(
    text_input: TextInput<'elem, TextInputMessage, T, R>,
    is_active: bool,
    text_input_generate: &Box<
        dyn Fn(
            TextInput<'elem, TextInputMessage, T, R>,
        ) -> TextInput<'elem, TextInputMessage, T, R>,
    >,
) -> TextInput<'elem, TextInputMessage, T, R>
where
    T: Catalog + 'static,
    R: text::Renderer,
{
    text_input_generate(
        text_input
            .on_input_maybe(is_active.then_some(TextInputMessage::OnInput))
            .on_paste(TextInputMessage::OnPaste)
            .on_submit(TextInputMessage::OnSubmit),
    )
}
