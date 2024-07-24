// Required for USB setup
use embassy_usb::class::cdc_acm::{CdcAcmClass, State};
use embassy_usb::UsbDevice;
use static_cell::StaticCell;

// Spawning
use embassy_executor::{SpawnError, Spawner};

// Logging
use defmt::info;

// Extra
use embassy_time::Timer;

use crate::platform::BoardInfo;
use crate::platform::{bind_interrupts, Driver, InterruptHandler, Usb};

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => InterruptHandler<Usb>;
});

pub async fn init(usb: Usb, info: &BoardInfo, spawner: Spawner) -> Result<(), SpawnError> {
    // Create the driver, from the HAL.
    let driver = Driver::new(usb, Irqs);

    // Create embassy-usb Config
    let config = {
        let mut config = embassy_usb::Config::new(0xc0de, 0xcafe);
        config.manufacturer = Some(info.manufacturer);
        config.product = Some(info.name);
        config.serial_number = Some(info.serial);
        config.max_power = 100;
        config.max_packet_size_0 = 64;

        // Required for windows compatibility.
        // https://developer.nordicsemi.com/nRF_Connect_SDK/doc/1.9.1/kconfig/CONFIG_CDC_ACM_IAD.html#help
        config.device_class = 0xEF;
        config.device_sub_class = 0x02;
        config.device_protocol = 0x01;
        config.composite_with_iads = true;
        config
    };

    // Create embassy-usb DeviceBuilder using the driver and config.
    // It needs some buffers for building the descriptors.
    let mut builder = {
        static CONFIG_DESCRIPTOR: StaticCell<[u8; 256]> = StaticCell::new();
        static BOS_DESCRIPTOR: StaticCell<[u8; 256]> = StaticCell::new();
        static CONTROL_BUF: StaticCell<[u8; 64]> = StaticCell::new();

        let builder = embassy_usb::Builder::new(
            driver,
            config,
            CONFIG_DESCRIPTOR.init([0; 256]),
            BOS_DESCRIPTOR.init([0; 256]),
            &mut [], // no msos descriptors
            CONTROL_BUF.init([0; 64]),
        );
        builder
    };

    // Create classes on the builder.
    let class = {
        static STATE: StaticCell<State> = StaticCell::new();
        let state = STATE.init(State::new());
        CdcAcmClass::new(&mut builder, state, 64)
    };

    // Build the builder.
    let usb = builder.build();

    // Run the USB device.
    spawner.spawn(usb_task(usb))?;

    spawner.spawn(logger(class))
}

#[embassy_executor::task]
async fn logger(mut class: CdcAcmClass<'static, Driver<'static, Usb>>) {
    // Do stuff with the class!
    class.wait_connection().await;
    info!("Connected");
    loop {
        Timer::after_secs(3).await;
        let res = class.write_packet(b"Hello, logger!\n").await;

        if res.is_err() {
            break;
        }
    }
    info!("Disconnected");
}

#[embassy_executor::task]
async fn usb_task(mut usb: UsbDevice<'static, Driver<'static, Usb>>) -> ! {
    usb.run().await
}
