extern crate rusty_6502_assembler;

use assembler::data_types::Bytes;
use rusty_6502_assembler::lib::{assembler, parser::line_regex};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};

fn main() {
    mod assembler_state {
        pub const ROM_SIZE: usize = 0x10000;
        static mut ROM: [u8; ROM_SIZE] = [0; ROM_SIZE];
        static mut PC: usize = 0x0000;
        pub fn push_byte(b: u8) -> Result<usize, &'static str> {
            unsafe { ROM[PC] = b };
            match increment_pc() {
                Ok(e) => {
                    unsafe { PC = e };
                    return Ok(e);
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
        fn increment_pc() -> Result<usize, &'static str> {
            let e: usize;
            unsafe {
                e = PC;
            }
            if e == 0xFFFFusize {
                return Err("Can't increment PC over 0xFFFF");
            } else {
                return Ok(e + 1);
            }
        }
        pub fn dump_rom() -> [u8; ROM_SIZE] {
            let dump: [u8; ROM_SIZE];
            unsafe {
                dump = ROM;
            }
            dump
        }
    }

    let mut items: Vec<String> = vec![];
    //#region Fill item vector
    {
        let reader = BufReader::new(File::open("data/data1.asm").expect("Couldn't open file"));
        for line in reader.lines() {
            let line = line.unwrap().trim().to_string();
            items.push(line);
        }
    }
    //#endregion
    let mut output_file = match OpenOptions::new().write(true).open("data/out.hex") {
        Ok(e) => e,
        _ => File::create("data/out.hex").expect("Couldn't create file"),
    };

    println!("{:#?}", items);

    for item in items {
        if rusty_6502_assembler::lib::parser::line_regex::opcode.is_match(&item) {
            let (opcode, operand): (Bytes, Bytes) = assembler::assemble_line(&item).unwrap();
            for i in 0..opcode.size {
                assembler_state::push_byte(opcode.bytes[i]).expect("Couldn't store byte");
            }
            for i in 0..operand.size {
                assembler_state::push_byte(operand.bytes[i]).expect("Couldn't store byte");
            }
        } else if line_regex::directive.is_match(&item) {
            println!("Process directive");
        } else {
            println!("Unexpected {}", item);
        }
    }
    let rom = assembler_state::dump_rom();
    for i in 0..assembler_state::ROM_SIZE {
        output_file.write(&rom[i..i + 1]).expect("Couldn't save");
    }
}
/*
A9FE
AA
8D0100
69
01
*/
