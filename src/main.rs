#![no_std]
#![no_main]

use embassy_executor::Spawner;
use {defmt_rtt as _, panic_probe as _};

// Platform peripheral setup
mod platform;

// USB Emulated Serial Support (CDC-ACM)
#[cfg(feature = "usb")]
mod usb;

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
    spawner
        .spawn(blinky::blink(
            board.heartbeat_led,
            blinky::BlinkConfig::new(500, 3000),
        ))
        .expect("failed to spawn LED heartbeat task");

    #[cfg(feature = "usb")]
    usb::init(board.usb_terminal, &board.info, spawner)
        .await
        .expect("failed to initialize USB")
}
