use data_encoding::HEXUPPER;
use ring::digest::{Context, Digest, SHA256};
use std::fs::File;
use std::io::{BufReader, Read};
use dict::{ Dict, DictIface };
use walkdir::{DirEntry, WalkDir};
use clap::Parser;
use std::path::Path;
use std::ffi::OsStr;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path where the search of duplicate files should happen
    #[arg(short, long, default_value = ".")]
    path: String,

    /// Files are duplicate if they have same SHA256 and same name
    #[arg(short, long)]
    name: bool,

    /// If you want to skip hidden files
    #[arg(short, long)]
    skip_hidden: bool,

    /// Size of the file in MB to above which the files will be compared
    #[arg(short = 'z' , long, default_value_t = 0)]
    size: u64,
}

fn sha256_digest<R: Read>(mut reader: R) -> Result<Digest, std::io::Error> {
    let mut context = Context::new(&SHA256);
    let mut buffer = [0; 1024];

    loop {
        let count = reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        context.update(&buffer[..count]);
    }

    Ok(context.finish())
}

fn is_hidden (file: &DirEntry) -> bool {
    file.file_name().to_str().unwrap().starts_with(".") | file.path().to_str().unwrap().find(".git").is_some()
}

fn main() -> Result<(), std::io::Error> {

    let args = Args::parse();

    println!("Running with configuration: path: {}, skip_hidden: {}, size: {}, name: {}", args.path, args.skip_hidden, args.size, args.name);
    let mut dict = Dict::<String>::new();

    for p in WalkDir::new(args.path)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| !e.file_type().is_dir()) {
        if args.skip_hidden == true && is_hidden(&p) {
            println!("Skipping file: {}", p.path().to_string_lossy());
            continue;
        }
        let path = String::from(p.path().to_string_lossy());
        let file_name = String::from(p.file_name().to_string_lossy());
        let input = File::open(&path)?;
//        println!("size : {} args.size : {}", input.metadata().unwrap().len(), args.size * 1024 * 1024);
        if (args.size * 1024 * 1024) > input.metadata().unwrap().len() {
            continue;
        }
        let reader = BufReader::new(&input);
        let digest = sha256_digest(reader)?;
        let digest_str =  HEXUPPER.encode(digest.as_ref());
        if dict.contains_key(&digest_str) {
            let existing_path = dict.get(&digest_str);
            let existing_path1 = Path::new(existing_path.unwrap());
            let existing_file_name = existing_path1.file_stem();
            let file_name_os_str = OsStr::new(&file_name);
            if args.name {
                if file_name_os_str == existing_file_name.unwrap() {
                    println!("File {:?} is duplicate of {:?} size: {}MB", &path, existing_path.unwrap(), input.metadata().unwrap().len()/1024/1024);
                }
            } else {
                println!("File {:?} is duplicate of {:?}", &path, existing_path.unwrap());
            }

        } else {
            dict.add(digest_str, path);
        }
    }


    Ok(())
}
