use std::env;

use structopt::StructOpt;

/* @see https://colobu.com/2019/09/29/rust-lib-per-week-structopt/  */
#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    #[warn(dead_code)]
    #[structopt(help="input number")]
    number : usize,
    #[structopt(help="output path")]
    output : Option<String>,
}

fn main() {

    println!("{:?}", Opt::from_args());

    let optObj = Opt::from_args();

    let number = optObj.number;

    fb_loop(number);

    return; 


    let args = env::args().skip(1).collect::<Vec<String>>();


    if args.len() != 1 {
        std::process::exit(-1);
    }
    let number = args[0].parse::<usize>().unwrap();
    fb_loop(number);
    // println!("Hello, world!");
}


fn fb_loop(n: usize) {
    let mut a = 1;
    let mut b = 1;
    let mut i = 2;

    loop {
        fib_tmp( &mut a,  &mut b);
        println!("next val is {}", b);
        i = i+1;
        if i > n {
            break;
        }
    }

}
fn fib_tmp(z: &mut usize,x: & mut usize)
{
    let c = *z + *x;
    *z = *x;
    *x = c;

}