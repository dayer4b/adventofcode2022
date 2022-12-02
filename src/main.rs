use adventofcode2022::*;
use clap::Parser;

mod day2;
mod day1;

pub use crate::day2::*;
pub use crate::day1::*;

fn main() {

    let args = Args::parse();
    dbg!(&args);

    select(args.clone());
}

fn select(args: Args) {
    match args.day {

        1 => day1(args.clone()),
        2 => day2(args.clone()),
        _ => println!("bad value for day"),

    }
}

