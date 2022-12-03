use std::collections::HashSet;
use std::fs;
use std::io;
use std::io::BufRead;

use itertools::Itertools;

struct ElfRucksack {
    // Part 1
    large_compartiment1: HashSet<char>,
    large_compartiment2: HashSet<char>,

    // Part 2
    bag_content: HashSet<char>,
}

impl ElfRucksack {
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

impl std::fmt::Debug for ElfRucksack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f
            .debug_struct("ElfRucksack")
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
        .map(|line| ElfRucksack::new(&line))
        .collect::<Vec<_>>();

    // Part 1: Find the common items between compartiments of the bag
    let common_items_summed_priority = rucksacks
        .iter()
        .map(|sack| sack.common_item_in_compartiments())
        .map(determine_item_priority)
        .sum::<i32>();

    println!("Part 1: sum of priorities for the common items: {}", common_items_summed_priority);

    // Part 2: Each group has three bags and one item in common between the three.
    // The item in common determines the badge elves wear.
    let badges_priorities_sum = rucksacks
        .chunks(3)
        // Make the chunk a bit easier to work with by separating the three members explicitely.
        // This will panic if there are not exactly three items.
        .map(|group| (&group[0], &group[1], &group[2]))
        // More set mathematics. This time it's a three-way intersection on hashmaps.
        .map(|(elf1_rucksack, elf2_rucksac, elf3_rucksack)| {
            elf1_rucksack.bag_content
                // First intersection between elf 1 and elf 2
                .intersection(&elf2_rucksac.bag_content)
                .copied()
                .collect::<HashSet<_>>()

                // Add elf 3 to the mix
                .intersection(&elf3_rucksack.bag_content)
                .copied()
                .next()
                .expect("At least one item should be common in the three bags")
        })
        // Badges have been founds, time to assign the priorities like part 1
        .map(determine_item_priority)
        .sum::<i32>();

    println!("Part 2: Sum of badge stickers priorities: {}", badges_priorities_sum);

    Ok(())
}
