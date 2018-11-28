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
    MultiBytes(Vec<u8>),
    None,
}
impl std::convert::From<String> for ParamTypes {
    fn from(from: String) -> ParamTypes {
        let f = from.trim();
        lazy_static! {
            static ref rgx_ubyte: regex::Regex =
                regex::Regex::new(r"#\$(?P<d>[0-9A-Fa-f]{2})").unwrap();
            static ref rgx_addr: regex::Regex =
                regex::Regex::new(r"\$(?P<d>[0-9A-Fa-f]{4})").unwrap();
            static ref rgx_ushort: regex::Regex =
                regex::Regex::new(r"#\$(?P<d>[0-9A-Fa-f]{4})").unwrap();
            static ref rgx_multi_byte: regex::Regex =
                regex::Regex::new(r"(?P<d>[0-9A-Fa-f]{2}(,[0-9A-Fa-f]{2})*)").unwrap();
        }

        if rgx_ubyte.is_match(&from) {
            return ParamTypes::UByte(
                u8::from_str_radix(&f[2..4], 16).expect("Couldn't parse to u8"),
            );
        }
        if rgx_addr.is_match(&from) {
            return ParamTypes::Addr(
                u16::from_str_radix(&f[1..5], 16).expect("Couldn't parse to u16"),
            );
        }
        if rgx_ushort.is_match(&from) {
            return ParamTypes::UBhort(
                u16::from_str_radix(&f[2..6], 16).expect("Couldn't parse to u16"),
            );
        }
        if rgx_multi_byte.is_match(&from) {
            let from = rgx_multi_byte.replace_all(&from,"$d");
            let from = from.split(",");
            let mut bytes:Vec<u8> = vec![];
            for b in from {
                bytes.push(u8::from_str_radix(&b, 16).expect("Couldn't parse to u8"));
            }
            return ParamTypes::MultiBytes(bytes);
        }

        return ParamTypes::None;
    }
}

use lib::assembler::Rom;

type DirectiveFn = fn(&mut Rom, ParamTypes);
type DirectiveName = &'static str;

pub static DIRECTIVES: [(DirectiveName, DirectiveFn); 2] = [
    include_directive!(".org", "./directives/org.rs"),
    include_directive!(".bytes", "./directives/bytes.rs"),
];

pub fn find_directive(name: &str) -> Option<usize> {
    let mut i: usize = 0;
    for directive in DIRECTIVES.iter() {
        if directive.0 == name {
            return Some(i);
        }
        i += 1;
    }
    None
}
