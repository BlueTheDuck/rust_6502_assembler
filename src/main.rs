extern crate rusty_6502_assembler;

use assembler::data_types::Bytes;
use rusty_6502_assembler::lib::assembler;
use rusty_6502_assembler::lib::opcode_manager::Opcode;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};

fn main() {
    let mut items: Vec<String> = vec![];
    {
        let reader = BufReader::new(File::open("data/data1.asm").expect("Couldn't open file"));
        for line in reader.lines() {
            let line = line.unwrap();
            items.push(line);
        }
    }
    let mut output_file = match OpenOptions::new().write(true).open("data/out.hex") {
        Ok(e) => e,
        _ => File::create("data/out.hex").expect("Couldn't create file"),
    };

    println!("{:#?}", items);

    for item in items {
        let (opcode, operand): (&Opcode, Bytes) = assembler::assemble_line(&item).unwrap();
        output_file.write(&[opcode.value]).expect("Oh");
        output_file.write(&operand.bytes[0..operand.quant]);
    }
}
/*
A9FE
AA
8D0100
69
01
*/
