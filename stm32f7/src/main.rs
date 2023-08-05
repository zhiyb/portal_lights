#![no_std]
#![no_main]

use cortex_m::asm;
use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use cortex_m::peripheral::{syst, Peripherals};

// pick a panicking behavior
// use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take().unwrap();
    let mut systick = peripherals.SYST;
    systick.set_clock_source(syst::SystClkSource::External);
    systick.set_reload(16_000_000 / 8);
    systick.clear_current();
    systick.enable_counter();

    loop {
        while !systick.has_wrapped() {}
        hprintln!("Hello, world!");
        //panic!("Oops");
        //asm::bkpt();
    }
}
