use std::{
    path::{Path, PathBuf},
    process,
};

use crate::compiler;
use clap::Parser;

fn parse_source_path(s: &str) -> Result<Vec<PathBuf>, String> {
    let mut source_dirs: Vec<PathBuf> = Vec::new();
    for path in s.split(":") {
        let sp = Path::new(path);
        if sp.is_dir() {
            source_dirs.push(sp.to_path_buf())
        } else {
            eprintln!("Warning: source path entry {} is not a directory", path)
        }
    }
    Ok(source_dirs)
}

/// Compile a collection of schism source files.
#[derive(Parser, Debug)]
#[command(version, name="compile", about, long_about = None )]
pub struct CompileCommandArgs {
    // A colon-separated list of directories where module sources can be found.
    #[arg(short = 'p', long, value_parser = parse_source_path)]
    pub source_path: std::vec::Vec<PathBuf>,

    /// The .schism files to be compiled.
    pub sources: Vec<String>,
}

pub fn run_compiler() {
    let args = CompileCommandArgs::parse();

    if args.sources.len() == 0 {
        eprintln!("Error: at least one source file must be supplied to the compile command");
        process::exit(1);
    }

    println!(
        "Starting compilation:\n\tSourcePath={:?}\n\tSources to compile={:?}",
        args.source_path, args.sources
    );
    let mut comp = compiler::Compiler::new(args.source_path);
    let result = comp.compile(args.sources);
    match result {
        Ok(_) => println!("Compilation was successful"),
        Err(err) => println!("Compilation failed with error: {}", err),
    }
}
