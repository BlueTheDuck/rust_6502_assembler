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
        let mut opcode: Option<&rusty_6502_assembler::manager::Opcode> = None;
        let mut op_mode = rusty_6502_assembler::manager::AddressingModes::None;

        if let Some(operand) = operand {
            op_mode = rusty_6502_assembler::manager::identify_operand(operand);
            println!("{} has {:?}", name, &op_mode);
        } else {
            op_mode = rusty_6502_assembler::manager::AddressingModes::Implicit;
            println!("No operand provided");
        }
        let opcode = rusty_6502_assembler::manager::get_hex(name, op_mode);
        if let Some(opcode) = opcode {
            println!("{} = {:X}{}", opcode.name,opcode.value,match operand{Some(e)=>{e} _=>{""}});
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
