use std::io::BufRead;
use log::debug;
use log::info;
use regex::Regex;
use indexmap::IndexMap;
use adventofcode2022::*;


struct Supplies {
    stacks: IndexMap< i8, Vec<&'static str>>
}

impl Supplies {
    fn new(is_sample: bool) -> Supplies {
        if is_sample {
            let mut map = IndexMap::new();
            map.insert(1, vec!["[Z]","[N]"]);
            map.insert(2, vec!["[M]","[C]","[D]"]);
            map.insert(3, vec!["[P]"]);

            Supplies {
                stacks: map
            }
        }else{
            let mut map = IndexMap::new();
            map.insert(1, vec!["[Z]","[J]","[N]","[W]","[P]","[S]",]);
            map.insert(2, vec!["[G]","[S]","[T]",]);
            map.insert(3, vec!["[V]","[Q]","[R]","[L]","[H]",]);
            map.insert(4, vec!["[V]","[S]","[T]","[D]",]);
            map.insert(5, vec!["[Q]","[Z]","[T]","[D]","[B]","[M]","[J]",]);
            map.insert(6, vec!["[M]","[W]","[T]","[J]","[D]","[C]","[Z]","[L]",]);
            map.insert(7, vec!["[L]","[P]","[M]","[W]","[G]","[T]","[J]",]);
            map.insert(8, vec!["[N]","[G]","[M]","[T]","[B]","[F]","[Q]","[H]",]);
            map.insert(9, vec!["[R]","[D]","[G]","[C]","[P]","[B]","[Q]","[W]",]);

            Supplies {
                stacks: map
            }
        }
    }

    // info output of stack_map
    pub fn display_top(&self) {
        let mut top = Vec::<String>::new();
        for (key, value) in self.stacks.clone().into_iter() {
            if value.len() > 0 {
                let this_stack_top = value.clone().pop().expect("string").to_string();
                debug!("stack {} has {} on top", key, this_stack_top);
                top.push(this_stack_top);
            } else {
                debug!("stack {} has [_] on top", key);
                top.push("[_]".to_string());
            }
        }
        info!("{}", top.join("").replace("[","").replace("]",""));
    }

    // debug output of stack_map
    pub fn display_all(&self) {
        for (key, value) in self.stacks.clone().into_iter() {
            debug!("stack {} (bottom to top)", key);

            for piece in value {
                debug!("\t{}", piece);
            }
        }
    }

    fn execute_instruction(&mut self, instruction: &str, part: u8) {
        let re = Regex::new(r"move (\d{1,2}) from (\d) to (\d)").unwrap();
        self.display_all();
        debug!("{}", instruction);
        for cap in re.captures_iter(instruction) {
            let source = &cap[2].parse::<i8>().unwrap();
            let dest = &cap[3].parse::<i8>().unwrap();
            let quantity = *&cap[1].parse::<i8>().unwrap();
            debug!("from stack {}, we will move {} crates to stack {}", source, quantity, dest);

            let mut temp_stack = Vec::<&str>::new();

            for _ in 0..quantity {

                let source_stack = self.stacks.get_mut(source).unwrap();
                let popped_value = source_stack.pop().expect("this should be a string");
                debug!("popped off {}", popped_value);

                if part==1 {
                    let dest_stack = self.stacks.get_mut(dest).unwrap();
                    dest_stack.push(popped_value);
                } else if part==2 {
                    temp_stack.push(popped_value);
                }

            }

            if part==2 {
                let dest_stack = self.stacks.get_mut(dest).unwrap();
                temp_stack.reverse();
                dest_stack.append(&mut temp_stack);
            }

        }

        self.display_all();
    }

}


pub fn day5(args: Args) {

    let reader = get_input(args.clone());
    let mut supplies = Supplies::new(args.sample);
    let mut instruction_count = 0;

    for line in reader.lines() {

        let line = line.unwrap();

        if line.starts_with("move") {
            supplies.execute_instruction(&line, args.part);
            instruction_count += 1;
            debug!("followed {} instructions so far... ", instruction_count);
        }
    }

    supplies.display_top();

}
