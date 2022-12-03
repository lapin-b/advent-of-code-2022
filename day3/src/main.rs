use std::collections::HashSet;
use std::fs;
use std::io;
use std::io::BufRead;

#[derive(Debug)]
struct ElfRucksak {
    large_compartiment1: HashSet<char>,
    large_compartiment2: HashSet<char>
}

impl ElfRucksak {
    fn new(contents: &str) -> Self {
        debug_assert_eq!(contents.len() % 2, 0);
        let compartiments = contents.split_at(contents.len() / 2);

        Self {
            large_compartiment1: compartiments.0.chars().collect(),
            large_compartiment2: compartiments.1.chars().collect()
        }
    }

    // According to the assignment, there's always a common item between the two compartiments
    fn common_item_in_compartiments(&self) -> char {
        self.large_compartiment1.intersection(&self.large_compartiment2)
            .copied()
            .next()
            .expect("The instructions say that there's one item in common, found none")
    }
}

fn determine_item_priority(item: char) -> i32 {
    if item.is_uppercase() {
        item as i32 - 65 + 27
    } else {
        item as i32 - 96
    }
}

fn main() -> anyhow::Result<()> {
    let filename = utils::get_file_from_argv("files/day3/test.txt");
    let file_content = io::BufReader::new(fs::File::open(filename.as_ref())?);

    let rucksacks = file_content
        .lines()
        .map(Result::unwrap)
        .map(|line| ElfRucksak::new(&line))
        .collect::<Vec<_>>();

    // Part 1: Find the common items
    let common_items_summed_priority = rucksacks
        .iter()
        .map(|sack| sack.common_item_in_compartiments())
        .map(determine_item_priority)
        .sum::<i32>();

    println!("Part 1: sum of priorities for the common items: {}", common_items_summed_priority);

    Ok(())
}
