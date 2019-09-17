use super::Regex;
use super::RegexMap;
use crate::BTreeMap;

use serde::ser::{SerializeStruct, Serializer};
use serde::{Deserialize, Serialize};
use std::rc::Rc;

pub type Word = u8;
pub struct ByteCode(pub Vec<Word>);
pub type Parameter = Rc<Value>;

lazy_static! {
    pub static ref VALUE_REGEXS: RegexMap = {
        let mut _regexs = BTreeMap::new();
        _regexs.insert(
            "address",
            Regex::new(r#"\$(?P<ADDR>[0-9A-F]+)"#).expect("Regex building failed"),
        );
        _regexs.insert(
            "number",
            Regex::new(r#"\#\$(?P<HEX>[0-9A-F]+)"#).expect("Regex building failed"),
        );
        _regexs.insert(
            "label",
            Regex::new(r#"(?P<LABEL>[a-z]{1,})$"#).expect("Regex building failed"),
        );
        _regexs
    };
}

//#region Types
#[derive(Debug, Deserialize)]
pub enum Address {
    INT(Word),
    DOUBLE { lo: Word, hi: Word, ind: bool },
    LABEL(String),
}
#[derive(Debug, Deserialize)]
pub enum Value {
    ADDRESS(Address),
    BYTES(Vec<Word>),
    NONE,
}
#[derive(Debug)]
pub struct Opcode {
    pub name: String,
    pub parameter: Parameter,
}
//#endregion

impl Value {
    pub fn new(value: String) -> Result<Self, (&'static str, String)> {
        let mut captures: Result<regex::Captures, &'static str> =
            Err("No regex matched the pased value to Value::new(string)");
        let out: Result<Value, (&'static str, String)>;
        let mut matches_index: usize = std::usize::MAX;
        let tests = ["number", "address", "label"];
        for (i, test) in tests
            .into_iter()
            .enumerate()
            .map(|(i, x)| (i, &VALUE_REGEXS[x]))
        {
            if test.is_match(&value) {
                matches_index = i;
                captures = Ok(test.captures(&value).expect("Couldn't capture values"));
                break;
            }
        }

        if captures.is_err() {
            return Err((captures.err().unwrap(), value));
        }

        let captures = captures.unwrap();
        out = match tests[matches_index] {
            "address" => match u16::from_str_radix(&captures["ADDR"], 16) {
                Ok(addr) => {
                    if captures["ADDR"].len() == 4 {
                        Ok(Value::ADDRESS(Address::DOUBLE {
                            hi: ((addr >> 8) & 0xFF) as u8,
                            lo: (addr & 0xFF) as u8,
                            ind: false,
                        }))
                    } else if captures["ADDR"].len() == 2 {
                        Ok(Value::ADDRESS(Address::INT((addr & 0xFF) as u8)))
                    } else {
                        Err(("Address is invalid in length", value))
                    }
                }
                Err(_) => Err(("The provided address isn't valid hex", value)),
            },
            "number" => {
                let number =
                    u8::from_str_radix(&captures["HEX"], 16).expect("Number provided is invalid");
                Ok(Value::BYTES(vec![number]))
            }
            "label" => Ok(Value::ADDRESS(Address::LABEL(
                captures["LABEL"].to_string(),
            ))),
            _ => panic!(""),
        };
        return out;
    }
}
impl std::fmt::UpperHex for ByteCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{:#?}",
            self.0
                .iter()
                .map(|byte| format!("{:02X}", byte))
                .collect::<String>()
        )
    }
}
//#region Trait Display implementations
impl std::fmt::Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let value = match self {
            Address::LABEL(name) => format!("{}", name),
            Address::INT(value) => format!("${:02X}", value),
            Address::DOUBLE { hi, lo, ind } => format!("${:02X}{:02X}", hi, lo),
        };
        write!(f, "ADDRESS({})", value)
    }
}
impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let value = match self {
            Value::BYTES(array) => format!("{}", {
                let mut arr_fmt: String = "".to_string();
                for (i, item) in array.iter().enumerate() {
                    if i == 0 {
                        arr_fmt = format!("0x{:X}", item)
                    }
                    if i < array.len() - 1 {
                        arr_fmt = format!("{},", arr_fmt);
                    }
                }
                format!("{}", arr_fmt)
            }),
            Value::ADDRESS(address) => format!("{}", address),
            Value::NONE => "".to_string(),
        };
        write!(f, "{}", value)
    }
}
//#endregion
