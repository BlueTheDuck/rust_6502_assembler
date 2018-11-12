extern crate rusty_6502_assembler;
#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;
use rusty_6502_assembler::manager::{AddressingModes, Opcode};
use std::fs::File;
use std::io::{BufRead, BufReader};

mod assembler {
    pub mod data_types {
        pub struct Bytes {
            pub quant: usize,
            pub bytes: [u8; 4],
        }
        impl Default for Bytes {
            fn default() -> Self {
                Bytes {
                    quant: 4,
                    bytes: [0, 0, 0, 0],
                }
            }
        }
        impl Bytes {
            pub fn from(n: u32) -> Self {
                let mut b: Bytes = Bytes::default();
                for i in 0..4 {
                    let disp = (i as u32) * 8;
                    b.bytes[i] = ((n & 0xFFu32 << disp) >> disp) as u8;
                }
                b
            }
        }
    }
    fn operand_to_bytes(operand: &str) -> data_types::Bytes {
        lazy_static! {
            static ref regex_extract_numbers: regex::Regex =
                regex::Regex::new(r"\#?\$?(?P<d>[0-9A-Fa-f]+)").unwrap();
        }

        let clean_operand = regex_extract_numbers.replace_all(operand, "$d");
        let mut numberic_operand: u32 =
            u32::from_str_radix(&clean_operand, 16).expect("Couldn't parse operand");
        if clean_operand.len() == 4 {
            let temp = numberic_operand & 0xFF;
            numberic_operand = (numberic_operand & 0xFF00) >> 8;
            numberic_operand += temp << 8;
        }
        data_types::Bytes::from(numberic_operand)
    }
    pub fn assemble_line(line: &String) {
        let mut op_iter = line.split(" ");
        let name = op_iter.next().unwrap();
        let operand: Option<&str> = op_iter.next();
        let binary_operand: data_types::Bytes;
        let opcode: Option<&rusty_6502_assembler::manager::Opcode>;
        let op_mode: rusty_6502_assembler::manager::AddressingModes;

        if let Some(operand) = operand {
            op_mode = rusty_6502_assembler::manager::identify_operand(operand);
            println!("{} has {:?}", name, &op_mode);
            binary_operand = operand_to_bytes(operand);
        } else {
            op_mode = rusty_6502_assembler::manager::AddressingModes::Implicit;
            println!("No operand provided");
            binary_operand =  data_types::Bytes::default();
        }
        opcode = rusty_6502_assembler::manager::get_hex(name, op_mode);

        if let Some(opcode) = opcode {
            println!(
                "{} = {:X}{}",
                opcode.name,
                opcode.value,
                match operand {
                    Some(_) => {(format!(
                        "{:X}{:X}{:X}{:X}",
                        binary_operand.bytes[0],
                        binary_operand.bytes[1],
                        binary_operand.bytes[2],
                        binary_operand.bytes[3]
                    )).to_string()},
                    _ => "".to_string(),
                }
            );
        } else {
            println!("Error");
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
