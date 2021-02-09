extern crate bio;
use bio::utils::Text;
use clap::{App, Arg};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::result::Result;
use std::vec::Vec;

fn main() -> Result<(), std::io::Error> {
    let args = App::new("grabrust-these-contigs")
        .version("0.1")
        .author("Kevin Amses")
        .about("Grabs contigs from an indexed fasta by accession.")
        .arg(
            Arg::new("wanted")
                .short('w')
                .long("wanted")
                .value_name("FILE")
                .about("Path to file that lists sequence accessions to grab.")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::new("fasta")
                .long("fasta")
                .value_name("FILE")
                .about("Path to FASTA file to grab contigs from.")
                .takes_value(true)
                .required(true),
        )
        .get_matches();
    let wanted_path = Path::new(args.value_of("wanted").unwrap());
    let wanted_fobj = File::open(wanted_path).unwrap();
    let reader = BufReader::new(wanted_fobj);
    let mut wanted = Vec::new();
    for line in reader.lines() {
        match line {
            Ok(x) => wanted.push(x),
            Err(_e) => (),
        }
    }

    let fasta_path = Path::new(args.value_of("fasta").unwrap());
    let mut index_reader = bio::io::fasta::IndexedReader::from_file(&fasta_path).unwrap();

    for want in wanted.iter() {
        let mut text = Text::new();
        match index_reader.fetch_all(&want) {
            Ok(()) => {
                index_reader.read(&mut text).unwrap();
                println!(">{}\n{}", want, String::from_utf8(text).unwrap());
            }
            Err(_e) => panic!("Unable to locate contig: {}", &want),
        }
    }
    Ok(())
}