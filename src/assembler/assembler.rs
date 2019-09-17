use super::types;
use super::types::{Address, ByteCode, Value, Word};
use super::{addressing, opcodes};

pub trait Assemblable {
    fn assemble(&self) -> ByteCode;
}

impl Assemblable for Address {
    fn assemble(&self) -> ByteCode {
        match self {
            Address::INT(word) => ByteCode(vec![*word]),
            Address::DOUBLE { lo, hi, ind } => ByteCode(vec![*lo, *hi]),
            Address::LABEL(name) => ByteCode(vec![0xDE,0xAD])//unimplemented!("Labels are not implemented yet"),
        }
    }
}
impl Assemblable for Value {
    fn assemble(&self) -> ByteCode {
        match self {
            Value::ADDRESS(address) => address.assemble(),
            Value::BYTES(bytes) => ByteCode(bytes.clone()),
            Value::NONE => ByteCode(vec![]),
        }
    }
}
impl Assemblable for types::Opcode {
    fn assemble(&self) -> ByteCode {
        let parameter_info = addressing::identify(&self.parameter);
        let opcode_info = opcodes::find(|op| op.name == self.name && op.mode == parameter_info)
            .expect( &format!("No opcode found with {} and {:?}",self.name,parameter_info)  );
        let mut assembly: Vec<Word> = (*self.parameter).assemble().0;
        assembly.insert(0, opcode_info.code);
        ByteCode(assembly)
    }
}
