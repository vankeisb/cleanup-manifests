extern crate core;

use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let args = env::args().nth(1);
    match args {
        Some(path) => {
            let p = Path::new(&path);
            match cleanup(&p) {
                Ok(nb) => println!("{} replaced", nb),
                Err(e) => println!("Error : {}", e)
            }
        },
        None => println!("No args supplied. Please provide a path.")
    }
}

fn cleanup(path: &Path) -> std::io::Result<i32> {
    let is_dir = fs::metadata(path).map(|md| md.is_dir())?;
    if is_dir {
        let children = fs::read_dir(path)?;
        let mut res = 0;
        for child in children {
            let path = child.map(|c| c.path())?;
            let nb = cleanup(path.as_path())?;
            res += nb;
        }
        Ok(res)
    } else if path
        .file_name()
        .and_then(|name| name.to_str())
        .map(|name| name.ends_with("pom.xml"))
        .unwrap_or(false) {
        Ok(1)
    } else {
        Ok(0)
    }
}