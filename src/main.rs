mod codegen;
mod convert;

use clap::{Parser, command};
use std::{
    fs::{self, DirEntry},
    path::{Path, PathBuf},
};
use tiled::{Loader, Map};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(required = true)]
    input_directory: PathBuf,
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
    let mut loader = Loader::new();
    let tmx = gather_tmx(args.input_directory.clone())?;
    let maps_result: Result<Vec<Map>, _> =
        tmx.iter().map(|f| loader.load_tmx_map(f.path())).collect();
    let maps = maps_result?;

    for map in maps.iter() {
        let header_ast = convert::generate_header(map)?;
        let header = codegen::generate(header_ast);
        println!("{}", header);

        let src_ast = convert::generate_src(map)?;
        let src = codegen::generate(src_ast);
        println!("{}", src);
    }

    // fs::create_dir_all(args.output_directory.clone())?;

    Ok(())
}

fn gather_tmx<P>(from: P) -> anyhow::Result<Vec<DirEntry>>
where
    P: AsRef<Path>,
{
    Ok(fs::read_dir(from)?
        .filter_map(|result| result.ok())
        .filter(|entry| entry.path().extension().is_some_and(|ext| ext == "tmx"))
        .collect::<Vec<_>>())
}
