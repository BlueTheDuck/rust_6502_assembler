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
                    quant: 0,
                    bytes: [0, 0, 0, 0],
                }
            }
        }
        impl std::convert::From<u8> for Bytes {
            fn from(n: u8) -> Self {
                println!("Converting {:#X} to x16",n);
                let mut b: Bytes = Bytes::default();
                for i in 0..2 {
                    let disp: u64 = i * 8;
                    let i: usize = i as usize;
                    b.bytes[i] = ((n as u64 & (0xFFu64 << disp)) >> disp) as u8;
                }
                b.quant = 1;
                b
            }
        }
        impl std::convert::From<u16> for Bytes {
            fn from(n: u16) -> Self {
                println!("Converting {:#X} to x16",n);
                let mut b: Bytes = Bytes::default();
                for i in 0..2 {
                    let disp: u64 = i * 8;
                    let i: usize = i as usize;
                    b.bytes[i] = ((n as u64 & (0xFFu64 << disp)) >> disp) as u8;
                }
                b.quant = 1;
                b
            }
        }
        impl std::convert::From<u32> for Bytes {
            fn from(n: u32) -> Self {
                println!("Converting {:#X} to x32",n);
                let mut b: Bytes = Bytes::default();
                for i in 0..4 {
                    let disp: u64 = i * 8;
                    let i: usize = i as usize;
                    b.bytes[i] = ((n as u64 & (0xFFu64 << disp)) >> disp) as u8;
                }
                b.quant = 1;
                b
            }
        }
        impl std::fmt::UpperHex for Bytes {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                let mut s: String = String::from("");
                for i in 0..self.quant {
                    s = s + &format!("{:X}", self.bytes[i]);
                }
                write!(f, "{}", s)
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
            return data_types::Bytes::from(numberic_operand as u16);
        } else if clean_operand.len()==2{
            return data_types::Bytes::from(numberic_operand as u8);
        }
        assert!(false);
        data_types::Bytes::default()
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
            //println!("{} has {:?}", name, &op_mode);
            binary_operand = operand_to_bytes(operand);
        } else {
            op_mode = rusty_6502_assembler::manager::AddressingModes::Implicit;
            //println!("No operand provided");
            binary_operand = data_types::Bytes::default();
        }
        opcode = rusty_6502_assembler::manager::get_hex(name, op_mode);

        if let Some(opcode) = opcode {
            println!(
                "{} = {:X}{:X}",
                opcode.name,
                opcode.value,
                binary_operand
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
        println!("Assembling {}", &item);
        assembler::assemble_line(&item);
    }
}
