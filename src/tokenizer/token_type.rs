use super::types;
use super::RegexMap;
use crate::regex::Regex;
use crate::BTreeMap;
use serde::ser::{Serialize, Serializer, SerializeTupleVariant};
use std::rc::Rc;

lazy_static! {
    pub static ref TOKEN_REGEXS: RegexMap = {
        let mut _regexs = BTreeMap::new();
        _regexs.insert(
            "opcode",
            Regex::new(r#"^\w{3}$"#).expect("Regex building failed"),
        );
        _regexs.insert(
            "label",
            Regex::new(r#"\w{1,}:"#).expect("Regex building failed"),
        );
        _regexs.insert(
            "value",
            Regex::new(r#"(#?$[0-9A-F]{2,4})|\w{1,}"#).expect("Regex building failed"),
        );
        _regexs
    };
}

#[derive(Debug)]
pub enum TokenType {
    LABEL(String),
    OPCODE(types::Opcode),
    VALUE(types::parameter),
}

impl TokenType {
    pub fn new<S: Into<String>>(data: S) -> Result<Self, &'static str> {
        let data: String = data.into();

        // Loop thru the 3 types of tokens. Append the name with the regex
        for (token_type, test) in ["opcode", "label", "value"]
            .into_iter()
            .map(|x| (x, &TOKEN_REGEXS[x]))
        {
            if test.is_match(&data) {
                let owned = data.to_owned();
                let value = match *token_type {
                    "opcode" => TokenType::OPCODE(types::Opcode {
                        name: owned,
                        parameter: Rc::new(types::Value::NONE),
                    }),
                    "label" => TokenType::LABEL(owned.trim_end_matches(":").to_string()),
                    "value" => match types::Value::new(owned) {
                        Ok(value) => TokenType::VALUE(Rc::new(value)),
                        Err(err) => {
                            return Err(err);
                        }
                    },
                    _ => panic!("How did you get here?"),
                };
                return Ok(value);
            }
        }
        return Err("The value provided couldn't be parsed");
    }
    //#region is_*(&self) -> bool
    pub fn is_label(&self) -> bool {
        if let TokenType::LABEL(_) = self {
            return true;
        }
        return false;
    }
    pub fn is_opcode(&self) -> bool {
        if let TokenType::OPCODE(_) = self {
            return true;
        }
        return false;
    }
    pub fn is_value(&self) -> bool {
        if let TokenType::VALUE(_) = self {
            return true;
        }
        return false;
    }
    //#endregion
    //#region unwrap_*(self) -> * -- CONSUMES SELF!
    pub fn unwrap_opcode(self) -> types::Opcode {
        match self {
            TokenType::OPCODE(opcode) => opcode,
            _ => panic!("The wrapped value is not an opcode"),
        }
    }
    pub fn unwrap_value(self) -> Rc<types::Value> {
        match self {
            TokenType::VALUE(value) => value.clone(),
            _ => panic!("The wrapped value is not a value"),
        }
    }
    pub fn unwrap_label(self) -> String {
        match self {
            TokenType::LABEL(label) => label,
            _ => panic!("The wrapped value is not a label"),
        }
    }
    //#endregion
    //#region get_*(&self) -> &*
    pub fn get_value(&self) -> Rc<types::Value> {
        match self {
            TokenType::VALUE(value) => value.clone(),
            _ => {
                panic!("The wrapped value is not a value");
            }
        }
    }
    //#endregion
}
impl Serialize for TokenType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok,S::Error>
    where
        S: Serializer,
    {
        match self {
            TokenType::LABEL(name) => {
                let mut state: S::SerializeTupleVariant = serializer.serialize_tuple_variant("TokenType",0,"LABEL",3)?;
                state.serialize_field(name)?;
                state.end()
            },
            TokenType::OPCODE(opcode) => {
                let mut state: S::SerializeTupleVariant = serializer.serialize_tuple_variant("TokenType",1,"OPCODE",3)?;
                state.serialize_field(opcode)?;
                state.end()
            },
            TokenType::VALUE(val) => {
                let mut state: S::SerializeTupleVariant = serializer.serialize_tuple_variant("TokenType",2,"VALUE",3)?;
                state.serialize_field(&**val)?;
                state.end()
            },
        }
    }
}
impl<'a> std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let token_value = match self {
            TokenType::LABEL(label) => format!("LABEL_DEF({})", label),
            TokenType::OPCODE(opcode) => format!("OPCODE({})", opcode.name),
            TokenType::VALUE(value) => format!("VALUE({})", value),
        };
        write!(f, "{}", token_value)
    }
}
