
pub enum LineType {
    Opcode,
    Directive,
}
pub mod line_regex {
    lazy_static! {
        pub static ref directive: regex::Regex =
            regex::Regex::new(r"^\..+$").expect("Failed regex creation 'directive'");
        pub static ref opcode: regex::Regex = regex::Regex::new(r"^[A-Z]{3}(\s#?\$[A-Z0-9]{2,4})?$")
            .expect("Failed regex creation 'opcode'");
    }
}
