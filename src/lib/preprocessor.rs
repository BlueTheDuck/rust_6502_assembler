macro_rules! include_directive {
    ($name:expr,$file:expr) => {
        ($name, include!($file))
    };
}

#[derive(Debug)]
pub enum ParamTypes {
    UByte(u8),
    UBhort(u16),
    Addr(u16),
    None,
}
impl std::convert::From<String> for ParamTypes {
    fn from(f: String) -> ParamTypes {
        let f = f.trim();
        //#$xx -> 4
        //#$xxyy -> 6
        //$xxyy -> 5
        match f.len() {
            4 => {
                ParamTypes::UByte(u8::from_str_radix(&f[2..4], 16).expect("Couldn't parse to u8"))
            }
            5 => ParamTypes::Addr(
                u16::from_str_radix(&f[1..5], 16).expect("Couldn't parse to u16"),
            ),
            6 => {
                ParamTypes::UBhort(u16::from_str_radix(&f[2..6], 16).expect("Couldn't parse to u16"))
            }
            _ => ParamTypes::None,
        }
    }
}

use lib::assembler::Rom;

type DirectiveFn = fn(&mut Rom, ParamTypes);
type DirectiveName = &'static str;

pub static DIRECTIVES: [(DirectiveName, DirectiveFn); 1] =
    [include_directive!(".org", "./directives/org.rs")];

pub fn find_directive(name: &str) -> Option<usize> {
    let mut i:usize = 0;
    for directive in DIRECTIVES.iter() {
        if directive.0 == name {
            return Some(i);
        }
        i += 1;
    }
    None
}
