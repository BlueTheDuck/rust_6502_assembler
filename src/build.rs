use std::{
    env,
    error::Error,
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, BufWriter, Seek, SeekFrom, Write},
    path::Path,
};

fn main() -> Result<(), Box<Error>> {
    let out_dir = env::var("OUT_DIR")?;
    let mut opcode_code_generator: String = String::from("");

    opcode_code_generator = get_code();
    let mut opcodemanagermod_code = include_str!("opcode_manager.rs");
    let mut opcodemanagermod_code: Vec<&str> = opcodemanagermod_code.lines().collect();
    let mut opcodemanagermod_code_total: String = "".to_string();

    println!("Finding");

    let found:bool = false;
    let mut i = 0;
    for line in &opcodemanagermod_code {
        i = i + 1;
        opcodemanagermod_code_total += &line;
        opcodemanagermod_code_total += "\n";
        if line == &"    //#region Opcode-static-list" {
            println!("Found!");
            //opcodemanagermod_code_total += &opcode_code_generator.clone();
        }
        if found&&line!="];" {

        }
    }

    std::fs::write("src/opcode_manager.rs", opcodemanagermod_code_total);

    Ok(())
}

fn get_code() -> String {
    let mut opcode_csv = include_str!("../data/opcodestable.csv").split_whitespace();
    let mut opcode_code_generator: String = String::from("");
    let mut opcodes_counter = 0;

    for line in opcode_csv {
        opcodes_counter += 1;
        if opcodes_counter == 1 {
            continue;
        }

        let items: Vec<&str> = line.split(",").collect();
        let code_line = format!(
            "    create_opcode!(\"{}\", \"{}\", 0x{});\n",
            items[0], items[1], items[2]
        );
        opcode_code_generator += &code_line.clone();
    }

    opcode_code_generator = format!(
        "static opcode_list: [Opcode; {}] = [\n{}];",
        opcodes_counter, opcode_code_generator
    );

    opcode_code_generator
}
