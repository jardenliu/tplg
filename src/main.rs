mod args;
mod utils;
mod generate;
use std::path::Path;
use args::Args;
use clap::Parser;

fn main() {
    let args: Args = Args::parse();
    let template_dir = args.template.as_deref().unwrap_or(".template");
    let template_dir = Path::new(template_dir);
    let output_dir = Path::new(&args.out);
    generate::generate_files(template_dir, output_dir);
}