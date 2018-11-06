use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

extern crate regex;

mod assembler {
    pub mod types {}
    pub fn assemble_line(line: &String) {}
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
mod opcode_manager {
    //#region Datatypes
    pub enum Addressing_modes {
        Immediate, /* # */
        Implicit,  /* impl */
    }
    pub struct Opcode<'a> {
        pub name: &'a str,
        pub addr_mode: Addressing_modes,
        pub value: u8,
    }
    //#endregion
    macro_rules! create_opcode {
        ($name:expr,"#",$val:expr) => {
            Opcode {
                name: $name,
                addr_mode: Addressing_modes::Immediate,
                value: $val,
            }
        };
        ($name:expr,"impl",$val:expr) => {
            Opcode {
                name: $name,
                addr_mode: Addressing_modes::Implicit,
                value: $val,
            }
        };
        ($name:expr,$addr_mode:expr,$val:expr) => {
            Opcode {
                name: $name,
                addr_mode: $addr_mode,
                value: $val,
            }
        };
    }
    //#region Opcode static list
    static opcode_list: [Opcode; 2] = [
        create_opcode!("LDA", "#", 0xA9),
        create_opcode!("LDX", "#", 0xA2),
    ];
    //#endregion
}

fn main() {
    let mut items: Vec<String> = vec![];
    {
        let reader = BufReader::new(File::open("data/data1.asm").expect("Couldn't open file"));
        for line in reader.lines() {
            let line = line.unwrap();
            items.push(line);
        }
        items.reverse();
    }
    println!("{:#?}", items);

    for item in items {
        assembler::assemble_line(&item);
    }
}
