pub mod my_tooltip;
pub mod num_input;
pub mod select_file;
pub mod start_progress;
pub mod updated;

use iced::Element;
use iced::widget::{button, container, row, text, text_input};
use std::path::PathBuf;

pub fn create_param<'elem, UiElement, Message: 'elem, Theme, Renderer>(
    placeholder: impl text::IntoFragment<'elem>,
    input: UiElement,
) -> Element<'elem, Message, Theme, Renderer>
where
    UiElement: Into<Element<'elem, Message, Theme, Renderer>>,
    Renderer: iced::advanced::Renderer + iced::advanced::text::Renderer + 'elem,
    Theme: container::Catalog + text::Catalog + 'elem,
{
    row![container(text(placeholder)).align_right(200), input.into()]
        .spacing(5)
        .into()
}

pub fn create_str_param<'elem, F, FM, M, T, R>(
    placeholder: impl text::IntoFragment<'elem>,
    value: &impl ToString,
    message: F,
) -> Element<'elem, M, T, R>
where
    R: iced::advanced::Renderer + iced::advanced::text::Renderer + 'elem,
    T: container::Catalog + text::Catalog + text_input::Catalog + 'elem,
    M: From<FM> + Clone + 'elem,
    F: Fn(String) -> FM + 'elem,
    FM: std::clone::Clone + 'elem,
{
    create_param(
        placeholder,
        text_input("", &value.to_string()).on_input(message),
    )
    .map(Into::into)
}

pub fn create_path_param<'elem, IM, SM, M, T, R>(
    placeholder: impl text::IntoFragment<'elem>,
    value: impl ToString,
    text_input_message: IM,
    button_message: SM,
) -> Element<'elem, M, T, R>
where
    R: iced::advanced::Renderer + iced::advanced::text::Renderer + 'elem,
    T: container::Catalog + text::Catalog + text_input::Catalog + button::Catalog + 'elem,
    SM: Clone + 'elem,
    IM: Fn(PathBuf) -> SM + Clone + 'elem,
    M: From<SM> + 'elem,
{
    create_param(
        placeholder,
        row![
            Element::from(
                text_input("", &value.to_string())
                    .on_input(move |str| text_input_message(PathBuf::from(str)))
            ),
            Element::from(button("выбрать папку").on_press(button_message)),
        ]
        .spacing(10),
    )
    .map(Into::into)
}
