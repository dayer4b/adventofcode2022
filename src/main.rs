use std::fs::File;
use std::io::{prelude::*, BufReader};
use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
struct Args {
   /// Day of the AOC to run
   #[arg(short, long, default_value_t = 1)]
   day: u8,

   /// Part of the puzzle to run for given Day
   #[arg(short, long, default_value_t = 1)]
   part: u8,

   /// Use this flag to run with sample input
   #[arg(short, long, required = false)]
   sample: bool,
}


fn main() {

    let args = Args::parse();
    dbg!(&args);

    select(args.clone());
}

fn select(args: Args) {
    match args.day {

        1 => day1(args.clone()),
        _ => println!("bad value for day"),

    }
}


fn day1(args: Args) {

    let file_path = format!("inputs/day{}/part{}.txt", args.day, args.part);
    let file = File::open(file_path).expect("Cannot open input file");
    let reader = BufReader::new(file);

    let mut all_elves = Vec::new();
    let mut all_elf_totals = Vec::new();
    let mut current_elf: Vec<i32> = Vec::new();

    for line in reader.lines() {

        if line.as_ref().expect("string equality fail").eq("") {
            all_elves.push(current_elf);
            current_elf = Vec::new();
        } else {
            let thisline = format!("{}",line.expect("this should be a string"));
            current_elf.push(thisline.parse::<i32>().unwrap());
        }

    }


    for elf in all_elves {

        let mut total: i32 = 0;

        for i in elf.iter() {
            total += i;
        }

        all_elf_totals.push(total);

    }

    all_elf_totals.sort();

    let elf_quantity = all_elf_totals.len();

    let top_three_elf_totals = all_elf_totals.get(elf_quantity - 1).expect("there should be an integer here") +
        all_elf_totals.get(elf_quantity - 2).expect("there should be an integer here") +
        all_elf_totals.get(elf_quantity - 3).expect("there should be an integer here");

    println!("top elf total: {:?}",all_elf_totals.get(elf_quantity - 1).expect("there should be an integer here"));
    println!("top three elf totals: {:?}", top_three_elf_totals);


}


