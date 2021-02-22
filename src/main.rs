mod cpu;

extern crate clap;
use clap::{App, Arg};
use crate::cpu::Cpu;
use crate::cpu::MAX_REGISTER_INDEX;

#[derive(Debug)]
pub enum Error {
    InvalidOpCode(u8),
    Op0OutOfRange,
    Op1OutOfRange,
    AdditionOverflow(u16, u16),
}

fn get_init_values() -> [u16; MAX_REGISTER_INDEX as usize +1] {
    let mut registers = [0u16; MAX_REGISTER_INDEX as usize + 1];
    for (index, register) in registers.iter_mut().enumerate() {
        *register = index as u16 * 0x10;
    }
    registers
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
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("intructionsfile")
                .help("do-core1 instructions file to execute")
                .takes_value(true)
        )
        .get_matches();

    let insn_string = arguments
        .value_of("instruction")
        .expect("Missing --instruction argument")
        .trim_start_matches("0x");

    // Convert an hexadecimal formatted string into a u16
    let insn = u16::from_str_radix(insn_string, 16).unwrap();
    let registers = get_init_values();
    let mut cpu = Cpu::new(registers, insn);

    cpu.dump("Initial CPU State");

    cpu.run()?;

    cpu.dump("Final CPU State");

    Ok(())
}
