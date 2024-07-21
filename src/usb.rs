use defmt::{info, panic, unwrap};
use defmt_bbq::DefmtConsumer;
use embassy_executor::Spawner;
use embassy_rp::bind_interrupts;
use embassy_rp::peripherals::USB;
use embassy_rp::usb::{Driver, InterruptHandler};
use embassy_time::Timer;
use embassy_usb::class::cdc_acm::{CdcAcmClass, State};
use embassy_usb::driver::EndpointError;
use embassy_usb::UsbDevice;
use static_cell::StaticCell;

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => InterruptHandler<USB>;
});

pub async fn setup(usb: USB, spawner: Spawner, consumer: DefmtConsumer) {
    // Create the driver, from the HAL.
    let driver = Driver::new(usb, Irqs);

    // Create embassy-usb Config
    let config = {
        let mut config = embassy_usb::Config::new(0xc0de, 0xcafe);
        config.manufacturer = Some("MLSVRTS");
        config.product = Some("RP2040");
        config.serial_number = Some("VRTS-2040-1");
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
    unwrap!(spawner.spawn(usb_task(usb)));

    unwrap!(spawner.spawn(echo_task(class, consumer)));
}

#[embassy_executor::task]
async fn echo_task(
    mut class: CdcAcmClass<'static, Driver<'static, USB>>,
    mut consumer: DefmtConsumer,
) {
    // Do stuff with the class!
    loop {
        class.wait_connection().await;
        info!("Connected");

        // let mut data = [0u8; 64];
        // let map = [
        //     b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'A', b'B', b'C', b'D', b'E',
        //     b'F',
        // ];
        while let Ok(grant) = consumer.read() {
            // do something with `bytes`, like send
            // it over a serial port..
            Timer::after_secs(1).await;

            let res = class.write_packet(b"alive\n").await;
            // // let res = class.write_packet(&grant).await;
            // let rlen = core::cmp::min(16, grant.len());

            // if rlen == 0 {
            //     continue;
            // }

            // for i in 0..rlen {
            //     let upper = (grant[i] & 0xF0) >> 4;
            //     let lower = grant[i] & 0x0F;

            //     data[i * 2] = map[upper as usize];
            //     data[(i * 2) + 1] = map[lower as usize];
            // }

            // // let res = class.write_packet(&grant[..rlen]).await;
            // let res = class.write_packet(&data[..(rlen * 2)]).await;

            // // Then when done, make sure you release the grant
            // // to free the space for future logging.
            // grant.release(rlen);

            if res.is_err() {
                break;
            }
        }
        info!("Disconnected");
    }
}

type MyUsbDriver = Driver<'static, USB>;
type MyUsbDevice = UsbDevice<'static, MyUsbDriver>;

#[embassy_executor::task]
async fn usb_task(mut usb: MyUsbDevice) -> ! {
    usb.run().await
}

struct Disconnected {}

impl From<EndpointError> for Disconnected {
    fn from(val: EndpointError) -> Self {
        match val {
            EndpointError::BufferOverflow => panic!("Buffer overflow"),
            EndpointError::Disabled => Disconnected {},
        }
    }
}
