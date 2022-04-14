extern crate oxipng;
use clap::{Arg, Command, ArgMatches};
use log::{debug, error, info};
use oxipng::PngError;
use std::path::{Path, PathBuf}; 
use std::process::exit;

/// Parse the cli args for a filepath
fn getfp(opt: &str, matches: &ArgMatches) -> PathBuf {
    let file = Path::new(matches.value_of(opt).unwrap_or(""));
    if file.as_os_str() == "" {
        error!("Input file paths cannot be null");
        exit(1);
    }
    file.to_path_buf()
}

/// Create the command line interface
fn build_cli() -> Command<'static> {
    let app = Command::new("Optim")
        .version("0.1.0")
        .author("Joseph Diza <josephm.diza@gmail.com>")
        .about("Precisely optimizes the sizes of various images")
        .arg(Arg::new("input")
            .help("Input image file path"))
        .arg(Arg::new("output")
            .help("Output image file path"))
        .arg(Arg::new("optimizer")
            .short('o')
            .long("optimizer")
            .default_value("oxipng")
            .help("Optimizer to use. Optimizers available: [oxipng]"));
    app
}

/// Optimize the file size of the png image using oxipng
fn oxipng_optimize(input: PathBuf, output: PathBuf) -> Result<(), PngError> {
    let infile = oxipng::InFile::Path(input);
    let outfile = oxipng::OutFile::Path(Some(output));
    let options = oxipng::Options::from_preset(4);
    let png = oxipng::optimize(&infile, &outfile, &options);
    png
}

fn main() {
    pretty_env_logger::init();
    let matches = build_cli().get_matches();

    let input: PathBuf = getfp("input", &matches);
    let output: PathBuf = getfp("output", &matches);
    let optimizer = matches.value_of("optimizer").unwrap();

    debug!("Input {}\nOutput {}\nOptimizer {}", input.display(), output.display(), optimizer);

    match optimizer {
        "oxipng" => {
            let png = oxipng_optimize(input, output);
            info!("{:?}\n", png);
        }
        _ => {
            error!("Unable to determine optimizer");
        }
    }

}

