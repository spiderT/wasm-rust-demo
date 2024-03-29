use crate::iter::ImageIterator;
use crate::{helpers, PrettierImage, Rgb};
use image::GenericImageView;
use image::Pixel as ImagePixel;
use palette::{FromColor, IntoColor};
use palette::{Hsla, Hsva, Hue, Lcha, Saturate, Shade, Srgba};

#[cfg(feature = "enable_wasm")]
use wasm_bindgen::prelude::*;

// LCh颜色空间中的图像处理
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn lch(prettier_image: &mut PrettierImage, mode: &str, amt: f32) {
    let img = helpers::dyn_image_from_raw(prettier_image);
    let (width, height) = img.dimensions();
    let mut img = img.to_rgba8();

    for (x, y) in ImageIterator::new(width, height) {
        let px_data = img.get_pixel(x, y).channels();
        let lch_colour: Lcha = Srgba::new(
            px_data[0] as f32 / 255.0,
            px_data[1] as f32 / 255.0,
            px_data[2] as f32 / 255.0,
            px_data[3] as f32 / 255.0,
        )
        .into_linear()
        .into_color();

        let new_color = match mode {
            "desaturate" => lch_colour.desaturate(amt),
            "saturate" => lch_colour.saturate(amt),
            "lighten" => lch_colour.lighten(amt),
            "darken" => lch_colour.darken(amt),
            "shift_hue" => lch_colour.shift_hue(amt * 360.0),
            _ => lch_colour.saturate(amt),
        };
        let final_color: Srgba = Srgba::from_linear(new_color.into_color()).into_format();

        let components = final_color.into_components();

        img.put_pixel(
            x,
            y,
            image::Rgba([
                (components.0 * 255.0) as u8,
                (components.1 * 255.0) as u8,
                (components.2 * 255.0) as u8,
                (components.3 * 255.0) as u8,
            ]),
        );
    }
    prettier_image.raw_pixels = img.to_vec();
}

// HSL颜色空间中的图像操作
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn hsl(prettier_image: &mut PrettierImage, mode: &str, amt: f32) {
    let mut img = helpers::dyn_image_from_raw(prettier_image).to_rgba8();
    for (x, y) in ImageIterator::with_dimension(&img.dimensions()) {
        let px_data = img.get_pixel(x, y).channels();

        let colour = Srgba::new(
            px_data[0] as f32 / 255.0,
            px_data[1] as f32 / 255.0,
            px_data[2] as f32 / 255.0,
            px_data[3] as f32 / 255.0,
        );

        let hsl_colour = Hsla::from_color(colour);

        let new_color = match mode {
            "desaturate" => hsl_colour.desaturate(amt),
            "saturate" => hsl_colour.saturate(amt),
            "lighten" => hsl_colour.lighten(amt),
            "darken" => hsl_colour.darken(amt),
            "shift_hue" => hsl_colour.shift_hue(amt * 360.0),
            _ => hsl_colour.saturate(amt),
        };
        let final_color = Srgba::from_color(new_color);

        let components = final_color.into_components();

        img.put_pixel(
            x,
            y,
            image::Rgba([
                (components.0 * 255.0) as u8,
                (components.1 * 255.0) as u8,
                (components.2 * 255.0) as u8,
                (components.3 * 255.0) as u8,
            ]),
        );
    }

    prettier_image.raw_pixels = img.to_vec();
}

// HSV颜色空间中的图像操作
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn hsv(prettier_image: &mut PrettierImage, mode: &str, amt: f32) {
    let img = helpers::dyn_image_from_raw(prettier_image);
    let (width, height) = img.dimensions();
    let mut img = img.to_rgba8();

    for (x, y) in ImageIterator::new(width, height) {
        let px_data = img.get_pixel(x, y).channels();

        let color = Srgba::new(
            px_data[0] as f32 / 255.0,
            px_data[1] as f32 / 255.0,
            px_data[2] as f32 / 255.0,
            px_data[3] as f32 / 255.0,
        );

        let hsv_colour = Hsva::from_color(color);

        let new_color = match mode {
            "desaturate" => hsv_colour.desaturate(amt),
            "saturate" => hsv_colour.saturate(amt),
            "lighten" => hsv_colour.lighten(amt),
            "darken" => hsv_colour.darken(amt),
            "shift_hue" => hsv_colour.shift_hue(amt * 360.0),
            _ => hsv_colour.saturate(amt),
        };

        let srgba_new_color = Srgba::from_color(new_color);

        let components = srgba_new_color.into_components();

        img.put_pixel(
            x,
            y,
            image::Rgba([
                (components.0 * 255.0) as u8,
                (components.1 * 255.0) as u8,
                (components.2 * 255.0) as u8,
                (components.3 * 255.0) as u8,
            ]),
        );
    }
    prettier_image.raw_pixels = img.to_vec();
}

#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn hue_rotate_hsl(img: &mut PrettierImage, degrees: f32) {
    hsl(img, "shift_hue", degrees);
}

#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn hue_rotate_hsv(img: &mut PrettierImage, degrees: f32) {
    hsv(img, "shift_hue", degrees);
}

#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn hue_rotate_lch(img: &mut PrettierImage, degrees: f32) {
    lch(img, "shift_hue", degrees)
}

#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn saturate_hsl(img: &mut PrettierImage, level: f32) {
    hsl(img, "saturate", level)
}

#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn saturate_lch(img: &mut PrettierImage, level: f32) {
    lch(img, "saturate", level)
}

#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn saturate_hsv(img: &mut PrettierImage, level: f32) {
    hsv(img, "saturate", level)
}

#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn lighten_lch(img: &mut PrettierImage, level: f32) {
    lch(img, "lighten", level)
}

#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn lighten_hsl(img: &mut PrettierImage, level: f32) {
    hsl(img, "lighten", level)
}

#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn lighten_hsv(img: &mut PrettierImage, level: f32) {
    hsv(img, "lighten", level)
}

#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn darken_lch(img: &mut PrettierImage, level: f32) {
    lch(img, "darken", level)
}

#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn darken_hsl(img: &mut PrettierImage, level: f32) {
    hsl(img, "darken", level)
}

#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn darken_hsv(img: &mut PrettierImage, level: f32) {
    hsv(img, "darken", level)
}

#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn desaturate_hsv(img: &mut PrettierImage, level: f32) {
    hsv(img, "desaturate", level)
}

#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn desaturate_hsl(img: &mut PrettierImage, level: f32) {
    hsl(img, "desaturate", level)
}

#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn desaturate_lch(img: &mut PrettierImage, level: f32) {
    lch(img, "desaturate", level)
}

// 将图像与单一颜色混合，传递“不透明度
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn mix_with_colour(prettier_image: &mut PrettierImage, mix_colour: Rgb, opacity: f32) {
    let img = helpers::dyn_image_from_raw(prettier_image);
    let (width, height) = img.dimensions();
    let mut img = img.to_rgba8();

    let mix_red_offset = mix_colour.r as f32 * opacity;
    let mix_green_offset = mix_colour.g as f32 * opacity;
    let mix_blue_offset = mix_colour.b as f32 * opacity;
    let factor = 1.0 - opacity;

    for (x, y) in ImageIterator::new(width, height) {
        let px = img.get_pixel(x, y);
        let channels = px.channels();

        let r_value = mix_red_offset + (channels[0] as f32) * factor;
        let g_value = mix_green_offset + (channels[1] as f32) * factor;
        let b_value = mix_blue_offset + (channels[2] as f32) * factor;
        let alpha = channels[3];
        img.put_pixel(
            x,
            y,
            image::Rgba([r_value as u8, g_value as u8, b_value as u8, alpha]),
        );
    }
    prettier_image.raw_pixels = img.to_vec();
}
