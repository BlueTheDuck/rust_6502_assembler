use lib::opcode_manager::{get_hex, identify_operand, AddressingModes, Opcode};

pub const ROM_SIZE: usize = 0x10000;
pub struct Rom {
    pub rom: [u8; ROM_SIZE],
    pub pc: usize,
}
impl Rom {
    pub fn new() -> Rom {
        let new_rom = Rom {
            rom: [0; ROM_SIZE],
            pc: 0,
        };
        new_rom
    }
    pub fn push_byte(&mut self, b: u8) {
        self.rom[self.pc] = b;
        self.increment_pc();
    }
    fn increment_pc(&mut self) {
        self.pc += 1;
        self.pc = self.pc & 0xFFFF;
    }
}
impl std::ops::Index<usize> for Rom {
    type Output = u8;
    fn index(&self, addr: usize) -> &Self::Output {
        &self.rom[addr]
    }
}

pub mod data_types {
    pub struct Bytes {
        pub size: usize,
        pub bytes: [u8; 4],
    }
    impl Default for Bytes {
        fn default() -> Self {
            Bytes {
                size: 0,
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
            b.size = 1;
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
            b.size = 2;
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
            b.size = 4;
            b
        }
    }
    //#endregion
    //#region Formatters
    impl std::fmt::UpperHex for Bytes {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            let mut s: String = String::from("");
            for i in 0..self.size {
                s = s + &format!("{:02X}", self.bytes[i]);
            }
            write!(f, "{}", s)
        }
    }
    impl std::fmt::Debug for Bytes {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{:X}", self)
        }
    }
    impl std::fmt::Display for Bytes {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{:X?}", self)
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
pub fn assemble_line(line: &String) -> Result<(data_types::Bytes, data_types::Bytes), &str> {
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
        return Ok((data_types::Bytes::from(opcode.value), binary_operand));
    } else {
        return Err("Couldn't assemble line. Invalid opcode");
    }
}
