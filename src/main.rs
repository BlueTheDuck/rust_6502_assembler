/* #![allow(unused_variables)] */
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_assignments)]

#[macro_use]
extern crate lazy_static;
extern crate regex;
#[macro_use]
extern crate structopt;

use std::collections::BTreeMap;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Write};

use structopt::StructOpt;

use serde::ser::{Serialize, SerializeStruct};
use serde_json::Result;

mod tokenizer;
use tokenizer::tree::Tree;

mod assembler;
use assembler::Assemblable;

#[derive(StructOpt, Debug)]
enum Args {
    #[structopt()]
    Assemble {
        #[structopt(parse(from_str), short, long)]
        in_file: String,
        #[structopt(parse(from_str), short, long)]
        out_file: Option<String>,
    },
}

fn main() {
    let mut file_in: BufReader<File>;
    let mut file_out_json: BufWriter<File>;
    let mut file_out: BufWriter<File>;

    let args = Args::from_args();
    match args {
        Args::Assemble { in_file, out_file } => {
            file_in = BufReader::new(
                OpenOptions::new()
                    .read(true)
                    .open(in_file)
                    .expect("Couldn't open input file"),
            );
            file_out = BufWriter::new(
                OpenOptions::new()
                    .write(true)
                    .create(true)
                    .open(out_file.unwrap_or("output/default.hex".to_string()))
                    .expect("Couldn't open out file"),
            );
            let mut obj_code: assembler::ObjectCode = Default::default();
            let mut tree: Tree = Tree::parse(file_in);
            for token in &*tree {
                let token: &tokenizer::token_type::TokenType = token;
                if token.is_opcode() {
                    let opcode: &tokenizer::types::Opcode = token.get_opcode();
                    let bytes: tokenizer::types::ByteCode = opcode.assemble();
                    //file_out.write(&bytes.0);
                    for byte in bytes.0 {
                        obj_code.rom.push(byte);
                    }
                }
            }
            println!("{:02X?}", obj_code.rom);
        }
    };

    /*  file_out_json = BufWriter::new(
        OpenOptions::new()
            .write(true)
            .create(true)
            .open("object/custom.json")
            .expect("Couldn't open out file"),
    );

    /*     tree.insert(0, tokenizer::token_type::TokenType::new("LDA").unwrap()) */
    /*         .expect(":("); */
    /*     tree.insert(1, tokenizer::token_type::TokenType::new("$0100").unwrap()) */
    /*         .expect(":("); */
    let json: String = format!(
        "{}",
        serde_json::to_string(&tree).expect("Couldn't jsonize the tree")
    );
    file_out_json
        .write(json.as_ref())
        .expect("Couldn't write file"); */
}
