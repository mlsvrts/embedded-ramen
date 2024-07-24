use crate::platform::gpio::{AnyPin, Output};
use defmt::*;
use embassy_time::Timer;

pub struct BlinkConfig {
    on_time: u64,
    off_time: u64,
}

impl BlinkConfig {
    pub fn new(on_time: u64, off_time: u64) -> Self {
        Self { on_time, off_time }
    }
}

#[embassy_executor::task]
pub async fn blink(mut led: Output<'static, AnyPin>, cfg: BlinkConfig) {
    info!("Starting LED heartbeat...");
    loop {
        led.set_high();
        Timer::after_millis(cfg.on_time).await;

        led.set_low();
        Timer::after_millis(cfg.off_time).await;
    }
}
