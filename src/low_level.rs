use crate::{Error, HardwareDriver, RomCode};
use std::mem;

//const ROMCODE_SIZE: u8 = 8;
const CMD_SEARCHROM: u8 = 0xF0;
const CMD_READROM: u8 = 0x33;
const CMD_MATCHROM: u8 = 0x55;
const CMD_SKIPROM: u8 = 0xCC;
const SEARCH_FIRST: u8 = 0xFF;
const PRESENCE_ERR: u8 = 0xFF;
const DATA_ERR: u8 = 0xFE;
const LAST_DEVICE: u8 = 0x00;

pub fn search_rom<T: HardwareDriver>(driver: &mut T, diff: u8) -> Result<RomCode, Error> {
    driver.reset()?;

    driver.write_byte(CMD_SEARCHROM)?; // ROM search command
    let mut next_diff: u8 = LAST_DEVICE;

    let mut rom_code = RomCode::default();

    let mut i: u8 = mem::size_of::<RomCode>() as u8 * 8;
    for id in rom_code.0.iter_mut() {
        // process 8 bits
        for j in 0..8 {
            let mut b = driver.read_bit()?;
            match (b, driver.read_bit()?) {
                (true, true) => return Err(Error::DataError),
                (false, false) => {
                    // 2 devices
                    if diff > i || ((*id & 1) != 0 && diff != i) {
                        b = true; // now 1
                        next_diff = i; // next pass 0
                    }
                }
                _ => {}
            };

            driver.write_bit(b)?;               // write bit
            *id >>= 1;
            if b { *id |= 0x80; }			// store bit
            i = i.saturating_sub(1);
        }
    }

    Ok(rom_code)
}
