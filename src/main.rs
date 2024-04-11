use std::fs::File;
use std::io::prelude::*;
use std::env;
use std::path::{Path, PathBuf};
use std::ffi::OsStr;
use std::fs;

fn get_extension_from_filename(filename: &PathBuf) -> Option<&str> {
    Path::new(filename)
        .extension()
        .and_then(OsStr::to_str)
}

fn list_zip_contents(reader: impl Read + Seek) -> zip::result::ZipResult<()> {
    let mut zip = zip::ZipArchive::new(reader)?;

    for i in 0..zip.len() {
        let file = zip.by_index(i)?;
        println!("Filename: {}", file.name());
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> { 
    let args: Vec<String> = env::args().collect();
    let paths = fs::read_dir(format!("./{}", args[1])).unwrap();

    for path in paths {
        let path = path.unwrap().path();
        let file = File::open(&path)?;
        if get_extension_from_filename(&path) == Some("zip") {
            list_zip_contents(file)?;
        }
        
    }
    
    Ok(())
}
