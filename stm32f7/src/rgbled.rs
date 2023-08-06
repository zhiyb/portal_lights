//use paste::paste;
use core::mem::MaybeUninit;

//use cortex_m::asm;
use stm32f7::stm32f7x2::Peripherals;

use crate::config::CONFIG;

// RGB LED matrix connections:
//
// PA0  -> R1
// PA1  -> G1
// PA2  -> B1
//
// PA4  -> R2
// PA5  -> G2
// PA6  -> B2
//
// PA9  -> R3
// PA10 -> G3
// PA8  -> B3
//
// PA3  -> A1
// PA7  -> A2
// PA11 -> A3
// PA15 -> A4

const LED_NUM: usize = 12;

// Allocated DMA RAM for refreshing LEDs
const LED_GREYSCALE: usize = 256;
const LED_SCAN_OFF:  usize = 1;
const LED_SCAN_ROWS: usize = 4;
const LED_SCAN_SIZE: usize = LED_SCAN_ROWS * (LED_GREYSCALE + LED_SCAN_OFF);

#[link_section = ".dmaram"]
static mut LED_RAM: MaybeUninit<[[u16; LED_GREYSCALE + LED_SCAN_OFF]; LED_SCAN_ROWS]> = MaybeUninit::uninit();

// Current LED states
static mut LED_COLOUR: [[u8; 3]; LED_NUM] = [[0; 3]; LED_NUM];

// GPIO update rate
const LED_GPIO_RATE: usize = LED_SCAN_SIZE * 480;

// Gamma correction = 4.0
// https://victornpb.github.io/gamma-table-generator/
const GAMMA: [u8; 256] = [
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
//macro_rules! set_gpio_out_pp_ls {
//    ($gpio:expr, $pin:literal) => {
//        paste! {
//            $gpio.moder.modify(|_, w| w.[<moder $pin>]().output());
//            $gpio.otyper.modify(|_, w| w.[<ot $pin>]().push_pull());
//            $gpio.ospeedr.modify(|_, w| w.[<ospeedr $pin>]().low_speed());
//        }
//    };
//}

/// Initialise RGB LED matrix
pub fn rgbled_init(p: &Peripherals) {
    // Power on GPIOA bank
    p.RCC.ahb1enr.modify(|_, w| w.gpioaen().set_bit());
    // Configure GPIOA bank, IO in output mode
    p.GPIOA.odr.modify(|_, w| w
        .odr0 ().high()
        .odr1 ().high()
        .odr2 ().high()
        .odr3 ().high()
        .odr4 ().high()
        .odr5 ().high()
        .odr6 ().high()
        .odr7 ().high()
        .odr8 ().high()
        .odr9 ().high()
        .odr10().high()
        .odr11().high()
        .odr15().high());
    p.GPIOA.moder.modify(|_, w| w
        .moder0 ().output()
        .moder1 ().output()
        .moder2 ().output()
        .moder3 ().output()
        .moder4 ().output()
        .moder5 ().output()
        .moder6 ().output()
        .moder7 ().output()
        .moder8 ().output()
        .moder9 ().output()
        .moder10().output()
        .moder11().output()
        .moder15().output());
    p.GPIOA.otyper.modify(|_, w| w
        .ot0 ().push_pull()
        .ot1 ().push_pull()
        .ot2 ().push_pull()
        .ot3 ().push_pull()
        .ot4 ().push_pull()
        .ot5 ().push_pull()
        .ot6 ().push_pull()
        .ot7 ().push_pull()
        .ot8 ().push_pull()
        .ot9 ().push_pull()
        .ot10().push_pull()
        .ot11().push_pull()
        .ot15().push_pull());
    p.GPIOA.pupdr.modify(|_, w| w
        .pupdr0 ().floating()
        .pupdr1 ().floating()
        .pupdr2 ().floating()
        .pupdr3 ().floating()
        .pupdr4 ().floating()
        .pupdr5 ().floating()
        .pupdr6 ().floating()
        .pupdr7 ().floating()
        .pupdr8 ().floating()
        .pupdr9 ().floating()
        .pupdr10().floating()
        .pupdr11().floating()
        .pupdr15().floating());
    p.GPIOA.ospeedr.modify(|_, w| w
        .ospeedr0 ().low_speed()
        .ospeedr1 ().low_speed()
        .ospeedr2 ().low_speed()
        .ospeedr3 ().low_speed()
        .ospeedr4 ().low_speed()
        .ospeedr5 ().low_speed()
        .ospeedr6 ().low_speed()
        .ospeedr7 ().low_speed()
        .ospeedr8 ().low_speed()
        .ospeedr9 ().low_speed()
        .ospeedr10().low_speed()
        .ospeedr11().low_speed()
        .ospeedr15().low_speed());

    // Use DMA2 stream 1 Channel 7 to transfer LED refresh data to GPIO
    // DMA request trigger is TIM8_UP

    // Initialised DMA2 stream 1 Channel 7
    let dma = &p.DMA2;
    let dmast = &p.DMA2.st[1];
    p.RCC.ahb1enr.modify(|_, w| w.dma2en().enabled());
    dmast.cr.write(|w| w.en().disabled());
	// Clear DMA flags
    dma.lifcr.write(|w| w
        .ctcif1().clear()
        .chtif1().clear()
        .cteif1().clear()
        .cdmeif1().clear()
        .cfeif1().clear());
    // Memory to peripheral, circular, 32bit -> 16bit, very high priority
    dmast.cr.write(|w| w
        .chsel().bits(7)
        .mburst().single()
        .pburst().single()
        .dbm().disabled()
        .pl().very_high()
        .msize().bits32()
        .psize().bits16()
        .minc().incremented()
        .pinc().fixed()
        .circ().enabled()
        .dir().memory_to_peripheral()
        .pfctrl().dma()
        .tcie().disabled()
        .htie().disabled()
        .teie().disabled()
        .dmeie().disabled()
        .en().disabled());
	// FIFO control
    dmast.fcr.write(|w| w
        .dmdis().disabled()
        .fth().full()
        .feie().disabled());
    // Peripheral address
    dmast.par.write(|w| unsafe {w.pa().bits(p.GPIOA.odr.as_ptr() as u32)});
	// Memory address
    dmast.m0ar.write(|w| unsafe {w.m0a().bits(LED_RAM.as_mut_ptr() as u32)});
	// Number of data items
    dmast.ndtr.write(|w| w.ndt().bits(LED_SCAN_SIZE as u16));

    // Initialise timer 8
    let tim = &p.TIM8;
    p.RCC.apb2enr.modify(|_, w| w.tim8en().enabled());
    tim.cr1.write(|w| w.cen().disabled());
	// Auto reload buffered, edge-align mode, continuous
    tim.cr1.write(|w| w
        .ckd().div1()
        .arpe().enabled()
        .cms().edge_aligned()
        .dir().up()
        .opm().disabled()
        .urs().counter_only()
        .udis().enabled()
        .cen().disabled());
    // Prescaler div 1
    tim.psc.write(|w| w.psc().bits(0));
    // Timer overflow value
    tim.arr.write(|w| w.arr().bits((CONFIG.apb2timclk_rate / LED_GPIO_RATE as u32) as u16));
    // No repetition counter
    tim.rcr.reset();
    // No trigger output
    tim.cr2.reset();
    // Not slave mode
    tim.smcr.reset();
    // Break function disable
    tim.bdtr.reset();
    // Disable CC channels
    tim.ccmr1_output().reset();
    tim.ccmr2_output().reset();
    tim.ccer.reset();
    // Enable DMA request from update event, disable interrupts
    tim.dier.write(|w| w.ude().enabled());
    // Reset counter values
    tim.cnt.reset();

    // Initialise DMA RAM
    rgbled_init_ram();

    // Enable DMA and timer
    dmast.cr.modify(|_, w| w.en().enabled());
    tim.cr1.modify(|_, w| w.cen().enabled());
}

const fn scan_value(rgb: [(bool, bool, bool); 3], row: u16) -> u16 {
    let r1 = !rgb[0].0 as u16;
    let g1 = !rgb[0].1 as u16;
    let b1 = !rgb[0].2 as u16;
    let r2 = !rgb[1].0 as u16;
    let g2 = !rgb[1].1 as u16;
    let b2 = !rgb[1].2 as u16;
    let r3 = !rgb[2].0 as u16;
    let g3 = !rgb[2].1 as u16;
    let b3 = !rgb[2].2 as u16;
    // Interleaved scan rows
    let a1 = (!row >> 0) & 1;
    let a2 = (!row >> 2) & 1;
    let a3 = (!row >> 1) & 1;
    let a4 = (!row >> 3) & 1;
    return (r1 << 0) | (g1 << 1) | (b1 << 2)
        | (r2 << 4) | (g2 << 5) | (b2 << 6)
        | (r3 << 9) | (g3 << 10) | (b3 << 8)
        | (a1 << 3) | (a2 << 7) | (a3 << 11) | (a4 << 15);
}

fn rgbled_init_ram() {
    // Clear LED greyscale RAM buffer
    let disabled = scan_value([
        (false, false, false),
        (false, false, false),
        (false, false, false),
        ], 0);
    // Extra greyscale entries for disabling LED outputs before row switch
    unsafe {LED_RAM.write([[disabled; LED_GREYSCALE + LED_SCAN_OFF]; LED_SCAN_ROWS]);}
}

fn rgbled_update_row(mut row: usize) {
    let ram = unsafe {LED_RAM.assume_init_mut()};
    let mut led = unsafe {[
        LED_COLOUR[row + 4 * 0],
        LED_COLOUR[row + 4 * 1],
        LED_COLOUR[row + 4 * 2],
    ]};
    for n in &mut led {
        for c in n {
            *c = GAMMA[*c as usize];
        }
    }
    row = [0, 2, 1, 3][row];
    for gs in 0 ..= 255 as u8 {
        ram[row][gs as usize] = scan_value([
            (led[0][0] > gs, led[0][1] > gs, led[0][2] > gs),
            (led[1][0] > gs, led[1][1] > gs, led[1][2] > gs),
            (led[2][0] > gs, led[2][1] > gs, led[2][2] > gs),
            ], 1 << row);
    }
}

pub fn rgbled_set(n: usize, rgb: [u8; 3]) {
    assert!(n < LED_NUM);
    unsafe {LED_COLOUR[n as usize] = rgb;}
    rgbled_update_row((n % 4) as usize);
}

pub fn rgbled_set_all(rgb: [[u8; 3]; LED_NUM]) {
    unsafe {LED_COLOUR = rgb;}
    for i in 0 .. LED_SCAN_ROWS {
        rgbled_update_row(i);
    }
}

pub fn rgbled_test(n: usize, i: u32) {
    let colour = [
        ((i >>  0) & 0xff) as u8,
        ((i >>  8) & 0xff) as u8,
        ((i >> 16) & 0xff) as u8,
    ];

    let mut rgb;
    if true {
        rgb = [[0u8; 3]; LED_NUM];
        if n == 0 {
            for c in &mut rgb {
                *c = colour;
            }
        } else {
            let nled = (n - 1) % LED_NUM;
            rgb[nled] = colour;
        }

    } else {
        let nled = n % LED_NUM;
        rgb = unsafe {LED_COLOUR};
        rgb[nled] = colour;
    }

    rgbled_set_all(rgb);
}
