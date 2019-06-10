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

/* use serde::ser::{Serialize,SerializeStruct}; */
use serde_json::Result;

mod tokenizer;
use tokenizer::parse;
use tokenizer::tree::Tree;

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
    
    tree = parse(file);

    println!(
        "{}",
        serde_json::to_string(&tree).expect("Couldn't jsonaize the tree")
    );
}
