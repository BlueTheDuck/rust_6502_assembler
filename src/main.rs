extern crate rusty_6502_assembler;
#[macro_use]
extern crate lazy_static;

use rusty_6502_assembler::lib as lib_assembler;
use lib_assembler::{assembler, parser::line_regex};
use assembler::data_types::Bytes;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};

fn main() {
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

    let mut rom_state:assembler::rom = assembler::rom::new();


    for item in items {
        let item:String = item;
        if rusty_6502_assembler::lib::parser::line_regex::opcode.is_match(&item) {
            let (opcode, operand): (Bytes, Bytes) = assembler::assemble_line(&item).unwrap();
            for i in 0..opcode.size {
                rom_state.push_byte(opcode.bytes[i]);
            }
            for i in 0..operand.size {
                rom_state.push_byte(operand.bytes[i]);
            }
        } else if line_regex::directive.is_match(&item) {
            println!("Process directive '{}'",item);
            let sep = match item.find(" ") {
                Some(i) => i,
                None => item.len()
            };
            let name = &item[0..sep];
            let operand = item[sep..].trim().to_string();
            let operand = lib_assembler::preprocessor::ParamTypes::from(operand);
            let dir_index = match lib_assembler::preprocessor::find_directive(name) {
                Some(e) => e,
                None => panic!("Directive not found")
            };
            lib_assembler::preprocessor::directives[dir_index].1(&mut rom_state,operand);
        } else {
            println!("Unexpected {}", item);
        }
    }

    for i in 0..assembler::ROM_SIZE {
        output_file
            .write(&[rom_state[i]])
            .expect("Coudln't save");
    }
}
/*
A9FE
AA
8D0100
69
01
*/
