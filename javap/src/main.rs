use chrono::{DateTime, Local};
use jclass::ClassFile;
use jclass::attribute::class::ClassAttr;
use sha2::Digest;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

fn handle_jclass_arg() -> String {
    std::env::args()
        .nth(1)
        .expect("Please provide a class file as argument")
}

fn main() {
    let original_java_class_path = handle_jclass_arg();
    let resolved_path_str = original_java_class_path
        .trim_end_matches(".class")
        .replace('.', "/")
        + ".class";
    let resolved_path = PathBuf::from(resolved_path_str);
    let mut file = File::open(&resolved_path).unwrap_or_else(|_| {
        eprintln!("Error: class not found: {original_java_class_path}");
        std::process::exit(2);
    });

    let m = file.metadata().expect("Metadata err");
    let modified_time = m.modified().expect("Error getting modified time");
    let datetime: DateTime<Local> = modified_time.into();
    let formatted_date = datetime.format("%b %d, %Y").to_string();
    let file_size = m.len();

    let mut buf = Vec::with_capacity(m.len() as usize);
    file.read_to_end(&mut buf).expect("Problem with read");
    let class_hash = sha2::Sha256::digest(&buf)
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<String>();

    let class = ClassFile::try_from(buf).unwrap();
    let absolute_path = std::fs::canonicalize(&resolved_path).expect("Error getting absolute path");
    println!("Classfile {}", absolute_path.display());
    println!(
        "  Last modified {}; size {} bytes",
        formatted_date, file_size
    );
    println!("  SHA-256 checksum {}", class_hash);
    if let Some(ClassAttr::SourceFile(sourcefile_index)) = class
        .attributes
        .iter()
        .find(|v| matches!(v, ClassAttr::SourceFile(_)))
    {
        let source_file_name = class.cp.get_utf8(sourcefile_index).unwrap();
        println!("  Compiled from \"{}\"", source_file_name);
    }
    print!("{class}");
}
