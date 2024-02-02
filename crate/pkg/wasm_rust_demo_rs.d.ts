/* tslint:disable */
/* eslint-disable */
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
export function firenze(img: PrettierImage): void;
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
* @param {PrettierImage} prettier_image
* @param {Rgb} mix_colour
* @param {number} opacity
*/
export function mix_with_colour(prettier_image: PrettierImage, mix_colour: Rgb, opacity: number): void;
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
}
