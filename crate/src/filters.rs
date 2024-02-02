use crate::channels::alter_channels;
use crate::colour_spaces;
use crate::colour_spaces::mix_with_colour;
use crate::effects::{adjust_contrast, inc_brightness};
use crate::{PrettierImage, Rgb};

#[cfg(feature = "enable_wasm")]
use wasm_bindgen::prelude::*;

#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn filter(img: &mut PrettierImage, filter_name: &str) {
    let oceanic_rgb = Rgb::new(0, 89, 173);
    let islands_rgb = Rgb::new(0, 24, 95);
    let marine_rgb = Rgb::new(0, 14, 119);
    let seagreen_rgb = Rgb::new(0, 68, 62);
    let flagblue_rgb = Rgb::new(0, 0, 131);
    let diamante_rgb = Rgb::new(30, 82, 87);
    let liquid_rgb = Rgb::new(0, 10, 75);
    let vintage_rgb = Rgb::new(120, 70, 13);
    let perfume_rgb = Rgb::new(80, 40, 120);
    let serenity_rgb = Rgb::new(10, 40, 90);

    match filter_name {
        "oceanic" => mix_with_colour(img, oceanic_rgb, 0.2),
        "islands" => mix_with_colour(img, islands_rgb, 0.2),
        "marine" => mix_with_colour(img, marine_rgb, 0.2),
        "seagreen" => mix_with_colour(img, seagreen_rgb, 0.2),
        "flagblue" => mix_with_colour(img, flagblue_rgb, 0.2),
        "diamante" => mix_with_colour(img, diamante_rgb, 0.1),
        "liquid" => mix_with_colour(img, liquid_rgb, 0.2),
        "vintage" => mix_with_colour(img, vintage_rgb, 0.2),
        "perfume" => mix_with_colour(img, perfume_rgb, 0.2),
        "serenity" => mix_with_colour(img, serenity_rgb, 0.2),
        "golden" => golden(img),
        "pastel_pink" => pastel_pink(img),
        "cali" => cali(img),
        "firenze" => firenze(img),
        "lofi" => lofi(img),
        &_ => todo!(),
    };
}

#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn lofi(img: &mut PrettierImage) {
    adjust_contrast(img, 30.0);
    colour_spaces::saturate_hsl(img, 0.2);
}

#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn pastel_pink(img: &mut PrettierImage) {
    alter_channels(img, 80, 12, 20);
    adjust_contrast(img, 30.0);
}

#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn golden(img: &mut PrettierImage) {
    let vignette_rgb = Rgb::new(235, 145, 50);
    mix_with_colour(img, vignette_rgb, 0.2);
    adjust_contrast(img, 30.0);
}

#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn cali(img: &mut PrettierImage) {
    let cali_rgb = Rgb::new(255, 45, 75);
    colour_spaces::mix_with_colour(img, cali_rgb, 0.1);
    adjust_contrast(img, 50.0);
}

#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn firenze(img: &mut PrettierImage) {
    let cali_rgb = Rgb::new(255, 47, 78);
    colour_spaces::mix_with_colour(img, cali_rgb, 0.1);

    inc_brightness(img, 30);
    adjust_contrast(img, 50.0);
}
