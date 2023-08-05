pub struct Config {
    pub sysclk_rate: u32,
    pub systick_rate: u32,
}

pub const CONFIG: Config = Config {
    sysclk_rate: 216_000_000,
    systick_rate: 1_000,
};
