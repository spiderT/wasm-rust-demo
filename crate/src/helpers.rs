use crate::PrettierImage;
use image::DynamicImage::ImageRgba8;
use image::{DynamicImage, ImageBuffer};

#[cfg(feature = "enable_wasm")]
extern crate wasm_bindgen;

// 转换为DynamicImage类型
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
