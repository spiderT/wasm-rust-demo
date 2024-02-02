use crate::helpers;
use crate::iter::ImageIterator;
use crate::PrettierImage;
use image::Pixel;
use image::{GenericImage, GenericImageView};

#[cfg(feature = "enable_wasm")]
use wasm_bindgen::prelude::*;

// 将图像的亮度增加一个因子
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn inc_brightness(prettier_image: &mut PrettierImage, brightness: u8) {
    let end = prettier_image.get_raw_pixels().len() - 4;
    for i in (0..end).step_by(4) {
        let r_val = prettier_image.raw_pixels[i];
        let g_val = prettier_image.raw_pixels[i + 1];
        let b_val = prettier_image.raw_pixels[i + 2];

        if r_val <= 255 - brightness {
            prettier_image.raw_pixels[i] += brightness;
        } else {
            prettier_image.raw_pixels[i] = 255;
        }
        if g_val <= 255 - brightness {
            prettier_image.raw_pixels[i + 1] += brightness;
        } else {
            prettier_image.raw_pixels[1] = 255
        }

        if b_val <= 255 - brightness {
            prettier_image.raw_pixels[i + 2] += brightness;
        } else {
            prettier_image.raw_pixels[i + 2] = 255
        }
    }
}

// 通过因子调整图像的对比度
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn adjust_contrast(prettier_image: &mut PrettierImage, contrast: f32) {
    let mut img = helpers::dyn_image_from_raw(prettier_image);

    let clamped_contrast = contrast.clamp(-255.0, 255.0);

    let factor = (259.0 * (clamped_contrast + 255.0)) / (255.0 * (259.0 - clamped_contrast));
    let mut lookup_table: Vec<u8> = vec![0; 256];
    let offset = -128.0 * factor + 128.0;

    for (i, table) in lookup_table.iter_mut().enumerate().take(256_usize) {
        let new_val = i as f32 * factor + offset;
        *table = new_val.clamp(0.0, 255.0) as u8;
    }

    for (x, y) in ImageIterator::with_dimension(&img.dimensions()) {
        let mut px = img.get_pixel(x, y);
        let channels = px.channels();

        px = image::Rgba([
            lookup_table[channels[0] as usize],
            lookup_table[channels[1] as usize],
            lookup_table[channels[2] as usize],
            255,
        ]);
        img.put_pixel(x, y, px);
    }

    prettier_image.raw_pixels = img.to_bytes();
}
