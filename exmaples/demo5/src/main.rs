mod opt;
mod service;
use clap::Parser;
use opt::Opts;
mod pb;
mod storage;
mod error;

fn main() {
    let args = Opts::parse();

    println!("{:?}", args);
}
