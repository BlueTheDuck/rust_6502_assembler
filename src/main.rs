extern crate rusty_6502_assembler;

use std::fs::File;
use std::io::{BufRead, BufReader};
use rusty_6502_assembler::lib::{assembler,opcode_manager};

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
        assembler::assemble_line(&item);
    }
}
