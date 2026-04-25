//! Set of interface elements 

pub mod my_tooltip;
pub mod num_input;
pub mod select_file;
pub mod start_progress;

use iced::widget::{button, container, row, text, text_input};
use iced::{Element, Length};
use std::path::PathBuf;

/// Allows you to easily create user-input fields so that the input fields are aligned
/// ```
/// use iced_helper::ui_elements::ParamSettings;
/// use iced::Element;
///
/// #[derive(Clone)]
/// enum Message {
///     SetParam1(String),
/// }
///
/// struct State {
///     param1: String,
/// }
///
/// impl State {
///     fn view(&self) -> Element<'_, Message> {
///         let param_settings = ParamSettings {name_size: 100.};
///         param_settings.create_str_param("param1", &self.param1, Message::SetParam1)
///     }
/// }
/// ```
pub struct ParamSettings<L: Into<Length> + Clone> {
    /// The distance at which all input fields will be located
    pub name_size: L,
}

/// Type of requested path
pub enum PathType {
    File,
    Dir,
}

impl<L: Into<Length> + Clone> ParamSettings<L> {
    /// Creates a parameter with the specified input field
    pub fn create_param<'elem, UiElement, Message: 'elem, Theme, Renderer>(
        &self,
        name: impl text::IntoFragment<'elem>,
        input: UiElement,
    ) -> Element<'elem, Message, Theme, Renderer>
    where
        UiElement: Into<Element<'elem, Message, Theme, Renderer>>,
        Renderer: iced::advanced::Renderer + iced::advanced::text::Renderer + 'elem,
        Theme: container::Catalog + text::Catalog + 'elem,
    {
        row![
            container(text(name)).align_right(self.name_size.clone()),
            input.into()
        ]
        .spacing(5)
        .into()
    }

    /// Creates a parameter with an input string
    pub fn create_str_param<'elem, F, FM, M, T, R>(
        &self,
        name: impl text::IntoFragment<'elem>,
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
        self.create_param(name, text_input("", &value.to_string()).on_input(message))
            .map(Into::into)
    }

    /// Path input fields with the option to select a path via a system dialog
    ///
    /// # Arguments
    ///
    /// - `name` - Parameter name
    /// - `value` - Current value
    /// - `path_type` - Type of requested path
    /// - `text_input_message` - A message called to pass the entered path
    /// - `button_message` - A message that appears when the button is clicked, indicating that the user wants to select a path via a system dialog or by some other means
    ///
    /// # Example
    ///
    /// ```
    /// use iced_helper::ui_elements::{
    ///     ParamSettings,
    ///     PathType,
    ///     select_file::{
    ///         select_file,
    ///         FileTypes,
    ///         TypeAction,
    ///     }
    /// };
    /// use iced::Element;
    /// use std::path::PathBuf;
    /// use iced::Task;
    ///
    /// #[derive(Clone)]
    /// enum Message {
    ///     SetPathParam(PathBuf),
    ///     SelectPathParam,
    /// }
    ///
    /// struct State {
    ///     path_param: PathBuf,
    /// }
    ///
    /// impl State {
    ///     fn update(&mut self, message: Message) -> Task<Message> {
    ///         match message {
    ///             Message::SetPathParam(path) => {
    ///                 self.path_param = path;
    ///                 Task::none()
    ///             },
    ///             Message::SelectPathParam => select_file(
    ///                Message::SetPathParam,
    ///                FileTypes::Files(
    ///                    TypeAction::Save,
    ///                    vec![
    ///                        (
    ///                            "image",
    ///                            &[
    ///                                "avif", "bmp", "dds", "exr", "ff", "gif", "hdr", "ico", "jpeg",
    ///                                "png", "pnm", "qoi", "tga", "tiff", "webp", "raw",
    ///                            ],
    ///                        ),
    ///                        ("text", &["txt"]),
    ///                    ],
    ///                ),
    ///            )
    ///         }
    ///     }
    ///
    ///     fn view(&self) -> Element<'_, Message> {
    ///         let param_settings = ParamSettings {name_size: 100.};
    ///         param_settings.create_path_param(
    ///             "path_param", 
    ///             &self.path_param.to_str().unwrap(), 
    ///             PathType::File,
    ///             Message::SetPathParam,
    ///             Message::SelectPathParam,
    ///         )
    ///     }
    /// }
    /// ```
    pub fn create_path_param<'elem, IM, SM, M, T, R>(
        &self,
        name: impl text::IntoFragment<'elem>,
        value: impl ToString,
        path_type: PathType,
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
        self.create_param(
            name,
            row![
                Element::from(
                    text_input("", &value.to_string())
                        .on_input(move |str| text_input_message(PathBuf::from(str)))
                ),
                Element::from(
                    button(match path_type {
                        PathType::Dir => "выбрать папку",
                        PathType::File => "выбрать файл",
                    })
                    .on_press(button_message)
                ),
            ]
            .spacing(10),
        )
        .map(Into::into)
    }
}
