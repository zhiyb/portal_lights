#![no_std]
#![no_main]

use cortex_m::asm;
use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use cortex_m::peripheral::syst;

use stm32f7::stm32f7x2::{interrupt, CorePeripherals, Peripherals, Interrupt, NVIC, RCC};

// pick a panicking behavior
// use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

pub mod rcc;

const SYSTICK_RATE: u32 = 1000;
const SYSCLK_RATE: u32 = 216_000_000;

#[entry]
fn main() -> ! {
    let p = Peripherals::take().unwrap();
    rcc::rcc_init(&p);

    let cp = CorePeripherals::take().unwrap();
    let mut systick = cp.SYST;
    systick.set_clock_source(syst::SystClkSource::Core);
    systick.set_reload(SYSCLK_RATE / SYSTICK_RATE);
    systick.clear_current();
    systick.enable_counter();

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
