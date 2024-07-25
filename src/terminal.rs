//! Provides a 'terminal' for controlling various hardware devices
//!
//! Dispatches commands to hardware peripherals and returns their results.

use defmt::*;
use defmt_bbq::DefmtConsumer;
use embassy_time::Timer;

use crate::usb::CdcChannel;

pub struct CommKind(char);

const DEFMT_COMM: CommKind = CommKind('D');
const STRING_COMM: CommKind = CommKind('S');

pub struct MessageHeader {
    kind: CommKind,
    length: usize,
}

pub struct Terminal {
    // TODO...
    #[cfg(feature = "defmt-terminal")]
    pub defmt: DefmtConsumer,

    #[cfg(feature = "usb")]
    pub class: CdcChannel,

    /// Initialize to false...
    pub is_connected: bool,
}

enum ProcessResult {
    Op,
    NoOp,
}

impl Terminal {
    pub fn init(&mut self) {
        self.is_connected = false;
    }

    pub async fn connect(&mut self) {
        self.class.wait_connection().await;
        self.is_connected = true;
        info!("terminal connected!");
    }

    async fn process(&mut self) -> ProcessResult {
        let mut result = ProcessResult::NoOp;

        // TODO ...
        // Get the DEFMT messages
        if let Ok(grant) = self.defmt.read() {
            let len = grant.len();

            let header = MessageHeader {
                kind: DEFMT_COMM,
                length: grant.len(),
            };

            if self.send(header, &grant).await.is_ok() {
                grant.release(len);
            }

            result = ProcessResult::Op;
        } else {
            let msg = b"heartbeat message <3";
            let header = MessageHeader {
                kind: STRING_COMM,
                length: msg.len(),
            };

            self.send(header, msg).await.ok();
        }

        result
    }

    pub async fn send(&mut self, header: MessageHeader, data: &[u8]) -> Result<(), ()> {
        // TODO: Static cell?
        // Build out the fixed-length message header bytes
        let mut header_bytes: [u8; 16] = [0u8; 16];
        header_bytes[0] = header.kind.0 as u8;
        header_bytes[15] = b':';

        // Turn the length into a nice, readable, string
        let mut buffer = itoa::Buffer::new();
        let strver = buffer.format(header.length).as_bytes();
        header_bytes[1..strver.len() + 1].copy_from_slice(strver);

        // Write the header
        if let Err(e) = self.class.write_packet(&header_bytes).await {
            error!("failed to write usb packet header: {}", e);
            return Err(());
        }

        // Write the data
        let max_size: usize = self.class.max_packet_size() as usize;
        // Send the raw data over the wire
        for chunk in data.chunks(max_size) {
            if let Err(e) = self.class.write_packet(chunk).await {
                error!("failed to write usb packet data: {}", e);
                return Err(());
            }
        }

        // Write EOM
        // TODO: It seems bad to do this a new transaction...?
        if let Err(e) = self.class.write_packet(&[b'\n']).await {
            error!("failed to write usb packet EOM: {}", e);
            return Err(());
        }

        Ok(())
    }
}

#[embassy_executor::task]
pub async fn task(mut term: Terminal) {
    term.init();

    loop {
        if !term.is_connected {
            term.connect().await;
        }

        match term.process().await {
            ProcessResult::Op => Timer::after_millis(10),
            ProcessResult::NoOp => Timer::after_millis(1000),
        }
        .await;
    }
}
