use std::fs::File;
use std::io::{BufReader};
use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Args {
   /// Day of the AOC to run
   #[arg(short, long, default_value_t = 1)]
   pub day: u8,

   /// Part of the puzzle to run for given Day
   #[arg(short, long, default_value_t = 1)]
   pub part: u8,

   /// Use this flag to run with sample input
   #[arg(short, long, required = false)]
   pub sample: bool,
}


pub fn get_input(args: Args) -> BufReader<File> {
    let mut file_path = format!("inputs/day{}/part1.txt", args.day);
    if args.sample {
        file_path = format!("inputs/day{}/sample.txt", args.day);
    }
    let file = File::open(file_path).expect("Cannot open input file");
    BufReader::new(file)
}

