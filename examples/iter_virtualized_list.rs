use iced::{
    Element, Theme,
    widget::{container, text},
};
use iced_helper::widgets::virtualized_list::api::virtualized_list;
use tracing::Level;
use tracing_subscriber::{filter::Targets, fmt, layer::SubscriberExt, util::SubscriberInitExt};

struct TestState;

impl TestState {
    fn view(_: &TestState) -> Element<'_, (), iced::Theme, iced::Renderer> {
        container(
            container(
                virtualized_list(0..(u32::MAX), |data| {
                    container(text!("elem: ({data})"))
                        .padding(5)
                        .style(|theme| container::success(theme))
                        .into()
                })
                .spacing(15),
            )
            .style(|theme| container::warning(theme)),
        )
        .padding(100)
        .into()
    }
}

fn main() {
    let filter = Targets::new()
        .with_target("iced_helper", Level::DEBUG)
        .with_default(Level::INFO);

    tracing_subscriber::registry()
        .with(fmt::Layer::new())
        .with(filter)
        .init();

    iced::application::<TestState, (), iced::Theme, iced::Renderer>(
        || TestState,
        |_: &mut TestState, _: ()| {},
        TestState::view,
    )
    .theme(Theme::Dark)
    .run()
    .unwrap();
}
