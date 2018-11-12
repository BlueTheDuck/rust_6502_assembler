extern crate regex;

#[macro_use]
extern crate lazy_static;

use regex::Regex;

pub mod manager {
    mod operand_regexs {
        lazy_static! {
            pub static ref immediate: regex::Regex =
                regex::Regex::new(r"\#\$[0-9A-F]{2}").expect("Failed parsing regex");
            pub static ref absolute: regex::Regex =
                regex::Regex::new(r"\$[0-9A-F]{4}").expect("Failed parsing regex");
            pub static ref zero_page: regex::Regex =
                regex::Regex::new(r"\$[0-9A-F]{2}").expect("Failed parsing regex");
            
        }
    }

    //#region Datatypes
    #[derive(PartialEq, Debug,Clone,Copy)]
    pub enum AddressingModes {
        Immediate, /* # */
        Implicit,  /* impl */
        Absolute,
        ZeroPage,
        Accumulator,
        None,
        Error,
    }
    #[derive(Debug)]
    pub struct Opcode<'a> {
        pub name: &'a str,
        pub addr_mode: AddressingModes,
        pub value: u8,
    }
    //#endregion
    //#region Macros
    macro_rules! create_opcode {
        ($name:expr,$addr_mode:expr,$val:expr) => {
            Opcode {
                name: $name,
                addr_mode: $addr_mode,
                value: $val,
            }
        };
    }
    //#endregion
    //#region Opcode-static-list
    static OPCODE_LIST: [Opcode; 53] = [
        create_opcode!("ADC", AddressingModes::Immediate, 0x69),
        create_opcode!("ADC", AddressingModes::Absolute, 0x6D),
        create_opcode!("ADC", AddressingModes::ZeroPage, 0x65),
        create_opcode!("AND", AddressingModes::Immediate, 0x29),
        create_opcode!("AND", AddressingModes::Absolute, 0x2D),
        create_opcode!("AND", AddressingModes::ZeroPage, 0x25),
        create_opcode!("BIT", AddressingModes::Absolute, 0x2C),
        create_opcode!("BIT", AddressingModes::ZeroPage, 0x24),
        create_opcode!("CLV", AddressingModes::Implicit, 0xB8),
        create_opcode!("CMP", AddressingModes::Immediate, 0xC9),
        create_opcode!("CMP", AddressingModes::Absolute, 0xCD),
        create_opcode!("CMP", AddressingModes::ZeroPage, 0xC5),
        create_opcode!("CPX", AddressingModes::Immediate, 0xE0),
        create_opcode!("CPX", AddressingModes::Absolute, 0xEC),
        create_opcode!("CPX", AddressingModes::ZeroPage, 0xE4),
        create_opcode!("CPY", AddressingModes::Immediate, 0xC0),
        create_opcode!("CPY", AddressingModes::Absolute, 0xCC),
        create_opcode!("CPY", AddressingModes::ZeroPage, 0xC4),
        create_opcode!("DEC", AddressingModes::Absolute, 0xCE),
        create_opcode!("DEC", AddressingModes::ZeroPage, 0xC6),
        create_opcode!("DEX", AddressingModes::Implicit, 0xCA),
        create_opcode!("DEY", AddressingModes::Implicit, 0x88),
        create_opcode!("INC", AddressingModes::Absolute, 0xEE),
        create_opcode!("INC", AddressingModes::ZeroPage, 0xE6),
        create_opcode!("INX", AddressingModes::Implicit, 0xE8),
        create_opcode!("INY", AddressingModes::Implicit, 0xC8),
        create_opcode!("JMP", AddressingModes::Absolute, 0x4C),
        create_opcode!("LDA", AddressingModes::Immediate, 0xA9),
        create_opcode!("LDA", AddressingModes::Absolute, 0xAD),
        create_opcode!("LDA", AddressingModes::ZeroPage, 0xA5),
        create_opcode!("LDX", AddressingModes::Immediate, 0xA2),
        create_opcode!("LDX", AddressingModes::Absolute, 0xAE),
        create_opcode!("LDX", AddressingModes::ZeroPage, 0xA6),
        create_opcode!("LDY", AddressingModes::Immediate, 0xA0),
        create_opcode!("LDY", AddressingModes::Absolute, 0xAC),
        create_opcode!("LDY", AddressingModes::ZeroPage, 0xA4),
        create_opcode!("LSR", AddressingModes::Absolute, 0x4E),
        create_opcode!("LSR", AddressingModes::ZeroPage, 0x46),
        create_opcode!("LSR", AddressingModes::Accumulator, 0x4A),
        create_opcode!("NOP", AddressingModes::Implicit, 0xEA),
        create_opcode!("PHA", AddressingModes::Implicit, 0x48),
        create_opcode!("PLA", AddressingModes::Implicit, 0x68),
        create_opcode!("RTS", AddressingModes::Implicit, 0x60),
        create_opcode!("STA", AddressingModes::Absolute, 0x8D),
        create_opcode!("STA", AddressingModes::ZeroPage, 0x85),
        create_opcode!("STX", AddressingModes::Absolute, 0x8E),
        create_opcode!("STX", AddressingModes::ZeroPage, 0x86),
        create_opcode!("STY", AddressingModes::Absolute, 0x8C),
        create_opcode!("STY", AddressingModes::ZeroPage, 0x84),
        create_opcode!("TAX", AddressingModes::Implicit, 0xAA),
        create_opcode!("TAY", AddressingModes::Implicit, 0xA8),
        create_opcode!("TXA", AddressingModes::Implicit, 0x8A),
        create_opcode!("TYA", AddressingModes::Implicit, 0x98),
    ];
    //#endregion

    pub fn identify_operand(operand: &str) -> AddressingModes {
        if operand_regexs::immediate.is_match(operand) {
            return AddressingModes::Immediate;
        }
        if operand_regexs::absolute.is_match(operand) {
            return AddressingModes::Absolute;
        }
        if operand_regexs::zero_page.is_match(operand) {
            return AddressingModes::ZeroPage;
        }
        

        AddressingModes::Error
    }
    pub fn get_hex(opcode: &str, operand_mode: AddressingModes) -> Option<&Opcode> {
        let mut i = 0;
        while i < OPCODE_LIST.len() {
            let actual_op:&Opcode = &OPCODE_LIST[i];
            if actual_op.name == opcode && actual_op.addr_mode == operand_mode {
                return Some(&OPCODE_LIST[i]);
            }
            i += 1;
        }
        None
    }
}
