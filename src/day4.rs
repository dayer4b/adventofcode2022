use std::io::BufRead;
use log::debug;
use log::info;
use std::ops::RangeInclusive;
use adventofcode2022::*;


fn segment_to_range(segment: &str) -> RangeInclusive<i32> {
    let endpoints : Vec<&str> = segment.split("-").collect();
    let range_start = endpoints.get(0).expect("should be an integer").parse::<i32>().unwrap();
    let range_end = endpoints.get(1).expect("should be an integer").parse::<i32>().unwrap();

    RangeInclusive::new(range_start,range_end)
}

pub fn day4(args: Args) {

    let reader = get_input(args.clone());

    let mut containment_total:i32 = 0;
    let mut overlap_total:i32 = 0;

    for line in reader.lines() {

        let line = line.unwrap();

        let segments: Vec<&str> = line.split(",").collect();

        let range_one = segment_to_range(segments.get(0).expect("should be a string"));
        let range_two = segment_to_range(segments.get(1).expect("should be a string"));

        debug!("Range one: {} - {}", range_one.start(), range_one.end());
        debug!("Range two: {} - {}", range_two.start(), range_two.end());

        // look for fully contained ranges
        if range_one.start() >= range_two.start() && range_one.end() <= range_two.end() {
            containment_total += 1;
            debug!("range one is fully contained in range two!");
        } else if range_two.start() >= range_one.start() && range_two.end() <= range_one.end() {
            containment_total += 1;
            debug!("range two is fully contained in range one!");
        }

        // part 2, look for overlapping ranges
        for assignment in range_two {
            if range_one.contains(&assignment) {
                overlap_total += 1;
                debug!("this assignment pair has ranges that overlap!");
                break;
            }
        }

    }

    info!("The total pairs with fully contained ranges is: {}", containment_total);
    info!("The total pairs with any overlapping ranges is: {}", overlap_total);
}
