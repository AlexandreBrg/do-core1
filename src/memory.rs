use crate::{Error, MAX_MEMORY_SIZE};

#[derive(Debug)]
pub struct Memory {
    slots: [u16; (MAX_MEMORY_SIZE / 16) as usize]
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            slots: get_memory()
        }
    }

    pub fn store(&mut self, addr: u8, value: usize) -> Result<(), Error> {
        self.slots[addr as usize] = value as u16;
        Ok(())
    }

    pub fn load(&self, addr: u8) -> Result<u16, Error> {
        match self.slots.get(addr as usize) {
            Some(value) => Ok(value.clone()),
            _ => Err(Error::MemoryEmpty(addr))
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

fn get_memory() -> [u16; MAX_MEMORY_SIZE / 16] {
    let mut slots = [0u16; MAX_MEMORY_SIZE / 16];
    for (_, slot) in slots.iter_mut().enumerate() {
        *slot = 0x00;
    }
    slots
}

#[cfg(test)]
mod tests {
    use crate::Error;
    use crate::memory::Memory;

    #[test]
    fn test_load_empty_memory() -> Result<(), Error> {
        let memory = Memory::new();

        assert!(memory.load(0x01).is_err());
        Ok(())
    }

    #[test]
    fn test_store_and_load() -> Result<(), Error> {
        let mut memory = Memory::new();

        memory.store(0x00, 0x01)?;
        assert_eq!(memory.load(0x00)?, 0x01);

        Ok(())
    }
}