use iced::widget::button::{Catalog as ButtonCatalog, Status as ButtonStatus};
use iced::widget::{container, container::{Catalog, Style}};
use iced::widget::tooltip;
use iced::{Border, Element};

pub fn my_tooltip<M: 'static, T, R>(
    base_elem: impl Into<Element<'static, M, T, R>>,
    tooltip_elem: impl Into<Element<'static, M, T, R>>,
) -> Element<'static, M, T, R>
where
    R: iced::advanced::text::Renderer + 'static,
    T: Catalog + ButtonCatalog + 'static,
    <T as Catalog>::Class<'static>: From<Box<dyn for<'a> Fn(&'a T) -> Style>>,
{
    tooltip(
        base_elem,
        container(tooltip_elem).padding(4).style(|theme| {
            let base_style = ButtonCatalog::style(
                theme,
                &<T as ButtonCatalog>::default(),
                ButtonStatus::Disabled,
            );

            let mut result = Catalog::style(theme, &<T as Catalog>::default());
            result.background = base_style.background;
            result.border = Border {
                color: base_style.text_color,
                width: 2.,
                radius: base_style.border.radius,
            };
            result
        }),
        tooltip::Position::Bottom,
    )
    .into()
}
