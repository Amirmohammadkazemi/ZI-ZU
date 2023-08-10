extern crate flate2;

use std::io::BufReader;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::env::args;
use std::fs::File;
use std::io::copy;
use std::time::Instant;

//TODO: help function

//TODO: handle ```zip<Input: BufRead, E>(mut input: &mut Input) -> Result<(), E>{``` error while using zip function in main
pub fn zip() {
        let mut input: BufReader<File> = BufReader::new(File::open(args().nth(2).unwrap()).unwrap());
        let output: File = File::create(args().nth(3).unwrap()).unwrap();
        let mut encoder: GzEncoder<File> = GzEncoder::new(output, Compression::default());
        let start:Instant = Instant::now();
        copy(&mut input, &mut encoder).unwrap();
        let output: File = encoder.finish().unwrap();
        println!(
            "Source len: {:?}",
            input.get_ref().metadata().unwrap().len()
        );
        println!("Target len: {:?}", output.metadata().unwrap().len());
        println!("Elapsed: {:?}", start.elapsed());
}

//pub fn unzip() {

//}

fn main() {

    if args().len() != 4 {
        eprintln!("Shape of command: `Work` `Source` `Target` or try --help for more information");
        return;
    }

    let args: Vec<String> = args().collect();
    //let file_path: &String = &args[0];
    let work: &String = &args[1];
    //let file_name: &String = &args[2];
    //let target_file: &String = &args[3];
    
    // println!("{}", file_path);
    // println!("{}", work);
    // println!("{}", file_name);
    // println!("{}", target_file);

    if work == "zip" {
        zip();
    }
    else {
        println!("Unsupported command {:?}", work);
        //TODO: println!("Try --help for more information");
    }
    
}

//TODO: path not found error handling