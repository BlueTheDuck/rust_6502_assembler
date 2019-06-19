use crate::regex::Regex;
use super::types::{Value,Address};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum AddressingModes {
    A,
    IMPL,
    IMM,
    ZPG,
    ZPGX,
    ZPGY,
    IND,
    INDX,
    INDY,
    ABS,
    ABSX,
    ABSY,
    REL,
}

/* pub fn identify(value: String) -> AddressingModes {
    unimplemented!();
    //let x = Regex::new(r#"\$(?P<ADDR>[0-9A-F]+)"#).expect("Regex building failed");
} */
pub fn identify(value: &Value) -> AddressingModes {
    match value {
        Value::NONE => AddressingModes::IMPL,
        Value::BYTES(bytes) => AddressingModes::IMM,
        Value::ADDRESS(address) => match address {
            Address::DOUBLE{lo,hi} => AddressingModes::ABS,
            Address::INT(byte) => AddressingModes::ZPG,
            Address::LABEL(name) => unimplemented!("We don't support labels yet"),
        }
        _ => panic!("nani nani"),
    }
}