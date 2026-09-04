pub mod api;
pub mod style;

pub use api::*;

use iced::{
    Element, Length, Rectangle, Size, Transformation, Vector,
    advanced::{
        Layout, Renderer, Widget,
        graphics::{
            Mesh, color,
            mesh::{self, SolidVertex2D},
        },
        layout::{Limits, Node},
        renderer::Style,
        widget::Tree,
    },
    mouse,
};
use std::f32::consts::PI;

pub struct CircularProgressbar<'elem, T: style::Catalog> {
    progress: f32,
    thickness: Thickness,
    start: f32,
    size: Size<Length>,
    class: T::Class<'elem>,
}

pub enum Thickness {
    Full,
    Relative(f32),
    Fixed(f32)
}

impl Thickness {
    pub fn get_thickness(&self, r: f32) -> f32 {
        match self {
            Thickness::Full => r,
            Thickness::Relative(ratio) => ratio.clamp(0., 1.) * r,
            Thickness::Fixed(thickness) => thickness.clamp(0., r),
        }
    }
}

impl<'elem, M, T: style::Catalog + 'elem, R: Renderer + mesh::Renderer>
    From<CircularProgressbar<'elem, T>> for Element<'elem, M, T, R>
{
    fn from(value: CircularProgressbar<'elem, T>) -> Self {
        Element::new(value)
    }
}

impl<'elem, M, T: style::Catalog, R: Renderer + mesh::Renderer> Widget<M, T, R>
    for CircularProgressbar<'elem, T>
{
    fn size(&self) -> Size<Length> {
        self.size
    }

    fn layout(&mut self, _tree: &mut Tree, _renderer: &R, limits: &Limits) -> Node {
        let size = limits.max().height.min(limits.max().width);
        Node::new(Size {
            width: size,
            height: size,
        })
    }

    fn draw(
        &self,
        _tree: &Tree,
        renderer: &mut R,
        theme: &T,
        _style: &Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        let style = theme.style(&self.class);

        let r = layout.bounds().height.min(layout.bounds().width) / 2.;
        let full_count_points = (PI * r.sqrt()) as u32;
        let count_points = (full_count_points as f32 * self.progress) as u32 + 2;

        let thickness = self.thickness.get_thickness(r);

        if count_points > 1 {
            let mesh = Mesh::Solid {
                buffers: mesh::Indexed {
                    vertices: [SolidVertex2D {
                        position: [0., 0.],
                        color: color::pack(style.color),
                    }]
                    .into_iter()
                    .chain(
                        (0..count_points)
                            .map(|i| {
                                (if i == count_points - 1 {
                                    PI * 2. * self.progress
                                } else {
                                    (PI * 2. / full_count_points as f32) * i as f32
                                }) + self.start
                            })
                            .flat_map(|rad| [SolidVertex2D {
                                position: [rad.cos() * r, rad.sin() * r],
                                color: color::pack(style.color),
                            }, SolidVertex2D {
                                position: [rad.cos() * (r - thickness), rad.sin() * (r - thickness)],
                                color: color::pack(style.color),
                            }, ]),
                    )
                    .collect(),
                    indices: (1..=(count_points*2))
                        .collect::<Vec<_>>()
                        .windows(3)
                        .flatten()
                        .cloned()
                        .collect(),
                },
                transformation: Transformation::IDENTITY,
                clip_bounds: Rectangle::INFINITE,
            };
            renderer.with_translation(
                Vector::new(layout.bounds().x + r, layout.bounds().y + r),
                |renderer| {
                    renderer.draw_mesh(mesh);
                },
            );
        }
    }
}
