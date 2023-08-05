#![no_std]
#![no_main]

pub mod rcc;
pub mod config;

//use cortex_m::asm;
use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use cortex_m::peripheral::syst;

use stm32f7::stm32f7x2::{interrupt, CorePeripherals, Peripherals, Interrupt, NVIC, RCC};

// pick a panicking behavior
// use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use config::CONFIG;

fn init(p: &mut Peripherals, cp: &mut CorePeripherals) {
    rcc::rcc_init(p);

    let systick = &mut cp.SYST;
    systick.set_clock_source(syst::SystClkSource::Core);
    systick.set_reload(CONFIG.sysclk_rate / CONFIG.systick_rate);
    systick.clear_current();
    systick.enable_counter();

    cp.SCB.enable_icache();
    cp.SCB.enable_dcache(&mut cp.CPUID);
}

#[entry]
fn main() -> ! {
    let mut p = Peripherals::take().unwrap();
    let mut cp = CorePeripherals::take().unwrap();

    init(&mut p, &mut cp);

    let mut systick = cp.SYST;

    let mut i: u32 = 0;
    loop {
        while !systick.has_wrapped() {}
        i += 1;
        if i != 3000 {continue;}

        hprintln!("Hello, world!");
        i = 0;
    }

    //panic!("Oops");
    //asm::bkpt();
}

#[interrupt]
fn EXTI0() {
    hprintln!(".");
}
