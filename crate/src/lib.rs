use base64::{decode, encode};
use image::DynamicImage::ImageRgba8;
use image::GenericImageView;
use serde::{Deserialize, Serialize};

#[cfg(feature = "enable_wasm")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "wasm-bindgen")]
use wasm_bindgen::Clamped;

#[cfg(feature = "web-sys")]
use web_sys::{Blob, CanvasRenderingContext2d, HtmlCanvasElement, HtmlImageElement, ImageData};

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

    pub fn new_from_base64(base64: &str) -> PrettierImage {
        base64_to_image(base64)
    }

    pub fn new_from_byteslice(vec: Vec<u8>) -> PrettierImage {
        let slice = vec.as_slice();

        let img = image::load_from_memory(slice).unwrap();

        let raw_pixels = img.to_rgba8().to_vec();

        PrettierImage {
            raw_pixels,
            width: img.width(),
            height: img.height(),
        }
    }

    #[cfg(feature = "web-sys")]
    pub fn new_from_blob(blob: Blob) -> PrettierImage {
        let bytes: js_sys::Uint8Array = js_sys::Uint8Array::new(&blob);

        let vec = bytes.to_vec();

        PrettierImage::new_from_byteslice(vec)
    }

    #[cfg(feature = "web-sys")]
    pub fn new_from_image(image: HtmlImageElement) -> PrettierImage {
        set_panic_hook();

        let document = web_sys::window().unwrap().document().unwrap();

        let canvas = document
            .create_element("canvas")
            .unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .unwrap();

        canvas.set_width(image.width());
        canvas.set_height(image.height());

        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();

        context
            .draw_image_with_html_image_element(&image, 0.0, 0.0)
            .unwrap();

        open_image(canvas, context)
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

    pub fn get_base64(&self) -> String {
        let mut img = helpers::dyn_image_from_raw(self);
        img = ImageRgba8(img.to_rgba8());

        let mut buffer = vec![];
        img.write_to(&mut buffer, image::ImageOutputFormat::Png)
            .unwrap();
        let base64 = encode(&buffer);

        let res_base64 = format!("data:image/png;base64,{}", base64.replace("\r\n", ""));

        res_base64
    }

    pub fn get_bytes(&self) -> Vec<u8> {
        let mut img = helpers::dyn_image_from_raw(self);
        img = ImageRgba8(img.to_rgba8());
        let mut buffer = vec![];
        img.write_to(&mut buffer, image::ImageOutputFormat::Png)
            .unwrap();
        buffer
    }

    pub fn get_bytes_jpeg(&self, quality: u8) -> Vec<u8> {
        let mut img = helpers::dyn_image_from_raw(self);
        img = ImageRgba8(img.to_rgba8());
        let mut buffer = vec![];
        let out_format = image::ImageOutputFormat::Jpeg(quality);
        img.write_to(&mut buffer, out_format).unwrap();
        buffer
    }

    #[cfg(all(feature = "web-sys", feature = "wasm-bindgen"))]
    #[allow(clippy::unnecessary_mut_passed)]
    pub fn get_image_data(&mut self) -> ImageData {
        ImageData::new_with_u8_clamped_array_and_sh(
            Clamped(&mut self.raw_pixels),
            self.width,
            self.height,
        )
        .unwrap()
    }

    #[cfg(feature = "web-sys")]
    pub fn set_imgdata(&mut self, img_data: ImageData) {
        let width = img_data.width();
        let height = img_data.height();
        let raw_pixels = to_raw_pixels(img_data);
        self.width = width;
        self.height = height;
        self.raw_pixels = raw_pixels;
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

    pub fn set_red(&mut self, r: u8) {
        self.r = r;
    }

    pub fn set_green(&mut self, g: u8) {
        self.g = g;
    }

    pub fn set_blue(&mut self, b: u8) {
        self.b = b;
    }

    pub fn get_red(&self) -> u8 {
        self.r
    }

    pub fn get_green(&self) -> u8 {
        self.g
    }

    pub fn get_blue(&self) -> u8 {
        self.b
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

    pub fn set_red(&mut self, r: u8) {
        self.r = r;
    }

    pub fn set_green(&mut self, g: u8) {
        self.g = g;
    }

    pub fn set_blue(&mut self, b: u8) {
        self.b = b;
    }

    pub fn set_alpha(&mut self, a: u8) {
        self.a = a;
    }

    pub fn get_red(&self) -> u8 {
        self.r
    }

    pub fn get_green(&self) -> u8 {
        self.g
    }

    pub fn get_blue(&self) -> u8 {
        self.b
    }

    pub fn get_alpha(&self) -> u8 {
        self.a
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

///! [temp] Check if WASM is supported.
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

#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn base64_to_image(base64: &str) -> PrettierImage {
    let base64_to_vec: Vec<u8> = base64_to_vec(base64);

    let slice = base64_to_vec.as_slice();

    let mut img = image::load_from_memory(slice).unwrap();
    img = ImageRgba8(img.to_rgba8());
    let raw_pixels = img.to_bytes();

    PrettierImage {
        raw_pixels,
        width: img.width(),
        height: img.height(),
    }
}

#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn base64_to_vec(base64: &str) -> Vec<u8> {
    decode(base64).unwrap()
}

#[cfg(all(feature = "web-sys", feature = "wasm-bindgen"))]
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
#[allow(clippy::unnecessary_mut_passed)]
pub fn to_image_data(prettier_image: PrettierImage) -> ImageData {
    let mut raw_pixels = prettier_image.raw_pixels;
    let width = prettier_image.width;
    let height = prettier_image.height;
    ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut raw_pixels), width, height).unwrap()
}

fn set_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

pub mod channels;
pub mod helpers;
mod iter;
pub mod filters;
pub mod colour_spaces;
pub mod effects;