//! Abstracts platform specific setup

// Re-export RP2040 defintions
#[cfg(feature = "rp2040")]
pub use rp2040::*;

// Raspberry PI 2040
#[cfg(feature = "rp2040")]
pub mod rp2040 {
    // Re-export GPIO
    pub use embassy_rp::gpio;

    use super::Board;
    use gpio::{AnyPin, Level, Output};

    pub fn init() -> Board {
        let peripherals = embassy_rp::init(Default::default());

        Board {
            #[cfg(feature = "blinky")]
            heartbeat_led: Output::new(AnyPin::from(peripherals.PIN_13), Level::Low),
        }
    }
}

pub struct Board {
    /// The board heartbeat LED
    #[cfg(feature = "blinky")]
    pub heartbeat_led: gpio::Output<'static, gpio::AnyPin>,
}
