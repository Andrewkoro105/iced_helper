//! Build and show dropdown menus.
use iced::advanced::layout::{self, Layout};
use iced::advanced::renderer;
use iced::advanced::text::{self, Text};
use iced::advanced::widget::tree::{self, Tree};
use iced::alignment;
use iced::border::{self, Border};
use iced::mouse;
use iced::overlay;
use iced::touch;
use iced::widget::scrollable::{self, Scrollable};
use iced::window;
use iced::{
    Background, Color, Event, Length, Padding, Pixels, Point, Rectangle, Shadow, Size, Theme,
    Vector, advanced::Clipboard,
};
use iced::{Element, advanced::Shell, advanced::Widget};

pub struct OptionsStyle {
    pub text_color: Color,
    pub background: Background,
    pub border: Border,
}

pub struct Menu<'a, 'b, T, Message, Theme = iced::Theme, Renderer = iced::Renderer>
where
    Theme: Catalog,
    Renderer: text::Renderer,
    T: 'b,
    'b: 'a,
{
    state: &'a mut State,
    options: &'a [T],
    hovered_option: &'a mut Option<usize>,
    on_selected: Box<dyn FnMut(T) -> Message + 'a>,
    on_option_hovered: Option<&'a dyn Fn(T) -> Message>,
    width: f32,
    padding: Padding,
    text_size: Option<Pixels>,
    text_line_height: text::LineHeight,
    text_shaping: text::Shaping,
    font: Option<Renderer::Font>,
    class: &'a <Theme as Catalog>::Class<'b, T>,
}

impl<'a, 'b, T, Message, Theme, Renderer> Menu<'a, 'b, T, Message, Theme, Renderer>
where
    T: ToString + Clone + 'b,
    Message: 'a,
    Theme: Catalog + 'a,
    Renderer: text::Renderer + 'a,
    'b: 'a,
{
    pub fn new(
        state: &'a mut State,
        options: &'a [T],
        hovered_option: &'a mut Option<usize>,
        on_selected: impl FnMut(T) -> Message + 'a,
        on_option_hovered: Option<&'a dyn Fn(T) -> Message>,
        class: &'a <Theme as Catalog>::Class<'b, T>,
    ) -> Self {
        Menu {
            state,
            options,
            hovered_option,
            on_selected: Box::new(on_selected),
            on_option_hovered,
            width: 0.0,
            padding: Padding::ZERO,
            text_size: None,
            text_line_height: text::LineHeight::default(),
            text_shaping: text::Shaping::default(),
            font: None,
            class,
        }
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    pub fn padding<P: Into<Padding>>(mut self, padding: P) -> Self {
        self.padding = padding.into();
        self
    }

    pub fn text_size(mut self, text_size: impl Into<Pixels>) -> Self {
        self.text_size = Some(text_size.into());
        self
    }

    pub fn text_line_height(mut self, line_height: impl Into<text::LineHeight>) -> Self {
        self.text_line_height = line_height.into();
        self
    }

    pub fn text_shaping(mut self, shaping: text::Shaping) -> Self {
        self.text_shaping = shaping;
        self
    }

    pub fn font(mut self, font: impl Into<Renderer::Font>) -> Self {
        self.font = Some(font.into());
        self
    }

    pub fn overlay(
        self,
        position: Point,
        viewport: Rectangle,
        target_height: f32,
        menu_height: Length,
    ) -> overlay::Element<'a, Message, Theme, Renderer> {
        overlay::Element::new(Box::new(Overlay::new(
            position,
            viewport,
            self,
            target_height,
            menu_height,
        )))
    }
}

#[derive(Debug)]
pub struct State {
    tree: Tree,
}

impl State {
    pub fn new() -> Self {
        Self {
            tree: Tree::empty(),
        }
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

struct Overlay<'a, 'b, T, Message, Theme, Renderer>
where
    Theme: Catalog,
    Renderer: text::Renderer,
    T: 'b,
    'b: 'a,
{
    position: Point,
    viewport: Rectangle,
    tree: &'a mut Tree,
    list: Scrollable<'a, Message, Theme, Renderer>,
    width: f32,
    target_height: f32,
    class: &'a <Theme as Catalog>::Class<'b, T>,
}

impl<'a, 'b, T, Message, Theme, Renderer> Overlay<'a, 'b, T, Message, Theme, Renderer>
where
    Message: 'a,
    Theme: Catalog + scrollable::Catalog + 'a,
    Renderer: text::Renderer + 'a,
    T: Clone + ToString + 'b,
    'b: 'a,
{
    pub fn new(
        position: Point,
        viewport: Rectangle,
        menu: Menu<'a, 'b, T, Message, Theme, Renderer>,
        target_height: f32,
        menu_height: Length,
    ) -> Self {
        let Menu {
            state,
            options,
            hovered_option,
            on_selected,
            on_option_hovered,
            width,
            padding,
            font,
            text_size,
            text_line_height,
            text_shaping,
            class,
        } = menu;

        let list = Scrollable::new(List {
            options,
            hovered_option,
            on_selected,
            on_option_hovered,
            font,
            text_size,
            text_line_height,
            text_shaping,
            padding,
            class,
        })
        .height(menu_height);

        state.tree.diff(&list as &dyn Widget<_, _, _>);

        Self {
            position,
            viewport,
            tree: &mut state.tree,
            list,
            width,
            target_height,
            class,
        }
    }
}

impl<'a, 'b, T, Message, Theme, Renderer> iced::advanced::Overlay<Message, Theme, Renderer>
    for Overlay<'a, 'b, T, Message, Theme, Renderer>
where
    Theme: Catalog,
    Renderer: text::Renderer,
    T: 'b,
    'b: 'a,
{
    fn layout(&mut self, renderer: &Renderer, bounds: Size) -> layout::Node {
        let space_below = bounds.height - (self.position.y + self.target_height);
        let space_above = self.position.y;

        let limits = layout::Limits::new(
            Size::ZERO,
            Size::new(
                bounds.width - self.position.x,
                if space_below > space_above {
                    space_below
                } else {
                    space_above
                },
            ),
        )
        .width(self.width);

        let node = self.list.layout(self.tree, renderer, &limits);
        let size = node.size();

        node.move_to(if space_below > space_above {
            self.position + Vector::new(0.0, self.target_height)
        } else {
            self.position - Vector::new(0.0, size.height)
        })
    }

    fn update(
        &mut self,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
    ) {
        let bounds = layout.bounds();
        self.list.update(
            self.tree, event, layout, cursor, renderer, clipboard, shell, &bounds,
        );
    }

    fn mouse_interaction(
        &self,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.list
            .mouse_interaction(self.tree, layout, cursor, &self.viewport, renderer)
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        theme: &Theme,
        defaults: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
    ) {
        let bounds = layout.bounds();
        let style = Catalog::style(theme, self.class, Option::<&T>::None, Status::Active);

        renderer.fill_quad(
            renderer::Quad {
                bounds,
                border: style.border,
                shadow: style.shadow,
                ..renderer::Quad::default()
            },
            style.background,
        );

        self.list.draw(
            self.tree, renderer, theme, defaults, layout, cursor, &bounds,
        );
    }
}

struct List<'a, 'b, T, Message, Theme, Renderer>
where
    Theme: Catalog,
    Renderer: text::Renderer,
    T: 'b,
    'b: 'a,
{
    options: &'a [T],
    hovered_option: &'a mut Option<usize>,
    on_selected: Box<dyn FnMut(T) -> Message + 'a>,
    on_option_hovered: Option<&'a dyn Fn(T) -> Message>,
    padding: Padding,
    text_size: Option<Pixels>,
    text_line_height: text::LineHeight,
    text_shaping: text::Shaping,
    font: Option<Renderer::Font>,
    class: &'a <Theme as Catalog>::Class<'b, T>,
}

struct ListState {
    is_hovered: Option<bool>,
}

impl<'a, 'b, T, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for List<'a, 'b, T, Message, Theme, Renderer>
where
    T: Clone + ToString + 'b,
    Theme: Catalog,
    Renderer: text::Renderer,
    'b: 'a,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<Option<bool>>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(ListState { is_hovered: None })
    }

    fn size(&self) -> Size<Length> {
        Size {
            width: Length::Fill,
            height: Length::Shrink,
        }
    }

    fn layout(
        &mut self,
        _tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let text_size = self.text_size.unwrap_or_else(|| renderer.default_size());
        let text_line_height = self.text_line_height.to_absolute(text_size);

        let size = {
            let intrinsic = Size::new(
                0.0,
                (f32::from(text_line_height) + self.padding.y()) * self.options.len() as f32,
            );
            limits.resolve(Length::Fill, Length::Shrink, intrinsic)
        };

        layout::Node::new(size)
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) {
        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                if cursor.is_over(layout.bounds())
                    && let Some(index) = *self.hovered_option
                    && let Some(option) = self.options.get(index)
                {
                    shell.publish((self.on_selected)(option.clone()));
                    shell.capture_event();
                }
            }
            Event::Mouse(mouse::Event::CursorMoved { .. }) => {
                if let Some(cursor_position) = cursor.position_in(layout.bounds()) {
                    let text_size = self.text_size.unwrap_or_else(|| renderer.default_size());
                    let option_height =
                        f32::from(self.text_line_height.to_absolute(text_size)) + self.padding.y();
                    let new_hovered_option = (cursor_position.y / option_height) as usize;

                    if *self.hovered_option != Some(new_hovered_option)
                        && let Some(option) = self.options.get(new_hovered_option)
                    {
                        if let Some(on_option_hovered) = self.on_option_hovered {
                            shell.publish(on_option_hovered(option.clone()));
                        }
                        shell.request_redraw();
                    }
                    *self.hovered_option = Some(new_hovered_option);
                }
            }
            Event::Touch(touch::Event::FingerPressed { .. }) => {
                if let Some(cursor_position) = cursor.position_in(layout.bounds()) {
                    let text_size = self.text_size.unwrap_or_else(|| renderer.default_size());
                    let option_height =
                        f32::from(self.text_line_height.to_absolute(text_size)) + self.padding.y();
                    *self.hovered_option = Some((cursor_position.y / option_height) as usize);

                    if let Some(index) = *self.hovered_option
                        && let Some(option) = self.options.get(index)
                    {
                        shell.publish((self.on_selected)(option.clone()));
                        shell.capture_event();
                    }
                }
            }
            _ => {}
        }

        let state = tree.state.downcast_mut::<ListState>();
        if let Event::Window(window::Event::RedrawRequested(_now)) = event {
            state.is_hovered = Some(cursor.is_over(layout.bounds()));
        } else if state
            .is_hovered
            .is_some_and(|is_hovered| is_hovered != cursor.is_over(layout.bounds()))
        {
            shell.request_redraw();
        }
    }

    fn mouse_interaction(
        &self,
        _tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
        _renderer: &Renderer,
    ) -> mouse::Interaction {
        if cursor.is_over(layout.bounds()) {
            mouse::Interaction::Pointer
        } else {
            mouse::Interaction::default()
        }
    }

    fn draw(
        &self,
        _tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let bounds: Rectangle = layout.bounds();
        let text_size = self.text_size.unwrap_or_else(|| renderer.default_size());
        let option_height =
            f32::from(self.text_line_height.to_absolute(text_size)) + self.padding.y();

        let offset = viewport.y - bounds.y;
        let start = (offset / option_height) as usize;
        let end = ((offset + viewport.height) / option_height).ceil() as usize;

        let visible_options = &self.options[start..end.min(self.options.len())];

        for (i, option) in visible_options.iter().enumerate() {
            let i = start + i;
            let is_selected = *self.hovered_option == Some(i);
            let style = Catalog::style(
                theme,
                self.class,
                Some(option),
                Status::from_hovered(is_selected),
            );

            let bounds = Rectangle {
                x: bounds.x,
                y: bounds.y + (option_height * i as f32),
                width: bounds.width,
                height: option_height,
            };

            renderer.fill_quad(
                renderer::Quad {
                    bounds: Rectangle {
                        x: bounds.x + style.border.width,
                        width: bounds.width - style.border.width * 2.0,
                        ..bounds
                    },
                    border: border::rounded(style.border.radius),
                    ..renderer::Quad::default()
                },
                style.background,
            );

            renderer.fill_text(
                Text {
                    content: option.to_string(),
                    bounds: Size::new(f32::INFINITY, bounds.height),
                    size: text_size,
                    line_height: self.text_line_height,
                    font: self.font.unwrap_or_else(|| renderer.default_font()),
                    align_x: text::Alignment::Default,
                    align_y: alignment::Vertical::Center,
                    shaping: self.text_shaping,
                    wrapping: text::Wrapping::default(),
                },
                Point::new(bounds.x + self.padding.left, bounds.center_y()),
                style.text_color,
                *viewport,
            );
        }
    }
}

impl<'a, 'b, T, Message, Theme, Renderer> From<List<'a, 'b, T, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    T: ToString + Clone + 'b,
    Message: 'a,
    Theme: 'a + Catalog,
    Renderer: 'a + text::Renderer,
    'b: 'a,
{
    fn from(list: List<'a, 'b, T, Message, Theme, Renderer>) -> Self {
        Element::new(list)
    }
}

/// The appearance of a [`Menu`].
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Style {
    pub background: Background,
    pub border: Border,
    pub text_color: Color,
    pub shadow: Shadow,
}

pub enum Status {
    Active,
    Hovered,
}

impl Status {
    pub fn from_hovered(hovered: bool) -> Self {
        if hovered { Self::Hovered } else { Self::Active }
    }
}

/// The theme catalog of a [`Menu`].
pub trait Catalog: scrollable::Catalog {
    type Class<'a, T>;

    fn default<'a, T: 'a>() -> <Self as Catalog>::Class<'a, T>;

    fn default_scrollable<'a>() -> <Self as scrollable::Catalog>::Class<'a> {
        <Self as scrollable::Catalog>::default()
    }

    fn style<'a, T>(
        &self,
        class: &<Self as Catalog>::Class<'a, T>,
        option: Option<&T>,
        status: Status,
    ) -> Style;
}

/// A styling function for a [`Menu`].
pub type StyleFn<'a, Theme, T> = Box<dyn for<'x> Fn(&Theme, Option<&'x T>, Status) -> Style + 'a>;

impl Catalog for Theme {
    type Class<'a, T> = StyleFn<'a, Self, T>;

    fn default<'a, T: 'a>() -> StyleFn<'a, Self, T> {
        Box::new(default::<T>)
    }

    fn style<'a, T>(
        &self,
        class: &<Self as Catalog>::Class<'a, T>,
        option: Option<&T>,
        status: Status,
    ) -> Style {
        class(self, option, status)
    }
}

/// The default style of the list of a [`Menu`].
pub fn default<T>(theme: &Theme, _: Option<&T>, status: Status) -> Style {
    let palette = theme.extended_palette();

    match status {
        Status::Active => Style {
            background: palette.background.weak.color.into(),
            border: Border {
                width: 1.0,
                radius: 0.0.into(),
                color: palette.background.strong.color,
            },
            text_color: palette.background.weak.text,
            shadow: Shadow::default(),
        },
        Status::Hovered => Style {
            background: palette.primary.strong.color.into(),
            border: Border {
                width: 1.0,
                radius: 0.0.into(),
                color: palette.background.strong.color,
            },
            text_color: palette.primary.strong.text,
            shadow: Shadow::default(),
        },
    }
}
