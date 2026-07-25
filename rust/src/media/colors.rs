use dominant_color_rs::{Settings, dominant_colors};
use image::DynamicImage;

pub fn get_colors(img: DynamicImage) -> Vec<[f32; 3]> {
    let settings = Settings::default();
    
    let colors = dominant_colors(&img, &settings);
    colors
}