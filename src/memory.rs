use crate::{Error, MAX_MEMORY_SIZE};

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

    pub fn store(&mut self, addr: u8, value: usize) -> Result<(), Error> {
        self.slots[addr as usize] = value as u16;
        Ok(())
    }

    pub fn load(&self, addr: u8) -> Result<u16, Error> {
        match self.slots.get(addr as usize) {
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
}