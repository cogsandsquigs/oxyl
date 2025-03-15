use clap::Parser;
use oxylc::{
    compile::{lowering::CLowerer, parser},
    fst::{visitor::FstVisitor, File},
};
use std::{fs, path::PathBuf};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    file: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();
    let contents = fs::read_to_string(args.file)?;

    // NOTE: Error type can't be returned since it's dependent on the lifetime of `contents`.
    // Therefore we handle it here.
    let parsed: File = match parser::parse(&contents) {
        Ok(parsed) => parsed,
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    };
    let output = CLowerer.visit_file(&parsed);

    println!("{}", output);

    Ok(())
}
