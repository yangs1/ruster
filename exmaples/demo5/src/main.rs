mod opt;
mod service;
use clap::Parser;
use opt::Opts;

fn main() {
    let args = Opts::parse();

    println!("{:?}", args);
}
