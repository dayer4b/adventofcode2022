use std::io::BufRead;
use log::info;
use adventofcode2022::*;

pub fn day1(args: Args) {

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

    info!("top elf total: {:?}",all_elf_totals.get(elf_quantity - 1).expect("there should be an integer here"));
    info!("top three elf totals: {:?}", top_three_elf_totals);


}


