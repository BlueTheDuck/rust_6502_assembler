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

use serde::{Deserialize, Serialize};
use serde_json::Result;

mod tokenizer;

use tokenizer::token_type::TokenType;
use tokenizer::tree::Tree;

fn main() {
    let file_desc: File;
    let mut file: BufReader<File>;
    let mut tree: Tree;
    let file_desc_out: File;
    let mut file_out: BufWriter<File>;

    file_desc = OpenOptions::new()
        .read(true)
        .open("input/basic_test.asm")
        .expect("Couldn't open input file");
    file = BufReader::new(file_desc);

    file_desc_out = OpenOptions::new()
        .write(true)
        .create(true)
        .open("output/basic_test.json")
        .expect("Couldn't open out file");
    file_out = BufWriter::new(file_desc_out);

    tree = Tree::new();

    for line in file.lines().map(|line|line.expect("Failed to read line")) {
        for item in line.split_whitespace() {
            tree.insert(TokenType::new(item).expect(&format!("Invalid item {}", item)))
                .expect("Error building tree");
        }
    }

    /* for (i, tkn) in tokens.iter().enumerate() {
        println!("{}:\t{}", i, tkn);
    } */
    println!("{}", tree);

    let jason = serde_json::to_string("Hello");
    println!("{}", jason.expect("..."));
}
