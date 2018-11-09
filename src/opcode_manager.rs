extern crate regex;

pub mod opcode_manager {
    //#region Datatypes
    pub enum Addressing_modes {
        Immediate, /* # */
        Implicit,  /* impl */
        None,
    }
    pub struct Opcode<'a> {
        pub name: &'a str,
        pub addr_mode: Addressing_modes,
        pub value: u8,
    }
    //#endregion  
    //#region Macros
    macro_rules! mode_name_to_enum {
        ("#")=> {Addressing_modes::Immediate};
        ("impl")=>{Addressing_modes::Implicit};
        ($_:expr)=>{Addressing_modes::None};
    }
    macro_rules! create_opcode {
        ($name:expr,$addr_mode:expr,$val:expr) => {
            Opcode {
                name: $name,
                addr_mode: mode_name_to_enum!($addr_mode),
                value: $val,
            }
        };
    }
    //#endregion
    //#region Opcode-static-list
    static opcode_list: [Opcode; 53] = [
        create_opcode!("ADC", "#", 0x69),
        create_opcode!("ADC", "abs", 0x6D),
        create_opcode!("ADC", "zpg", 0x65),
        create_opcode!("AND", "#", 0x29),
        create_opcode!("AND", "abs", 0x2D),
        create_opcode!("AND", "zpg", 0x25),
        create_opcode!("BIT", "abs", 0x2C),
        create_opcode!("BIT", "zpg", 0x24),
        create_opcode!("CLV", "impl", 0xB8),
        create_opcode!("CMP", "#", 0xC9),
        create_opcode!("CMP", "abs", 0xCD),
        create_opcode!("CMP", "zpg", 0xC5),
        create_opcode!("CPX", "#", 0xE0),
        create_opcode!("CPX", "abs", 0xEC),
        create_opcode!("CPX", "zpg", 0xE4),
        create_opcode!("CPY", "#", 0xC0),
        create_opcode!("CPY", "abs", 0xCC),
        create_opcode!("CPY", "zpg", 0xC4),
        create_opcode!("DEC", "abs", 0xCE),
        create_opcode!("DEC", "zpg", 0xC6),
        create_opcode!("DEX", "impl", 0xCA),
        create_opcode!("DEY", "impl", 0x88),
        create_opcode!("INC", "abs", 0xEE),
        create_opcode!("INC", "zpg", 0xE6),
        create_opcode!("INX", "impl", 0xE8),
        create_opcode!("INY", "impl", 0xC8),
        create_opcode!("JMP", "abs", 0x4C),
        create_opcode!("LDA", "#", 0xA9),
        create_opcode!("LDA", "abs", 0xAD),
        create_opcode!("LDA", "zpg", 0xA5),
        create_opcode!("LDX", "#", 0xA2),
        create_opcode!("LDX", "abs", 0xAE),
        create_opcode!("LDX", "zpg", 0xA6),
        create_opcode!("LDY", "#", 0xA0),
        create_opcode!("LDY", "abs", 0xAC),
        create_opcode!("LDY", "zpg", 0xA4),
        create_opcode!("LSR", "abs", 0x4E),
        create_opcode!("LSR", "zpg", 0x46),
        create_opcode!("LSR", "A", 0x4A),
        create_opcode!("NOP", "impl", 0xEA),
        create_opcode!("PHA", "impl", 0x48),
        create_opcode!("PLA", "impl", 0x68),
        create_opcode!("RTS", "impl", 0x60),
        create_opcode!("STA", "abs", 0x8D),
        create_opcode!("STA", "zpg", 0x85),
        create_opcode!("STX", "abs", 0x8E),
        create_opcode!("STX", "zpg", 0x86),
        create_opcode!("STY", "abs", 0x8C),
        create_opcode!("STY", "zpg", 0x84),
        create_opcode!("TAX", "impl", 0xAA),
        create_opcode!("TAY", "impl", 0xA8),
        create_opcode!("TXA", "impl", 0x8A),
        create_opcode!("TYA", "impl", 0x98),
    ];
    //#endregion
    pub fn identify_operand(operand:&str)->Addressing_modes {
        let Immediate = regex::Regex::new(r"[0-9A-F]{2}").expect("Failed parsing regex");
        
        if Immediate.is_match(operand) {
            return Addressing_modes::Immediate;
        }
        Addressing_modes::None
    }
}
