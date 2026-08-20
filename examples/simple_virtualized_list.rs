use iced::{
    Element, Theme,
    widget::{container, text},
};
use iced_helper::widgets::virtualized_list::api::virtualized_list;
use std::time::Instant;
use tracing::{Level, info};
use tracing_subscriber::{filter::Targets, fmt, layer::SubscriberExt, util::SubscriberInitExt};

struct TestState {
    data: Vec<String>,
}

impl TestState {
    fn view(state: &TestState) -> Element<'_, (), iced::Theme, iced::Renderer> {
        container(
            container(virtualized_list(&state.data, |data| {
                container(text!("elem: ({data})"))
                    .padding(5)
                    .style(|theme| container::success(theme))
                    .into()
            }).spacing(100).gap(20.))
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
        || TestState {
            data: {
                let start = Instant::now();
                let count = 50u64;
                let result = (0..count)
                    .map(|dig| {
                        if count > 100 && dig % (count / 100) == 0 {
                            info!("load: {}%", dig / (count / 100));
                        }
                        format!("dig: {dig}")
                    })
                    .collect();
                info!("load time: {:?}", start.elapsed());
                result
            },
        },
        |_: &mut TestState, _: ()| {},
        TestState::view,
    )
    .theme(Theme::Dark)
    .run()
    .unwrap();
}
