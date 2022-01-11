use clap::{Arg, App, ArgMatches};
use std::path::{Path, PathBuf}; 

use std::process::exit;

fn getfp(opt: &str, matches: &ArgMatches) -> PathBuf {
    let file = Path::new(matches.value_of(opt).unwrap_or(""));
    if file.as_os_str() == "" { exit(1); }
    file.to_path_buf()
}

fn main() {
    let matches = App::new("Optim")
        .version("0.1.0")
        .author("Joseph Diza")
        .about("Precisely optimizes the sizes of various images")
        .arg(Arg::new("input").help("Input image file path"))
        .arg(Arg::new("output").help("Output image file path"))
        .get_matches();
    let input: PathBuf = getfp("input", &matches);
    let output: PathBuf = getfp("input", &matches);

    println!("Input {}\nOutput {}", input.display(), output.display());
}

