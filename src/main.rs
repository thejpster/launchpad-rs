//! The bootloader application
//! This application uses launchpad-rs.

#![no_std]
#![no_main]
#![feature(asm)]
#![crate_type="staticlib"]

// ****************************************************************************
//
// Imports
//
// ****************************************************************************

extern crate stellaris_launchpad_bootloader;
extern crate embedded_serial;
extern crate tockloader_proto;

use stellaris_launchpad_bootloader::board;
use stellaris_launchpad_bootloader::delay;
use stellaris_launchpad_bootloader::cpu::uart;

use embedded_serial::{MutBlockingTx, MutNonBlockingRx};
use tockloader_proto::{ResponseEncoder, CommandDecoder};

// ****************************************************************************
//
// Public Types
//
// ****************************************************************************


// None

// ****************************************************************************
//
// Private Types
//
// ****************************************************************************

// None

// ****************************************************************************
//
// Public Data
//
// ****************************************************************************

// None

// ****************************************************************************
//
// Public Functions
//
// ****************************************************************************
fn handle_getattr(index: u8) -> Option<tockloader_proto::Response<'static>> {
    match index {
        0 => Some(tockloader_proto::Response::GetAttr {
            key: "board\x00\x00\x00".as_bytes(),
            value: "Stellaris Launchpad".as_bytes(),
        }),
        1 => Some(tockloader_proto::Response::GetAttr {
            key: "arch\x00\x00\x00\x00".as_bytes(),
            value: "cortex-m4".as_bytes(),
        }),
        _ => Some(tockloader_proto::Response::GetAttr {
            key: "\x00\x00\x00\x00\x00\x00\x00\x00".as_bytes(),
            value: "".as_bytes(),
        }),
    }
}

#[no_mangle]
pub extern "C" fn main() {
    let mut uart = uart::Uart::new(uart::UartId::Uart0, 115200, uart::NewlineMode::SwapLFtoCRLF);
    let mut decoder = CommandDecoder::new();
    board::led_off(board::Led::Green);
    delay(100);
    board::led_on(board::Led::Green);
    delay(100);
    board::led_off(board::Led::Green);
    delay(1000);
    board::led_on(board::Led::Green);
    loop {
        if let Ok(Some(ch)) = uart.getc_try() {
            board::led_off(board::Led::Green);
            let mut need_reset = false;
            let response = match decoder.receive(ch) {
                Ok(None) => None,
                Ok(Some(tockloader_proto::Command::Ping)) => Some(tockloader_proto::Response::Pong),
                Ok(Some(tockloader_proto::Command::Info)) => panic!(),
                Ok(Some(tockloader_proto::Command::Id)) => panic!(),
                Ok(Some(tockloader_proto::Command::Reset)) => {
                    need_reset = true;
                    None
                },
                Ok(Some(tockloader_proto::Command::ErasePage { address })) => panic!(),
                Ok(Some(tockloader_proto::Command::WritePage { address, data })) => panic!(),
                Ok(Some(tockloader_proto::Command::EraseExBlock { address })) => panic!(),
                Ok(Some(tockloader_proto::Command::WriteExPage { address, data })) => panic!(),
                Ok(Some(tockloader_proto::Command::CrcRxBuffer)) => panic!(),
                Ok(Some(tockloader_proto::Command::ReadRange {
                            address,
                            length: u16,
                        })) => panic!(),
                Ok(Some(tockloader_proto::Command::ExReadRange {
                            address,
                            length: u16,
                        })) => panic!(),
                Ok(Some(tockloader_proto::Command::SetAttr { index, key, value })) => panic!(),
                Ok(Some(tockloader_proto::Command::GetAttr { index })) => handle_getattr(index),
                Ok(Some(tockloader_proto::Command::CrcIntFlash { address, length })) => panic!(),
                Ok(Some(tockloader_proto::Command::CrcExtFlash { address, length })) => panic!(),
                Ok(Some(tockloader_proto::Command::EraseExPage { address })) => panic!(),
                Ok(Some(tockloader_proto::Command::ExtFlashInit)) => panic!(),
                Ok(Some(tockloader_proto::Command::ClockOut)) => panic!(),
                Ok(Some(tockloader_proto::Command::WriteFlashUserPages { page1, page2 })) => {
                    panic!()
                }
                Ok(Some(tockloader_proto::Command::ChangeBaud { mode, baud })) => panic!(),
                Err(_) => Some(tockloader_proto::Response::InternalError),
            };
            if need_reset {
                decoder.reset();
            }
            if let Some(response) = response {
                board::led_on(board::Led::Blue);
                let mut encoder = ResponseEncoder::new(&response).unwrap();
                while let Some(byte) = encoder.next() {
                    uart.putc(byte);
                }
                board::led_off(board::Led::Blue);
            }
            board::led_on(board::Led::Green);
        }
    }
}

// ****************************************************************************
//
// Private Functions
//
// ****************************************************************************

// None

// ****************************************************************************
//
// End Of File
//
// ****************************************************************************
