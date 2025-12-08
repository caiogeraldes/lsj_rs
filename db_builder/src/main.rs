#[macro_use]
extern crate log;
extern crate pretty_env_logger;
use bincode;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, path::Path};

use tei_parser::parser::{Entry, parse_tei};

#[derive(Deserialize, Serialize)]
struct Entries(HashMap<String, Entry>);

impl Entries {
    fn new(value: HashMap<String, Entry>) -> Self {
        Entries(value)
    }
}

fn main() {
    pretty_env_logger::init();
    let mut entries: HashMap<String, Entry> = HashMap::new();
    for f in fs::read_dir("../assets/lsjlogeion").unwrap() {
        match f {
            Ok(f) => {
                if f.file_name().into_string().unwrap().ends_with(".xml")
                    && !f.file_name().into_string().unwrap().ends_with("01.xml")
                {
                    info!("Parsing {:?}", f.path());
                    let tei_file = fs::read_to_string(f.path()).unwrap();
                    for entry in parse_tei(&tei_file) {
                        entries.insert(entry.key.clone(), entry);
                    }
                }
            }
            Err(e) => panic!("{}", e),
        }
    }
    info!("Total entries: {}", entries.len());
    info!("Generating database");
    let config = bincode::config::standard();
    let output_file_path = Path::new("../assets/db");
    let mut output_file = fs::File::create_new(output_file_path).unwrap();
    match bincode::serde::encode_into_std_write(Entries::new(entries), &mut output_file, config) {
        Ok(e) => {
            info!("wrote {} bytes at {:?}", e, output_file_path);
        }
        Err(e) => panic!("{}", e),
    }
}
