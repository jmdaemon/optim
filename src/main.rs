use clap::{Arg, App, ArgMatches};
extern crate oxipng;
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
        .arg(Arg::new("optimizer").help("Optimizer to use"))
        .get_matches();

    let input: PathBuf = getfp("input", &matches);
    let output: PathBuf = getfp("output", &matches);
    let optimizer = matches.value_of("optimizer").unwrap_or("oxipng");

    println!("Input {}\nOutput {}\nOptimizer {}", input.display(), output.display(), optimizer);

    match optimizer {
        //"oxipng" => {
        _ => {
            let infile = oxipng::InFile::Path(input);
            let outfile = oxipng::OutFile::Path(Some(output));
            let options = oxipng::Options::from_preset(4);
            let png = oxipng::optimize(&infile, &outfile, &options);
            println!("{:?}\n", png);
        }
    }

}

