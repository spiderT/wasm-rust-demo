use image::Pixel as OtherPixel;

use image::{GenericImage, GenericImageView};

use crate::helpers;
use crate::iter::ImageIterator;
use crate::{PrettierImage, Rgb};
use palette::{FromColor, IntoColor};
use palette::{Hue, Lab, Lch, Saturate, Shade, Srgb, Srgba};

#[cfg(feature = "enable_wasm")]
use wasm_bindgen::prelude::*;

#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn alter_channel(img: &mut PrettierImage, channel: usize, amt: i16) {
    if channel > 2 {
        panic!("Invalid channel index passed. Channel must be 0, 1, or 2 (Red=0, Green=1, Blue=2)");
    }
    if amt > 255 {
        panic!("Amount to increment/decrement should be between -255 and 255");
    }
    let end = img.raw_pixels.len();

    for i in (channel..end).step_by(4) {
        let inc_val: i16 = img.raw_pixels[i] as i16 + amt;
        img.raw_pixels[i] = inc_val.clamp(0, 255) as u8;
    }
}

#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn alter_red_channel(prettier_image: &mut PrettierImage, amt: i16) {
    alter_channel(prettier_image, 0, amt)
}

#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn alter_green_channel(img: &mut PrettierImage, amt: i16) {
    alter_channel(img, 1, amt)
}

#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn alter_blue_channel(img: &mut PrettierImage, amt: i16) {
    alter_channel(img, 2, amt)
}

#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn alter_two_channels(
    img: &mut PrettierImage,
    channel1: usize,
    amt1: i16,
    channel2: usize,
    amt2: i16,
) {
    if channel1 > 2 {
        panic!("Invalid channel index passed. Channel1 must be equal to 0, 1, or 2.");
    }
    if channel2 > 2 {
        panic!("Invalid channel index passed. Channel2 must be equal to 0, 1, or 2");
    }
    if amt1 > 255 {
        panic!("Amount to inc/dec channel by should be between -255 and 255");
    }
    if amt2 > 255 {
        panic!("Amount to inc/dec channel by should be between -255 and 255");
    }
    let end = img.raw_pixels.len();

    for i in (0..end).step_by(4) {
        let inc_val1: i16 = img.raw_pixels[i + channel1] as i16 + amt1;
        let inc_val2: i16 = img.raw_pixels[i + channel2] as i16 + amt2;

        img.raw_pixels[i + channel1] = inc_val1.clamp(0, 255) as u8;
        img.raw_pixels[i + channel2] = inc_val2.clamp(0, 255) as u8;
    }
}

#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn alter_channels(img: &mut PrettierImage, r_amt: i16, g_amt: i16, b_amt: i16) {
    if r_amt > 255 {
        panic!("Invalid r_amt passed. Amount to inc/dec channel by should be between -255 and 255");
    }
    if g_amt > 255 {
        panic!("Invalid g_amt passed. Amount to inc/dec channel by should be between -255 and 255");
    }
    if b_amt > 255 {
        panic!("Invalid b_amt passed. Amount to inc/dec channel by should be between -255 and 255");
    }
    let end = img.raw_pixels.len();

    for i in (0..end).step_by(4) {
        let r_val: i16 = img.raw_pixels[i] as i16 + r_amt;
        let g_val: i16 = img.raw_pixels[i + 1] as i16 + g_amt;
        let b_val: i16 = img.raw_pixels[i + 2] as i16 + b_amt;

        img.raw_pixels[i] = r_val.clamp(0, 255) as u8;
        img.raw_pixels[i + 1] = g_val.clamp(0, 255) as u8;
        img.raw_pixels[i + 2] = b_val.clamp(0, 255) as u8;
    }
}

#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn remove_channel(img: &mut PrettierImage, channel: usize, min_filter: u8) {
    if channel > 2 {
        panic!("Invalid channel index passed. Channel must be equal to 0, 1, or 2.");
    }
    let end = img.raw_pixels.len();
    for i in (channel..end).step_by(4) {
        if img.raw_pixels[i] < min_filter {
            img.raw_pixels[i] = 0;
        };
    }
}

#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn remove_red_channel(img: &mut PrettierImage, min_filter: u8) {
    remove_channel(img, 0, min_filter)
}

#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn remove_green_channel(img: &mut PrettierImage, min_filter: u8) {
    remove_channel(img, 1, min_filter)
}

#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn remove_blue_channel(img: &mut PrettierImage, min_filter: u8) {
    remove_channel(img, 2, min_filter)
}

#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn swap_channels(img: &mut PrettierImage, mut channel1: usize, mut channel2: usize) {
    if channel1 > 2 {
        panic!("Invalid channel index passed. Channel1 must be equal to 0, 1, or 2.");
    }
    if channel2 > 2 {
        panic!("Invalid channel index passed. Channel2 must be equal to 0, 1, or 2.");
    }
    let end = img.raw_pixels.len();

    if channel1 > channel2 {
        std::mem::swap(&mut channel1, &mut channel2);
    }

    for i in (channel1..end).step_by(4) {
        let difference = channel2 - channel1;

        img.raw_pixels.swap(i, i + difference);
    }
}

#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn invert(prettier_image: &mut PrettierImage) {
    let end = prettier_image.get_raw_pixels().len();

    for i in (0..end).step_by(4) {
        let r_val = prettier_image.raw_pixels[i];
        let g_val = prettier_image.raw_pixels[i + 1];
        let b_val = prettier_image.raw_pixels[i + 2];

        prettier_image.raw_pixels[i] = 255 - r_val;
        prettier_image.raw_pixels[i + 1] = 255 - g_val;
        prettier_image.raw_pixels[i + 2] = 255 - b_val;
    }
}

#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn selective_hue_rotate(prettier_image: &mut PrettierImage, ref_color: Rgb, degrees: f32) {
    let img = helpers::dyn_image_from_raw(prettier_image);
    let (width, height) = img.dimensions();

    let mut img = img.to_rgba8();
    for (x, y) in ImageIterator::new(width, height) {
        let px = img.get_pixel(x, y);

        let lab: Lab = Srgb::new(
            ref_color.r as f32 / 255.0,
            ref_color.g as f32 / 255.0,
            ref_color.b as f32 / 255.0,
        )
        .into_color();
        let channels = px.channels();
        let r_val: f32 = channels[0] as f32 / 255.0;
        let g_val: f32 = channels[1] as f32 / 255.0;
        let b_val: f32 = channels[2] as f32 / 255.0;

        let px_lab: Lab = Srgb::new(r_val, g_val, b_val).into_color();

        let sim = color_sim(lab, px_lab);
        if sim > 0 && sim < 40 {
            let px_data = img.get_pixel(x, y).channels();
            let color = Srgba::new(
                px_data[0] as f32,
                px_data[1] as f32,
                px_data[2] as f32,
                255.0,
            );
            let hue_rotated_color = Lch::from_color(color).shift_hue(degrees);

            let final_color: Srgba =
                Srgba::from_linear(hue_rotated_color.into_color()).into_format();

            let components = final_color.into_components();

            img.put_pixel(
                x,
                y,
                image::Rgba([
                    (components.0 * 255.0) as u8,
                    (components.1 * 255.0) as u8,
                    (components.2 * 255.0) as u8,
                    255,
                ]),
            );
        }
    }

    prettier_image.raw_pixels = img.to_vec();
}

#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn selective_color_convert(
    prettier_image: &mut PrettierImage,
    ref_color: Rgb,
    new_color: Rgb,
    fraction: f32,
) {
    let buffer = prettier_image.raw_pixels.as_mut_slice();

    let ref_lab: Lab = Srgb::new(
        ref_color.r as f32 / 255.0,
        ref_color.g as f32 / 255.0,
        ref_color.b as f32 / 255.0,
    )
    .into_color();

    for px in buffer.chunks_mut(4) {
        let px_lab: Lab = Srgb::new(
            px[0] as f32 / 255.0,
            px[1] as f32 / 255.0,
            px[2] as f32 / 255.0,
        )
        .into_color();
        let sim = color_sim(ref_lab, px_lab);

        if sim > 0 && sim < 40 {
            px[0] = ((px[0] as f32) + fraction * ((new_color.r as f32) - (px[0] as f32)))
                .clamp(0.0, 255.0) as u8;
            px[1] = ((px[1] as f32) + fraction * ((new_color.g as f32) - (px[1] as f32)))
                .clamp(0.0, 255.0) as u8;
            px[2] = ((px[2] as f32) + fraction * ((new_color.b as f32) - (px[2] as f32)))
                .clamp(0.0, 255.0) as u8;
        }
    }
}

#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn selective_lighten(img: &mut PrettierImage, ref_color: Rgb, amt: f32) {
    selective(img, "lighten", ref_color, amt)
}

#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn selective_desaturate(img: &mut PrettierImage, ref_color: Rgb, amt: f32) {
    selective(img, "desaturate", ref_color, amt)
}

#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn selective_saturate(img: &mut PrettierImage, ref_color: Rgb, amt: f32) {
    selective(img, "saturate", ref_color, amt);
}

fn selective(prettier_image: &mut PrettierImage, mode: &'static str, ref_color: Rgb, amt: f32) {
    let img = helpers::dyn_image_from_raw(prettier_image);
    let (width, height) = img.dimensions();
    let mut img = img.to_rgba8();

    for (x, y) in ImageIterator::new(width, height) {
        let px = img.get_pixel(x, y);

        let lab: Lab = Srgb::new(
            ref_color.r as f32 / 255.0,
            ref_color.g as f32 / 255.0,
            ref_color.b as f32 / 255.0,
        )
        .into_color();
        let channels = px.channels();
        let r_val: f32 = channels[0] as f32 / 255.0;
        let g_val: f32 = channels[1] as f32 / 255.0;
        let b_val: f32 = channels[2] as f32 / 255.0;

        let px_lab: Lab = Srgb::new(r_val, g_val, b_val).into_color();

        let sim = color_sim(lab, px_lab);
        if sim > 0 && sim < 40 {
            let px_data = img.get_pixel(x, y).channels();
            let lch_colour: Lch = Srgb::new(px_data[0], px_data[1], px_data[2])
                .into_format()
                .into_linear()
                .into_color();

            let new_color = match mode {
                "desaturate" => lch_colour.desaturate(amt),
                "saturate" => lch_colour.saturate(amt),
                "lighten" => lch_colour.lighten(amt),
                "darken" => lch_colour.darken(amt),
                _ => lch_colour.saturate(amt),
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
                    255,
                ]),
            );
        }
    }

    prettier_image.raw_pixels = img.to_vec();
}

#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn selective_greyscale(mut prettier_image: PrettierImage, ref_color: Rgb) {
    let mut img = helpers::dyn_image_from_raw(&prettier_image);

    for (x, y) in ImageIterator::new(img.width(), img.height()) {
        let mut px = img.get_pixel(x, y);

        let lab: Lab = Srgb::new(
            ref_color.r as f32 / 255.0,
            ref_color.g as f32 / 255.0,
            ref_color.b as f32 / 255.0,
        )
        .into_color();
        let channels = px.channels();
        let r_val: f32 = channels[0] as f32 / 255.0;
        let g_val: f32 = channels[1] as f32 / 255.0;
        let b_val: f32 = channels[2] as f32 / 255.0;

        let px_lab: Lab = Srgb::new(r_val, g_val, b_val).into_color();

        let sim = color_sim(lab, px_lab);
        if sim > 30 {
            let avg =
                channels[0] as f32 * 0.3 + channels[1] as f32 * 0.59 + channels[2] as f32 * 0.11;
            px = image::Rgba([avg as u8, avg as u8, avg as u8, 255]);
        }
        img.put_pixel(x, y, px);
    }

    let raw_pixels = img.to_bytes();
    prettier_image.raw_pixels = raw_pixels;
}

pub fn color_sim(lab1: Lab, lab2: Lab) -> i64 {
    let l_comp = lab2.l - lab1.l;
    let a_comp = lab2.a - lab1.a;
    let b_comp = lab2.b - lab1.b;

    let l_comp_sq = l_comp.powf(2.0);
    let a_comp_sq = a_comp.powf(2.0);
    let b_comp_sq = b_comp.powf(2.0);

    let total = l_comp_sq + a_comp_sq + b_comp_sq;
    (total as f64).sqrt() as i64 + 1
}
