use std::io::{BufRead,BufReader,Read};
use super::tree::Tree;
use super::token_type::TokenType;

pub fn parse<T>(file: BufReader<T>) -> Tree
where T: Read {
    let mut tree = Tree::new();
    for line in file.lines().map(|l|l.expect("Couldn't read line")) {
        for item in line.split_whitespace() {
            tree.insert(TokenType::new(item).expect(&format!("Invalid item {}", item)))
                .expect("Error building tree");
        }
    }
    tree
}