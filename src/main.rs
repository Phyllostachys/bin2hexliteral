use clap::Parser;

use std::io;
use std::io::Read;
use std::{fs, path::PathBuf};

/// Args for input file
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// File to process
    #[arg(short, long)]
    file: PathBuf,

    /// Amount of hex literals to output on each line
    #[arg(short, long, default_value = "16")]
    output_length: usize,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let mut f = fs::File::open(args.file)?;
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer)?;

    for line in buffer.chunks(args.output_length) {
        for byte in line {
            print!("0x{:02x}, ", byte);
        }
        println!();
    }

    Ok(())
}
