use std::time::Duration;
use onewire_rs;
use onewire_rs::Error;
use rppal::gpio::Gpio;

pub struct Driver {
    pin: u8,
}

impl Driver {
    pub fn new(pin: u8) -> rppal::gpio::Result<Self> {
        rppal::system::DeviceInfo::new()?.model();  // check ability model
        Gpio::new()?.get(pin)?; // check ability pin

        Ok(Self {
            pin
        })
    }

    fn set_low(&self) -> rppal::gpio::Result<rppal::gpio::OutputPin> {
        Ok(Gpio::new()?.get(self.pin)?.into_output_low())
    }

    fn set_input(&self) -> rppal::gpio::Result<rppal::gpio::InputPin> {
        Ok(Gpio::new()?.get(self.pin)?.into_input())
    }
}

fn precision_delay(duration: std::time::Duration) {
    let t = std::time::Instant::now();
    while t.elapsed() < duration {
        unsafe {
            std::arch::asm!("nop");
        }
    }
}

impl onewire_rs::HardwareDriver for Driver {
    fn init(&mut self) -> Result<(), Error> {
        Ok(())
    }

    fn reset(&mut self) -> Result<bool, Error> {
        let pin = self.set_low().map_err(|e| Error::Io(e.to_string()))?;
        precision_delay(Duration::from_micros(480));
        drop(pin);

        let pin = self.set_input().map_err(|e| Error::Io(e.to_string()))?;
        precision_delay(Duration::from_micros(15));

        let status = pin.is_low();
        precision_delay(Duration::from_micros(420));

        Ok(status)
    }

    fn write_bit(&mut self, val: bool) -> Result<(), Error> {
        todo!()
    }

    fn read_bit(&mut self) -> Result<bool, Error> {
        todo!()
    }

    fn write_byte(&mut self, val: u8) -> Result<(), Error> {
        todo!()
    }

    fn read_byte(&mut self) -> Result<u8, Error> {
        todo!()
    }
}