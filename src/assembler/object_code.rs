
use super::types;
use std::collections::HashMap;

pub struct ObjectCode {
    pub labels: HashMap<String,u16>,
    pub rom: Vec<types::Word>
}
impl std::default::Default for ObjectCode {
    fn default() -> ObjectCode {
        ObjectCode {
            labels: HashMap::new(),
            rom: vec![],
        }
    }
}