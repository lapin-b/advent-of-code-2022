use std::fmt::{Display, Formatter};
use std::fs;
use std::io;
use std::io::BufRead;
use regex::Regex;

#[derive(Copy, Clone, Debug)]
struct AssignmentBounds {
    pub start: u32,
    pub end: u32,
}

impl AssignmentBounds {
    pub fn fully_contains(&self, other: &Self) -> bool {
        self.start >= other.start && self.end <= other.end
    }

    pub fn overlaps_with(&self, other: &Self) -> bool {
        self.start <= other.end && self.end >= other.start
    }
}

impl std::fmt::Display for AssignmentBounds {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.start, self.end)
    }
}

#[derive(Copy, Clone)]
struct AssignmentPair(AssignmentBounds, AssignmentBounds);

fn main() {
    let file_content_regex = Regex::new(r"^(\d+)-(\d+),(\d+)-(\d+)$")
        .expect("Invalid Regex");

    let filename = utils::get_file_from_argv("files/day4/test.txt");
    let file = io::BufReader::new(fs::File::open(filename.as_ref()).unwrap());

    let assignment_pairs = file.lines()
        .map(Result::unwrap)
        .filter_map(|line|
            file_content_regex
                .captures(&line)
                .map(|captures|
                    AssignmentPair(
                        AssignmentBounds {
                            start: captures.get(1).unwrap().as_str().parse().unwrap(),
                            end: captures.get(2).unwrap().as_str().parse().unwrap(),
                        },
                        AssignmentBounds {
                            start: captures.get(3).unwrap().as_str().parse().unwrap(),
                            end: captures.get(4).unwrap().as_str().parse().unwrap(),
                        },
                    )
                )
        )
        .collect::<Vec<_>>();

    let fully_contains_count = assignment_pairs
        .iter()
        .filter(|pair| pair.0.fully_contains(&pair.1) || pair.1.fully_contains(&pair.0))
        .count();


    let overlapping_count = assignment_pairs
        .iter()
        .filter(|pair| pair.0.overlaps_with(&pair.1) || pair.1.overlaps_with(&pair.0))
        .count();

    println!("Part 1: {} ranges full contains the other", fully_contains_count);
    println!("Part 2: {} ranges overlap each other", overlapping_count);
}
