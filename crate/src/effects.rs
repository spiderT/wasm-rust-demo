use crate::helpers;
use crate::iter::ImageIterator;
use crate::{PrettierImage, Rgb};
use image::Pixel;
use image::Rgba;
use image::{GenericImage, GenericImageView};
use imageproc::drawing::draw_filled_rect_mut;
use imageproc::rect::Rect;
use perlin2d::PerlinNoise2D;
use std::collections::HashMap;
use std::f64;

#[cfg(feature = "enable_wasm")]
use wasm_bindgen::prelude::*;

#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn offset(prettier_image: &mut PrettierImage, channel_index: usize, offset: u32) {
    if channel_index > 2 {
        panic!("Invalid channel index passed. Channel1 must be equal to 0, 1, or 2.");
    }

    let mut img = helpers::dyn_image_from_raw(prettier_image);
    let (width, height) = img.dimensions();

    for x in 0..width - 10 {
        for y in 0..height - 10 {
            let px = img.get_pixel(x, y);

            if x + offset < width - 1 && y + offset < height - 1 {
                let offset_px = img.get_pixel(x + offset, y + offset);
                let offset_px_channels = offset_px.channels();

                let px_channels = px.channels();

                let px = match channel_index {
                    0 => image::Rgba([offset_px_channels[0], px_channels[1], px_channels[2], 255]),
                    1 => image::Rgba([px_channels[0], offset_px_channels[1], px_channels[2], 255]),
                    2 => image::Rgba([px_channels[0], px_channels[1], offset_px_channels[2], 255]),
                    _ => image::Rgba([px_channels[0], px_channels[1], offset_px_channels[2], 255]),
                };
                img.put_pixel(x, y, px);
            }
        }
    }
    let raw_pixels = img.to_bytes();
    prettier_image.raw_pixels = raw_pixels;
}

#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn offset_red(img: &mut PrettierImage, offset_amt: u32) {
    offset(img, 0, offset_amt)
}

#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn offset_green(img: &mut PrettierImage, offset_amt: u32) {
    offset(img, 1, offset_amt)
}

#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn offset_blue(img: &mut PrettierImage, offset_amt: u32) {
    offset(img, 2, offset_amt)
}

#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn multiple_offsets(
    prettier_image: &mut PrettierImage,
    offset: u32,
    channel_index: usize,
    channel_index2: usize,
) {
    if channel_index > 2 {
        panic!("Invalid channel index passed. Channel1 must be equal to 0, 1, or 2.");
    }
    if channel_index2 > 2 {
        panic!("Invalid channel index passed. Channel2 must be equal to 0, 1, or 2.");
    }
    let mut img = helpers::dyn_image_from_raw(prettier_image);
    let (width, height) = img.dimensions();

    for (x, y) in ImageIterator::new(width, height) {
        let mut px = img.get_pixel(x, y);

        if x + offset < width - 1 && y + offset < height - 1 {
            let offset_px = img.get_pixel(x + offset, y);

            px[channel_index] = offset_px[channel_index];
        }

        if x as i32 - offset as i32 > 0 && y as i32 - offset as i32 > 0 {
            let offset_px2 = img.get_pixel(x - offset, y);

            px[channel_index2] = offset_px2[channel_index2];
        }

        img.put_pixel(x, y, px);
    }
    let raw_pixels = img.to_bytes();
    prettier_image.raw_pixels = raw_pixels;
}

pub fn halftone(prettier_image: &mut PrettierImage) {
    let mut img = helpers::dyn_image_from_raw(prettier_image);
    let (width, height) = img.dimensions();

    for x in (0..width - 4).step_by(2_usize) {
        for y in (0..height - 4).step_by(2_usize) {
            let mut px1 = img.get_pixel(x, y);
            let mut px2 = img.get_pixel(x, y + 1);
            let mut px3 = img.get_pixel(x + 1, y);
            let mut px4 = img.get_pixel(x + 1, y + 1);

            let gray1 = (px1[0] as f64 * 0.299) + (px1[1] as f64 * 0.587) + (px1[2] as f64 * 0.114);
            let gray2 = (px2[0] as f64 * 0.299) + (px2[1] as f64 * 0.587) + (px2[2] as f64 * 0.114);
            let gray3 = (px3[0] as f64 * 0.299) + (px3[1] as f64 * 0.587) + (px3[2] as f64 * 0.114);
            let gray4 = (px4[0] as f64 * 0.299) + (px4[1] as f64 * 0.587) + (px4[2] as f64 * 0.114);

            let sat = (gray1 + gray2 + gray3 + gray4) / 4.0;

            if sat > 200.0 {
                px1[0] = 255;
                px1[1] = 255;
                px1[2] = 255;

                px2[0] = 255;
                px2[1] = 255;
                px2[2] = 255;

                px3[0] = 255;
                px3[1] = 255;
                px3[2] = 255;

                px4[0] = 255;
                px4[1] = 255;
                px4[2] = 255;
            } else if sat > 159.0 {
                px1[0] = 255;
                px1[1] = 255;
                px1[2] = 255;

                px2[0] = 0;
                px2[1] = 0;
                px2[2] = 0;

                px3[0] = 255;
                px3[1] = 255;
                px3[2] = 255;

                px4[0] = 255;
                px4[1] = 255;
                px4[2] = 255;
            } else if sat > 95.0 {
                px1[0] = 255;
                px1[1] = 255;
                px1[2] = 255;

                px2[0] = 0;
                px2[1] = 0;
                px2[2] = 0;

                px3[0] = 0;
                px3[1] = 0;
                px3[2] = 0;

                px4[0] = 255;
                px4[1] = 255;
                px4[2] = 255;
            } else if sat > 32.0 {
                px1[0] = 0;
                px1[1] = 0;
                px1[2] = 0;

                px2[0] = 255;
                px2[1] = 255;
                px2[2] = 255;

                px3[0] = 0;
                px3[1] = 0;
                px3[2] = 0;

                px4[0] = 0;
                px4[1] = 0;
                px4[2] = 0;
            } else {
                px1[0] = 0;
                px1[1] = 0;
                px1[2] = 0;

                px2[0] = 0;
                px2[1] = 0;
                px2[2] = 0;

                px3[0] = 0;
                px3[1] = 0;
                px3[2] = 0;

                px4[0] = 0;
                px4[1] = 0;
                px4[2] = 0;
            }

            img.put_pixel(x, y, px1);
            // img.put_pixel(x, y + 1, px2);
        }
    }
    let raw_pixels = img.to_bytes();
    prettier_image.raw_pixels = raw_pixels;
}

#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn primary(img: &mut PrettierImage) {
    let end = img.raw_pixels.len() - 4;

    for i in (0..end).step_by(4) {
        let mut r_val = img.raw_pixels[0];
        let mut g_val = img.raw_pixels[1];
        let mut b_val = img.raw_pixels[2];

        if r_val > 128 {
            r_val = 255;
        } else {
            r_val = 0;
        }

        if g_val > 128 {
            g_val = 255;
        } else {
            g_val = 0;
        }

        if b_val > 128 {
            g_val = 255;
        } else {
            b_val = 0;
        }

        img.raw_pixels[i] = r_val;
        img.raw_pixels[i + 1] = g_val;
        img.raw_pixels[i + 2] = b_val;
    }
}

#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn colorize(prettier_image: &mut PrettierImage) {
    let mut img = helpers::dyn_image_from_raw(prettier_image);
    let threshold = 220;

    for (x, y) in ImageIterator::with_dimension(&img.dimensions()) {
        let mut px = img.get_pixel(x, y);
        let channels = px.channels();
        let px_as_rgb = Rgb {
            r: channels[0],
            g: channels[1],
            b: channels[2],
        };

        let baseline_color = Rgb {
            r: 0,
            g: 255,
            b: 255,
        };

        let square_distance = crate::helpers::square_distance(baseline_color, px_as_rgb);

        let mut r = channels[0] as f32;
        let mut g = channels[1] as f32;
        let mut b = channels[2] as f32;

        if square_distance < i32::pow(threshold, 2) {
            r *= 0.5;
            g *= 1.25;
            b *= 0.5;
        }

        px = image::Rgba([r as u8, g as u8, b as u8, 255]);
        img.put_pixel(x, y, px);
    }
    let raw_pixels = img.to_bytes();
    prettier_image.raw_pixels = raw_pixels;
}

#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn solarize(prettier_image: &mut PrettierImage) {
    let end = prettier_image.get_raw_pixels().len();

    for i in (0..end).step_by(4) {
        let r_val = prettier_image.raw_pixels[i];

        if 200 - r_val as i32 > 0 {
            prettier_image.raw_pixels[i] = 200 - r_val;
        }
    }
}

#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn solarize_retimg(prettier_image: &PrettierImage) -> PrettierImage {
    let mut img = helpers::dyn_image_from_raw(prettier_image);

    for (x, y) in ImageIterator::with_dimension(&img.dimensions()) {
        let mut px = img.get_pixel(x, y);
        let channels = px.channels();
        if 200_i32 - channels[0] as i32 > 0 {
            let new_r_val = 200 - channels[0];
            px = image::Rgba([new_r_val, channels[1], channels[2], channels[3]]);
        }
        img.put_pixel(x, y, px);
    }

    PrettierImage {
        raw_pixels: img.to_bytes(),
        width: img.width(),
        height: img.height(),
    }
}

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

#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn tint(prettier_image: &mut PrettierImage, r_offset: u32, g_offset: u32, b_offset: u32) {
    let mut img = helpers::dyn_image_from_raw(prettier_image);

    for (x, y) in ImageIterator::with_dimension(&img.dimensions()) {
        let mut px = img.get_pixel(x, y);
        let channels = px.channels();
        let (r_val, g_val, b_val) = (channels[0] as u32, channels[1] as u32, channels[2] as u32);

        let new_r_val = if r_val + r_offset < 255 {
            r_val as u8 + r_offset as u8
        } else {
            255
        };
        let new_g_val = if g_val + g_offset < 255 {
            g_val as u8 + g_offset as u8
        } else {
            255
        };
        let new_b_val = if b_val + b_offset < 255 {
            b_val as u8 + b_offset as u8
        } else {
            255
        };

        px = image::Rgba([new_r_val, new_g_val, new_b_val, 255]);

        img.put_pixel(x, y, px);
    }

    let raw_pixels = img.to_bytes();
    prettier_image.raw_pixels = raw_pixels;
}

fn draw_horizontal_strips(prettier_image: &mut PrettierImage, num_strips: u8, color: Rgb) {
    let mut img = helpers::dyn_image_from_raw(prettier_image);
    let (width, height) = img.dimensions();

    let total_strips = (num_strips * 2) - 1;
    let height_strip = height / total_strips as u32;
    let mut y_pos: u32 = 0;
    for i in 1..num_strips {
        draw_filled_rect_mut(
            &mut img,
            Rect::at(0, (y_pos + height_strip) as i32).of_size(width, height_strip),
            Rgba([color.r, color.g, color.b, 255u8]),
        );
        y_pos = i as u32 * (height_strip * 2);
    }

    let raw_pixels = img.to_bytes();
    prettier_image.raw_pixels = raw_pixels;
}

#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn horizontal_strips(prettier_image: &mut PrettierImage, num_strips: u8) {
    let color = Rgb {
        r: 255,
        g: 255,
        b: 255,
    };
    draw_horizontal_strips(prettier_image, num_strips, color)
}

#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn color_horizontal_strips(prettier_image: &mut PrettierImage, num_strips: u8, color: Rgb) {
    draw_horizontal_strips(prettier_image, num_strips, color)
}

fn draw_vertical_strips(prettier_image: &mut PrettierImage, num_strips: u8, color: Rgb) {
    let mut img = helpers::dyn_image_from_raw(prettier_image);
    let (width, height) = img.dimensions();

    let total_strips = (num_strips * 2) - 1;
    let width_strip = width / total_strips as u32;
    let mut x_pos: u32 = 0;
    for i in 1..num_strips {
        draw_filled_rect_mut(
            &mut img,
            Rect::at((x_pos + width_strip) as i32, 0).of_size(width_strip, height),
            Rgba([color.r, color.g, color.b, 255u8]),
        );
        x_pos = i as u32 * (width_strip * 2);
    }

    let raw_pixels = img.to_bytes();
    prettier_image.raw_pixels = raw_pixels;
}

#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn vertical_strips(prettier_image: &mut PrettierImage, num_strips: u8) {
    let color = Rgb {
        r: 255,
        g: 255,
        b: 255,
    };
    draw_vertical_strips(prettier_image, num_strips, color)
}

#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn color_vertical_strips(prettier_image: &mut PrettierImage, num_strips: u8, color: Rgb) {
    draw_vertical_strips(prettier_image, num_strips, color)
}

struct Intensity {
    val: i32,
    r: i32,
    g: i32,
    b: i32,
}
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn oil(prettier_image: &mut PrettierImage, radius: i32, intensity: f64) {
    let img = helpers::dyn_image_from_raw(prettier_image);
    let (width, height) = img.dimensions();
    let mut target = image::DynamicImage::new_rgba8(width, height);
    let mut pixel_intensity_count: HashMap<usize, Intensity>;
    let mut intensity_lut = vec![vec![0; width as usize]; height as usize];

    for y in 0..height {
        for x in 0..width {
            let single_pix = img.get_pixel(x, y);
            let current_val = single_pix.channels();
            let avg = (current_val[0] as i32 + current_val[1] as i32 + current_val[2] as i32)
                as f64
                / 3.0;
            let val = (avg * intensity) / 255.0;
            intensity_lut[y as usize][x as usize] = val.round() as usize;
        }
    }

    for y in 0..height {
        for x in 0..width {
            pixel_intensity_count = HashMap::new();

            for yy in -radius..=radius {
                let yyy = (y as i32) + yy;
                for xx in -radius..=radius {
                    let xxx = (x as i32) + xx;
                    if yyy > 0 && yyy < (height as i32) && xxx > 0 && xxx < (width as i32) {
                        let idx_x = xxx as usize;
                        let idx_y = yyy as usize;
                        let intensity_val = intensity_lut[idx_y][idx_x];
                        let single_pix = img.get_pixel(idx_x as u32, idx_y as u32);
                        let pix = single_pix.channels();
                        match pixel_intensity_count.get_mut(&(intensity_val)) {
                            Some(val) => {
                                val.val += 1;
                                val.r += pix[0] as i32;
                                val.g += pix[1] as i32;
                                val.b += pix[2] as i32;
                            }
                            None => {
                                pixel_intensity_count.insert(
                                    intensity_val,
                                    Intensity {
                                        val: 1,
                                        r: pix[0] as i32,
                                        g: pix[1] as i32,
                                        b: pix[2] as i32,
                                    },
                                );
                            }
                        }
                    }
                }
            }

            let mut map_vec: Vec<_> = pixel_intensity_count.iter().collect();
            map_vec.sort_by(|a, b| (b.1.val - a.1.val).cmp(&0));

            let cur_max = map_vec[0].1;
            target.put_pixel(
                x,
                y,
                Rgba::<u8>([
                    (cur_max.r / cur_max.val) as u8,
                    (cur_max.g / cur_max.val) as u8,
                    (cur_max.b / cur_max.val) as u8,
                    255,
                ]),
            )
        }
    }
    let raw_pixels = target.to_bytes();
    prettier_image.raw_pixels = raw_pixels;
}
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn frosted_glass(prettier_image: &mut PrettierImage) {
    let img_orig_buf = prettier_image.get_raw_pixels();
    let width = prettier_image.get_width();
    let height = prettier_image.get_height();
    let end = img_orig_buf.len();

    let mut img_buf = Vec::<u8>::new();
    Vec::resize(&mut img_buf, end, 0_u8);

    let perlin = PerlinNoise2D::new(2, 10.0, 10.0, 10.0, 1.0, (100.0, 100.0), 0.5, 101);

    for pixel in (0..end).step_by(4) {
        let x = (pixel / 4) / width as usize;
        let y = (pixel / 4) % width as usize;

        let res = [
            perlin.get_noise(x as f64, y as f64) - 0.5,
            (perlin.get_noise(100.0 + x as f64, y as f64) - 0.5) * 4.0,
        ];

        let x_new = f64::clamp(f64::floor(x as f64 + res[0]), 0.0, height as f64 - 1.0);
        let x_new = x_new as usize;
        let y_new = f64::clamp(f64::floor(y as f64 + res[1]), 0.0, width as f64 - 1.0);
        let y_new = y_new as usize;

        let pixel_new = (x_new * width as usize + y_new) * 4;
        if pixel_new > end {
            continue;
        }
        img_buf[pixel] = img_orig_buf[pixel_new];
        img_buf[pixel + 1] = img_orig_buf[pixel_new + 1];
        img_buf[pixel + 2] = img_orig_buf[pixel_new + 2];
        img_buf[pixel + 3] = img_orig_buf[pixel_new + 3];
    }

    prettier_image.raw_pixels = img_buf;
}

#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn pixelize(prettier_image: &mut PrettierImage, pixel_size: i32) {
    let step_size = pixel_size.max(0) as usize;
    if step_size <= 1 {
        return;
    }

    let buf = prettier_image.raw_pixels.as_mut_slice();
    let width = prettier_image.width as usize;
    let height = prettier_image.height as usize;

    for sy in (0..height).step_by(step_size) {
        let src_row_index = sy * width;

        for sx in (0..width).step_by(step_size) {
            let src_index = 4 * (src_row_index + sx);
            let block_end_y = (sy + step_size).min(height);
            let block_end_x = (sx + step_size).min(width);

            for dy in sy..block_end_y {
                let dst_row_index = dy * width;

                for dx in sx..block_end_x {
                    let dst_index = 4 * (dst_row_index + dx);
                    buf[dst_index] = buf[src_index];
                    buf[dst_index + 1] = buf[src_index + 1];
                    buf[dst_index + 2] = buf[src_index + 2];
                    buf[dst_index + 3] = buf[src_index + 3];
                }
            }
        }
    }
}

#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn normalize(prettier_image: &mut PrettierImage) {
    let buf = prettier_image.raw_pixels.as_mut_slice();
    let buf_size = buf.len();

    let mut min_rgb = Rgb::new(255, 255, 255);
    let mut max_rgb = Rgb::new(0, 0, 0);

    for i in (0..buf_size).step_by(4) {
        let r = buf[i];
        let g = buf[i + 1];
        let b = buf[i + 2];

        min_rgb.r = min_rgb.r.min(r);
        min_rgb.g = min_rgb.g.min(g);
        min_rgb.b = min_rgb.b.min(b);

        max_rgb.r = max_rgb.r.max(r);
        max_rgb.g = max_rgb.g.max(g);
        max_rgb.b = max_rgb.b.max(b);
    }

    let min_r = min_rgb.r as i32;
    let min_g = min_rgb.g as i32;
    let min_b = min_rgb.b as i32;
    let delta_r = (max_rgb.r as i32) - min_r;
    let delta_g = (max_rgb.g as i32) - min_g;
    let delta_b = (max_rgb.b as i32) - min_b;

    for i in (0..buf_size).step_by(4) {
        let r = buf[i] as i32;
        let g = buf[i + 1] as i32;
        let b = buf[i + 2] as i32;

        if delta_r > 0 {
            buf[i] = (((r - min_r) * 255) / delta_r) as u8;
        }

        if delta_b > 0 {
            buf[i + 1] = (((g - min_g) * 255) / delta_g) as u8;
        }

        if delta_b > 0 {
            buf[i + 2] = (((b - min_b) * 255) / delta_b) as u8;
        }
    }
}

#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn dither(prettier_image: &mut PrettierImage, depth: u32) {
    let width = prettier_image.get_width() as usize;
    let height = prettier_image.get_height() as usize;
    let buf = prettier_image.raw_pixels.as_mut_slice();
    let channels = 4;
    let chan_stride = 1;
    let col_stride = chan_stride * channels;
    let row_stride = col_stride * width;

    let depth = depth.clamp(1, 8);
    let num_colours = u16::pow(2, depth);
    let quant_rate = (256_u16 / num_colours) as u8;
    let mut lookup_table: Vec<u8> = vec![0; 256];
    for (tbl_idx, table) in lookup_table.iter_mut().enumerate().take(256_usize) {
        let downscaled_val = (tbl_idx as u8) / quant_rate;
        let upscaled_val = downscaled_val * quant_rate;
        *table = upscaled_val.clamp(0, 255);
    }

    for row in 0..height - 1 {
        for col in 0..width - 1 {
            for chan in 0..channels - 1 {
                let buf_idx = row * row_stride + col * col_stride + chan * chan_stride;
                let old_pixel = buf[buf_idx];
                let new_pixel = lookup_table[old_pixel as usize];

                buf[buf_idx] = new_pixel;

                let quant_error = (old_pixel as i16) - (new_pixel as i16);

                let buf_idx = row * row_stride + (col + 1) * col_stride + chan * chan_stride;
                let new_pixel = (buf[buf_idx] as i16) + (quant_error * 7) / 16;
                buf[buf_idx] = new_pixel.clamp(0, 255) as u8;

                let buf_idx =
                    (row + 1) * row_stride + col * col_stride - col_stride + chan * chan_stride;
                let new_pixel = (buf[buf_idx] as i16) + (quant_error * 3) / 16;
                buf[buf_idx] = new_pixel.clamp(0, 255) as u8;

                let buf_idx = (row + 1) * row_stride + col * col_stride + chan * chan_stride;
                let new_pixel = (buf[buf_idx] as i16) + (quant_error * 5) / 16;
                buf[buf_idx] = new_pixel.clamp(0, 255) as u8;

                let buf_idx = (row + 1) * row_stride + (col + 1) * col_stride + chan * chan_stride;
                let new_pixel = (buf[buf_idx] as i16) + quant_error / 16;
                buf[buf_idx] = new_pixel.clamp(0, 255) as u8;
            }
        }
    }
}

fn create_gradient_map(color_a: Rgb, color_b: Rgb) -> Vec<Rgb> {
    let mut gradient_map = vec![Rgb::new(0, 0, 0); 256];

    for (px, pos) in gradient_map.iter_mut().zip(0_u32..) {
        let inv_pos = 256 - pos;

        px.r =
            (((color_a.r as u32) * inv_pos + (color_b.r as u32) * pos) / 256).clamp(0, 255) as u8;
        px.g =
            (((color_a.g as u32) * inv_pos + (color_b.g as u32) * pos) / 256).clamp(0, 255) as u8;
        px.b =
            (((color_a.b as u32) * inv_pos + (color_b.b as u32) * pos) / 256).clamp(0, 255) as u8;
    }

    gradient_map
}

#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn duotone(prettier_image: &mut PrettierImage, color_a: Rgb, color_b: Rgb) {
    let gradient_map = create_gradient_map(color_a, color_b);
    let buf = prettier_image.raw_pixels.as_mut_slice();

    for px in buf.chunks_mut(4) {
        let luma = (((px[0] as u32) * 2126 + (px[1] as u32) * 7152 + (px[2] as u32) * 722) / 10000)
            .clamp(0, 255);

        let mapped_luma = &gradient_map[luma as usize];
        px[0] = mapped_luma.r;
        px[1] = mapped_luma.g;
        px[2] = mapped_luma.b;
    }
}
