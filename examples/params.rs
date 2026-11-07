use iced::{
    Element, Renderer, Task, Theme,
    widget::{column, row, text},
};
use iced_helper::{
    ui_elements::{
        ParamSettings, PathType,
        dictionary::Dictionary,
        select_file::{FileTypes, select_file},
    },
    widgets::num_input::num_input,
};
use std::path::PathBuf;
use tracing::{Level, info};
use tracing_subscriber::{filter::Targets, fmt, layer::SubscriberExt, util::SubscriberInitExt};

struct TestState {
    path: PathBuf,
    ps: ParamSettings<u32>,
    num_f32: f32,
    num_u8: u8,
    color: f32,
}

impl TestState {
    fn view(state: &TestState) -> Element<'_, TestMessage, iced::Theme, iced::Renderer> {
        info!("view");
        column![
            state.ps.create_path_param(
                "path",
                state.path.to_string_lossy(),
                PathType::Dir,
                TestMessage::Setpath,
                TestMessage::SelectPath
            ),
            state.ps.create_param(
                "f32",
                num_input(&state.num_f32).input_num_message(TestMessage::NumF32)
            ),
            state.ps.create_param(
                "u8",
                num_input(&state.num_u8)
                    .input_num_message(TestMessage::NumU8)
                    .positive()
            ),
            state.ps.create_param(
                "color",
                row![
                    num_input(&state.color)
                        .input_num_message(TestMessage::Color)
                        .positive()
                        .cast(
                            |value| {
                                let result = *value / 255.;
                                if result < 0. {
                                    0.
                                } else if result > 1. {
                                    1.
                                } else {
                                    result
                                }
                            },
                            |value| {
                                let result = (*value * 255. * 10.0).round() / 10.0;
                                if result < 0. {
                                    0.
                                } else if result > 255. {
                                    255.
                                } else {
                                    result
                                }
                            }
                        ),
                    text!("{}", state.color)
                ]
                .spacing(5)
            ),
        ]
        .padding(5)
        .spacing(5)
        .into()
    }
}

#[derive(Debug, Clone)]
enum TestMessage {
    SelectPath,
    Setpath(PathBuf),
    NumF32(f32),
    NumU8(u8),
    Color(f32),
}

fn main() {
    let filter = Targets::new()
        .with_target("iced_helper", Level::TRACE)
        .with_default(Level::INFO);

    tracing_subscriber::registry()
        .with(fmt::Layer::new())
        .with(filter)
        .init();

    iced::application::<TestState, TestMessage, Theme, Renderer>(
        || TestState {
            path: PathBuf::new(),
            ps: ParamSettings {
                name_size: 50,
                dictionary: Dictionary::default(),
            },
            num_f32: 5.,
            num_u8: 6,
            color: 0.,
        },
        |this: &mut TestState, message: TestMessage| -> Task<TestMessage> {
            match message {
                TestMessage::Setpath(path) => {
                    this.path = path;
                    Task::none()
                }
                TestMessage::SelectPath => {
                    select_file(TestMessage::Setpath, FileTypes::Dir, PathBuf::new(), "")
                }
                TestMessage::NumF32(num) => {
                    this.num_f32 = num;
                    Task::none()
                }
                TestMessage::NumU8(num) => {
                    this.num_u8 = num;
                    Task::none()
                }
                TestMessage::Color(num) => {
                    this.color = num;
                    Task::none()
                }
            }
        },
        TestState::view,
    )
    .theme(Theme::Dark)
    .run()
    .unwrap();
}
