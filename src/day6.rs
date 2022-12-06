use std::io::BufRead;
use log::info;
use adventofcode2022::*;


fn unique_sequence(size: usize, chars: &Vec<char>) {
    let slices = chars.windows(size);

    for (iteration, item) in slices.enumerate() {
        let mut item_vec: Vec<char> = item.to_vec();
        item_vec.sort();
        item_vec.dedup();
        if item_vec.len() == size {
            info!("{} unique characters found complete after {} characters", item_vec.len(), iteration + size);
            break;
        }
    }
}

pub fn day6(args: Args) {

    let reader = get_input(args.clone());

    for line in reader.lines() {
        let line = line.unwrap();
        let chars: Vec<char> = line.chars().collect();
        unique_sequence(4, &chars);
        unique_sequence(14, &chars);
    }
}
