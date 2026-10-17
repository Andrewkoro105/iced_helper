use iced::{
    Element, Renderer, Task, Theme, widget::column ,
};
use iced_helper::
    ui_elements::{
        ParamSettings, PathType,
        select_file::{FileTypes, select_file},
    }
;
use std::path::PathBuf;
use tracing::Level;
use tracing_subscriber::{filter::Targets, fmt, layer::SubscriberExt, util::SubscriberInitExt};

struct TestState {
    pub path: PathBuf,
}

impl TestState {
    fn view(state: &TestState) -> Element<'_, TestMessage, iced::Theme, iced::Renderer> {
        let ps = ParamSettings { name_size: 200 };
        column![ps.create_path_param(
            "path",
            state.path.to_string_lossy(),
            PathType::Dir,
            TestMessage::Setpath,
            TestMessage::SelectPath
        )]
        .into()
    }
}

#[derive(Debug, Clone)]
enum TestMessage {
    SelectPath,
    Setpath(PathBuf),
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
        },
        |this: &mut TestState, message: TestMessage| -> Task<TestMessage> {
            match message {
                TestMessage::Setpath(path) => {
                    this.path = path;
                    Task::none()
                }
                TestMessage::SelectPath => {
                    select_file(TestMessage::Setpath, FileTypes::Dir, "~".into(), "")
                }
            }
        },
        TestState::view,
    )
    .theme(Theme::Dark)
    .run()
    .unwrap();
}
