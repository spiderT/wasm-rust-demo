let wasm;
export function __wbg_set_wasm(val) {
    wasm = val;
}


const heap = new Array(128).fill(undefined);

heap.push(undefined, null, true, false);

function getObject(idx) { return heap[idx]; }

let heap_next = heap.length;

function dropObject(idx) {
    if (idx < 132) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}

function debugString(val) {
    // primitive types
    const type = typeof val;
    if (type == 'number' || type == 'boolean' || val == null) {
        return  `${val}`;
    }
    if (type == 'string') {
        return `"${val}"`;
    }
    if (type == 'symbol') {
        const description = val.description;
        if (description == null) {
            return 'Symbol';
        } else {
            return `Symbol(${description})`;
        }
    }
    if (type == 'function') {
        const name = val.name;
        if (typeof name == 'string' && name.length > 0) {
            return `Function(${name})`;
        } else {
            return 'Function';
        }
    }
    // objects
    if (Array.isArray(val)) {
        const length = val.length;
        let debug = '[';
        if (length > 0) {
            debug += debugString(val[0]);
        }
        for(let i = 1; i < length; i++) {
            debug += ', ' + debugString(val[i]);
        }
        debug += ']';
        return debug;
    }
    // Test for built-in
    const builtInMatches = /\[object ([^\]]+)\]/.exec(toString.call(val));
    let className;
    if (builtInMatches.length > 1) {
        className = builtInMatches[1];
    } else {
        // Failed to match the standard '[object ClassName]'
        return toString.call(val);
    }
    if (className == 'Object') {
        // we're a user defined class or Object
        // JSON.stringify avoids problems with cycles, and is generally much
        // easier than looping through ownProperties of `val`.
        try {
            return 'Object(' + JSON.stringify(val) + ')';
        } catch (_) {
            return 'Object';
        }
    }
    // errors
    if (val instanceof Error) {
        return `${val.name}: ${val.message}\n${val.stack}`;
    }
    // TODO we could test for more things here, like `Set`s and `Map`s.
    return className;
}

let WASM_VECTOR_LEN = 0;

let cachedUint8Memory0 = null;

function getUint8Memory0() {
    if (cachedUint8Memory0 === null || cachedUint8Memory0.byteLength === 0) {
        cachedUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8Memory0;
}

const lTextEncoder = typeof TextEncoder === 'undefined' ? (0, module.require)('util').TextEncoder : TextEncoder;

let cachedTextEncoder = new lTextEncoder('utf-8');

const encodeString = (typeof cachedTextEncoder.encodeInto === 'function'
    ? function (arg, view) {
    return cachedTextEncoder.encodeInto(arg, view);
}
    : function (arg, view) {
    const buf = cachedTextEncoder.encode(arg);
    view.set(buf);
    return {
        read: arg.length,
        written: buf.length
    };
});

function passStringToWasm0(arg, malloc, realloc) {

    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length) >>> 0;
        getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len) >>> 0;

    const mem = getUint8Memory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }

    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3) >>> 0;
        const view = getUint8Memory0().subarray(ptr + offset, ptr + len);
        const ret = encodeString(arg, view);

        offset += ret.written;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

let cachedInt32Memory0 = null;

function getInt32Memory0() {
    if (cachedInt32Memory0 === null || cachedInt32Memory0.byteLength === 0) {
        cachedInt32Memory0 = new Int32Array(wasm.memory.buffer);
    }
    return cachedInt32Memory0;
}

const lTextDecoder = typeof TextDecoder === 'undefined' ? (0, module.require)('util').TextDecoder : TextDecoder;

let cachedTextDecoder = new lTextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}

function _assertClass(instance, klass) {
    if (!(instance instanceof klass)) {
        throw new Error(`expected instance of ${klass.name}`);
    }
    return instance.ptr;
}
/**
* @param {PrettierImage} img
* @param {string} filter_name
*/
export function filter(img, filter_name) {
    _assertClass(img, PrettierImage);
    const ptr0 = passStringToWasm0(filter_name, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    wasm.filter(img.__wbg_ptr, ptr0, len0);
}

/**
* @param {PrettierImage} img
*/
export function lofi(img) {
    _assertClass(img, PrettierImage);
    wasm.lofi(img.__wbg_ptr);
}

/**
* @param {PrettierImage} img
*/
export function pastel_pink(img) {
    _assertClass(img, PrettierImage);
    wasm.pastel_pink(img.__wbg_ptr);
}

/**
* @param {PrettierImage} img
*/
export function golden(img) {
    _assertClass(img, PrettierImage);
    wasm.golden(img.__wbg_ptr);
}

/**
* @param {PrettierImage} img
*/
export function cali(img) {
    _assertClass(img, PrettierImage);
    wasm.cali(img.__wbg_ptr);
}

/**
* @param {PrettierImage} img
*/
export function firenze(img) {
    _assertClass(img, PrettierImage);
    wasm.firenze(img.__wbg_ptr);
}

/**
* @param {PrettierImage} prettier_image
* @param {number} brightness
*/
export function inc_brightness(prettier_image, brightness) {
    _assertClass(prettier_image, PrettierImage);
    wasm.inc_brightness(prettier_image.__wbg_ptr, brightness);
}

/**
* @param {PrettierImage} prettier_image
* @param {number} contrast
*/
export function adjust_contrast(prettier_image, contrast) {
    _assertClass(prettier_image, PrettierImage);
    wasm.adjust_contrast(prettier_image.__wbg_ptr, contrast);
}

function passArray8ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 1) >>> 0;
    getUint8Memory0().set(arg, ptr / 1);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

function getArrayU8FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint8Memory0().subarray(ptr / 1, ptr / 1 + len);
}
/**
*/
export function run() {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        wasm.run(retptr);
        var r0 = getInt32Memory0()[retptr / 4 + 0];
        var r1 = getInt32Memory0()[retptr / 4 + 1];
        if (r1) {
            throw takeObject(r0);
        }
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}

let stack_pointer = 128;

function addBorrowedObject(obj) {
    if (stack_pointer == 1) throw new Error('out of js stack');
    heap[--stack_pointer] = obj;
    return stack_pointer;
}
/**
* @param {HTMLCanvasElement} canvas
* @param {CanvasRenderingContext2D} ctx
* @returns {ImageData}
*/
export function get_image_data(canvas, ctx) {
    try {
        const ret = wasm.get_image_data(addBorrowedObject(canvas), addBorrowedObject(ctx));
        return takeObject(ret);
    } finally {
        heap[stack_pointer++] = undefined;
        heap[stack_pointer++] = undefined;
    }
}

/**
* @param {HTMLCanvasElement} canvas
* @param {CanvasRenderingContext2D} ctx
* @param {PrettierImage} new_image
*/
export function putImageData(canvas, ctx, new_image) {
    _assertClass(new_image, PrettierImage);
    var ptr0 = new_image.__destroy_into_raw();
    wasm.putImageData(addHeapObject(canvas), addHeapObject(ctx), ptr0);
}

/**
* @param {HTMLCanvasElement} canvas
* @param {CanvasRenderingContext2D} ctx
* @returns {PrettierImage}
*/
export function open_image(canvas, ctx) {
    const ret = wasm.open_image(addHeapObject(canvas), addHeapObject(ctx));
    return PrettierImage.__wrap(ret);
}

/**
* @param {ImageData} imgdata
* @returns {Uint8Array}
*/
export function to_raw_pixels(imgdata) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        wasm.to_raw_pixels(retptr, addHeapObject(imgdata));
        var r0 = getInt32Memory0()[retptr / 4 + 0];
        var r1 = getInt32Memory0()[retptr / 4 + 1];
        var v1 = getArrayU8FromWasm0(r0, r1).slice();
        wasm.__wbindgen_free(r0, r1 * 1);
        return v1;
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}

/**
* @param {PrettierImage} prettier_image
* @param {string} mode
* @param {number} amt
*/
export function lch(prettier_image, mode, amt) {
    _assertClass(prettier_image, PrettierImage);
    const ptr0 = passStringToWasm0(mode, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    wasm.lch(prettier_image.__wbg_ptr, ptr0, len0, amt);
}

/**
* @param {PrettierImage} prettier_image
* @param {string} mode
* @param {number} amt
*/
export function hsl(prettier_image, mode, amt) {
    _assertClass(prettier_image, PrettierImage);
    const ptr0 = passStringToWasm0(mode, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    wasm.hsl(prettier_image.__wbg_ptr, ptr0, len0, amt);
}

/**
* @param {PrettierImage} prettier_image
* @param {string} mode
* @param {number} amt
*/
export function hsv(prettier_image, mode, amt) {
    _assertClass(prettier_image, PrettierImage);
    const ptr0 = passStringToWasm0(mode, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    wasm.hsv(prettier_image.__wbg_ptr, ptr0, len0, amt);
}

/**
* @param {PrettierImage} img
* @param {number} degrees
*/
export function hue_rotate_hsl(img, degrees) {
    _assertClass(img, PrettierImage);
    wasm.hue_rotate_hsl(img.__wbg_ptr, degrees);
}

/**
* @param {PrettierImage} img
* @param {number} degrees
*/
export function hue_rotate_hsv(img, degrees) {
    _assertClass(img, PrettierImage);
    wasm.hue_rotate_hsv(img.__wbg_ptr, degrees);
}

/**
* @param {PrettierImage} img
* @param {number} degrees
*/
export function hue_rotate_lch(img, degrees) {
    _assertClass(img, PrettierImage);
    wasm.hue_rotate_lch(img.__wbg_ptr, degrees);
}

/**
* @param {PrettierImage} img
* @param {number} level
*/
export function saturate_hsl(img, level) {
    _assertClass(img, PrettierImage);
    wasm.saturate_hsl(img.__wbg_ptr, level);
}

/**
* @param {PrettierImage} img
* @param {number} level
*/
export function saturate_lch(img, level) {
    _assertClass(img, PrettierImage);
    wasm.saturate_lch(img.__wbg_ptr, level);
}

/**
* @param {PrettierImage} img
* @param {number} level
*/
export function saturate_hsv(img, level) {
    _assertClass(img, PrettierImage);
    wasm.saturate_hsv(img.__wbg_ptr, level);
}

/**
* @param {PrettierImage} img
* @param {number} level
*/
export function lighten_lch(img, level) {
    _assertClass(img, PrettierImage);
    wasm.lighten_lch(img.__wbg_ptr, level);
}

/**
* @param {PrettierImage} img
* @param {number} level
*/
export function lighten_hsl(img, level) {
    _assertClass(img, PrettierImage);
    wasm.lighten_hsl(img.__wbg_ptr, level);
}

/**
* @param {PrettierImage} img
* @param {number} level
*/
export function lighten_hsv(img, level) {
    _assertClass(img, PrettierImage);
    wasm.lighten_hsv(img.__wbg_ptr, level);
}

/**
* @param {PrettierImage} img
* @param {number} level
*/
export function darken_lch(img, level) {
    _assertClass(img, PrettierImage);
    wasm.darken_lch(img.__wbg_ptr, level);
}

/**
* @param {PrettierImage} img
* @param {number} level
*/
export function darken_hsl(img, level) {
    _assertClass(img, PrettierImage);
    wasm.darken_hsl(img.__wbg_ptr, level);
}

/**
* @param {PrettierImage} img
* @param {number} level
*/
export function darken_hsv(img, level) {
    _assertClass(img, PrettierImage);
    wasm.darken_hsv(img.__wbg_ptr, level);
}

/**
* @param {PrettierImage} img
* @param {number} level
*/
export function desaturate_hsv(img, level) {
    _assertClass(img, PrettierImage);
    wasm.desaturate_hsv(img.__wbg_ptr, level);
}

/**
* @param {PrettierImage} img
* @param {number} level
*/
export function desaturate_hsl(img, level) {
    _assertClass(img, PrettierImage);
    wasm.desaturate_hsl(img.__wbg_ptr, level);
}

/**
* @param {PrettierImage} img
* @param {number} level
*/
export function desaturate_lch(img, level) {
    _assertClass(img, PrettierImage);
    wasm.desaturate_lch(img.__wbg_ptr, level);
}

/**
* @param {PrettierImage} prettier_image
* @param {Rgb} mix_colour
* @param {number} opacity
*/
export function mix_with_colour(prettier_image, mix_colour, opacity) {
    _assertClass(prettier_image, PrettierImage);
    _assertClass(mix_colour, Rgb);
    var ptr0 = mix_colour.__destroy_into_raw();
    wasm.mix_with_colour(prettier_image.__wbg_ptr, ptr0, opacity);
}

/**
* @param {PrettierImage} img
* @param {number} channel
* @param {number} amt
*/
export function alter_channel(img, channel, amt) {
    _assertClass(img, PrettierImage);
    wasm.alter_channel(img.__wbg_ptr, channel, amt);
}

/**
* @param {PrettierImage} prettier_image
* @param {number} amt
*/
export function alter_red_channel(prettier_image, amt) {
    _assertClass(prettier_image, PrettierImage);
    wasm.alter_red_channel(prettier_image.__wbg_ptr, amt);
}

/**
* @param {PrettierImage} img
* @param {number} amt
*/
export function alter_green_channel(img, amt) {
    _assertClass(img, PrettierImage);
    wasm.alter_green_channel(img.__wbg_ptr, amt);
}

/**
* @param {PrettierImage} img
* @param {number} amt
*/
export function alter_blue_channel(img, amt) {
    _assertClass(img, PrettierImage);
    wasm.alter_blue_channel(img.__wbg_ptr, amt);
}

/**
* @param {PrettierImage} img
* @param {number} r_amt
* @param {number} g_amt
* @param {number} b_amt
*/
export function alter_channels(img, r_amt, g_amt, b_amt) {
    _assertClass(img, PrettierImage);
    wasm.alter_channels(img.__wbg_ptr, r_amt, g_amt, b_amt);
}

/**
* @param {PrettierImage} img
* @param {number} channel
* @param {number} min_filter
*/
export function remove_channel(img, channel, min_filter) {
    _assertClass(img, PrettierImage);
    wasm.remove_channel(img.__wbg_ptr, channel, min_filter);
}

/**
* @param {PrettierImage} img
* @param {number} min_filter
*/
export function remove_red_channel(img, min_filter) {
    _assertClass(img, PrettierImage);
    wasm.remove_red_channel(img.__wbg_ptr, min_filter);
}

/**
* @param {PrettierImage} img
* @param {number} min_filter
*/
export function remove_green_channel(img, min_filter) {
    _assertClass(img, PrettierImage);
    wasm.remove_green_channel(img.__wbg_ptr, min_filter);
}

/**
* @param {PrettierImage} img
* @param {number} min_filter
*/
export function remove_blue_channel(img, min_filter) {
    _assertClass(img, PrettierImage);
    wasm.remove_blue_channel(img.__wbg_ptr, min_filter);
}

/**
* @param {PrettierImage} img
* @param {number} channel1
* @param {number} channel2
*/
export function swap_channels(img, channel1, channel2) {
    _assertClass(img, PrettierImage);
    wasm.swap_channels(img.__wbg_ptr, channel1, channel2);
}

/**
* @param {PrettierImage} prettier_image
*/
export function invert(prettier_image) {
    _assertClass(prettier_image, PrettierImage);
    wasm.invert(prettier_image.__wbg_ptr);
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

function handleError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        wasm.__wbindgen_exn_store(addHeapObject(e));
    }
}

let cachedUint8ClampedMemory0 = null;

function getUint8ClampedMemory0() {
    if (cachedUint8ClampedMemory0 === null || cachedUint8ClampedMemory0.byteLength === 0) {
        cachedUint8ClampedMemory0 = new Uint8ClampedArray(wasm.memory.buffer);
    }
    return cachedUint8ClampedMemory0;
}

function getClampedArrayU8FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint8ClampedMemory0().subarray(ptr / 1, ptr / 1 + len);
}
/**
*/
export class PrettierImage {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(PrettierImage.prototype);
        obj.__wbg_ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_prettierimage_free(ptr);
    }
    /**
    * @param {Uint8Array} raw_pixels
    * @param {number} width
    * @param {number} height
    */
    constructor(raw_pixels, width, height) {
        const ptr0 = passArray8ToWasm0(raw_pixels, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.prettierimage_new(ptr0, len0, width, height);
        return PrettierImage.__wrap(ret);
    }
    /**
    * @returns {number}
    */
    get_width() {
        const ret = wasm.prettierimage_get_width(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
    * @returns {Uint8Array}
    */
    get_raw_pixels() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.prettierimage_get_raw_pixels(retptr, this.__wbg_ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var v1 = getArrayU8FromWasm0(r0, r1).slice();
            wasm.__wbindgen_free(r0, r1 * 1);
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @returns {number}
    */
    get_height() {
        const ret = wasm.prettierimage_get_height(this.__wbg_ptr);
        return ret >>> 0;
    }
}
/**
*/
export class Rgb {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Rgb.prototype);
        obj.__wbg_ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_rgb_free(ptr);
    }
    /**
    * @param {number} r
    * @param {number} g
    * @param {number} b
    */
    constructor(r, g, b) {
        const ret = wasm.rgb_new(r, g, b);
        return Rgb.__wrap(ret);
    }
}
/**
*/
export class Rgba {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Rgba.prototype);
        obj.__wbg_ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_rgba_free(ptr);
    }
    /**
    * @param {number} r
    * @param {number} g
    * @param {number} b
    * @param {number} a
    */
    constructor(r, g, b, a) {
        const ret = wasm.rgba_new(r, g, b, a);
        return Rgba.__wrap(ret);
    }
}

export function __wbindgen_object_drop_ref(arg0) {
    takeObject(arg0);
};

export function __wbg_new_abda76e883ba8a5f() {
    const ret = new Error();
    return addHeapObject(ret);
};

export function __wbg_stack_658279fe44541cf6(arg0, arg1) {
    const ret = getObject(arg1).stack;
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len1;
    getInt32Memory0()[arg0 / 4 + 0] = ptr1;
};

export function __wbg_error_f851667af71bcfc6(arg0, arg1) {
    let deferred0_0;
    let deferred0_1;
    try {
        deferred0_0 = arg0;
        deferred0_1 = arg1;
        console.error(getStringFromWasm0(arg0, arg1));
    } finally {
        wasm.__wbindgen_free(deferred0_0, deferred0_1);
    }
};

export function __wbg_body_483afe07b0958d3b(arg0) {
    const ret = getObject(arg0).body;
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
};

export function __wbg_createElement_5281e2aae74efc9d() { return handleError(function (arg0, arg1, arg2) {
    const ret = getObject(arg0).createElement(getStringFromWasm0(arg1, arg2));
    return addHeapObject(ret);
}, arguments) };

export function __wbg_instanceof_Window_f2bf9e8e91f1be0d(arg0) {
    let result;
    try {
        result = getObject(arg0) instanceof Window;
    } catch {
        result = false;
    }
    const ret = result;
    return ret;
};

export function __wbg_document_a11e2f345af07033(arg0) {
    const ret = getObject(arg0).document;
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
};

export function __wbg_getImageData_956c421f9b7cdfe7() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
    const ret = getObject(arg0).getImageData(arg1, arg2, arg3, arg4);
    return addHeapObject(ret);
}, arguments) };

export function __wbg_putImageData_0009acf77045dec6() { return handleError(function (arg0, arg1, arg2, arg3) {
    getObject(arg0).putImageData(getObject(arg1), arg2, arg3);
}, arguments) };

export function __wbg_settextContent_ea2ce5e8fc889af7(arg0, arg1, arg2) {
    getObject(arg0).textContent = arg1 === 0 ? undefined : getStringFromWasm0(arg1, arg2);
};

export function __wbg_appendChild_173b88a25c048f2b() { return handleError(function (arg0, arg1) {
    const ret = getObject(arg0).appendChild(getObject(arg1));
    return addHeapObject(ret);
}, arguments) };

export function __wbg_width_3a395887a577233b(arg0) {
    const ret = getObject(arg0).width;
    return ret;
};

export function __wbg_height_b7046017c4148386(arg0) {
    const ret = getObject(arg0).height;
    return ret;
};

export function __wbg_data_f319d3380b214a26(arg0, arg1) {
    const ret = getObject(arg1).data;
    const ptr1 = passArray8ToWasm0(ret, wasm.__wbindgen_malloc);
    const len1 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len1;
    getInt32Memory0()[arg0 / 4 + 0] = ptr1;
};

export function __wbg_newwithu8clampedarrayandsh_c90e0c3609c4ecc2() { return handleError(function (arg0, arg1, arg2, arg3) {
    const ret = new ImageData(getClampedArrayU8FromWasm0(arg0, arg1), arg2 >>> 0, arg3 >>> 0);
    return addHeapObject(ret);
}, arguments) };

export function __wbg_newnoargs_e643855c6572a4a8(arg0, arg1) {
    const ret = new Function(getStringFromWasm0(arg0, arg1));
    return addHeapObject(ret);
};

export function __wbg_call_f96b398515635514() { return handleError(function (arg0, arg1) {
    const ret = getObject(arg0).call(getObject(arg1));
    return addHeapObject(ret);
}, arguments) };

export function __wbindgen_object_clone_ref(arg0) {
    const ret = getObject(arg0);
    return addHeapObject(ret);
};

export function __wbg_self_b9aad7f1c618bfaf() { return handleError(function () {
    const ret = self.self;
    return addHeapObject(ret);
}, arguments) };

export function __wbg_window_55e469842c98b086() { return handleError(function () {
    const ret = window.window;
    return addHeapObject(ret);
}, arguments) };

export function __wbg_globalThis_d0957e302752547e() { return handleError(function () {
    const ret = globalThis.globalThis;
    return addHeapObject(ret);
}, arguments) };

export function __wbg_global_ae2f87312b8987fb() { return handleError(function () {
    const ret = global.global;
    return addHeapObject(ret);
}, arguments) };

export function __wbindgen_is_undefined(arg0) {
    const ret = getObject(arg0) === undefined;
    return ret;
};

export function __wbindgen_debug_string(arg0, arg1) {
    const ret = debugString(getObject(arg1));
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len1;
    getInt32Memory0()[arg0 / 4 + 0] = ptr1;
};

export function __wbindgen_throw(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
};

