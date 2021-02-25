use std::collections::HashMap;
use crate::Error;

#[derive(Debug)]
pub struct Memory {
    slots: HashMap<u8, u16>
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            slots: HashMap::new()
        }
    }

    pub fn store(&mut self, addr: u8, value: u16) -> Result<(), Error> {
        self.slots.insert(addr, value);
        Ok(())
    }

    pub fn load(&self, addr: u8) -> Result<u16, Error> {
        match self.slots.contains_key(&addr) {
            true => Ok(self.slots[&addr].clone()),
            _ => Err(Error::MemoryEmpty(addr))
        }
    }

    pub fn dump(&self, preamble: &str) {
        println!("do-core-1: {}", preamble);
        for (addr, value) in self.slots.iter() {
            println!("\t{}\t{}", addr, value);
        }
    }
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