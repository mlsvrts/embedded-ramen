#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::gpio;
use embassy_time::Timer;
use gpio::{Level, Output};
use {defmt_bbq as _, panic_probe as _};

// Add USB serial support !
mod usb;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    // Log queue
    let consumer = defmt_bbq::init().unwrap();

    let p = embassy_rp::init(Default::default());

    usb::setup(p.USB, spawner, consumer).await;

    let mut led = Output::new(p.PIN_13, Level::Low);

    loop {
        info!("led on!");
        led.set_high();
        Timer::after_secs(2).await;

        info!("led off!");
        led.set_low();
        Timer::after_secs(1).await;
    }
}
