//! Abstracts platform specific setup

// Re-export RP2040 defintions
#[cfg(feature = "rp2040")]
pub use rp2040::*;

// Raspberry PI 2040
#[cfg(feature = "rp2040")]
pub mod rp2040 {
    use defmt::*;
    pub use embassy_rp::gpio;
    use embassy_rp::Peripherals;
    use gpio::{AnyPin, Level, Output};

    use super::Platform;

    pub struct Board {
        /// Platform peripherals singleton
        peripherals: Peripherals,
    }

    impl Platform for Board {
        fn init() -> Self {
            Board {
                peripherals: embassy_rp::init(Default::default()),
            }
        }

        #[cfg(feature = "blinky")]
        fn blink_led(self) -> Output<'static, AnyPin> {
            info!("Using PIN:13 as Blink LED");
            Output::new(AnyPin::from(self.peripherals.PIN_13), Level::Low)
        }
    }
}

pub trait Platform {
    /// Setup platform peripherals
    fn init() -> Self;

    /// Get the blink led for this platform
    #[cfg(feature = "blinky")]
    fn blink_led(self) -> gpio::Output<'static, gpio::AnyPin>;
}

pub fn init() -> Board {
    Board::init()
}
