use super::token_type::TokenType;
use super::types::{Address, Opcode, Value};
use serde::ser::{Serialize, SerializeSeq, SerializeStruct, SerializeTupleVariant, Serializer};

/* #region Typers */
impl Serialize for Address {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Address::DOUBLE { lo, hi, ind } => {
                let mut seq = serializer.serialize_struct("Address", 2)?;
                seq.serialize_field("lo", lo)?;
                seq.serialize_field("hi", hi)?;
                seq.end()
            }
            Address::INT(number) => serializer.serialize_u8(*number),
            Address::LABEL(label) => {
                let mut seq = serializer.serialize_tuple_variant("", 2, "LABEL", 1)?;
                seq.serialize_field(label)?;
                seq.end()
            }
        }
    }
}
impl Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Value::ADDRESS(addr) => {
                let mut seq = serializer.serialize_tuple_variant("Value", 0, "ADDRESS", 1)?;
                seq.serialize_field(addr)?;
                seq.end()
            }
            Value::BYTES(bytes) => {
                /* let mut seq = serializer.serialize_tuple_variant("Value", 1, "BYTES", 1)?;
                seq.serialize_field(bytes)?;
                seq.end() */
                let mut seq = serializer.serialize_seq(Some(bytes.len()))?;
                for b in bytes {
                    seq.serialize_element(b)?;
                }
                seq.end()
            }
            Value::NONE => serializer.serialize_unit_variant("Value", 2, "NONE"),
        }
    }
}
impl Serialize for Opcode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Opcode", 2)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("parameter", &*self.parameter)?;
        state.end()
    }
}
/* #endregion */
impl Serialize for TokenType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            TokenType::LABEL(name) => {
                let mut state: S::SerializeTupleVariant =
                    serializer.serialize_tuple_variant("TokenType", 0, "LABEL", 3)?;
                state.serialize_field(name)?;
                state.end()
            }
            TokenType::OPCODE(opcode) => {
                let mut state: S::SerializeTupleVariant =
                    serializer.serialize_tuple_variant("TokenType", 1, "OPCODE", 1)?;
                state.serialize_field(opcode)?;
                state.end()
            }
            TokenType::VALUE(val) => {
                use std::borrow::Borrow;
                match val.borrow() {
                    Value::ADDRESS(addr) => {
                        let mut seq =
                            serializer.serialize_tuple_variant("Value", 0, "ADDRESS", 1)?;
                        seq.serialize_field(addr)?;
                        seq.end()
                    }
                    Value::BYTES(bytes) => {
                        serializer.serialize_newtype_variant("Value", 1, "BYTES", bytes)
                    }
                    Value::NONE => serializer.serialize_unit_variant("Value", 2, "NONE"),
                }
            } /* TokenType::VALUE(val) => {
                let mut state: S::SerializeTupleVariant =
                    serializer.serialize_tuple_variant("TokenType", 2, "VALUE", 3)?;
                state.serialize_field(&**val)?;
                state.end()
              } */
        }
    }
}
