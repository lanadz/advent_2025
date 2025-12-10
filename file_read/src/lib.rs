use std::fs::File;
use std::env;
use std::io::{self, BufRead, BufReader, Lines};
use std::path::PathBuf;

fn input_root() -> PathBuf {
    env::var("CARGO_MANIFEST_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from(env!("CARGO_MANIFEST_DIR")))
}

fn input_path(filename: &str) -> PathBuf {
    input_root().join("src").join("input").join(filename)
}

fn file_open(file_name: &str) -> io::Result<File> {
    dbg!(input_path(file_name));
    File::open(input_path(file_name))
}

// usage: for line in read_to_buffer_lines("input.txt")? { ... }
pub fn read_to_buffer_lines(name: &str) -> io::Result<Lines<BufReader<File>>> {
    let file = file_open(name)?;
    Ok(BufReader::new(file).lines())
}
