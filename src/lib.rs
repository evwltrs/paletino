extern crate image;

use image::GenericImageView;

mod app;
pub use app::App;

#[derive(Clone, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b }
    }
}

pub fn median_cut(image: &image::DynamicImage, num_colors: usize) -> Vec<Color> {
    let pixels = image
        .pixels()
        .map(|p| {
            let rgba = p.2;
            Color::new(rgba[0], rgba[1], rgba[2])
        })
        .collect::<Vec<Color>>();

    let mut colors = vec![pixels];

    while colors.len() < num_colors {
        let len = colors.len();
        for i in 0..len {
            let mut color_group = colors.swap_remove(i);
            color_group.sort_by(|a, b| {
                let ar = a.r as u32;
                let ag = a.g as u32;
                let ab = a.b as u32;
                let br = b.r as u32;
                let bg = b.g as u32;
                let bb = b.b as u32;
                (ar * ar + ag * ag + ab * ab).cmp(&(br * br + bg * bg + bb * bb))
            });
            let split_index = color_group.len() / 2;
            let (group1, group2) = color_group.split_at_mut(split_index);
            colors.push(group1.to_vec());
            colors.push(group2.to_vec());
        }
    }

    let mut palette = Vec::new();
    for color_group in colors.iter() {
        let sum_r: u32 = color_group.iter().map(|c| c.r as u32).sum();
        let sum_g: u32 = color_group.iter().map(|c| c.g as u32).sum();
        let sum_b: u32 = color_group.iter().map(|c| c.b as u32).sum();
        let num_colors = color_group.len() as u32;
        let avg_color = Color::new(
            (sum_r / num_colors) as u8,
            (sum_g / num_colors) as u8,
            (sum_b / num_colors) as u8,
        );
        palette.push(avg_color);
    }

    palette
}
