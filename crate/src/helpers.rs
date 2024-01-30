use crate::{PrettierImage, Rgb};
use image::DynamicImage::ImageRgba8;
use image::{DynamicImage, ImageBuffer};

#[cfg(feature = "enable_wasm")]
extern crate wasm_bindgen;

pub fn square_distance(color1: Rgb, color2: Rgb) -> i32 {
    let (r1, g1, b1) = (color1.r as i32, color1.g as i32, color1.b as i32);
    let (r2, g2, b2) = (color2.r as i32, color2.g as i32, color2.b as i32);
    i32::pow(r1 - r2, 2) + i32::pow(g1 - g2, 2) + i32::pow(b1 - b2, 2)
}

pub fn open_dyn_image(img_path: &'static str) -> DynamicImage {
    image::open(img_path).unwrap()
}

pub fn save_dyn_image(img: DynamicImage, filtered_img_path: &str) {
    img.save(filtered_img_path).unwrap();
}

pub fn get_pixels(img: DynamicImage) -> Vec<u8> {
    img.to_bytes()
}

pub fn dyn_image_from_raw(prettier_image: &PrettierImage) -> DynamicImage {
    let _len_vec = prettier_image.raw_pixels.len() as u128;
    let raw_pixels = &prettier_image.raw_pixels;
    let img_buffer = ImageBuffer::from_vec(
        prettier_image.width,
        prettier_image.height,
        raw_pixels.to_vec(),
    )
    .unwrap();
    ImageRgba8(img_buffer)
}
