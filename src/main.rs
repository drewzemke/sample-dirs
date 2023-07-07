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
    fs,
    path::{Path, PathBuf},
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
    let subdirectories = read_dir(&args.in_dir, &ReadTarget::Dirs)?;

    // get the list of files to copy
    let files = subdirectories
        .iter()
        .map(|dir| {
            // read files in subdirec
            let files = read_dir(dir, &ReadTarget::Files)?;

            let mut sampled_files = vec![PathBuf::new(); args.number];
            sample(files.into_iter(), &mut sampled_files);
            Ok(sampled_files)
        })
        .collect::<std::result::Result<Vec<Vec<PathBuf>>, String>>()?
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();

    // create the output directories
    for in_subdir in &subdirectories {
        let out_subdir = args
            .out_dir
            .join(in_subdir.strip_prefix(&args.in_dir).unwrap());
        fs::create_dir_all(out_subdir).unwrap();
    }

    // copy files to output directory
    for in_path in files.iter() {
        let out_path = args
            .out_dir
            .join(in_path.strip_prefix(&args.in_dir).unwrap());
        let _ = fs::copy(in_path, out_path).unwrap();
    }

    Ok(())
}

enum ReadTarget {
    Files,
    Dirs,
}
fn read_dir(dir: &PathBuf, target: &ReadTarget) -> Result<Vec<PathBuf>, String> {
    let filter = match target {
        ReadTarget::Files => Path::is_file,
        ReadTarget::Dirs => Path::is_dir,
    };
    let dirs = fs::read_dir(dir)
        .map_err(|e| format!("Could not read directory '{}': {e}", dir.display()))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("Could not read directory '{}': {e}", dir.display()))?
        .into_iter()
        .map(|entry| entry.path())
        .filter(|path| filter(path))
        .collect();
    Ok(dirs)
}
