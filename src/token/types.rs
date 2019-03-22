use super::RegexMap;
use crate::regex::Regex;
use crate::BTreeMap;

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

#[derive(Debug)]
pub enum Address {
    INT(u8),
    DOUBLE { lo: u8, hi: u8 },
    LABEL(String),
}
#[derive(Debug)]
pub enum Value {
    ADDRESS(Address),
    BYTES(Vec<u8>),
}
#[derive(Debug)]
pub struct Opcode<'a> {
    pub name: String,
    pub parameter: Option<&'a Value>,
}

impl Value {
    pub fn new(value: String) -> Result<Self, &'static str> {
        let mut captures: Result<regex::Captures, &'static str> =
            Err("NO regex matched the pased value to Value::new(string)");
        let out: Value;
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
            return Err(captures.err().unwrap());
        }

        let captures = captures.unwrap();
        out = match tests[matches_index] {
            "address" => match u16::from_str_radix(&captures["ADDR"], 16) {
                Ok(addr) => {
                    if captures["ADDR"].len() == 4 {
                        Value::ADDRESS(Address::DOUBLE {
                            hi: ((addr >> 8) & 0xFF) as u8,
                            lo: (addr & 0xFF) as u8,
                        })
                    } else if captures["ADDR"].len() == 2 {
                        Value::ADDRESS(Address::INT((addr & 0xFF) as u8))
                    } else {
                        return Err("Address is invalid in length");
                    }
                }
                Err(_) => {
                    return Err("The provided address isn't valid hex");
                }
            },
            "number" => {
                let number =
                    u8::from_str_radix(&captures["HEX"], 16).expect("Number provided is invalid");
                Value::BYTES(vec![number])
            }
            "label" => Value::ADDRESS(Address::LABEL(captures["LABEL"].to_string())),
            _ => panic!(""),
        };
        return Ok(out);
    }
}
//#region Traits implementations
impl std::fmt::Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let value = match self {
            Address::LABEL(name) => format!("{}", name),
            Address::INT(value) => format!("${:02X}", value),
            Address::DOUBLE { hi, lo } => format!("${:02X}{:02X}", hi, lo),
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
        };
        write!(f, "{}", value)
    }
}
//#endregion
