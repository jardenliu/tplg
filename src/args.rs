use clap::{Parser, command};

/// Simple program to generate files from templates
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Output directory path
    #[arg(short, long)]
    pub out: String,
}