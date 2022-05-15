use flate2::write::GzEncoder;
use flate2::Compression;
use std::env::args;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::path::PathBuf;

mod dir;

fn main() {
    if args().len() != 2 {
        eprintln!("Usage: ./compress_file `source`");
        return;
    }

    let source = args().nth(1).unwrap();

    let mut paths: Vec<PathBuf> = Vec::new();
    if Path::new(&source).is_file() {
        paths.push(PathBuf::from(source));
    } else {
        paths = dir::VisitDir::new(source)
            .unwrap()
            .filter_map(|e| Some(e.ok()?.path()))
            .collect::<Vec<_>>();
    }

    for p in paths.iter() {
        if Path::new(p).is_dir() {
            continue;
        }

        // Open file
        let mut file = File::open(p).expect("Failed to open file");

        // Print file size
        let original_size = file.metadata().unwrap().len();

        // Read file
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Failed to read file");

        // Gzip encode
        let output = Vec::new();
        let mut encoder = GzEncoder::new(output, Compression::default());
        encoder
            .write_all(contents.as_bytes())
            .expect("Error: write_all(contents.as_bytes())");
        let compressed_size = encoder.finish().unwrap().len();
        println!("> {:?} {} {}", p, original_size, compressed_size);
    }
}
