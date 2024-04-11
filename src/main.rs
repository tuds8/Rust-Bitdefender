use std::{
    ffi::OsStr,
    fs::File,
    io::{Read, Seek, Write},
};
use zip::result::ZipError;
use serde::Serialize;
use serde::Deserialize;
// use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(Serialize, Deserialize)]
struct FileData {
    name:String,
    filename: Vec<String>,
}

fn serialize_to<W: Write, T: ?Sized + Serialize>(mut writer: W, value: &T) -> Result<(), std::io::Error> {
    serde_json::to_writer(&mut writer, value)?;
    writer.write_all(b"\n")
}

/// Parametrul de tipul `impl Read + Seek` se numește "argument position impl trait" (APIT)
/// o formulare echivalentă ar fi `fn list_zip_contents<T: Read + Seek>(reader: T)`
/// `Read` și `Seek` sunt traits, care sunt oarecum similare cu interfețele din Java
///   o diferență este că traiturile nu sunt declarate direct de structuri (cum e în java `class C implements I`),
///   ci se pot declara separat: `impl Trait for Struct`
/// de asemenea generics în Rust diferă de cele din Java prin faptul că sunt monomorfice,
///   adică la compilare pentru o funcție generică se generează implementări separate pentru fiecare instanțiere cu argumente de tipuri diferite
///   (asta le aseamănă mai mult cu templates din C++)
/// https://doc.rust-lang.org/book/ch10-00-generics.html
///
/// deci practic lui `list_zip_contents` trebuie să-i dăm ca arugment o valoare al cărei tip implementează `Read` și `Seek`
///   un exemplu e `std::fs::File` (ar mai fi de exemplu `std::io::Cursor` cu care putem folosi un buffer din memorie)
fn list_zip_contents(reader: impl Read + Seek) -> Result<Vec<String>, ZipError> {
    let mut zip = zip::ZipArchive::new(reader)?;
    let mut filevector: Vec<String> = Vec::new();
    for i in 0..zip.len() {
        let file = zip.by_index(i)?;
        let filename: String = file.name().to_string();
        filevector.push(filename);
        // println!("\tFilename: {}", file.name());
    }
    return Ok(filevector);

}

/// La `Box<dyn std::error::Error>` vedem o altă utilizare a traiturilor, de data asta sub formă de "trait objects".
/// Obiectele de tipul `dyn Trait` sunt un fel de pointeri polimorfici la structuri care implementează `Trait`.
/// Din nou putem face o paralelă la Java sau C++, unde o variabilă de tipul `Error e` poate să referențieze o
///   instanță a orcărei clase care implementează interfața (sau extinde clasa de bază) `Error`.
///
/// Valorile de typ `dyn Trait` trebuie mereu să fie în spatele unei referințe: `Box<dyn Trait>`, `&dyn Trait`, `&mut dyn Trait`, etc,
///  asta e pentru că nu știm exact ce obiect e în spatele pointerului și ce size are (se zice că trait objects sunt `unsized types`)
///
/// https://doc.rust-lang.org/book/ch17-02-trait-objects.html
///
/// `Box<dyn std::error::Error>` e util ca tip de eroare fiindcă în principiu toate erorile în Rust implementează `std::error::Error`
///   deci se pot converti implicit la `Box<dyn std::error::Error>` (ceea ce se întâmplă când folosim operatorul `?` de propagare).
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();

    let dir = &args[1];
    let mut zip_files: Vec<FileData> = Vec::new();
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        
        let filename: String = path
            .file_name()
            .unwrap()
            .to_string_lossy()
            .to_string();


        if path.is_file() && path.extension() == Some(OsStr::new("zip")) {
            let file = File::open(&path)?;
            // println!("Contents of {:?}:", path);
            let fileslist = list_zip_contents(file)?;

            let fileData = FileData{
                name: filename,
                filename: fileslist,
            }; 
            zip_files.push(fileData);       
        } else {
            println!("Skipping {:?}", path);
        }
    }

    // println!("{:?}", serde_json::to_string(&zip_files));

    let mut outfile = File::create("./output/output.json")?;
    for elem in &zip_files {
        let _x = serialize_to(&mut outfile, elem);
    }

    let infile = File::open("./output/output.json")?;
    let reader = BufReader::new(infile);

    for line in reader.lines() {
        println!("{}", line?);
        print!("\n")
    }

    Ok(())
}
