use std::{
    env,
    error::Error,
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};

fn main() -> Result<(), Box<Error>> {
    let out_dir = env::var("OUT_DIR")?;
    let opcodetable_path = Path::new("data/opcodestable.csv");
    let mut data = include_str!("../data/opcodestable.csv").split_whitespace();
    /*let dest_path = Path::new(&out_dir).join("long_string.txt");
    let mut f = BufWriter::new(File::create(&dest_path)?);

    let long_string = "abc".repeat(100);
    write!(f, "{}", long_string)?;*/
    for line in data {
        let items = line.split(",");
        let code_line = format_args!("{} {} {}",items[0],items[0],items[0]);
        println!("{}",code_line);
    }

    Ok(())
}
