/* tslint:disable */
/* eslint-disable */
/**
* @param {PrettierImage} prettier_image
* @param {number} channel_index
* @param {number} offset
*/
export function offset(prettier_image: PrettierImage, channel_index: number, offset: number): void;
/**
* @param {PrettierImage} img
* @param {number} offset_amt
*/
export function offset_red(img: PrettierImage, offset_amt: number): void;
/**
* @param {PrettierImage} img
* @param {number} offset_amt
*/
export function offset_green(img: PrettierImage, offset_amt: number): void;
/**
* @param {PrettierImage} img
* @param {number} offset_amt
*/
export function offset_blue(img: PrettierImage, offset_amt: number): void;
/**
* @param {PrettierImage} prettier_image
* @param {number} offset
* @param {number} channel_index
* @param {number} channel_index2
*/
export function multiple_offsets(prettier_image: PrettierImage, offset: number, channel_index: number, channel_index2: number): void;
/**
* @param {PrettierImage} img
*/
export function primary(img: PrettierImage): void;
/**
* @param {PrettierImage} prettier_image
*/
export function colorize(prettier_image: PrettierImage): void;
/**
* @param {PrettierImage} prettier_image
*/
export function solarize(prettier_image: PrettierImage): void;
/**
* @param {PrettierImage} prettier_image
* @returns {PrettierImage}
*/
export function solarize_retimg(prettier_image: PrettierImage): PrettierImage;
/**
* @param {PrettierImage} prettier_image
* @param {number} brightness
*/
export function inc_brightness(prettier_image: PrettierImage, brightness: number): void;
/**
* @param {PrettierImage} prettier_image
* @param {number} contrast
*/
export function adjust_contrast(prettier_image: PrettierImage, contrast: number): void;
/**
* @param {PrettierImage} prettier_image
* @param {number} r_offset
* @param {number} g_offset
* @param {number} b_offset
*/
export function tint(prettier_image: PrettierImage, r_offset: number, g_offset: number, b_offset: number): void;
/**
* @param {PrettierImage} prettier_image
* @param {number} num_strips
*/
export function horizontal_strips(prettier_image: PrettierImage, num_strips: number): void;
/**
* @param {PrettierImage} prettier_image
* @param {number} num_strips
* @param {Rgb} color
*/
export function color_horizontal_strips(prettier_image: PrettierImage, num_strips: number, color: Rgb): void;
/**
* @param {PrettierImage} prettier_image
* @param {number} num_strips
*/
export function vertical_strips(prettier_image: PrettierImage, num_strips: number): void;
/**
* @param {PrettierImage} prettier_image
* @param {number} num_strips
* @param {Rgb} color
*/
export function color_vertical_strips(prettier_image: PrettierImage, num_strips: number, color: Rgb): void;
/**
* @param {PrettierImage} prettier_image
* @param {number} radius
* @param {number} intensity
*/
export function oil(prettier_image: PrettierImage, radius: number, intensity: number): void;
/**
* @param {PrettierImage} prettier_image
*/
export function frosted_glass(prettier_image: PrettierImage): void;
/**
* @param {PrettierImage} prettier_image
* @param {number} pixel_size
*/
export function pixelize(prettier_image: PrettierImage, pixel_size: number): void;
/**
* @param {PrettierImage} prettier_image
*/
export function normalize(prettier_image: PrettierImage): void;
/**
* @param {PrettierImage} prettier_image
* @param {number} depth
*/
export function dither(prettier_image: PrettierImage, depth: number): void;
/**
* @param {PrettierImage} prettier_image
* @param {Rgb} color_a
* @param {Rgb} color_b
*/
export function duotone(prettier_image: PrettierImage, color_a: Rgb, color_b: Rgb): void;
/**
* @param {PrettierImage} img
* @param {number} channel
* @param {number} amt
*/
export function alter_channel(img: PrettierImage, channel: number, amt: number): void;
/**
* @param {PrettierImage} prettier_image
* @param {number} amt
*/
export function alter_red_channel(prettier_image: PrettierImage, amt: number): void;
/**
* @param {PrettierImage} img
* @param {number} amt
*/
export function alter_green_channel(img: PrettierImage, amt: number): void;
/**
* @param {PrettierImage} img
* @param {number} amt
*/
export function alter_blue_channel(img: PrettierImage, amt: number): void;
/**
* @param {PrettierImage} img
* @param {number} channel1
* @param {number} amt1
* @param {number} channel2
* @param {number} amt2
*/
export function alter_two_channels(img: PrettierImage, channel1: number, amt1: number, channel2: number, amt2: number): void;
/**
* @param {PrettierImage} img
* @param {number} r_amt
* @param {number} g_amt
* @param {number} b_amt
*/
export function alter_channels(img: PrettierImage, r_amt: number, g_amt: number, b_amt: number): void;
/**
* @param {PrettierImage} img
* @param {number} channel
* @param {number} min_filter
*/
export function remove_channel(img: PrettierImage, channel: number, min_filter: number): void;
/**
* @param {PrettierImage} img
* @param {number} min_filter
*/
export function remove_red_channel(img: PrettierImage, min_filter: number): void;
/**
* @param {PrettierImage} img
* @param {number} min_filter
*/
export function remove_green_channel(img: PrettierImage, min_filter: number): void;
/**
* @param {PrettierImage} img
* @param {number} min_filter
*/
export function remove_blue_channel(img: PrettierImage, min_filter: number): void;
/**
* @param {PrettierImage} img
* @param {number} channel1
* @param {number} channel2
*/
export function swap_channels(img: PrettierImage, channel1: number, channel2: number): void;
/**
* @param {PrettierImage} prettier_image
*/
export function invert(prettier_image: PrettierImage): void;
/**
* @param {PrettierImage} prettier_image
* @param {Rgb} ref_color
* @param {number} degrees
*/
export function selective_hue_rotate(prettier_image: PrettierImage, ref_color: Rgb, degrees: number): void;
/**
* @param {PrettierImage} prettier_image
* @param {Rgb} ref_color
* @param {Rgb} new_color
* @param {number} fraction
*/
export function selective_color_convert(prettier_image: PrettierImage, ref_color: Rgb, new_color: Rgb, fraction: number): void;
/**
* @param {PrettierImage} img
* @param {Rgb} ref_color
* @param {number} amt
*/
export function selective_lighten(img: PrettierImage, ref_color: Rgb, amt: number): void;
/**
* @param {PrettierImage} img
* @param {Rgb} ref_color
* @param {number} amt
*/
export function selective_desaturate(img: PrettierImage, ref_color: Rgb, amt: number): void;
/**
* @param {PrettierImage} img
* @param {Rgb} ref_color
* @param {number} amt
*/
export function selective_saturate(img: PrettierImage, ref_color: Rgb, amt: number): void;
/**
* @param {PrettierImage} prettier_image
* @param {Rgb} ref_color
*/
export function selective_greyscale(prettier_image: PrettierImage, ref_color: Rgb): void;
/**
* @param {PrettierImage} prettier_image
* @param {number} red
* @param {number} green
* @param {number} blue
*/
export function gamma_correction(prettier_image: PrettierImage, red: number, green: number, blue: number): void;
/**
* @param {PrettierImage} prettier_image
* @param {string} mode
* @param {number} amt
*/
export function hsluv(prettier_image: PrettierImage, mode: string, amt: number): void;
/**
* @param {PrettierImage} prettier_image
* @param {string} mode
* @param {number} amt
*/
export function lch(prettier_image: PrettierImage, mode: string, amt: number): void;
/**
* @param {PrettierImage} prettier_image
* @param {string} mode
* @param {number} amt
*/
export function hsl(prettier_image: PrettierImage, mode: string, amt: number): void;
/**
* @param {PrettierImage} prettier_image
* @param {string} mode
* @param {number} amt
*/
export function hsv(prettier_image: PrettierImage, mode: string, amt: number): void;
/**
* @param {PrettierImage} img
* @param {number} degrees
*/
export function hue_rotate_hsl(img: PrettierImage, degrees: number): void;
/**
* @param {PrettierImage} img
* @param {number} degrees
*/
export function hue_rotate_hsv(img: PrettierImage, degrees: number): void;
/**
* @param {PrettierImage} img
* @param {number} degrees
*/
export function hue_rotate_lch(img: PrettierImage, degrees: number): void;
/**
* @param {PrettierImage} img
* @param {number} degrees
*/
export function hue_rotate_hsluv(img: PrettierImage, degrees: number): void;
/**
* @param {PrettierImage} img
* @param {number} level
*/
export function saturate_hsl(img: PrettierImage, level: number): void;
/**
* @param {PrettierImage} img
* @param {number} level
*/
export function saturate_lch(img: PrettierImage, level: number): void;
/**
* @param {PrettierImage} img
* @param {number} level
*/
export function saturate_hsluv(img: PrettierImage, level: number): void;
/**
* @param {PrettierImage} img
* @param {number} level
*/
export function saturate_hsv(img: PrettierImage, level: number): void;
/**
* @param {PrettierImage} img
* @param {number} level
*/
export function lighten_lch(img: PrettierImage, level: number): void;
/**
* @param {PrettierImage} img
* @param {number} level
*/
export function lighten_hsluv(img: PrettierImage, level: number): void;
/**
* @param {PrettierImage} img
* @param {number} level
*/
export function lighten_hsl(img: PrettierImage, level: number): void;
/**
* @param {PrettierImage} img
* @param {number} level
*/
export function lighten_hsv(img: PrettierImage, level: number): void;
/**
* @param {PrettierImage} img
* @param {number} level
*/
export function darken_lch(img: PrettierImage, level: number): void;
/**
* @param {PrettierImage} img
* @param {number} level
*/
export function darken_hsluv(img: PrettierImage, level: number): void;
/**
* @param {PrettierImage} img
* @param {number} level
*/
export function darken_hsl(img: PrettierImage, level: number): void;
/**
* @param {PrettierImage} img
* @param {number} level
*/
export function darken_hsv(img: PrettierImage, level: number): void;
/**
* @param {PrettierImage} img
* @param {number} level
*/
export function desaturate_hsv(img: PrettierImage, level: number): void;
/**
* @param {PrettierImage} img
* @param {number} level
*/
export function desaturate_hsl(img: PrettierImage, level: number): void;
/**
* @param {PrettierImage} img
* @param {number} level
*/
export function desaturate_lch(img: PrettierImage, level: number): void;
/**
* @param {PrettierImage} img
* @param {number} level
*/
export function desaturate_hsluv(img: PrettierImage, level: number): void;
/**
* @param {PrettierImage} prettier_image
* @param {Rgb} mix_colour
* @param {number} opacity
*/
export function mix_with_colour(prettier_image: PrettierImage, mix_colour: Rgb, opacity: number): void;
/**
*! [temp] Check if WASM is supported.
*/
export function run(): void;
/**
* @param {HTMLCanvasElement} canvas
* @param {CanvasRenderingContext2D} ctx
* @returns {ImageData}
*/
export function get_image_data(canvas: HTMLCanvasElement, ctx: CanvasRenderingContext2D): ImageData;
/**
* @param {HTMLCanvasElement} canvas
* @param {CanvasRenderingContext2D} ctx
* @param {PrettierImage} new_image
*/
export function putImageData(canvas: HTMLCanvasElement, ctx: CanvasRenderingContext2D, new_image: PrettierImage): void;
/**
* @param {HTMLCanvasElement} canvas
* @param {CanvasRenderingContext2D} ctx
* @returns {PrettierImage}
*/
export function open_image(canvas: HTMLCanvasElement, ctx: CanvasRenderingContext2D): PrettierImage;
/**
* @param {ImageData} imgdata
* @returns {Uint8Array}
*/
export function to_raw_pixels(imgdata: ImageData): Uint8Array;
/**
* @param {string} base64
* @returns {PrettierImage}
*/
export function base64_to_image(base64: string): PrettierImage;
/**
* @param {string} base64
* @returns {Uint8Array}
*/
export function base64_to_vec(base64: string): Uint8Array;
/**
* @param {PrettierImage} prettier_image
*/
export function neue(prettier_image: PrettierImage): void;
/**
* @param {PrettierImage} prettier_image
*/
export function lix(prettier_image: PrettierImage): void;
/**
* @param {PrettierImage} prettier_image
*/
export function ryo(prettier_image: PrettierImage): void;
/**
* @param {PrettierImage} img
* @param {string} filter_name
*/
export function filter(img: PrettierImage, filter_name: string): void;
/**
* @param {PrettierImage} img
*/
export function lofi(img: PrettierImage): void;
/**
* @param {PrettierImage} img
*/
export function pastel_pink(img: PrettierImage): void;
/**
* @param {PrettierImage} img
*/
export function golden(img: PrettierImage): void;
/**
* @param {PrettierImage} img
*/
export function cali(img: PrettierImage): void;
/**
* @param {PrettierImage} img
*/
export function duotone_violette(img: PrettierImage): void;
/**
* @param {PrettierImage} img
*/
export function duotone_horizon(img: PrettierImage): void;
/**
* @param {PrettierImage} img
* @param {Rgb} rgb_color
*/
export function duotone_tint(img: PrettierImage, rgb_color: Rgb): void;
/**
* @param {PrettierImage} img
*/
export function duotone_lilac(img: PrettierImage): void;
/**
* @param {PrettierImage} img
*/
export function duotone_ochre(img: PrettierImage): void;
/**
* @param {PrettierImage} img
*/
export function firenze(img: PrettierImage): void;
/**
*/
export class PrettierImage {
  free(): void;
/**
* @param {Uint8Array} raw_pixels
* @param {number} width
* @param {number} height
*/
  constructor(raw_pixels: Uint8Array, width: number, height: number);
/**
* @returns {number}
*/
  get_width(): number;
/**
* @returns {Uint8Array}
*/
  get_raw_pixels(): Uint8Array;
/**
* @returns {number}
*/
  get_height(): number;
}
/**
*/
export class Rgb {
  free(): void;
/**
* @param {number} r
* @param {number} g
* @param {number} b
*/
  constructor(r: number, g: number, b: number);
/**
* @param {number} r
*/
  set_red(r: number): void;
/**
* @param {number} g
*/
  set_green(g: number): void;
/**
* @param {number} b
*/
  set_blue(b: number): void;
/**
* @returns {number}
*/
  get_red(): number;
/**
* @returns {number}
*/
  get_green(): number;
/**
* @returns {number}
*/
  get_blue(): number;
}
/**
*/
export class Rgba {
  free(): void;
/**
* @param {number} r
* @param {number} g
* @param {number} b
* @param {number} a
*/
  constructor(r: number, g: number, b: number, a: number);
/**
* @param {number} r
*/
  set_red(r: number): void;
/**
* @param {number} g
*/
  set_green(g: number): void;
/**
* @param {number} b
*/
  set_blue(b: number): void;
/**
* @param {number} a
*/
  set_alpha(a: number): void;
/**
* @returns {number}
*/
  get_red(): number;
/**
* @returns {number}
*/
  get_green(): number;
/**
* @returns {number}
*/
  get_blue(): number;
/**
* @returns {number}
*/
  get_alpha(): number;
}
