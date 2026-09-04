use std::f32::consts::PI;

use iced::Size;

use crate::widgets::circular_progressbar::{CircularProgressbar, Thickness, style};

pub fn circular_progressbar<'elem, T: super::style::Catalog>(progress: f32) -> CircularProgressbar<'elem, T> {
    CircularProgressbar::new(progress)
}

impl<'elem, T: super::style::Catalog> CircularProgressbar<'elem, T> {
    pub fn new(progress: f32) -> Self {
        Self {
            progress,
            thickness: Thickness::Relative(0.3),
            start: -(PI / 2.),
            size: Size::new(iced::Length::Fill, iced::Length::Fill),
            class: T::default(),
        }
    }

    pub fn start(mut self, start: f32) -> Self {
        self.start = start;
        self
    }

    pub fn thickness(mut self, thickness: Thickness) -> Self {
        self.thickness = thickness;
        self
    }

    pub fn style(mut self, style: impl Fn(&T) -> style::Style + 'elem) -> Self
    where
        <T as style::Catalog>::Class<'elem>: From<style::StyleFn<'elem, T>>,
    {
        self.class = (Box::new(style) as style::StyleFn<'elem, T>).into();
        self
    }

    pub fn class(mut self, class: impl Into<T::Class<'elem>>) -> Self
    {
        self.class = class.into();
        self
    }
}
