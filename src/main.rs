extern crate flate2;

use std::io::BufReader;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::env::args;
use std::fs::{File};
use std::io::copy;
use std::time::Instant;
use std::process::exit;

fn show_help() {
    println!("HELLLLLLLLLPPPPPPPPP")
}

fn show_info() {
    println!("ZI-ZU v1.0.0");
}

pub fn zipping(inp_file: String, target: String) {
    println!("");
    println!("------------------------");
    println!("inpup file: {:?}", inp_file);
    println!("output file: {:?}", target);

        let mut input: BufReader<File> = BufReader::new(File::open(inp_file).unwrap());
        let output: File = File::create(target).unwrap();
        let mut encoder: GzEncoder<File> = GzEncoder::new(output, Compression::default());
        let start:Instant = Instant::now();
        copy(&mut input, &mut encoder).unwrap();
        let output: File = encoder.finish().unwrap();
        println!("------------------------");
        println!(
            "Source len: {:?}",
            input.get_ref().metadata().unwrap().len()
        );
        println!("Target len: {:?}", output.metadata().unwrap().len());
        println!("Elapsed: {:?}", start.elapsed());
}

pub fn unzip() {

}

fn main() {
    let mut args = args();
    let mut input_path = String::new();

    loop {
        let Some(arg) = args.next() else {
            break;
        };
        match arg.as_str() {
            "-z" | "--zip" => {
                zipping(args.next().unwrap(), args.next().unwrap());
            }
            "-u" | "--unzip" => {
                println!("Unzipping");
            }
            "-h" | "--help" => {
                show_info();
                show_help();
                exit(0);
            }
            "-v" | "--version" => {
                show_info();
                exit(0);
            }
            _ => {
                if arg.starts_with('-') {
                    println!("Invalid option ({})!", arg);
                    exit(-1);
                } else {
                    input_path = arg;
                }
            }
        } 
    }
    if input_path.is_empty() {
        println!("Not enough arguments!\nFor help -h or --help");
        exit(-1);
    }
    
}