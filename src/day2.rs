use std::io::BufRead;
use adventofcode2022::*;

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


pub fn day2(args: Args) {

    let reader = get_input(args.clone());

    let mut total:i32 = 0;

    for line in reader.lines() {
        let points = day2_decode(line.as_ref().expect("should be a string").as_ref(), &args.part);
        println!("{} scores {} points",line.expect("should be a string"), points);
        total += points;
    }

    println!("The total score is: {}", total);

}


