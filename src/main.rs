//use std::fs;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
// use std::io::BufferedReader;
// use std::io::{stdout, Write};

fn main() {
    /*
    let input_data = fs::read_to_string("inputs/day1/part1.txt")
        .expect("Should have been able to read the file");
    */

    let file = File::open("inputs/day1/part1.txt").expect("Cannot open input file");
    let reader = BufReader::new(file);

    // let mut all_elves : Vec::<Vec> = Vec::new();
    // let mut current_elf : Vec<str> = Vec::new();

    let mut all_elves = Vec::new();
    let mut current_elf: Vec<u32> = Vec::new();

    for line in reader.lines() {
        // println!("{}", line.expect("Cannot read line from buffer."));
        if line.as_ref().expect("equality fail").eq("") {
            // this line is empty, new vector please
            // current_elf = vec![];
            // println!("this is a blank line")

            all_elves.push(current_elf);
            current_elf = Vec::new();
        } else {
            let thisline = format!("{}",line.expect("this should be a string"));
            current_elf.push(thisline.parse::<u32>().unwrap());

            // println!("{}", line.expect("Cannot read line from buffer."));
        }
    }

    let mut peak_calories = 0;

    for elf in all_elves {
        // println!("this is an elf");

        let mut total: u32 = 0;

        for i in elf.iter() {
            total += i;
        }

        if total > peak_calories {
            peak_calories = total
        }

    }

    println!("{}", peak_calories);

//    println!("Input data:\n{input_data}");
}

