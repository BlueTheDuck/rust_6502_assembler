extern crate rusty_6502_assembler;

use std::fs::File;
use std::io::{BufRead, BufReader};
use rusty_6502_assembler::lib::{assembler};
use assembler::data_types::Bytes;
use rusty_6502_assembler::lib::opcode_manager::Opcode;

fn main() {
    let mut items: Vec<String> = vec![];
    {
        let reader = BufReader::new(File::open("data/data1.asm").expect("Couldn't open file"));
        for line in reader.lines() {
            let line = line.unwrap();
            items.push(line);
        }
    }
    println!("{:#?}", items);

    for item in items {
        println!("Assembling {}", &item);
        let assembled:(&Opcode,Bytes) = assembler::assemble_line(&item).unwrap();
        println!("{:X}{:X?}",assembler::data_types::Bytes::from(assembled.0.value),assembled.1);
    }
}
/*
A9FE
AA
8D0100
69
01
*/
