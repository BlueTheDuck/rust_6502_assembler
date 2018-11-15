use lib::opcode_manager::{get_hex, identify_operand, AddressingModes, Opcode};

pub mod data_types {
    pub struct Bytes {
        pub quant: usize,
        pub bytes: [u8; 4],
    }
    impl Default for Bytes {
        fn default() -> Self {
            Bytes {
                quant: 0,
                bytes: [0, 0, 0, 0],
            }
        }
    }
    //#region impl From
    impl std::convert::From<u8> for Bytes {
        fn from(n: u8) -> Self {
            let mut b: Bytes = Bytes::default();
            /*for i in 0..1 {
                let disp: u64 = i * 8;
                let i: usize = i as usize;
                b.bytes[i] = ((n as u64 & (0xFFu64 << disp)) >> disp) as u8;
            }*/
            b.bytes[0] = n;
            b.quant = 1;
            b
        }
    }
    impl std::convert::From<u16> for Bytes {
        fn from(n: u16) -> Self {
            let mut b: Bytes = Bytes::default();
            for i in 0..2 {
                let disp: u64 = i * 8;
                let i: usize = i as usize;
                b.bytes[i] = ((n as u64 & (0xFFu64 << disp)) >> disp) as u8;
            }
            b.quant = 2;
            b
        }
    }
    impl std::convert::From<u32> for Bytes {
        fn from(n: u32) -> Self {
            let mut b: Bytes = Bytes::default();
            for i in 0..4 {
                let disp: u32 = i * 8;
                let i: usize = i as usize;
                b.bytes[i] = ((n as u32 & (0xFFu32 << disp)) >> disp) as u8;
            }
            b.quant = 4;
            b
        }
    }
    //#endregion
    //#region Formatters
    impl std::fmt::UpperHex for Bytes {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            let mut s: String = String::from("");
            for i in 0..self.quant {
                s = s + &format!("{:02X}", self.bytes[i]);
            }
            write!(f, "{}", s)
        }
    }
    impl std::fmt::Debug for Bytes {
        fn fmt(&self,f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f,"{:X}",self)
        }
    }
    impl std::fmt::Display for Bytes {
        fn fmt(&self,f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f,"{:X?}",self)
        }
    }
    //#endregion
}
fn operand_to_bytes(operand: &str) -> data_types::Bytes {
    lazy_static! {
        static ref regex_extract_numbers: regex::Regex =
            regex::Regex::new(r"\#?\$?(?P<d>[0-9A-Fa-f]+)").unwrap();
    }

    let clean_operand = regex_extract_numbers.replace_all(operand, "$d");
    let numberic_operand: u32 =
        u32::from_str_radix(&clean_operand, 16).expect("Couldn't parse operand");
    if clean_operand.len() == 4 {
        /*let temp = numberic_operand & 0xFF;
        numberic_operand = (numberic_operand & 0xFF00) >> 8;
        numberic_operand += temp << 8;*/
        return data_types::Bytes::from(numberic_operand as u16);
    } else if clean_operand.len() == 2 {
        return data_types::Bytes::from(numberic_operand as u8);
    }
    assert!(false);
    data_types::Bytes::default()
}
pub fn assemble_line(line: &String) -> Result<(&Opcode,data_types::Bytes),&str> {
    let mut op_iter = line.split(" ");
    let name = op_iter.next().unwrap();
    let operand: Option<&str> = op_iter.next();
    let binary_operand: data_types::Bytes;
    let opcode: Option<&Opcode>;
    let op_mode: AddressingModes;

    if let Some(operand) = operand {
        op_mode = identify_operand(operand);
        binary_operand = operand_to_bytes(operand);
    } else {
        op_mode = AddressingModes::Implicit;
        binary_operand = data_types::Bytes::default();
    }
    opcode = get_hex(name, op_mode);

    if let Some(opcode) = opcode {
        return Ok((opcode,binary_operand));
    } else {
        return Err("Couldn't assemble line");
    }
}
