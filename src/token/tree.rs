use super::types::*;
use super::token_type::TokenType;
use super::types;

type TreeError = std::result::Result<(),&'static str>;

struct Tree<'a> {
    tokens: Vec<TokenType<'a>>,
}
impl<'a> Tree<'a> {
    fn new() -> Self {
        Tree {
            tokens: vec![]
        }
    }
    fn insert(&mut self,mut token: TokenType<'a>) -> TreeError {
        if token.is_value() {
            let last = self.tokens.pop().unwrap();
            match last {
                TokenType::OPCODE(mut opcode) => {
                    let last = TokenType::OPCODE(opcode);
                    self.tokens.push(last);
                    self.tokens.push(token);
                }
                _ => {return Err("Syntax error");}
            }
            /* if last.is_opcode() {
                let mut opcode = last.unwrap_opcode();
                let mut value = token.unwrap_value();
                opcode.parameter = Some(&value);
                self.insert(TokenType::new(opcode).unwrap());

                /* opcode.parameter = Some(&value);
                last = TokenType::OPCODE(opcode); */
            } else {
                self.insert(last);
                return Err("Syntax error");
            } */
        }
        self.tokens.push(token);
        return Ok(());
    }
}