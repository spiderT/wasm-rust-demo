use crate::PrettierImage;

#[cfg(feature = "enable_wasm")]
use wasm_bindgen::prelude::*;

// 将选择通道的值递增或递减一个常数来更改该通道
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

// 通过每像素向每个通道添加一个amt来增加所有3个通道的值
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

// 将某个通道设置为零，从而消除通道对像素最终渲染颜色的影响。
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

// 交换通道
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

    // r g b a, 所以step是4
    for i in (channel1..end).step_by(4) {
        let difference = channel2 - channel1;

        img.raw_pixels.swap(i, i + difference);
    }
}

// 反转RGB值
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
