use crate::{Error, MAX_MEMORY_SIZE};
use std::convert::TryFrom;

#[derive(Debug)]
pub struct Memory {
    slots: [u8; MAX_MEMORY_SIZE]
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            slots: [0u8; MAX_MEMORY_SIZE]
        }
    }

    pub fn store(&mut self, addr: usize, value: usize) -> Result<(), Error> {

        if addr > MAX_MEMORY_SIZE {
            return Err(Error::AddressOutOfRange(addr));
        }

        self.slots[addr] = match u8::try_from(value) {
            Err(_) => return Err(Error::ValueOutOfRange(value)),
            Ok(u8_value) => u8_value,
        };
        Ok(())
    }

    pub fn load(&self, addr: usize) -> Result<u8, Error> {
        if addr.gt(&MAX_MEMORY_SIZE)  {
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
    use crate::{Error, MAX_MEMORY_SIZE};
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
        assert!(memory.store(MAX_MEMORY_SIZE + 0x1, 0x00).is_err());
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
        assert!(memory.load(MAX_MEMORY_SIZE + 0x1).is_err());
        Ok(())
    }
}