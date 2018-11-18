mod line_parser {
    pub enum line_type {
        Opcode,
        Directive,
    }
    mod line_regex {
        lazy_static! {
            pub static ref directive = regex::Regex::new(r"^\.+$");
            pub static ref opcode = None;
        }
    }
}
