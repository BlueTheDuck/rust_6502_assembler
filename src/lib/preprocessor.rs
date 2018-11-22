//static mut HashMap = HashMap::new();
mod directives {
        pub static directives: std::collections::HashMap<&'static str, Box<Fn()>> =
            std::collections::HashMap::new();
    
}
/* || {
            use std::collections::HashMap;
            let mut hm: HashMap<&'static str, Box<Fn()>> = HashMap::new();
            hm.insert(".org", Box::new(|| {include!("./directives/org.rs")}));
            hm
        };*/