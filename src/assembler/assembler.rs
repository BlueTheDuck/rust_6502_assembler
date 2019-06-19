use super::types::{Word,Address,Value,ByteCode};
use super::types;
use super::{opcodes,addressing};

pub trait Assemblable {
    fn assemble(&self) -> ByteCode;
}


impl Assemblable for Address {
    fn assemble(&self) -> ByteCode {
        match self {
            Address::INT(word) => ByteCode(vec![*word]),
            Address::DOUBLE{lo,hi} => ByteCode(vec![*lo,*hi]),
            Address::LABEL(name) => unimplemented!("Labels are not implemented yet"),
        }
    }
}
impl Assemblable for Value {
    fn assemble(&self) -> ByteCode {
        match self {
            Value::ADDRESS(address) => address.assemble(),
            Value::BYTES(bytes) => ByteCode(bytes.clone()),
            Value::NONE => ByteCode(vec![])
        }
    }
}
impl Assemblable for types::Opcode {
    fn assemble(&self) -> ByteCode {
        let parameter_info = addressing::identify(&self.parameter);
        let opcode_info = opcodes::find(
            |op|
                op.name==self.name &&
                op.mode==parameter_info
            ).expect("No opcode found");
        let mut assembly: Vec<Word> = (*self.parameter).assemble().0;
        assembly.insert(0, opcode_info.code);
        ByteCode(assembly)
    }
}