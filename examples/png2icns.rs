extern crate icns;

use icns::{IconFamily, Image};
use std::env;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;

fn main() {
    if env::args().count() != 2 {
        println!("Usage: png2icns <path>");
        return;
    }
    let png_path = env::args().nth(1).unwrap();
    let png_path = Path::new(&png_path);
    let png_file = BufReader::new(File::open(png_path)
                                       .expect("failed to open PNG file"));
    let image = Image::read_png(png_file).expect("failed to read PNG file");
    let mut family = IconFamily::new();
    family.add_icon(&image).expect("failed to encode image");
    let icns_path = png_path.with_extension("icns");
    let icns_file = BufWriter::new(File::create(icns_path)
                                      .expect("failed to create ICNS file"));
    family.write(icns_file).expect("failed to write ICNS file");
}
