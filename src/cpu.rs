use crate::{Error, MAX_REGISTER_INDEX};
use crate::memory::Memory;

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum OpCode {
    LD = 0x00,
    ST = 0x01,
    ADD = 0x02,
    XOR = 0x03,
}

#[derive(Debug)]
pub struct Instruction {
    opcode: OpCode,
    op0: u8,
    op1: u8,
}

#[derive(Debug)]
pub struct Cpu {
    pub memory: Memory,
    registers: [u16; MAX_REGISTER_INDEX as usize + 1]
}


impl Cpu {
    pub fn new(memory: Memory) -> Cpu {
        Cpu {
            registers: get_register_zeros(),
            memory
        }
    }

    pub fn dump_state(&self, preamble: &str) {
        println!("do-core1: {}", preamble);
        for (index, register) in self.registers.iter().enumerate() {
            println!("\tR{}: {:#x?}", index, *register);
        }
    }

    pub fn process(&mut self, insn: u16) -> Result<(), Error> {
        let decoded_instruction = Instruction::disassemble(insn)?;
        println!("do-core-1: instruction decoded into {:?}", decoded_instruction);
        let op0 = decoded_instruction.op0 as usize;
        let op1 = decoded_instruction.op1 as usize;

        match decoded_instruction.opcode {
            OpCode::ADD => self.registers[op0] = self.add(self.registers[op0], self.registers[op1])?,
            OpCode::XOR => self.registers[op0] = self.xor(self.registers[op0], self.registers[op1]),
            OpCode::LD => self.registers[op0] = self.memory.load(op1)? as u16,
            OpCode::ST => self.memory.store(op0, op1 as usize)?
        }
        Ok(())
    }

    pub fn add(&self, op0: u16, op1: u16) -> Result<u16, Error> {
        op0.checked_add(op1)
            .ok_or(Error::AdditionOverflow(op0, op1))
    }

    pub fn xor(&self, op0: u16, op1: u16) -> u16 {
        op0 ^ op1
    }
}

impl Instruction {
    // Instruction constructor, a.k.a. disassembler.
    fn disassemble(insn: u16) -> Result<Instruction, Error> {
        let opcode = OpCode::from_u8((insn >> 8) as u8)?;
        let op0 = ((insn & 0xf0) >> 4) as u8;
        let op1: u8 = (insn & 0xf) as u8;

        if op0 > MAX_REGISTER_INDEX {
            return Err(Error::Op0OutOfRange);
        }

        if op1 > MAX_REGISTER_INDEX {
            return Err(Error::Op1OutOfRange);
        }

        Ok(Instruction { opcode, op0, op1 })
    }
}

impl OpCode {
    fn from_u8(opcode: u8) -> Result<OpCode, Error> {
        match opcode {
            0x00 => Ok(OpCode::LD),
            0x01 => Ok(OpCode::ST),
            0x02 => Ok(OpCode::ADD),
            0x03 => Ok(OpCode::XOR),
            _ => Err(Error::InvalidOpCode(opcode)),
        }
    }
}

fn get_register_zeros() -> [u16; MAX_REGISTER_INDEX as usize +1] {
    let mut registers = [0u16; MAX_REGISTER_INDEX as usize + 1];
    for (_, register) in registers.iter_mut().enumerate() {
        *register = 0x00;
    }
    registers
}

#[cfg(test)]
mod tests {
    use crate::{Error};
    use crate::cpu::{Instruction, OpCode};

    #[test]
    fn test_instruction_disassemble_add_r0_r1() -> Result<(), Error> {
        let insn_bytes: u16 = 0x201;
        let insn = Instruction::disassemble(insn_bytes)?;

        assert_eq!(insn.opcode, OpCode::ADD);
        assert_eq!(insn.op0, 0);
        assert_eq!(insn.op1, 1);

        Ok(())
    }

    #[test]
    fn test_instruction_disassemble_add_r9_r1() -> Result<(), Error> {
        let insn_bytes: u16 = 0x291;
        assert!(Instruction::disassemble(insn_bytes).is_err());

        Ok(())
    }

    #[test]
    fn test_instruction_disassemble_add_r0_r10() -> Result<(), Error> {
        let insn_bytes: u16 = 0x20a;
        assert!(Instruction::disassemble(insn_bytes).is_err());

        Ok(())
    }

    #[test]
    fn test_instruction_disassemble_add_r7_r2() -> Result<(), Error> {
        let insn_bytes: u16 = 0x272;
        let insn = Instruction::disassemble(insn_bytes)?;

        assert_eq!(insn.opcode, OpCode::ADD);
        assert_eq!(insn.op0, 7);
        assert_eq!(insn.op1, 2);

        Ok(())
    }

    #[test]
    fn test_instruction_disassemble_ld_r0_r1() -> Result<(), Error> {
        let insn_bytes: u16 = 0x01;
        let insn = Instruction::disassemble(insn_bytes)?;

        assert_eq!(insn.opcode, OpCode::LD);
        assert_eq!(insn.op0, 0);
        assert_eq!(insn.op1, 1);

        Ok(())
    }

    #[test]
    fn test_instruction_disassemble_xor_r2_r3() -> Result<(), Error> {
        let insn_bytes: u16 = 0x323;
        let insn = Instruction::disassemble(insn_bytes)?;

        assert_eq!(insn.opcode, OpCode::XOR);
        assert_eq!(insn.op0, 2);
        assert_eq!(insn.op1, 3);

        Ok(())
    }

    #[test]
    fn test_instruction_disassemble_st_r5_r0() -> Result<(), Error> {
        let insn_bytes: u16 = 0x150;
        let insn = Instruction::disassemble(insn_bytes)?;

        assert_eq!(insn.opcode, OpCode::ST);
        assert_eq!(insn.op0, 5);
        assert_eq!(insn.op1, 0);

        Ok(())
    }
}