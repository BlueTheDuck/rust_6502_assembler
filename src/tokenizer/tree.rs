use super::types::*;
use super::token_type::TokenType;
use super::types;

type TreeError = std::result::Result<(),&'static str>;

pub struct Tree {
    tokens: Vec<TokenType>,
}
impl Tree {
    pub fn new() -> Self {
        Tree {
            tokens: vec![]
        }
    }
    pub fn insert(&mut self,mut token: TokenType) -> TreeError {
        if token.is_value() {
            match self.tokens.pop().unwrap() {
                TokenType::OPCODE(mut opcode) => {
                    opcode.parameter = token.get_value().clone();
                    self.insert(TokenType::OPCODE(opcode)).expect("Error appending token on tree");
                    self.tokens.push(token);
                }
                _ => {return Err("Syntax error");}
            }
        } else {
            self.tokens.push(token);
        }
        return Ok(());
    }
}
impl<'a> std::fmt::Display for Tree {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let mut buf = "".to_string();
        for (i, tkn) in self.tokens.iter().enumerate() {
            buf = format!("{}\n{}:\t{}", buf, i, tkn);
        }
        write!(f,"{}",buf)
    }
}