use adventofcode2022::*;
use clap::Parser;
use crate::day1::*;
use crate::day2::*;
use crate::day3::*;

mod day1;
mod day2;
mod day3;

fn main() {

    env_logger::init();
    let args = Args::parse();
    dbg!(&args);

    select(args.clone());
}

fn select(args: Args) {
    match args.day {

        1 => day1(args.clone()),
        2 => day2(args.clone()),
        3 => day3(args.clone()),
        _ => println!("bad value for day"),

    }
}

