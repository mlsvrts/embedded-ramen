#![no_std]
#![no_main]

use embassy_executor::Spawner;
use panic_probe as _;

#[cfg(feature = "defmt-rtt")]
use defmt_rtt as _;

#[cfg(feature = "defmt-terminal")]
use defmt_bbq as _;

// Platform peripheral setup
mod platform;

/// Terminal control interface
mod terminal;

// USB Emulated Serial Support (CDC-ACM)
#[cfg(feature = "usb")]
mod usb;

// Status LED
#[cfg(feature = "blinky")]
mod blinky;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    // Log queue
    #[cfg(feature = "defmt-terminal")]
    let defmt_consumer = defmt_bbq::init().unwrap();

    let board = platform::init();

    // Optional (start heartbeat LED)
    #[cfg(feature = "blinky")]
    spawner
        .spawn(blinky::blink(
            board.heartbeat_led,
            blinky::BlinkConfig::new(500, 3000),
        ))
        .expect("failed to spawn LED heartbeat task");

    // Optional (use USB as terminal)
    #[cfg(feature = "usb")]
    let cdc = usb::init(board.usb_terminal, &board.info, spawner)
        .await
        .expect("failed to initialize USB");

    // Setup the terminal
    let term = terminal::Terminal {
        #[cfg(feature = "defmt-terminal")]
        defmt: defmt_consumer,
        #[cfg(feature = "usb")]
        class: cdc,
        is_connected: false,
    };

    spawner
        .spawn(terminal::task(term))
        .expect("failed to initialize terminal task");
}
