extern crate core;

use std::{env, io};
use std::fs;
use std::io::{BufRead, Write};
use std::path::Path;
use regex::Regex;
use lazy_static::lazy_static;

fn main() {
    let args = env::args().nth(1);
    match args {
        Some(path) => {
            println!("Cleaning MANIFEST files in {}", path);
            let p = Path::new(&path);
            match cleanup(&p) {
                Ok(replaced_files) =>
                    if replaced_files.len() == 0 {
                        println!("No matching MANIFEST.MF files found, nothing done.")
                    } else {
                        println!("Replaced :");
                        for f in replaced_files {
                            println!(" * {}", f)
                        }
                    }
                ,
                Err(e) => println!("Error : {}", e)
            }
        },
        None => println!("No args supplied. Please provide a path.")
    }
}

fn cleanup(path: &Path) -> io::Result<Vec<String>> {
    let is_dir = fs::metadata(path).map(|md| md.is_dir())?;
    if is_dir {
        let children = fs::read_dir(path)?;
        let mut res = Vec::new();
        for child in children {
            let path = child.map(|c| c.path())?;
            let mut handled_files = cleanup(path.as_path())?;
            res.append(&mut handled_files);
        }
        Ok(res)
    } else if path
        .file_name()
        .and_then(|name| name.to_str())
        .map(|name| name.ends_with("MANIFEST.MF"))
        .unwrap_or(false) {

        let res = cleanup_file(path)?;
        if res {
            Ok(
                path.to_str()
                    .map(String::from)
                    .map(|s| vec![s])
                    .unwrap_or(Vec::new())
            )
        } else {
            Ok(Vec::new())
        }
    } else {
        Ok(Vec::new())
    }
}

lazy_static! {
    static ref RE: Regex = Regex::new(r"^.*:.*$").unwrap();
}

fn cleanup_file(path: &Path) -> io::Result<bool> {
    let file = fs::File::open(path)?;
    let lines = io::BufReader::new(&file).lines();
    let mut write = true;
    let mut res = String::new();
    let mut touched = false;
    for line in lines {
        let ln = line?;
        if ln.starts_with("copyright:") {
            write = false;
            touched = true;
        } else if RE.is_match(&ln) {
            write = true;
        }
        if write {
            res.push_str(&ln);
            res.push('\n');
        }
    }
    if touched {
        let mut write_file = fs::File::create(path)?;
        write_file.write_all(res.as_bytes())?;
    }
    Ok(touched)
}