// Miscellaneous code that may or may not be used in the project

// Gamma correction = 2.8
// https://victornpb.github.io/gamma-table-generator/
pub const GAMMA: [u8; 256] = [
    0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,
    0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,
    0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,
    0,   0,   0,   0,   0,   0,   1,   1,   1,   1,   1,   1,   1,   1,   1,   1,
    1,   1,   1,   1,   1,   1,   1,   2,   2,   2,   2,   2,   2,   2,   2,   2,
    2,   3,   3,   3,   3,   3,   3,   3,   4,   4,   4,   4,   4,   5,   5,   5,
    5,   5,   6,   6,   6,   6,   7,   7,   7,   7,   8,   8,   8,   9,   9,   9,
    9,  10,  10,  11,  11,  11,  12,  12,  13,  13,  13,  14,  14,  15,  15,  16,
   16,  17,  17,  18,  18,  19,  19,  20,  21,  21,  22,  23,  23,  24,  25,  25,
   26,  27,  27,  28,  29,  30,  31,  31,  32,  33,  34,  35,  36,  37,  38,  39,
   40,  41,  42,  43,  44,  45,  46,  47,  48,  49,  50,  52,  53,  54,  55,  57,
   58,  59,  61,  62,  63,  65,  66,  68,  69,  71,  72,  74,  75,  77,  79,  80,
   82,  84,  85,  87,  89,  91,  93,  95,  96,  98, 100, 102, 104, 107, 109, 111,
  113, 115, 117, 120, 122, 124, 126, 129, 131, 134, 136, 139, 141, 144, 146, 149,
  152, 155, 157, 160, 163, 166, 169, 172, 175, 178, 181, 184, 187, 190, 194, 197,
  200, 203, 207, 210, 214, 217, 221, 224, 228, 232, 236, 239, 243, 247, 251, 255,
];

/// Set GPIO to output mode
#[macro_export]
macro_rules! set_gpio_out_pp_ls {
    ($gpio:expr, $pin:literal) => {
        paste! {
            $gpio.moder.modify(|_, w| w.[<moder $pin>]().output());
            $gpio.otyper.modify(|_, w| w.[<ot $pin>]().push_pull());
            $gpio.ospeedr.modify(|_, w| w.[<ospeedr $pin>]().low_speed());
        }
    };
}

/// HSV to RGB
pub const fn hsv_to_rgb(scale: i32, h: i32, s: i32, v: i32) -> [i32; 3] {
    // https://www.rapidtables.com/convert/color/hsv-to-rgb.html
    let c = v * s / scale;
    let t = (((h / 60) % (2 * scale) - 1 * scale)).abs();
    let x = c * (1 * scale - t) / scale;
    let m = v - c;
    let (r, g, b);
    if        (0)           <= h && h < (60 * scale) {
        (r, g ,b) = (c, x, 0);
    } else if (60 * scale)  <= h && h < (120 * scale) {
        (r, g ,b) = (x, c, 0);
    } else if (120 * scale) <= h && h < (180 * scale) {
        (r, g ,b) = (0, c, x);
    } else if (180 * scale) <= h && h < (240 * scale) {
        (r, g ,b) = (0, x, c);
    } else if (240 * scale) <= h && h < (300 * scale) {
        (r, g ,b) = (x, 0, c);
    } else {
        (r, g ,b) = (c, 0, x);
    }
    return [r + m, g + m, b + m];
}

/// Create a HUE LUT of 360 degrees
const fn hue_lut() -> [[i32; 3]; 1024] {
    let mut colour: [[i32; 3]; 1024] = [[0; 3]; 1024];
    let scale = 1024i32;
    let mut i = 0i32;
    while i < scale {
        let h = 360 * scale * i / scale;
        let s = scale;
        let v = scale;
        colour[i as usize] = hsv_to_rgb(scale, h, s, v);
        i += 1;
    }
    return colour;
}

/// A compile-time constant HUE LUT
pub const HUE_LUT: [[i32; 3]; 1024] = hue_lut();

/// Colour LUT created from https://tristen.ca/hcl-picker/
const fn hcl_lut() -> [[u8; 3]; 200] {
    let pilots: [u32; 200] = [
        0xFA8975,0xFB8977,0xFB8979,0xFC887B,0xFC887D,0xFC887F,0xFC8781,0xFC8784,
        0xFC8786,0xFC8788,0xFC878A,0xFC878C,0xFB868E,0xFB8790,0xFB8793,0xFA8795,
        0xFA8797,0xF98799,0xF8879B,0xF7879D,0xF7889F,0xF688A2,0xF588A4,0xF489A6,
        0xF289A8,0xF18AAA,0xF08AAC,0xEF8BAE,0xED8BB0,0xEC8CB2,0xEA8DB4,0xE98DB6,
        0xE78EB7,0xE58FB9,0xE390BB,0xE290BD,0xE091BE,0xDE92C0,0xDC93C2,0xD994C3,
        0xD794C5,0xD595C6,0xD396C8,0xD097C9,0xCE98CB,0xCB99CC,0xC99ACD,0xC69BCE,
        0xC49CCF,0xC19CD0,0xBE9DD1,0xBC9ED2,0xB99FD3,0xB6A0D4,0xB3A1D5,0xB0A2D6,
        0xADA3D6,0xAAA4D7,0xA7A4D7,0xA4A5D8,0xA1A6D8,0x9EA7D8,0x9BA8D9,0x98A9D9,
        0x95A9D9,0x91AAD9,0x8EABD9,0x8BACD9,0x88ADD9,0x85ADD9,0x81AED8,0x7EAFD8,
        0x7BAFD8,0x78B0D7,0x74B1D6,0x71B1D6,0x6EB2D5,0x6BB3D4,0x68B3D4,0x65B4D3,
        0x61B4D2,0x5EB5D1,0x5BB5D0,0x59B6CF,0x56B6CD,0x53B7CC,0x50B7CB,0x4EB8CA,
        0x4BB8C8,0x49B8C7,0x46B9C5,0x44B9C4,0x42BAC2,0x41BAC0,0x3FBABF,0x3EBBBD,
        0x3CBBBB,0x3BBBB9,0x3BBBB8,0x3ABCB6,0x3ABCB4,0x3ABCB2,0x3ABCB0,0x3ABCAE,
        0x3BBCAC,0x3CBDAA,0x3DBDA8,0x3EBDA6,0x40BDA3,0x41BDA1,0x43BD9F,0x44BD9D,
        0x46BD9B,0x48BD99,0x4ABD96,0x4CBD94,0x4FBD92,0x51BD90,0x53BD8E,0x55BD8B,
        0x58BD89,0x5ABD87,0x5CBC85,0x5FBC82,0x61BC80,0x64BC7E,0x66BC7C,0x68BC7A,
        0x6BBB78,0x6DBB76,0x70BB73,0x72BB71,0x75BA6F,0x77BA6D,0x7ABA6B,0x7CB969,
        0x7FB967,0x81B966,0x84B864,0x86B862,0x89B760,0x8BB75E,0x8DB75D,0x90B65B,
        0x92B659,0x95B558,0x97B556,0x9AB455,0x9CB454,0x9EB352,0xA1B351,0xA3B250,
        0xA6B14F,0xA8B14E,0xAAB04D,0xADB04C,0xAFAF4B,0xB1AE4A,0xB4AE49,0xB6AD49,
        0xB8AC48,0xBAAB48,0xBDAB48,0xBFAA47,0xC1A947,0xC3A847,0xC5A847,0xC8A747,
        0xCAA647,0xCCA547,0xCEA448,0xD0A448,0xD2A348,0xD4A249,0xD6A14A,0xD8A04A,
        0xDA9F4B,0xDB9F4C,0xDD9E4D,0xDF9D4E,0xE19C4F,0xE29B50,0xE49A51,0xE69952,
        0xE79954,0xE99855,0xEA9756,0xEC9658,0xED9559,0xEF945B,0xF0945C,0xF1935E,
        0xF29260,0xF39161,0xF59163,0xF69065,0xF78F67,0xF88E69,0xF88E6A,0xF98D6C
    ];
    let mut hcl: [[u8; 3]; 200] = [[0; 3]; 200];
    let mut i = 0;
    while i < pilots.len() {
        let c = pilots[i];
        hcl[i] = [
            ((c >> 16) & 0xff) as u8,
            ((c >>  8) & 0xff) as u8,
            ((c >>  0) & 0xff) as u8,
        ];
        i += 1;
    }
    return hcl;
}

/// A compile-time constant HCL LUT
pub const HCL_LUT: [[u8; 3]; 200] = hcl_lut();
