use iced::{Background, Border, Shadow, Theme, advanced::renderer::Quad};

pub enum Status {
    Active,
    Hovered,
    Dragged,
}

pub trait Catalog: Sized {
    type Class<'a>;

    fn default<'a>() -> Self::Class<'a>;

    fn style(&self, status: Status, item: &Self::Class<'_>) -> Style;
}

pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme, Status) -> Style + 'a>;

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(|theme, status| match status {
            Status::Active => Style {
                background: Background::Color(theme.extended_palette().background.weak.color),
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
            Status::Hovered => Style {
                background: Background::Color(theme.extended_palette().background.weak.color),
                border: Default::default(),
                shadow: Default::default(),
                snap: Quad::default().snap,
                scroller_background: Background::Color(
                    theme.extended_palette().primary.base.color,
                ),
                scroller_border: Default::default(),
                scroller_shadow: Default::default(),
                scroller_snap: Quad::default().snap,
            },
            Status::Dragged => Style {
                background: Background::Color(theme.extended_palette().background.weak.color),
                border: Default::default(),
                shadow: Default::default(),
                snap: Quad::default().snap,
                scroller_background: Background::Color(
                    theme.extended_palette().primary.strong.color,
                ),
                scroller_border: Default::default(),
                scroller_shadow: Default::default(),
                scroller_snap: Quad::default().snap,
            },
        })
    }

    fn style(&self, status: Status, class: &Self::Class<'_>) -> Style {
        class(self, status)
    }
}

pub struct Style {
    pub background: Background,
    pub border: Border,
    pub shadow: Shadow,
    pub snap: bool,

    pub scroller_background: Background,
    pub scroller_border: Border,
    pub scroller_shadow: Shadow,
    pub scroller_snap: bool,
}
