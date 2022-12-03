use std::io::BufRead;
use log::debug;
use log::info;
use substring::Substring;
use adventofcode2022::*;

fn day3_select_formula(code: &str, part: &u8) -> i32 {
    match part {
        1 => day3_process_string_part1(code.as_ref()),
        2 => 0,
        _ => 0,
    }
}

fn char_priority(letter: &char) -> i32 {
    let mut priority = *letter as i32 - 96;
    if priority < 0 {
        priority += 58;
    }
    priority
}

fn day3_process_string_part1(code: &str) -> i32 {
    let first_half = code.substring(0,code.len()/2);
    let second_half = code.substring(code.len()/2, code.len());
    debug!("{}", first_half);
    debug!("{}", second_half);

    let first_half_chars: Vec<char> = first_half.chars().collect();

    let mut priority = 0;

    for this_char in first_half_chars {
        if second_half.contains(this_char) {
            let this_priority = char_priority(&this_char);
            debug!("{} has a priority of {}", this_char, this_priority);
            priority += this_priority;
            break;
        }
    }

    priority
}

struct ElfGroup {
    badge_priority: i32,
}

impl ElfGroup {
    pub fn new(this_group: Vec<String>) -> ElfGroup {

        let badge: char = Self::get_badge(&this_group);

        ElfGroup {
            badge_priority: char_priority(&badge),
        }
    }

    fn get_badge(this_group: &Vec<String>) -> char {
        let elf1 = this_group.get(0).expect("string");
        let elf2 = this_group.get(1).expect("string");
        let elf3 = this_group.get(2).expect("string");

        let common1 = Self::get_common_chars(elf1, elf2);
        let common2 = Self::get_common_chars(&common1, elf3);

        let badge = common2.chars().last().unwrap();

        debug!("found {} as common among all 3 elves", badge);

        badge
    }

    fn get_common_chars(str1: &str, str2: &str) -> String {

        let (shorter, longer) = if str1.len() > str2.len() {
            (str2, str1)
        } else {
            (str1, str2)
        };

        let mut common = String::from("");
        let shorter_chars: Vec<char> = shorter.chars().collect();

        for this_char in shorter_chars {
            if longer.contains(this_char) {
                common.push(this_char);
            }
        }

        common
    }

}

pub fn day3(args: Args) {

    let reader = get_input(args.clone());

    let mut total:i32 = 0;

    if args.part == 1 {

        for line in reader.lines() {
            let priority = day3_select_formula(line.as_ref().expect("should be a string"), &args.part);
            debug!("{} scores {} priority",line.expect("should be a string"), priority);
            total += priority;
        }

        info!("The total priority is: {}", total);
    }
    else if args.part == 2 {

        let mut this_group = Vec::new();
        let mut all_groups = Vec::new();

        for (iter, line) in reader.lines().enumerate() {
            if iter % 3 == 0 && iter > 0 {
                all_groups.push(ElfGroup::new(this_group));
                this_group = Vec::new();
                debug!("  -- new group --  ");
            }
            debug!("{}", line.as_ref().expect("should be a string"));
            this_group.push(line.expect("ff"));
        }

        all_groups.push(ElfGroup::new(this_group));
        debug!("  -- new group --  ");

        let mut total_priority = 0;

        for group in all_groups {
            debug!("Elf Group priority: {}", group.badge_priority);
            total_priority += group.badge_priority;
        }

        info!("Total priority for all elf groups: {}", total_priority);

    }

}
