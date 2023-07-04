#![warn(
    clippy::all,
    clippy::style,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::pedantic
)]

use clap::Parser;
use reservoir_sampling::unweighted::l as sample;
use std::{
    error::Error,
    fs::{self, read_dir},
    path::PathBuf,
};

/// Randomly samples the top-level subdirectories of a given directory, and places the results in an output directory.
#[derive(Parser, Debug)]
struct Args {
    /// The directory containing subdirectories to select from.
    #[arg(short = 'd', long = "dir")]
    in_dir: PathBuf,

    /// The maximum number of files to copy from each subdirectory.
    #[arg(short = 'n', long = "num")]
    number: usize,

    /// Where to build the output directory.
    #[arg(short = 'o', long = "out")]
    out_dir: PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    // gather list of subdirectories
    let subdirectories: Vec<PathBuf> = read_dir(args.in_dir.clone())?
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.is_dir())
        .collect();

    // get the list of files to copy
    let files: Vec<PathBuf> = subdirectories
        .iter()
        .flat_map(|dir| {
            // read files in subdirec
            let files = read_dir(dir)
                .unwrap()
                .map(|entry| entry.unwrap().path())
                .filter(|path| path.is_file());

            let mut sampled_files = vec![PathBuf::new(); args.number];

            sample(files, &mut sampled_files);
            sampled_files
        })
        .collect();

    // create the output directories
    subdirectories.iter().for_each(|in_subdir| {
        let out_subdir = args
            .out_dir
            .join(in_subdir.strip_prefix(&args.in_dir).unwrap());
        fs::create_dir_all(out_subdir).unwrap();
    });

    // copy files to output directory
    files.iter().for_each(|in_path| {
        let out_path = args
            .out_dir
            .join(in_path.strip_prefix(&args.in_dir).unwrap());
        let _ = fs::copy(in_path, out_path).unwrap();
    });

    Ok(())
}
