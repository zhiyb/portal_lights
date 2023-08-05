//use paste::paste;

use core::default;

//use cortex_m::asm;
use stm32f7::stm32f7x2::Peripherals;

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
// PA8  -> B3
// PA9  -> R3
// PA10 -> G3
//
// PA3  -> A1
// PA7  -> A1
// PA11 -> A1
// PA15 -> A1

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
}

pub fn rgbled_test(p: &Peripherals, i: u32) {
    let s = (i / 100) % 16;
    match s {
        0 =>
            p.GPIOA.odr.modify(|_, w| w
                .odr0 ().low()
                .odr1 ().low()
                .odr2 ().low()
                .odr4 ().low()
                .odr5 ().low()
                .odr6 ().low()
                .odr8 ().low()
                .odr9 ().low()
                .odr10().low()

                .odr3 ().low()
                .odr7 ().high()
                .odr11().high()
                .odr15().high()),
        1 =>
            p.GPIOA.odr.modify(|_, w| w
                .odr0 ().low()
                .odr1 ().low()
                .odr2 ().low()
                .odr4 ().low()
                .odr5 ().low()
                .odr6 ().low()
                .odr8 ().low()
                .odr9 ().low()
                .odr10().low()

                .odr3 ().high()
                .odr7 ().low()
                .odr11().high()
                .odr15().high()),
        2 =>
            p.GPIOA.odr.modify(|_, w| w
                .odr0 ().low()
                .odr1 ().low()
                .odr2 ().low()
                .odr4 ().low()
                .odr5 ().low()
                .odr6 ().low()
                .odr8 ().low()
                .odr9 ().low()
                .odr10().low()

                .odr3 ().high()
                .odr7 ().high()
                .odr11().low()
                .odr15().high()),
        3 =>
            p.GPIOA.odr.modify(|_, w| w
                .odr0 ().low()
                .odr1 ().low()
                .odr2 ().low()
                .odr4 ().low()
                .odr5 ().low()
                .odr6 ().low()
                .odr8 ().low()
                .odr9 ().low()
                .odr10().low()

                .odr3 ().high()
                .odr7 ().high()
                .odr11().high()
                .odr15().low()),
        4 =>
            p.GPIOA.odr.modify(|_, w| w
                .odr0 ().low()
                .odr1 ().high()
                .odr2 ().high()
                .odr4 ().low()
                .odr5 ().high()
                .odr6 ().high()
                .odr8 ().high()
                .odr9 ().low()
                .odr10().high()

                .odr3 ().low()
                .odr7 ().high()
                .odr11().high()
                .odr15().high()),
        5 =>
            p.GPIOA.odr.modify(|_, w| w
                .odr0 ().low()
                .odr1 ().high()
                .odr2 ().high()
                .odr4 ().low()
                .odr5 ().high()
                .odr6 ().high()
                .odr8 ().high()
                .odr9 ().low()
                .odr10().high()

                .odr3 ().high()
                .odr7 ().low()
                .odr11().high()
                .odr15().high()),
        6 =>
            p.GPIOA.odr.modify(|_, w| w
                .odr0 ().low()
                .odr1 ().high()
                .odr2 ().high()
                .odr4 ().low()
                .odr5 ().high()
                .odr6 ().high()
                .odr8 ().high()
                .odr9 ().low()
                .odr10().high()

                .odr3 ().high()
                .odr7 ().high()
                .odr11().low()
                .odr15().high()),
        7 =>
            p.GPIOA.odr.modify(|_, w| w
                .odr0 ().low()
                .odr1 ().high()
                .odr2 ().high()
                .odr4 ().low()
                .odr5 ().high()
                .odr6 ().high()
                .odr8 ().high()
                .odr9 ().low()
                .odr10().high()

                .odr3 ().high()
                .odr7 ().high()
                .odr11().high()
                .odr15().low()),
        8 =>
            p.GPIOA.odr.modify(|_, w| w
                .odr0 ().high()
                .odr1 ().low()
                .odr2 ().high()
                .odr4 ().high()
                .odr5 ().low()
                .odr6 ().high()
                .odr8 ().high()
                .odr9 ().high()
                .odr10().low()

                .odr3 ().low()
                .odr7 ().high()
                .odr11().high()
                .odr15().high()),
        9 =>
            p.GPIOA.odr.modify(|_, w| w
                .odr0 ().high()
                .odr1 ().low()
                .odr2 ().high()
                .odr4 ().high()
                .odr5 ().low()
                .odr6 ().high()
                .odr8 ().high()
                .odr9 ().high()
                .odr10().low()

                .odr3 ().high()
                .odr7 ().low()
                .odr11().high()
                .odr15().high()),
        10 =>
            p.GPIOA.odr.modify(|_, w| w
                .odr0 ().high()
                .odr1 ().low()
                .odr2 ().high()
                .odr4 ().high()
                .odr5 ().low()
                .odr6 ().high()
                .odr8 ().high()
                .odr9 ().high()
                .odr10().low()

                .odr3 ().high()
                .odr7 ().high()
                .odr11().low()
                .odr15().high()),
        11 =>
            p.GPIOA.odr.modify(|_, w| w
                .odr0 ().high()
                .odr1 ().low()
                .odr2 ().high()
                .odr4 ().high()
                .odr5 ().low()
                .odr6 ().high()
                .odr8 ().high()
                .odr9 ().high()
                .odr10().low()

                .odr3 ().high()
                .odr7 ().high()
                .odr11().high()
                .odr15().low()),
        12 =>
            p.GPIOA.odr.modify(|_, w| w
                .odr0 ().high()
                .odr1 ().high()
                .odr2 ().low()
                .odr4 ().high()
                .odr5 ().high()
                .odr6 ().low()
                .odr8 ().low()
                .odr9 ().high()
                .odr10().high()

                .odr3 ().low()
                .odr7 ().high()
                .odr11().high()
                .odr15().high()),
        13 =>
            p.GPIOA.odr.modify(|_, w| w
                .odr0 ().high()
                .odr1 ().high()
                .odr2 ().low()
                .odr4 ().high()
                .odr5 ().high()
                .odr6 ().low()
                .odr8 ().low()
                .odr9 ().high()
                .odr10().high()

                .odr3 ().high()
                .odr7 ().low()
                .odr11().high()
                .odr15().high()),
        14 =>
            p.GPIOA.odr.modify(|_, w| w
                .odr0 ().high()
                .odr1 ().high()
                .odr2 ().low()
                .odr4 ().high()
                .odr5 ().high()
                .odr6 ().low()
                .odr8 ().low()
                .odr9 ().high()
                .odr10().high()

                .odr3 ().high()
                .odr7 ().high()
                .odr11().low()
                .odr15().high()),
        15 =>
            p.GPIOA.odr.modify(|_, w| w
                .odr0 ().high()
                .odr1 ().high()
                .odr2 ().low()
                .odr4 ().high()
                .odr5 ().high()
                .odr6 ().low()
                .odr8 ().low()
                .odr9 ().high()
                .odr10().high()

                .odr3 ().high()
                .odr7 ().high()
                .odr11().high()
                .odr15().low()),
        _ =>
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
                .odr15().high()),
    }
}
