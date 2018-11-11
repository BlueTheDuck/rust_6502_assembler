use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
extern crate rusty_6502_assembler;
use rusty_6502_assembler::manager;

extern crate regex;

mod assembler {
    pub mod types {}
    pub fn assemble_line(line: &String) {
        let mut op_iter = line.split(" ");
        let name = op_iter.next().unwrap();
        let operand = op_iter.next();
        let mut opcode:Option<&rusty_6502_assembler::manager::Opcode> = None;

        if let Some(operand) = operand {
            let op_mode = rusty_6502_assembler::manager::identify_operand(operand);
            opcode = rusty_6502_assembler::manager::get_hex(name, op_mode);
            println!("{} has {:?}", name,&op_mode);
        } else {
            println!("No operand provided");
        }
        if let Some(opcode) = opcode {
            println!("{:?}", opcode);
        } else {
            println!("Error");
        }
    }
}
mod data_types {
    struct Bytes {
        cant: usize,
        bytes: [u8; 4],
    }
    impl Bytes {
        fn new_8(param: u8) -> Bytes {
            Bytes {
                cant: 1,
                bytes: [param, 0, 0, 0],
            }
        }
    }

}

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
        assembler::assemble_line(&item);
    }
}
