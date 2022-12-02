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
        2 => day2(args.clone()),
        _ => println!("bad value for day"),

    }
}

fn day2_decode(code: &str, part: &u8) -> i32 {

    match part {

        1 => match code {
            "A X" => 1+3,
            "B X" => 1+0,
            "C X" => 1+6,

            "A Y" => 2+6,
            "B Y" => 2+3,
            "C Y" => 2+0,

            "A Z" => 3+0,
            "B Z" => 3+6,
            "C Z" => 3+3,
            _ => 0,
        },

        2 => match code {
            "A X" => 0+3,
            "B X" => 0+1,
            "C X" => 0+2,

            "A Y" => 3+1,
            "B Y" => 3+2,
            "C Y" => 3+3,

            "A Z" => 6+2,
            "B Z" => 6+3,
            "C Z" => 6+1,
            _ => 0,
        },

        _ => 0,
    }
}


fn day2(args: Args) {

    let reader = get_input(args.clone());

    let mut total:i32 = 0;

    for line in reader.lines() {
        let points = day2_decode(line.as_ref().expect("should be a string").as_ref(), &args.part);
        println!("{} scores {} points",line.expect("should be a string"), points);
        total += points;
    }

    println!("The total score is: {}", total);

}

fn get_input(args: Args) -> BufReader<File> {
    let mut file_path = format!("inputs/day{}/part1.txt", args.day);
    if args.sample {
        file_path = format!("inputs/day{}/sample.txt", args.day);
    }
    let file = File::open(file_path).expect("Cannot open input file");
    BufReader::new(file)
}

fn day1(args: Args) {

   let reader = get_input(args.clone());

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


