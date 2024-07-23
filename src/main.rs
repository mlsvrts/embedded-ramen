#![no_std]
#![no_main]

use embassy_executor::Spawner;
use {defmt_rtt as _, panic_probe as _};

// Platform peripheral setup
mod platform;

// USB Emulated Serial Support (CDC-ACM)
#[cfg(usb_cdc)]
mod usb_cdc;

// Status LED
#[cfg(feature = "blinky")]
mod blinky;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    // Log queue
    // let consumer = defmt_bbq::init().unwrap();
    // let p = embassy_rp::init(Default::default());
    // usb::setup(p.USB, spawner, consumer).await;

    let board = platform::init();

    // Optional setup
    #[cfg(feature = "blinky")]
    blinky::blink(spawner, board, blinky::BlinkConfig::new(500, 3000)).unwrap();
}
