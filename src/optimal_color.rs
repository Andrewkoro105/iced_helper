use iced::Color;

pub fn text_color(color: &Color) -> Color {
    let bg_luminance = color.relative_luminance();
    let black_luminance = Color::BLACK.relative_luminance();
    let white_luminance = Color::WHITE.relative_luminance();

    let contrast_with_black = contrast_ratio(bg_luminance, black_luminance);
    let contrast_with_white = contrast_ratio(bg_luminance, white_luminance);

    if contrast_with_black > contrast_with_white {
        Color::BLACK
    } else {
        Color::WHITE
    }
}

fn contrast_ratio(l1: f32, l2: f32) -> f32 {
    let (lighter, darker) = if l1 > l2 { (l1, l2) } else { (l2, l1) };
    (lighter + 0.05) / (darker + 0.05)
}

pub fn border_color(bg: Color, intensity: f32) -> Color {
    let intensity = intensity.clamp(0.0, 1.0);
    let luminance = bg.relative_luminance();

    if luminance > 0.5 {
        Color {
            r: bg.r * (1.0 - intensity),
            g: bg.g * (1.0 - intensity),
            b: bg.b * (1.0 - intensity),
            a: bg.a,
        }
    } else {
        Color {
            r: bg.r + (1.0 - bg.r) * intensity,
            g: bg.g + (1.0 - bg.g) * intensity,
            b: bg.b + (1.0 - bg.b) * intensity,
            a: bg.a,
        }
    }
}
