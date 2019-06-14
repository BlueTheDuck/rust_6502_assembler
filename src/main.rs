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

mod tokenizer;
use tokenizer::tree::Tree;

use serde_json::Result;

fn main() {
    let mut file: BufReader<File>;
    let mut file_out: BufWriter<File>;
    let mut tree: Tree;

    file = BufReader::new(
        OpenOptions::new()
            .read(true)
            .open("input/basic_test.asm")
            .expect("Couldn't open input file"),
    );

    file_out = BufWriter::new(
        OpenOptions::new()
            .write(true)
            .create(true)
            .open("output/basic_test.json")
            .expect("Couldn't open out file"),
    );

    tree = Tree::parse(file);
    tree.insert(0, tokenizer::token_type::TokenType::new("LDA").unwrap())
        .expect(":(");
    tree.insert(1, tokenizer::token_type::TokenType::new("#$22").unwrap())
        .expect(":(");
    let json: String = format!(
        "{}",
        serde_json::to_string(&tree).expect("Couldn't jsonaize the tree")
    );
    file_out.write(json.as_ref()).expect("Couldn't write file");
}
