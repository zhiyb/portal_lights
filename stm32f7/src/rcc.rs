//use cortex_m::asm;
use stm32f7::stm32f7x2::Peripherals;

/// Initialise system clock sources
pub fn rcc_init(p: &mut Peripherals) {
    let rcc = &mut p.RCC;
    let pwr = &mut p.PWR;
    let flash = &mut p.FLASH;

    // Enable HSE
    rcc.cr.modify(|_, w| w.hseon().set_bit());
    while rcc.cr.read().hserdy().is_not_ready() {}
    // Switch to HSE
    rcc.cfgr.modify(|_, w| w.sw().hse());
    while !rcc.cfgr.read().sws().is_hse() {}

    // Disable HSI and PLL
    rcc.cr.modify(|_, w| w
        .hsion().clear_bit()
        .pllon().clear_bit());
    while rcc.cr.read().pllrdy().is_ready() {}

    // Configure PLL (HSE, PLLM = 12, PLLN = 270, PLLP = 2, PLLQ = 8)
    rcc.pllcfgr.write(|w| unsafe {w
        .pllm().bits(12)
        .plln().bits(270)
        .pllp().div2()
        .pllq().bits(8)
        .pllsrc().hse()});

    // Enable power controller
    rcc.apb1enr.modify(|_, w| w.pwren().set_bit());
    // Regulator voltage scale 1
    pwr.cr1.write(|w| w.vos().scale1());
    // Enable PLL
    rcc.cr.modify(|_, w| w.pllon().set_bit());
    // Enable Over-drive mode
    pwr.cr1.modify(|_, w| w.oden().set_bit());
    while pwr.csr1.read().odrdy().bit_is_clear() {}
    pwr.cr1.modify(|_, w| w.odswen().set_bit());
    while pwr.csr1.read().odswrdy().bit_is_clear() {}
    // Set flash latency
    // ART enable, prefetch enable, 7 wait states
    flash.acr.write(|w| w
        .arten().set_bit()
        .prften().set_bit()
        .latency().ws7());
    // Set AHB & APB prescalers
    // AHB = 1, APB1 = 4, APB2 = 2
    rcc.cfgr.write(|w| w
        .hpre().div1()
        .ppre1().div4()
        .ppre2().div2());

    // Wait for PLL lock
    //while (!(RCC->CR & RCC_CR_PLLRDY));
    while rcc.cr.read().pllrdy().is_not_ready() {}
    // Switch to PLL
    rcc.cfgr.write(|w| w.sw().pll());
    while !rcc.cfgr.read().sws().is_pll() {}
    // Select peripheral clocks
    rcc.dckcfgr2.reset();
}
