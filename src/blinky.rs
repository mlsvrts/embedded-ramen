use crate::platform::gpio::{AnyPin, Output};
use crate::platform::{Board, Platform};
use core::result::Result;
use defmt::*;
use embassy_executor::{SpawnError, Spawner};
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

pub fn blink(spawner: Spawner, board: Board, cfg: BlinkConfig) -> Result<(), SpawnError> {
    let led = board.blink_led();

    spawner.spawn(blink_task(led, cfg))
}

#[embassy_executor::task]
async fn blink_task(mut led: Output<'static, AnyPin>, cfg: BlinkConfig) {
    info!("Starting LED heartbeat...");
    loop {
        led.set_high();
        Timer::after_millis(cfg.on_time).await;

        led.set_low();
        Timer::after_millis(cfg.off_time).await;
    }
}
