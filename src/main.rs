mod cpu;
mod memory;

extern crate clap;
use clap::{App, Arg};
use crate::cpu::Cpu;
use crate::memory::Memory;

// do-core1 register indexes range from 0 to 7.
pub const MAX_REGISTER_INDEX: u8 = 7;
// do-core1 memory size is 4096Kb
pub const MAX_MEMORY_SIZE: usize = 4096;

#[derive(Debug)]
pub enum Error {
    InvalidOpCode(u8),
    Op0OutOfRange,
    Op1OutOfRange,
    AdditionOverflow(u16, u16),
    StackOverflow(usize),
    AddressOutOfRange(usize),
    ValueOutOfRange(usize)
}

fn main() -> Result<(), Error> {
    let arguments = App::new("do-core1")
        .about("do-core1 emulator")
        .arg(
            Arg::with_name("instruction")
                .long("instruction")
                .short("i")
                .help("do-core1 instruction to execute")
                .takes_value(true)
        )
        .get_matches();

    let insn_string = arguments
        .value_of("instruction")
        .expect("Missing --instruction argument")
        .trim_start_matches("0x");

    // Convert an hexadecimal formatted string into a u16
    let insn = u16::from_str_radix(insn_string, 16).unwrap();
    let memory = Memory::new();
    let mut cpu = Cpu::new(memory);

    cpu.dump_state("Initial CPU State");
    cpu.memory.dump("Initial Memory State");

    cpu.process(insn)?;

    cpu.dump_state("Final CPU State");
    cpu.memory.dump("Final Memory State");

    Ok(())
}
