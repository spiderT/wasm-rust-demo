use serde::{Deserialize, Serialize};

#[cfg(feature = "enable_wasm")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "wasm-bindgen")]
use wasm_bindgen::Clamped;

#[cfg(feature = "web-sys")]
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, ImageData};

#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PrettierImage {
    raw_pixels: Vec<u8>,
    width: u32,
    height: u32,
}

#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
impl PrettierImage {
    #[cfg_attr(feature = "enable_wasm", wasm_bindgen(constructor))]
    pub fn new(raw_pixels: Vec<u8>, width: u32, height: u32) -> PrettierImage {
        PrettierImage {
            raw_pixels,
            width,
            height,
        }
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_raw_pixels(&self) -> Vec<u8> {
        self.raw_pixels.clone()
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }
}

#[cfg(feature = "web-sys")]
impl From<ImageData> for PrettierImage {
    fn from(imgdata: ImageData) -> Self {
        let width = imgdata.width();
        let height = imgdata.height();
        let raw_pixels = to_raw_pixels(imgdata);
        PrettierImage {
            raw_pixels,
            width,
            height,
        }
    }
}

#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Rgb {
    r: u8,
    g: u8,
    b: u8,
}

#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
impl Rgb {
    #[cfg_attr(feature = "enable_wasm", wasm_bindgen(constructor))]
    pub fn new(r: u8, g: u8, b: u8) -> Rgb {
        Rgb { r, g, b }
    }
}

impl From<Vec<u8>> for Rgb {
    fn from(vec: Vec<u8>) -> Self {
        if vec.len() != 3 {
            panic!("Vec length must be equal to 3.")
        }
        Rgb::new(vec[0], vec[1], vec[2])
    }
}

#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Rgba {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
impl Rgba {
    #[cfg_attr(feature = "enable_wasm", wasm_bindgen(constructor))]
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Rgba {
        Rgba { r, g, b, a }
    }
}

impl From<Vec<u8>> for Rgba {
    fn from(vec: Vec<u8>) -> Self {
        if vec.len() != 4 {
            panic!("Vec length must be equal to 4.")
        }
        Rgba::new(vec[0], vec[1], vec[2], vec[3])
    }
}

#[cfg(feature = "enable_wasm")]
#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    set_panic_hook();

    let window = web_sys::window().expect("No Window found, should have a Window");
    let document = window
        .document()
        .expect("No Document found, should have a Document");

    let p: web_sys::Node = document.create_element("p")?.into();
    p.set_text_content(Some("You're successfully running WASM!"));

    let body = document
        .body()
        .expect("ERR: No body found, should have a body");
    let body: &web_sys::Node = body.as_ref();
    body.append_child(&p)?;
    Ok(())
}

#[cfg(feature = "web-sys")]
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn get_image_data(canvas: &HtmlCanvasElement, ctx: &CanvasRenderingContext2d) -> ImageData {
    set_panic_hook();
    let width = canvas.width();
    let height = canvas.height();

    let data = ctx
        .get_image_data(0.0, 0.0, width as f64, height as f64)
        .unwrap();
    let _vec_data = data.data().to_vec();
    data
}

#[cfg(all(feature = "web-sys", feature = "wasm-bindgen"))]
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
#[allow(non_snake_case)]
#[allow(clippy::unnecessary_mut_passed)]
pub fn putImageData(
    canvas: HtmlCanvasElement,
    ctx: CanvasRenderingContext2d,
    new_image: PrettierImage,
) {
    let mut raw_pixels = new_image.raw_pixels;
    let new_img_data = ImageData::new_with_u8_clamped_array_and_sh(
        Clamped(&mut raw_pixels),
        canvas.width(),
        canvas.height(),
    );

    ctx.put_image_data(&new_img_data.unwrap(), 0.0, 0.0)
        .expect("Should put image data on Canvas");
}

#[cfg(feature = "web-sys")]
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
#[no_mangle]
pub fn open_image(canvas: HtmlCanvasElement, ctx: CanvasRenderingContext2d) -> PrettierImage {
    let imgdata = get_image_data(&canvas, &ctx);
    let raw_pixels = to_raw_pixels(imgdata);
    PrettierImage {
        raw_pixels,
        width: canvas.width(),
        height: canvas.height(),
    }
}

#[cfg(feature = "web-sys")]
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn to_raw_pixels(imgdata: ImageData) -> Vec<u8> {
    imgdata.data().to_vec()
}

fn set_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

pub mod channels;
pub mod colour_spaces;
pub mod effects;
pub mod filters;
pub mod helpers;
mod iter;
