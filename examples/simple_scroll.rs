use iced::{
    Alignment, Element, Length, Theme,
    widget::{container, row},
};
use iced_helper::widgets::scrollbar::api::scrollbar;
use tracing::{Level, info};
use tracing_subscriber::{filter::Targets, fmt, layer::SubscriberExt, util::SubscriberInitExt};

struct TestState;

impl TestState {
    fn view(_: &TestState) -> Element<'_, f32, iced::Theme, iced::Renderer> {
        row![
            container(scrollbar().on_scroll(|a| a))
                .width(Length::Fill)
                .height(Length::Fill)
                .align_x(Alignment::Center)
                .align_y(Alignment::Center)
                .padding(100),
            container(scrollbar().base_view(0.2).horizontal().on_scroll(|a| a))
                .width(Length::Fill)
                .height(Length::Fill)
                .align_x(Alignment::Center)
                .align_y(Alignment::Center)
                .padding(100)
        ]
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

    iced::application::<TestState, f32, iced::Theme, iced::Renderer>(
        || TestState,
        |_: &mut TestState, scroll: f32| info!("{scroll}"),
        TestState::view,
    )
    .theme(Theme::Dark)
    .run()
    .unwrap();
}
