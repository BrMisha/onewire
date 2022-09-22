mod low_level;

#[derive(Debug)]
pub enum Error {
    NoDeviceFound,
    DataError,
    Io(String),
    Other(String),
}

pub trait HardwareDriver {
    fn init(&mut self) -> Result<(), Error>;
    fn reset(&mut self) -> Result<bool, Error>;

    fn write_bit(&mut self, val: bool) -> Result<(), Error>;
    fn read_bit(&mut self) -> Result<bool, Error>;

    fn write_byte(&mut self, val: u8) -> Result<(), Error>;
    fn read_byte(&mut self) -> Result<u8, Error>;
}

#[derive(Default)]
pub struct RomCode([u8; 8]);

pub fn search_rom<T: HardwareDriver>(driver: &mut T, max_devices: u16) {
    let search = |diff: u8| {

    };
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
