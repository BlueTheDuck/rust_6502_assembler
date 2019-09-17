use super::types::{Address, Value};
use crate::regex::Regex;

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

pub fn identify(value: &Value) -> AddressingModes {
    match value {
        Value::NONE => AddressingModes::IMPL,
        Value::BYTES(bytes) => AddressingModes::IMM,
        Value::ADDRESS(address) => match address {
            Address::DOUBLE { lo, hi, ind } => if *ind {AddressingModes::IND} else {AddressingModes::ABS},
            Address::INT(byte) => AddressingModes::ZPG,
            Address::LABEL(name) => AddressingModes::ABS//unimplemented!("We don't support labels yet"),
        },
    }
}
