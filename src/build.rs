use std::{
    env,
    error::Error,
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, BufWriter, Seek, SeekFrom, Write},
    path::Path,
};

fn main() -> Result<(), Box<Error>> {
    let out_dir = env::var("OUT_DIR")?;
    let mut opcode_code_generator: String = get_code();

    let mut opmanager_code = include_str!("opcode_manager.rs");
    let mut opmanager_code_lines: Vec<&str> = opmanager_code.lines().collect();
    let mut opmanager_code_final: String = "".to_string();

    println!("Finding");

    let mut i = 0;
    let mut line: &str = opmanager_code_lines[i];
    let mut state = 0; //SEARCH SKIP END
    while i < opmanager_code_lines.len() {
        line = opmanager_code_lines[i];
        println!("<{}>", opmanager_code_lines[i]);
        if state == 0 {
            opmanager_code_final += &line;
            opmanager_code_final += "\n";
            if line == "    //#region Opcode-static-list" {
                println!("Found marker");
                state = 1;
                opmanager_code_final += &opcode_code_generator;
                opmanager_code_final += "\n";
                println!("Code: <<{}>>", opmanager_code_final);
            }
        }
        if state == 1 {
            if line == "    //#endregion" {
                state = 2;
            }
        }
        if state == 2 {
            opmanager_code_final += &line;
            opmanager_code_final += "\n";
        }
        i += 1;
    }

    std::fs::write("src/opcode_manager.rs", opmanager_code_final);

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
            "        create_opcode!(\"{}\", \"{}\", 0x{});\n",
            items[0], items[1], items[2]
        );
        opcode_code_generator += &code_line.clone();
    }

    opcode_code_generator = format!(
        "    static opcode_list: [Opcode; {}] = [\n{}    ];",
        opcodes_counter, opcode_code_generator
    );

    opcode_code_generator
}
