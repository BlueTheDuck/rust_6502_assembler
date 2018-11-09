use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

extern crate regex;

mod opcode_manager;
mod assembler {
    pub mod types {}
    pub fn assemble_line(line: &String) {
        
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
        items.reverse();
    }
    println!("{:#?}", items);

    for item in items {
        assembler::assemble_line(&item);
        
    }
}
