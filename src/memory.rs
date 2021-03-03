use crate::{Error, MAX_MEMORY_SIZE, MAX_MEMORY_SLOTS};
use std::convert::TryFrom;

#[derive(Debug)]
pub struct Memory {
    slots: [u16; (MAX_MEMORY_SIZE / 16) as usize]
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            slots: [0u16; MAX_MEMORY_SIZE / 16]
        }
    }

    pub fn store(&mut self, addr: usize, value: usize) -> Result<(), Error> {

        if addr > MAX_MEMORY_SLOTS {
            return Err(Error::AddressOutOfRange(addr));
        }

        self.slots[addr] = match u16::try_from(value) {
            Err(_) => return Err(Error::ValueOutOfRange(value)),
            Ok(u16_value) => u16_value,
        };
        Ok(())
    }

    pub fn load(&self, addr: usize) -> Result<u16, Error> {
        if addr.gt(&MAX_MEMORY_SLOTS)  {
            return Err(Error::AddressOutOfRange(addr));
        }

        match self.slots.get(addr) {
            Some(value) => Ok(value.clone()),
            _ => Err(Error::StackOverflow(addr as usize))
        }
    }

    pub fn dump(&self, preamble: &str) {
        println!("do-core-1: {}", preamble);
        for (addr, value) in self.slots.iter().enumerate() {
            match value {
                value if *value > 0x00 => println!("\t{}\t{:#x?}", addr, value),
                _ => ()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Error;
    use crate::memory::Memory;

    #[test]
    fn test_store_and_load() -> Result<(), Error> {
        let mut memory = Memory::new();

        memory.store(0x00, 0x01)?;
        assert_eq!(memory.load(0x00)?, 0x01);

        Ok(())
    }

    #[test]
    fn test_store_out_of_range() -> Result<(), Error> {
        let mut memory = Memory::new();
        assert!(memory.store(0xfff, 0x00).is_err());
        Ok(())
    }

    #[test]
    fn test_store_u32() -> Result<(), Error> {
        let mut memory = Memory::new();
        assert!(memory.store(0xff, u32::max_value() as usize).is_err());
        Ok(())
    }

    #[test]
    fn test_load_out_of_range() -> Result<(), Error> {
        let memory = Memory::new();
        assert!(memory.load(0xfff).is_err());
        Ok(())
    }
}