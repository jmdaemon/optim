// Third Party Crates
extern crate oxipng;
use oxipng::PngError;
use wasm_bindgen::prelude::*;

// Standard Library
use std::path::PathBuf;

//#[wasm_bindgen]
/// Optimize the file size of the png image using oxipng
pub fn oxipng_optimize(input: PathBuf, output: PathBuf) -> Result<(), PngError> {
    let infile = oxipng::InFile::Path(input);
    let outfile = oxipng::OutFile::Path(Some(output));
    let options = oxipng::Options::from_preset(4);
    let png = oxipng::optimize(&infile, &outfile, &options);
    png
}
