use super::types;
use super::RegexMap;
use crate::regex::Regex;
use crate::BTreeMap;

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
pub enum TokenType<'a> {
    LABEL(String),
    OPCODE(types::Opcode<'a>),
    VALUE(types::Value),
}
impl<'a> TokenType<'a> {
    pub fn new<S: Into<String>>(data: S) -> Result<Self, &'static str> {
        let data: String = data.into();
        for (token_type, test) in ["opcode", "label", "value"]
            .into_iter()
            .map(|x| (x, &TOKEN_REGEXS[x]))
        {
            if test.is_match(&data) {
                let owned = data.to_owned();
                let value = match *token_type {
                    "opcode" => TokenType::OPCODE(types::Opcode {
                        name: owned,
                        parameter: None,
                    }),
                    "label" => TokenType::LABEL(owned.trim_end_matches(":").to_string()),
                    "value" => match types::Value::new(owned) {
                        Ok(value) => TokenType::VALUE(value),
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
    pub fn unwrap_opcode(self) -> types::Opcode<'a> {
        match self {
            TokenType::OPCODE(opcode) => opcode,
            _ => panic!("The wrapped value is not an opcode"),
        }
    }
    pub fn unwrap_value(self) -> types::Value {
        match self {
            TokenType::VALUE(value) => value,
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
    pub fn get_value(&self) -> &types::Value {
        match &self {
            &TokenType::VALUE(value) => &value,
            _ => {panic!("The wrapped value is not a value");}
        }
    }
    //#endregion
}
impl<'a> std::fmt::Display for TokenType<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let token_value = match self {
            TokenType::LABEL(name) => format!("LABEL_DEF({})", name),
            TokenType::OPCODE(name) => format!("OPCODE({})", name.name),
            TokenType::VALUE(name) => format!("VALUE({})", name),
        };
        write!(f, "{}", token_value)
    }
}
