extern crate flate2;

use std::io;
use std::io::BufReader;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::env::args;
use std::fs::{File, self};
use std::io::copy;
use std::time::Instant;

//TODO: help function

pub fn ziping(inp_file: String, target: String) {
    println!("");
    println!("------------------------");
    println!("inpup files: {:?}", inp_file);
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
        println!("");
}


pub fn unzip() {
    let unzip_args: Vec<_> = std::env::args().collect();
    println!("{:?}", unzip_args); //debug
    let unzip_name: &std::path::Path = std::path::Path::new(&*unzip_args[2]);
    let file_name = fs::File::open(&unzip_name).unwrap();

    let mut arc = zip::ZipArchive::new(file_name).unwrap();

    for item in 0..arc.len() {
        let mut arc_file = arc.by_index(item).unwrap();

        let arc_out = match arc_file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };
        {
            let comment = arc_file.comment();
            if !comment.is_empty() {
                println!("File: {:?} Comment: {:?} ", item, comment);
            }
        }

        if(*arc_file.name()).ends_with('/') {
            print!("File {:?} Extracted to \"{:?}\"", item, arc_out.display());
            fs::create_dir_all(&arc_out).unwrap();
        }
        else {
            print!("File {:?} Extracted to \"{:?}\" ({:?}) bytes", item, arc_out.display(), arc_file.size());
        
            if let Some(p) = arc_out.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p).unwrap();
                }
            }
            let mut unzip_file = fs::File::create(&arc_out).unwrap();
            io::copy(&mut arc_file, &mut unzip_file).unwrap();
        }
    }
    #[cfg(Unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if let Some(mode) = file.unix_mode() {
            fs::set_permissions(&arc_out, fs::Permissions::from_mode(mode)).unwrap;
        }
    }

}
fn main() {

    if args().len() < 4 {
        eprintln!("Shape of command: `Work` `Source` `Target` or try --help for more information");
        return;
    }

    let args: Vec<String> = args().collect();

    //let file_path: &String = &args[0];
    let work: &String = &args[1];
    let inp_file: &String = &args[2];
    let target_file: &String = &args[args.len()-1];
    
    // println!("{}", file_path);
    // println!("{}", work);
    // println!("{}", file_name);
    // println!("{}", target_file);

    if work == "zip" {
        ziping(inp_file.to_string(),target_file.to_string());
    }
    else if work == "unzip" {
        unzip();
    }
    else {
        println!("Unsupported command {:?}", work);
        //TODO: println!("Try --help for more information");
    }

}
