pub struct Config {
    pub sysclk_rate: u32,
    pub systick_rate: u32,
    pub apb2clk_rate: u32,
    pub apb2timclk_rate: u32,
}

const SYSCLK_RATE: u32 = 216_000_000;

pub const CONFIG: Config = Config {
    sysclk_rate: SYSCLK_RATE,
    systick_rate: 1_000,
    apb2clk_rate: SYSCLK_RATE / 2,
    apb2timclk_rate: SYSCLK_RATE,
};
