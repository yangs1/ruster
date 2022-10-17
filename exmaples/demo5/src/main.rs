mod opt;
mod service;
use clap::Parser;
use opt::Opts;
mod error;
mod pb;
mod storage;

fn main() {
    let args = Opts::parse();

    println!("{:?}", args);
}
