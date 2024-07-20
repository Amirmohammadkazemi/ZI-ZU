extern crate flate2;
use std::env::args;
use std::process::exit;


fn show_help() {
    println!("HELLLLLLLLLPPPPPPPPP")
}

fn show_info() {
    println!("ZI-ZU v1.0.0");
}

fn zipping(){
    println!("Zipping");
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
                zipping();
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