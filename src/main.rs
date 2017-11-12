//! The bootloader application
//! This application uses launchpad-rs.
//!
//! This is what the flash looks like in the C bootloader. This section starts at 0x400, after the vectors
//! but before the `.text` section.
//!
//! ```
//! __attribute__ ((section(".attributes")))
//! struct {
//!     char    flag_bootloader_exists[14];
//!     char    flag_version_string[8];
//!     uint8_t flags_reserved[490];
//!     char    attribute00[ATTRIBUTES_00_LEN];
//!     uint8_t attribute00_padding[64-ATTRIBUTES_00_LEN];
//!     char    attribute01[ATTRIBUTES_01_LEN];
//!     uint8_t attribute01_padding[64-ATTRIBUTES_01_LEN];
//!     char    attribute02[ATTRIBUTES_02_LEN];
//!     uint8_t attribute02_padding[64-ATTRIBUTES_02_LEN];
//!     uint8_t attributes[832];
//! } attributes = {
//!     {'T', 'O', 'C', 'K', 'B', 'O', 'O', 'T', 'L', 'O', 'A', 'D', 'E', 'R'},
//!     {'0', '.', '6', '.', '0', '\0', '\0', '\0'},
//!     {0x00},
//!     ATTRIBUTES_00_DEF,
//!     {0x00},
//!     ATTRIBUTES_01_DEF,
//!     {0x00},
//!     ATTRIBUTES_02_DEF,
//!     {0x00},
//!     {0x00}
//! };
//! ```

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

#[repr(C)]
pub struct FlashInfo {
    flag_bootloader_exists: [u8; 14],
    flag_version_string: [u8; 8],
    flag_padding: [u8; 490],
    attributes: [Attribute; 16],
}

#[repr(C)]
pub struct Attribute {
    key: [u8; 8],
    length: u8,
    value: [u8; 47],
}

// ****************************************************************************
//
// Public Data
//
// ****************************************************************************

const BLANK_ATTRIBUTE: Attribute = Attribute {
    key: *b"\0\0\0\0\0\0\0\0",
    length: 0,
    value: *b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"
};

#[link_section = ".attributes"]
#[no_mangle]
pub static FLASH_INFO: FlashInfo = FlashInfo {
    flag_bootloader_exists: *b"TOCKBOOTLOADER",
    flag_version_string: *b"0.1.0\0\0\0",
    flag_padding: [0; 490],
    attributes: [
        Attribute {
            key: *b"board\0\0\0",
            length: 4,
            value: *b"stellaris launchpad\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        },
        Attribute {
            key: *b"arch\0\0\0\0",
            length: 9,
            value: *b"cortex-m4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        },
        Attribute {
            key: *b"jldevice",
            length: 11,
            value: *b"LM4F120H5QR\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        },
        BLANK_ATTRIBUTE,
        BLANK_ATTRIBUTE,
        BLANK_ATTRIBUTE,
        BLANK_ATTRIBUTE,
        BLANK_ATTRIBUTE,
        BLANK_ATTRIBUTE,
        BLANK_ATTRIBUTE,
        BLANK_ATTRIBUTE,
        BLANK_ATTRIBUTE,
        BLANK_ATTRIBUTE,
        BLANK_ATTRIBUTE,
        BLANK_ATTRIBUTE,
        BLANK_ATTRIBUTE,
    ]
};

// ****************************************************************************
//
// Public Functions
//
// ****************************************************************************
fn handle_getattr(index: u8) -> Option<tockloader_proto::Response<'static>> {
    let index = index as usize;
    if index < FLASH_INFO.attributes.len() {
        Some(tockloader_proto::Response::GetAttr {
            key: &FLASH_INFO.attributes[index].key,
            value: &FLASH_INFO.attributes[index].value,
        })
    } else {
        Some(tockloader_proto::Response::BadArguments)
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
                // Ok(Some(tockloader_proto::Command::Info)) => panic!(),
                // Ok(Some(tockloader_proto::Command::Id)) => panic!(),
                Ok(Some(tockloader_proto::Command::Reset)) => {
                    need_reset = true;
                    None
                }
                // Ok(Some(tockloader_proto::Command::ErasePage { address })) => panic!(),
                // Ok(Some(tockloader_proto::Command::WritePage { address, data })) => panic!(),
                // Ok(Some(tockloader_proto::Command::EraseExBlock { address })) => panic!(),
                // Ok(Some(tockloader_proto::Command::WriteExPage { address, data })) => panic!(),
                // Ok(Some(tockloader_proto::Command::CrcRxBuffer)) => panic!(),
                // Ok(Some(tockloader_proto::Command::ReadRange {
                //             address,
                //             length,
                //         })) => panic!(),
                // Ok(Some(tockloader_proto::Command::ExReadRange {
                //             address,
                //             length,
                //         })) => panic!(),
                // Ok(Some(tockloader_proto::Command::SetAttr { index, key, value })) => panic!(),
                Ok(Some(tockloader_proto::Command::GetAttr { index })) => handle_getattr(index),
                // Ok(Some(tockloader_proto::Command::CrcIntFlash { address, length })) => panic!(),
                // Ok(Some(tockloader_proto::Command::CrcExtFlash { address, length })) => panic!(),
                // Ok(Some(tockloader_proto::Command::EraseExPage { address })) => panic!(),
                // Ok(Some(tockloader_proto::Command::ExtFlashInit)) => panic!(),
                // Ok(Some(tockloader_proto::Command::ClockOut)) => panic!(),
                // Ok(Some(tockloader_proto::Command::WriteFlashUserPages { page1, page2 })) => {
                //     panic!()
                // }
                // Ok(Some(tockloader_proto::Command::ChangeBaud { mode, baud })) => panic!(),
                Ok(Some(_)) => Some(tockloader_proto::Response::Unknown),
                Err(_) => Some(tockloader_proto::Response::InternalError),
            };
            if need_reset {
                decoder.reset();
            }
            if let Some(response) = response {
                board::led_on(board::Led::Blue);
                let mut encoder = ResponseEncoder::new(&response).unwrap();
                while let Some(byte) = encoder.next() {
                    uart.putc(byte).unwrap();
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

// impl Default for Attribute {
//     fn default() -> Attribute {
//         Attribute {
//             key: *b"\0\0\0\0\0\0\0\0",
//             length: 0,
//             value: *b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"
//         }
//     }
// }

// ****************************************************************************
//
// End Of File
//
// ****************************************************************************
