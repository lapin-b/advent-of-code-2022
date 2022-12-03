use std::collections::HashSet;
use std::fs;
use std::io;
use std::io::BufRead;

use itertools::Itertools;

struct ElfRucksak {
    // Part 1
    large_compartiment1: HashSet<char>,
    large_compartiment2: HashSet<char>,

    // Part 2
    bag_content: HashSet<char>,
}

impl ElfRucksak {
    fn new(contents: &str) -> Self {
        debug_assert_eq!(contents.len() % 2, 0);
        let compartiments = contents.split_at(contents.len() / 2);

        Self {
            large_compartiment1: compartiments.0.chars().collect(),
            large_compartiment2: compartiments.1.chars().collect(),
            bag_content: contents.chars().collect(),
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

impl std::fmt::Debug for ElfRucksak {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f
            .debug_struct("ElfRucksak")
            .field("bag_content", &self.bag_content.iter().join(""))
            .finish()
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

    // Part 2: Each group has three items, so can we chunk ?
    let badges_priorities_sum = rucksacks
        .chunks(3)
        .map(|group| (&group[0], &group[1], &group[2]))
        .map(|(elf1_rucksack, elf2_rucksac, elf3_rucksack)| {
            let intermediary_intersection = elf1_rucksack.bag_content
                .intersection(&elf2_rucksac.bag_content)
                .copied()
                .collect::<HashSet<_>>();

            let badge = intermediary_intersection
                .intersection(&elf3_rucksack.bag_content)
                .copied()
                .next()
                .expect("At least one item should be common in the three bags");
            badge
        })
        // Badges have been founds, time to assign the priorities like part 1
        .map(|badge| determine_item_priority(badge))
        .sum::<i32>();

    println!("Part 2: Sum of badge stickers priorities: {}", badges_priorities_sum);

    Ok(())
}
