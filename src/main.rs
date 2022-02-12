use clap::Parser;

mod block;
mod cli;
mod clock;
mod day;
mod fmt;
mod infer;
mod month;
mod parse;

use cli::Args;

fn main() {
    let args = Args::parse();
    clock::punch(&args);
}
