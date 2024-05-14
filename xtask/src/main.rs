#![warn(rust_2018_idioms)]

use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
enum Args {
    Coverage {
        #[arg(short = 'C', long)]
        core_include_dir: Option<PathBuf>,
        #[arg(short = 'I', long)]
        studio_include_dir: Option<PathBuf>,
        #[arg(short, long)]
        print: bool,
        #[arg(short, long)]
        verbose: bool,
    },
}

mod coverage;

fn main() {
    color_eyre::install().unwrap();

    let args = Args::parse();
    match args {
        Args::Coverage {
            core_include_dir,
            studio_include_dir,
            print,
            verbose,
        } => {
            let core_include_dir =
                core_include_dir.unwrap_or_else(|| PathBuf::from("fmod/api/core/inc"));
            let studio_include_dir =
                studio_include_dir.unwrap_or_else(|| PathBuf::from("fmod/api/studio/inc"));
            if let Err(e) = coverage::coverage(core_include_dir, studio_include_dir, print, verbose)
            {
                eprintln!("Error: {:?}", e);
            }
        }
    }
}
