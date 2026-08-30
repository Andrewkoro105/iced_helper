use iced::{
    Alignment, Background, Element, Length, Theme,
    advanced::renderer::Quad,
    widget::{container, row},
};
use iced_helper::widgets::scrollbar::{self, scrollbar};
use tracing::{Level, info};
use tracing_subscriber::{filter::Targets, fmt, layer::SubscriberExt, util::SubscriberInitExt};

struct TestState;

impl TestState {
    fn view(_: &TestState) -> Element<'_, f32, iced::Theme, iced::Renderer> {
        row![
            container(
                scrollbar()
                    .on_scroll(|a| a)
                    .style(|theme: &iced::Theme, status| match status {
                        scrollbar::style::Status::Active => scrollbar::style::Style {
                            background: Background::Color(
                                theme.extended_palette().background.weak.color
                            ),
                            border: Default::default(),
                            shadow: Default::default(),
                            snap: Quad::default().snap,
                            scroller_background: Background::Color(
                                theme.extended_palette().background.strong.color,
                            ),
                            scroller_border: Default::default(),
                            scroller_shadow: Default::default(),
                            scroller_snap: Quad::default().snap,
                        },
                        scrollbar::style::Status::Hovered => scrollbar::style::Style {
                            background: Background::Color(
                                theme.extended_palette().background.weak.color
                            ),
                            border: Default::default(),
                            shadow: Default::default(),
                            snap: Quad::default().snap,
                            scroller_background: Background::Color(
                                theme.extended_palette().warning.base.color,
                            ),
                            scroller_border: Default::default(),
                            scroller_shadow: Default::default(),
                            scroller_snap: Quad::default().snap,
                        },
                        scrollbar::style::Status::Dragged => scrollbar::style::Style {
                            background: Background::Color(
                                theme.extended_palette().background.weak.color
                            ),
                            border: Default::default(),
                            shadow: Default::default(),
                            snap: Quad::default().snap,
                            scroller_background: Background::Color(
                                theme.extended_palette().success.base.color,
                            ),
                            scroller_border: Default::default(),
                            scroller_shadow: Default::default(),
                            scroller_snap: Quad::default().snap,
                        },
                    })
            )
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
