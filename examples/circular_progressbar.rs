use std::f32::consts::PI;

use iced::{
    Alignment, Element, Theme,
    widget::{column, container, row, slider},
};
use iced_helper::widgets::circular_progressbar::{Thickness, circular_progressbar, style::Style};
use tracing::Level;
use tracing_subscriber::{filter::Targets, fmt, layer::SubscriberExt, util::SubscriberInitExt};

struct TestState {
    progress: f32,
}

impl TestState {
    fn view(&self) -> Element<'_, f32, iced::Theme, iced::Renderer> {
        column![
            slider(0.0..=1.0, self.progress, |a| a).step(0.001_f32),
            row![
                container(
                    circular_progressbar(self.progress)
                        .thickness(Thickness::Fixed(100.))
                        .start(PI)
                        .style(|theme: &iced::Theme| Style {
                            color: theme.extended_palette().success.base.color
                        })
                )
                .align_x(Alignment::Center)
                .align_y(Alignment::Center),
                container(
                    circular_progressbar(self.progress)
                        .thickness(Thickness::Relative(0.1))
                        .start(PI / 3.)
                        .style(|theme: &iced::Theme| Style {
                            color: theme.extended_palette().warning.base.color
                        })
                )
                .align_x(Alignment::Center)
                .align_y(Alignment::Center),
                container(
                    circular_progressbar(self.progress)
                        .thickness(Thickness::Full)
                        .start(-PI / 2.)
                        .style(|theme: &iced::Theme| Style {
                            color: theme.extended_palette().primary.base.color
                        })
                )
                .align_x(Alignment::Center)
                .align_y(Alignment::Center),
                container(
                    circular_progressbar(self.progress)
                        .start(-(PI / 2.) + (PI * 8. * self.progress))
                        .style(|theme: &iced::Theme| Style {
                            color: match self.progress {
                                1. => theme.extended_palette().success.base.color,
                                0.8..1. => theme.extended_palette().warning.base.color,
                                _ => theme.extended_palette().background.strong.color,
                            }
                        })
                )
                .align_x(Alignment::Center)
                .align_y(Alignment::Center),
            ]
            .spacing(20),
        ]
        .spacing(20)
        .padding(20)
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
        || TestState { progress: 1. },
        |this: &mut TestState, progress: f32| this.progress = progress,
        TestState::view,
    )
    .theme(Theme::Dark)
    .run()
    .unwrap();
}
