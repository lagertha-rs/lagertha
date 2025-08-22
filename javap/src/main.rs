use std::fs::File;
use std::io::Read;
use vm::{parse_bin_class, JvmError};

fn main() -> Result<(), JvmError> {
    let mut file = File::open("Main.class").expect("Can't open Main.class");
    let m = file.metadata().expect("Metadata err");
    let mut buf = Vec::with_capacity(m.len() as usize);
    file.read_to_end(&mut buf).expect("Problem with read");

    if let Err(err) = parse_bin_class(buf) {
        eprintln!("{err}");
        std::process::exit(1)
    }

    Ok(())
}
