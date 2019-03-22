#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_assignments)]


#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::collections::BTreeMap;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Write};

mod token;

use token::token_type::TokenType;

fn main() {
    let file_desc: File;
    let mut file: BufReader<File>;
    let mut parts: Vec<&str>;
    let mut tokens: Vec<TokenType>;
    let file_desc_out: File;
    let mut file_out: BufWriter<File>;

    file_desc = OpenOptions::new()
        .read(true)
        .open("input/basic_test.asm")
        .expect("Couldn't open input file");
    file = BufReader::new(file_desc);
    parts = vec![];
    tokens = vec![];

    file_desc_out = OpenOptions::new()
        .write(true)
        .create(true)
        .open("output/basic_test.json")
        .expect("Couldn't open out file");
    file_out = BufWriter::new(file_desc_out);

    for line in file.lines() {
        let line = line.unwrap();
        for item in line.split_whitespace() {
            tokens.push(TokenType::new(item).expect(&format!("Invalid item {}", item)));
        }
    }

    for (i, tkn) in tokens.iter().enumerate() {
        println!("{}:\t{}", i, tkn);
    }

}
