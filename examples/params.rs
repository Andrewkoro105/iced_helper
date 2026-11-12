use iced::{
    Border, Color, Element, Renderer, Shadow, Task, Theme,
    widget::{column, row, text},
};
use iced_helper::{
    optimal_color::{border_color, text_color},
    ui_elements::{
        ParamSettings, PathType,
        dictionary::Dictionary,
        select_file::{FileTypes, select_file},
    },
    widgets::{
        menu,
        num_input::num_input,
        pick_list::{self, pick_list},
    },
};
use std::{path::PathBuf, str::FromStr};
use tracing::{Level, info};
use tracing_subscriber::{filter::Targets, fmt, layer::SubscriberExt, util::SubscriberInitExt};

struct TestState {
    path: PathBuf,
    ps: ParamSettings<u32>,
    num_f32: f32,
    num_u8: u8,
    color: f32,
    color_str: String,
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
            state.ps.create_param(
                "pick list",
                pick_list(
                    ["red".to_owned(), "green".to_owned(), "blue".to_owned()],
                    Some(state.color_str.clone()),
                    TestMessage::ColorStr
                )
                .style(|theme: &Theme, status| {
                    let palette = theme.extended_palette();

                    let mut active = pick_list::Style {
                        text_color: palette.background.weak.text,
                        background: palette.background.weak.color.into(),
                        placeholder_color: palette.secondary.base.color,
                        handle_color: palette.background.weak.text,
                        border: Border {
                            radius: 2.0.into(),
                            width: 1.0,
                            color: palette.background.strong.color,
                        },
                    };

                    match state.color_str.as_str() {
                        "red" => {
                            active.background = Color::from_str("#ff0000").unwrap().into();
                        }
                        "green" => {
                            active.background = Color::from_str("#00ff00").unwrap().into();
                        }
                        "blue" => {
                            active.background = Color::from_str("#0000ff").unwrap().into();
                        }
                        _ => {}
                    }

                    active.text_color = text_color(&active.text_color);
                    active.handle_color = active.text_color;
                    match status {
                        pick_list::Status::Active => active,
                        pick_list::Status::Hovered | pick_list::Status::Opened { .. } => {
                            pick_list::Style {
                                border: Border {
                                    color: palette.primary.strong.color,
                                    ..active.border
                                },
                                ..active
                            }
                        }
                    }
                })
                .menu_style(|theme: &Theme, option: Option<&String>, status| {
                    let palette = theme.extended_palette();

                    let background_color = match option.map(String::as_str) {
                        Some("red") => Color::from_str("#ff0000").unwrap(),
                        Some("green") => Color::from_str("#00ff00").unwrap(),
                        Some("blue") => Color::from_str("#0000ff").unwrap(),
                        _ => palette.background.weak.color.into(),
                    };
                    match status {
                        menu::Status::Active => menu::Style {
                            background: background_color.into(),
                            border: Border {
                                width: 1.0,
                                radius: 0.0.into(),
                                color: palette.background.strong.color,
                            },
                            text_color: text_color(&background_color),
                            shadow: Shadow::default(),
                        },
                        menu::Status::Hovered => {
                            let light = 0.3;
                            let selected_background_color = Color {
                                a: background_color.a + light,
                                r: background_color.r + light,
                                g: background_color.g + light,
                                b: background_color.b + light,
                            };
                            menu::Style {
                                background: selected_background_color.into(),
                                border: Border {
                                    width: 3.0,
                                    radius: 0.0.into(),
                                    color: border_color(selected_background_color, 1.0),
                                },
                                text_color: text_color(&selected_background_color),
                                shadow: Shadow::default(),
                            }
                        }
                    }
                })
            ),
            state.ps.create_param(
                "pick list",
                pick_list(
                    ["red".to_owned(), "green".to_owned(), "blue".to_owned()],
                    Some(state.color_str.clone()),
                    TestMessage::ColorStr
                ),
            )
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
    ColorStr(String),
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
            color_str: "red".to_owned(),
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
                TestMessage::ColorStr(color_str) => {
                    this.color_str = color_str;
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
