use iced::widget::{button, container, progress_bar, row, text};
use iced::{Alignment, Element, Length};

pub fn start_progress<'elem, P, M, T, R>(
    progress: Option<(P, impl ToString)>,
    start: M,
    stop: M,
    height: impl Into<Length>,
) -> Element<'elem, M, T, R>
where
    P: Clone,
    f32: From<P>,
    R: iced::advanced::Renderer + iced::advanced::text::Renderer + 'elem,
    T: container::Catalog + text::Catalog + button::Catalog + progress_bar::Catalog + 'elem,
    M: Clone + 'elem,
{
    container(if let Some((progress, comment)) = progress {
        row![
            text(format!(
                "{}: {:.2}%",
                comment.to_string(),
                f32::from(progress.clone()) * 100.
            )),
            progress_bar(0f32..=1f32, progress.into()),
            button("Стоп").on_press(stop),
        ]
        .spacing(20)
        .into()
    } else {
        Element::from(button("Старт").on_press(start))
    })
    .align_y(Alignment::End)
    .align_x(Alignment::End)
    .height(height)
    .width(Length::Fill)
    .into()
}
