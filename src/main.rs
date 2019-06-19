/* #![allow(unused_variables)] */
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_assignments)]

#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::collections::BTreeMap;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Write};

use serde::ser::{Serialize, SerializeStruct};
use serde_json::Result;

mod tokenizer;
use tokenizer::tree::Tree;

mod assembler;
use assembler::Assemblable;

fn main() {
    let mut file: BufReader<File>;
    let mut file_out_json: BufWriter<File>;
    let mut file_out: BufWriter<File>;
    let mut tree: Tree;

    file = BufReader::new(
        OpenOptions::new()
            .read(true)
            .open("input/full_test.asm")
            .expect("Couldn't open input file"),
    );

    file_out_json = BufWriter::new(
        OpenOptions::new()
            .write(true)
            .create(true)
            .open("object/full_test.json")
            .expect("Couldn't open out file"),
    );
    file_out = BufWriter::new(
        OpenOptions::new()
            .write(true)
            .create(true)
            .open("output/full_test.hex")
            .expect("Couldn't open out file"),
    );


    tree = Tree::parse(file);
/*     tree.insert(0, tokenizer::token_type::TokenType::new("LDA").unwrap()) */
/*         .expect(":("); */
/*     tree.insert(1, tokenizer::token_type::TokenType::new("$0100").unwrap()) */
/*         .expect(":("); */
    let json: String = format!(
        "{}",
        serde_json::to_string(&tree).expect("Couldn't jsonaize the tree")
    );
    file_out_json.write(json.as_ref()).expect("Couldn't write file");
    for token in &*tree {
        let token: &tokenizer::token_type::TokenType = token;
        if token.is_opcode() {
            let opcode: &tokenizer::types::Opcode = token.get_opcode();
            let bytes = opcode.assemble();
            file_out.write(&bytes.0);
        }
    }
}
