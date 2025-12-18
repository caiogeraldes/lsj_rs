use clap::Parser;
#[macro_use]
extern crate log;
use rand::seq::SliceRandom;
extern crate bincode;
extern crate pretty_env_logger;
use include_assets::{NamedArchive, include_dir};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tei_parser::parser::Entry;

mod query;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Entry being searched for
    pub(crate) text: Option<String>,

    /// Converts value to betacode.
    #[clap(short, long, action)]
    pub betacode: bool,

    /// Gets a random entry
    #[clap(short, long, action)]
    pub random: bool,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct Entries(HashMap<String, Entry>);

fn main() {
    pretty_env_logger::init();
    let args = Args::parse();

    if args.random && args.text.is_some() {
        warn!("Random and text both set, will ignore the random!");
    }

    let input_str: String = match args.text {
        Some(s) => match args.betacode {
            true => betacode::converter::revert(s),
            false => s,
        },
        None => match args.random {
            true => {
                let archive = NamedArchive::load(include_dir!("../assets"));
                let db = archive.get("db").unwrap();
                let config = bincode::config::standard();
                let entries: Entries = bincode::serde::decode_from_slice(db, config).unwrap().0;
                let mut keys: Vec<&String> = entries.0.keys().collect();
                let mut rng = rand::rng();
                keys.shuffle(&mut rng);
                keys[0].clone()
            }
            false => {
                eprintln! {"Empty string!"};
                std::process::exit(1)
            }
        },
    };

    let archive = NamedArchive::load(include_dir!("../assets"));
    let db = archive.get("db").unwrap();
    let config = bincode::config::standard();
    let entries: Entries = bincode::serde::decode_from_slice(db, config).unwrap().0;
    match query::query(input_str.clone(), &entries) {
        Some(entry) => println!("{}", entry),
        None => {
            warn!("Trying without diacritics");
            let db_no_dia = archive.get("db_no_dia").unwrap();
            let entries: Entries = bincode::serde::decode_from_slice(db_no_dia, config)
                .unwrap()
                .0;
            match query::query_no_diacritics(input_str.clone(), &entries) {
                Some(entry) => println!("{}", entry),
                None => {
                    for entry in entries.0.keys() {
                        println!("{}",entry);
                    }
                    eprintln!("Entry \"{}\" not found", input_str);
                    std::process::exit(1)
                }
            }
        }
    }
    std::process::exit(0);
}
