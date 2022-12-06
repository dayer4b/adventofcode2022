use std::io::BufRead;
use log::info;
use adventofcode2022::*;


pub fn day6(args: Args) {

    let reader = get_input(args.clone());

    for line in reader.lines() {

        let line = line.unwrap();

        let chars: Vec<char> = line.chars().collect();
        let starting_slices = chars.windows(4);
        let ending_slices = chars.windows(14);

        for (iteration, item) in starting_slices.enumerate() {
            let mut item_vec: Vec<char> = item.to_vec();
            item_vec.sort();
            item_vec.dedup();
            if item_vec.len() == 4 {
                info!("{} unique characters found complete after {} characters", item_vec.len(), iteration + 4);
                break;
            }
        }

        for (iteration, item) in ending_slices.enumerate() {
            let mut item_vec: Vec<char> = item.to_vec();
            item_vec.sort();
            item_vec.dedup();
            if item_vec.len() == 14 {
                info!("{} unique characters found complete after {} characters", item_vec.len(), iteration + 14);
                break;
            }
        }


    }

}
