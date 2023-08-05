use stm32f7::stm32f7x2::Peripherals;

// Button connections:
//
// PB0 -> Button 1
// PB1 -> Button 2
// PB2 -> Button 3
// PB3 -> Button 4
//
// PB5 -> Touch 1
// PB6 -> Touch 2
// PB7 -> Touch 3
// PB8 -> Touch 4

/// Initialise buttons
pub fn button_init(p: &Peripherals) {
    // Power on GPIOB bank
    p.RCC.ahb1enr.modify(|_, w| w.gpioben().set_bit());
    // Configure GPIOB bank, IO in floating input mode
    p.GPIOB.moder.modify(|_, w| w
        .moder0 ().input()
        .moder1 ().input()
        .moder2 ().input()
        .moder3 ().input()
        .moder5 ().input()
        .moder6 ().input()
        .moder7 ().input()
        .moder8 ().input());
    p.GPIOB.pupdr.modify(|_, w| w
        .pupdr0 ().floating()
        .pupdr1 ().floating()
        .pupdr2 ().floating()
        .pupdr3 ().floating()
        .pupdr5 ().floating()
        .pupdr6 ().floating()
        .pupdr7 ().floating()
        .pupdr8 ().floating());
}

pub fn button_read(p: &Peripherals) -> u32 {
    let v = p.GPIOB.idr.read().bits();
    return (v & 0x0f) | ((v >> 1) & 0xf0);
}
