#![warn(
    clippy::all,
    clippy::style,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::pedantic
)]

use clap::Parser;
use reservoir_sampling::unweighted::l as sample;
use std::{error::Error, fs, io, path::PathBuf};

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
    let subdirectories = read_subdirs(&args.in_dir)
        .map_err(|e| format!("Could not read directory '{}': {e}", args.in_dir.display()))?;

    // get the list of files to copy
    let files: Vec<PathBuf> = subdirectories
        .iter()
        .flat_map(|dir| {
            // read files in subdirec
            let files = fs::read_dir(dir)
                .unwrap()
                .map(|entry| entry.unwrap().path())
                .filter(|path| path.is_file());

            let mut sampled_files = vec![PathBuf::new(); args.number];

            sample(files, &mut sampled_files);
            sampled_files
        })
        .collect();

    // create the output directories
    for in_subdir in &subdirectories {
        let out_subdir = args
            .out_dir
            .join(in_subdir.strip_prefix(&args.in_dir).unwrap());
        fs::create_dir_all(out_subdir).unwrap();
    }

    // copy files to output directory
    for in_path in &files {
        let out_path = args
            .out_dir
            .join(in_path.strip_prefix(&args.in_dir).unwrap());
        let _ = fs::copy(in_path, out_path).unwrap();
    }

    Ok(())
}

fn read_subdirs(dir: &PathBuf) -> io::Result<Vec<PathBuf>> {
    let dirs = fs::read_dir(dir)?
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .map(|entry| entry.path())
        .filter(|path| path.is_dir())
        .collect();
    Ok(dirs)
}
