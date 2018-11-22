macro_rules! include_directive {
    ($name:expr,$file:expr) => {
        ($name, include!($file))
    };
}

#[derive(Debug)]
pub enum ParamTypes {
    u_byte(u8),
    u_short(u16),
    addr(u16),
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
                ParamTypes::u_byte(u8::from_str_radix(&f[2..4], 16).expect("Couldn't parse to u8"))
            }
            5 => ParamTypes::u_short(
                u16::from_str_radix(&f[1..5], 16).expect("Couldn't parse to u16"),
            ),
            6 => {
                ParamTypes::addr(u16::from_str_radix(&f[2..6], 16).expect("Couldn't parse to u16"))
            }
            _ => ParamTypes::None,
        }
    }
}

use lib::assembler::rom;

type directive_fn = fn(&mut rom, ParamTypes);
type directive_name = &'static str;

pub static directives: [(directive_name, directive_fn); 1] =
    [include_directive!(".org", "./directives/org.rs")];
pub fn find_directive(name: &str) -> Option<usize> {
    let mut i:usize = 0;
    for directive in directives.iter() {
        if directive.0 == name {
            return Some(i);
        }
        i += 1;
    }
    None
}
