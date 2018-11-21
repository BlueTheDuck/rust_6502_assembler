extern crate rusty_6502_assembler;

use assembler::data_types::Bytes;
use rusty_6502_assembler::lib::{assembler,opcode_manager::Opcode};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};

fn increment_pc(pc:&mut usize) -> Result<usize,&str> {
    if *pc==0xFFFFusize {
        return Err("Can't increment PC over 0xFFFF");
    } else {
        return Ok(pc+1);
    }
}

fn main() {
    const ROM_SIZE:usize = 0x10000;
    let mut rom:[u8;ROM_SIZE] = [0;ROM_SIZE];
    let mut pc:usize = 0;

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
                rom[pc] = opcode.bytes[i];
                println!("{:#X}", &rom[pc]);
                pc+=1;
            }
            for i in 0..operand.size {
                rom[pc] = operand.bytes[i];
                println!("{:#X}", &rom[pc]);
                pc+=1;
            }
        } else {
            println!("Unexpected {}", item);
        }
    }
    for i in 0..ROM_SIZE {
        output_file.write(&rom[i..i+1]).expect("Coudln't save");
    }
}
/*
A9FE
AA
8D0100
69
01
*/
