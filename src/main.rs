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
#![feature(core_intrinsics)]
#![crate_type="staticlib"]

// ****************************************************************************
//
// Imports
//
// ****************************************************************************

extern crate stellaris_launchpad_bootloader;
extern crate embedded_serial;
extern crate crc;
extern crate tockloader_proto as proto;

use stellaris_launchpad_bootloader::board;
use stellaris_launchpad_bootloader::delay;
use stellaris_launchpad_bootloader::cpu::uart;
use stellaris_launchpad_bootloader::cpu::flash;

use embedded_serial::{MutBlockingTx, MutNonBlockingRx};

use proto::{ResponseEncoder, CommandDecoder};

// ****************************************************************************
//
// Public Types
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
// Private Types
//
// ****************************************************************************

// None

// ****************************************************************************
//
// Public Data
//
// ****************************************************************************

#[link_section = ".attributes"]
#[no_mangle]
pub static FLASH_INFO: FlashInfo = FlashInfo {
    flag_bootloader_exists: *b"TOCKBOOTLOADER",
    // Must match VERSION_STRING below
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
// Private Data
//
// ****************************************************************************

// Must match flag_version_string above
const VERSION_STRING: &[u8] = b"{ \"version\":\"0.1.0\", \"name\":\"Tock Bootloader\" }";

const BLANK_ATTRIBUTE: Attribute = Attribute {
    key: *b"\0\0\0\0\0\0\0\0",
    length: 0,
    value: *b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"
};

// ****************************************************************************
//
// Public Functions
//
// ****************************************************************************

#[no_mangle]
pub extern "C" fn main() {
    let mut uart = uart::Uart::new(uart::UartId::Uart0, 115200, uart::NewlineMode::Binary);
    let mut decoder = CommandDecoder::new();

    board::led_on(board::Led::Red);
    delay(500);
    board::led_off(board::Led::Red);
    delay(500);
    board::led_on(board::Led::Green);
    delay(500);

    loop {
        if let Ok(Some(ch)) = uart.getc_try() {
            board::led_off(board::Led::Green);
            let mut need_reset = false;
            let response = match decoder.receive(ch) {
                Ok(None) => None,
                Ok(Some(proto::Command::Ping)) => Some(proto::Response::Pong),
                Ok(Some(proto::Command::Info)) => Some(proto::Response::Info { info: VERSION_STRING }),
                // Ok(Some(proto::Command::Id)) => panic!(),
                Ok(Some(proto::Command::Reset)) => {
                    need_reset = true;
                    None
                }
                Ok(Some(proto::Command::ErasePage { address })) => handle_erase_page(address),
                Ok(Some(proto::Command::WritePage { address, data })) => handle_write_page(address, data),
                // Ok(Some(proto::Command::EraseExBlock { address })) => panic!(),
                // Ok(Some(proto::Command::WriteExPage { address, data })) => panic!(),
                // Ok(Some(proto::Command::CrcRxBuffer)) => panic!(),
                Ok(Some(proto::Command::ReadRange {
                            address,
                            length,
                        })) => handle_rrange(address, length),
                // Ok(Some(proto::Command::ExReadRange {
                //             address,
                //             length,
                //         })) => panic!(),
                // Ok(Some(proto::Command::SetAttr { index, key, value })) => panic!(),
                Ok(Some(proto::Command::GetAttr { index })) => handle_getattr(index),
                Ok(Some(proto::Command::CrcIntFlash { address, length })) => handle_crc(address, length),
                // Ok(Some(proto::Command::CrcExtFlash { address, length })) => panic!(),
                // Ok(Some(proto::Command::EraseExPage { address })) => panic!(),
                // Ok(Some(proto::Command::ExtFlashInit)) => panic!(),
                // Ok(Some(proto::Command::ClockOut)) => panic!(),
                // Ok(Some(proto::Command::WriteFlashUserPages { page1, page2 })) => {
                //     panic!()
                // }
                // Ok(Some(proto::Command::ChangeBaud { mode, baud })) => panic!(),
                Ok(Some(_)) => Some(proto::Response::Unknown),
                Err(_) => Some(proto::Response::InternalError),
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

fn handle_crc(address: u32, length: u32) -> Option<proto::Response<'static>> {
    let data = unsafe { core::slice::from_raw_parts(address as *const u8, length as usize) };
    // tockloader Python says:
    // crc_function = crcmod.mkCrcFun(0x104c11db7, initCrc=0, xorOut=0xFFFFFFFF)
    let result = crc::crc32::checksum_ieee(data);
    Some(proto::Response::CrcIntFlash {
        crc: result
    })
}

fn handle_rrange(address: u32, length: u16) -> Option<proto::Response<'static>> {
    let data = unsafe { core::slice::from_raw_parts(address as *const u8, length as usize) };
    Some(proto::Response::ReadRange {
        data
    })
}

fn handle_getattr(index: u8) -> Option<proto::Response<'static>> {
    let index = index as usize;
    if index < FLASH_INFO.attributes.len() {
        Some(proto::Response::GetAttr {
            key: &FLASH_INFO.attributes[index].key,
            value: &FLASH_INFO.attributes[index].value,
        })
    } else {
        Some(proto::Response::BadArguments)
    }
}

fn handle_erase_page(address: u32) -> Option<proto::Response<'static>> {
    match flash::erase_page(flash::FlashAddress(address)) {
        Err(_) => Some(proto::Response::BadArguments),
        Ok(_) => Some(proto::Response::Ok),
    }
}

/// Convert four little-endian bytes to a U32
fn pack_le(data: &[u8]) -> u32 {
    ((data[3] as u32) << 24) |
    ((data[2] as u32) << 16) |
    ((data[1] as u32) << 8) |
    (data[0] as u32) << 0
}

fn handle_write_page(mut address: u32, data: &[u8]) -> Option<proto::Response<'static>> {
    // Ensure we've got a multiple of four bytes
    if (data.len() & 3) != 0 {
        return Some(proto::Response::BadArguments)
    }

    let mut offset = 0;
    let mut remaining_words = data.len() / 4;

    while remaining_words > 0 {
        let this_time = if remaining_words > flash::PAGE_LENGTH_WORDS { flash::PAGE_LENGTH_WORDS } else { remaining_words };
        let source = &data[offset..offset+(this_time*4)];
        // Pass `write_page` an iterator, so we don't need to duplicate the data.
        match flash::write_page(flash::FlashAddress(address), source.chunks(4).map(pack_le)) {
            Err(_) => return Some(proto::Response::BadArguments),
            Ok(_) => {},
        };
        remaining_words = remaining_words - this_time;
        address += this_time as u32 * 4;
        offset += this_time * 4;
    }
    Some(proto::Response::Ok)
}

// ****************************************************************************
//
// End Of File
//
// ****************************************************************************
