use adventofcode2022::*;
use clap::Parser;
use crate::day1::*;
use crate::day2::*;
use crate::day3::*;
use crate::day4::*;
use crate::day5::*;
use crate::day6::*;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

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
        4 => day4(args.clone()),
        5 => day5(args.clone()),
        6 => day6(args.clone()),
        _ => println!("bad value for day"),

    }
}

