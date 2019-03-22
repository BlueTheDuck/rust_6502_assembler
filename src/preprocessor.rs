#[macro_use]
extern crate argument_parser;
extern crate regex;

mod opcodes;
mod settings;
mod token;

use settings::Settings;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Write};

fn exmain() {
    let settings: settings::Settings = {
        let mut args: Vec<String> = {
            let mut args = std::env::args();
            args.next();
            let mut args: Vec<String> = args.collect();
            args.reverse();
            args
        };
        CreateSettings!(Arguments, infile, outfile);
        let arguments: Arguments = Arguments::from(args);
        Settings {
            infile: (arguments
                .infile
                .get(0)
                .map_or_else(|| String::from("input/basic_test.rs"), |x| x.clone())),
            outfile: (arguments
                .outfile
                .get(0)
                .map_or_else(|| String::from("input/basic_test.rs"), |x| x.clone())),
        }
    };
    println!("Processing: {} => {}", &settings.infile, &settings.outfile);

    //#region Initialize vars
    let mut items: Vec<String> = {
        let mut items = vec![];
        let reader =
            BufReader::new(File::open(&settings.infile).expect("Couldn't open input file"));
        for line in reader.lines() {
            let line: String = line.unwrap().trim().to_string();
            for s in line.split_whitespace() {
                items.push(s.to_string());
            }
        }
        items.reverse();
        items
    };
    let mut writer = BufWriter::new(
        OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(&settings.outfile)
            .expect("Couldn't open output file"),
    );
    //#endregion

    let mut address: u16 = 0x0000;
    while items.len() > 0 {
        let item = items.pop().unwrap();
        let opcode_found = opcodes::find(|o| return o.name == item);
        let out = format!(
            "{:04X}: {}\n",
            address,
            opcode_found.map_or(item, |x| format!("{:X}", x.code))
        );
        write!(writer, "{}", out);
        if let Some(op) = opcode_found {
            for i in 1..op.size {
                let out = format!("{:04X}: {}\n", address + i, items.pop().unwrap());
                write!(writer, "{}", out);
            }
        }
        address += opcode_found.map_or(0u16, |x| x.size);
    }
    //write!(writer,"{:X?}",b"Hola").expect("Couldn't write to file");
}
