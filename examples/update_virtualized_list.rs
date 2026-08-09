use iced::{
    Element, Renderer, Subscription, Task, Theme, time,
    widget::{Id, button, container, row, text},
};
use iced_helper::widgets::virtualized_list::{
    Pos, api::virtualized_list, operations::scroll_to::scroll_to,
};
use std::time::{Duration, Instant};
use tracing::{Level, info};
use tracing_subscriber::{filter::Targets, fmt, layer::SubscriberExt, util::SubscriberInitExt};

struct TestState {
    data: Vec<(u64, usize)>,
}

impl TestState {
    fn view(state: &TestState) -> Element<'_, TestMessage, iced::Theme, iced::Renderer> {
        container(
            container(
                virtualized_list(state.data.iter().enumerate(), |(i, data)| {
                    let data_str = if (i / 100) % 2 == 0 {
                        format!("{}\n", data.0).repeat(data.1)
                    } else {
                        data.0.to_string()
                    };
                    container(
                        row![
                            text!("elem: (\n{data_str})"),
                            button("add").on_press(TestMessage::AddInElem(data.0, 1))
                        ]
                        .spacing(10),
                    )
                    .padding(5)
                    .style(|theme| container::success(theme))
                    .into()
                })
                .spacing(15)
                .on_scroll(TestMessage::Scroll)
                .set_id(Id::new("vl")),
            )
            .style(|theme| container::warning(theme)),
        )
        .padding(100)
        .into()
    }
}

#[derive(Debug, Clone)]
enum TestMessage {
    Add(u64),
    AddInElem(u64, u64),
    Nl(usize),
    Scroll(Pos),
    SetScroll(Pos),
}

fn main() {
    let filter = Targets::new()
        //.with_target("iced_helper", Level::DEBUG)
        .with_default(Level::INFO);

    tracing_subscriber::registry()
        .with(fmt::Layer::new())
        .with(filter)
        .init();

    iced::application::<TestState, TestMessage, Theme, Renderer>(
        || TestState {
            data: {
                let start = Instant::now();
                let count = 2_000u64;
                let result = (0..count).zip(std::iter::repeat(1)).collect();
                info!("load time: {:?}", start.elapsed());
                result
            },
        },
        |this: &mut TestState, message: TestMessage| -> Task<TestMessage> {
            match message {
                TestMessage::Add(add) => {
                    this.data.iter_mut().for_each(|(a, _)| *a += add);
                    Task::none()
                }
                TestMessage::Nl(a) => {
                    this.data.iter_mut().for_each(|(_, b)| *b += a);
                    Task::none()
                }
                TestMessage::AddInElem(i, add) => {
                    this.data.iter_mut().find(|(a, _)| *a == i).unwrap().0 += add;
                    Task::none()
                }
                TestMessage::Scroll(pos) => {
                    info!("{pos:?}");
                    Task::none()
                }
                TestMessage::SetScroll(pos) => scroll_to(Id::new("vl"), pos),
            }
        },
        TestState::view,
    )
    .theme(Theme::Dark)
    .subscription(|_: &TestState| {
        Subscription::batch(vec![
            time::repeat(|| async { TestMessage::Add(2) }, Duration::from_secs(2)),
            time::repeat(|| async { TestMessage::Nl(1) }, Duration::from_secs(5)),
            time::repeat(
                || async { TestMessage::SetScroll(Pos::new(4, 0.5)) },
                Duration::from_secs(10),
            ),
        ])
    })
    .run()
    .unwrap();
}
