//! Abstracts platform specific setup

// Re-export RP2040 defintions
#[cfg(feature = "rp2040")]
pub use rp2040::*;

// Raspberry PI 2040
#[cfg(feature = "rp2040")]
pub mod rp2040 {
    // Re-export GPIO
    pub use embassy_rp::gpio;

    // Re-export interrupt binding
    pub use embassy_rp::bind_interrupts;

    // Re-export USB
    #[cfg(feature = "usb")]
    pub use embassy_rp::peripherals::USB as Usb;

    #[cfg(feature = "usb")]
    pub use embassy_rp::usb::{Driver, InterruptHandler};

    use super::{Board, BoardInfo};
    use gpio::{AnyPin, Level, Output};

    pub fn init() -> Board {
        let peripherals = embassy_rp::init(Default::default());

        let info = BoardInfo {
            name: "RP2040",
            serial: "RP2040-MLSVRTS", // TODO: Read from chip ?
            manufacturer: "Broadcom",
        };

        Board {
            info,

            #[cfg(feature = "blinky")]
            heartbeat_led: Output::new(AnyPin::from(peripherals.PIN_13), Level::Low),

            #[cfg(feature = "usb")]
            usb_terminal: peripherals.USB,
        }
    }
}

pub struct BoardInfo {
    /// The friendly device name
    pub name: &'static str,

    /// The serial number of this specific device
    pub serial: &'static str,

    /// The device manufacturer
    pub manufacturer: &'static str,
}

pub struct Board {
    /// Board information
    pub info: BoardInfo,

    /// The board heartbeat LED
    #[cfg(feature = "blinky")]
    pub heartbeat_led: gpio::Output<'static, gpio::AnyPin>,

    /// Main USB interface
    #[cfg(feature = "usb")]
    pub usb_terminal: Usb,
}
