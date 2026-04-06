mod codegen;
mod convert;

use anyhow::Context;
use clap::{Parser, command};
use std::{
    fs::{self, DirEntry},
    path::{Path, PathBuf},
};
use tiled::Loader;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(required = true)]
    input_file: PathBuf,
    #[arg(required = true, short, long)]
    output_directory: PathBuf,
}

fn main() {
    let args = Args::parse();

    if let Err(e) = cli(&args) {
        eprintln!("Error: {:#}", e);
    }
}

fn cli(args: &Args) -> anyhow::Result<()> {
    // Disabled until we need to actually tackle cross-map references
    // let tmx = gather_tmx(args.input_directory.clone())?;

    let mut loader = Loader::new();
    let map = loader.load_tmx_map(&args.input_file)?;

    fs::create_dir_all(args.output_directory.clone())?;

    let source_name = map
        .source
        .file_name()
        .context("Failed to retrieve file name from map source")?;
    let output_path = args.output_directory.join(source_name);

    let (header_ast, src_ast) = convert::build_ast(&map)?;

    let header = codegen::generate(header_ast);
    std::fs::write(output_path.with_extension("h"), header)?;

    let src = codegen::generate(src_ast);
    std::fs::write(output_path.with_extension("c"), src)?;

    Ok(())
}

fn _gather_tmx<P>(from: P) -> anyhow::Result<Vec<DirEntry>>
where
    P: AsRef<Path>,
{
    Ok(fs::read_dir(from)?
        .filter_map(|result| result.ok())
        .filter(|entry| entry.path().extension().is_some_and(|ext| ext == "tmx"))
        .collect::<Vec<_>>())
}
