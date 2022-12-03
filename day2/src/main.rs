use std::fs;
use std::io;
use std::io::BufRead;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GameMoves {
    Rock,
    Paper,
    Scissors
}

impl FromStr for GameMoves {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            // Part 1 was to interpret the 2nd column as our own move
            "A" | "X" => Ok(GameMoves::Rock),
            "B" | "Y" => Ok(GameMoves::Paper),
            "C" | "Z" => Ok(GameMoves::Scissors),
            _ => Err(format!("Invalid move {}", s))
        }
    }
}

impl GameMoves {
    fn get_chosen_move_score(&self) -> i32 {
        match self {
            GameMoves::Rock => 1,
            GameMoves::Paper => 2,
            GameMoves::Scissors => 3,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum RoundResult {
    Win,
    Lose,
    Draw
}

impl FromStr for RoundResult {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            // Part 2 was to interpret the second column as the needed outcome
            "X" => Ok(RoundResult::Lose),
            "Y" => Ok(RoundResult::Draw),
            "Z" => Ok(RoundResult::Win),
            _ => Err(format!("Invalid needed game result {}", s))
        }
    }
}

impl RoundResult {
    fn get_round_result_score(&self) -> i32 {
        match self {
            RoundResult::Win => 6,
            RoundResult::Lose => 0,
            RoundResult::Draw => 3
        }
    }
}

#[derive(Debug, Clone)]
struct RoundLine {
    adv_move: GameMoves,
    p2_needed_round_end: RoundResult,
    p1_my_move: GameMoves,
}

impl RoundLine {
    fn calculate_game_score_part1(&self) -> i32 {
        let round_result = match self.p1_my_move {
            GameMoves::Rock => match self.adv_move {
                GameMoves::Paper => RoundResult::Lose,
                GameMoves::Scissors => RoundResult::Win,
                GameMoves::Rock => RoundResult::Draw,
            },
            GameMoves::Paper => match self.adv_move {
                GameMoves::Rock => RoundResult::Win,
                GameMoves::Scissors => RoundResult::Lose,
                GameMoves::Paper => RoundResult::Draw,
            },
            GameMoves::Scissors => match self.adv_move {
                GameMoves::Paper => RoundResult::Win,
                GameMoves::Rock => RoundResult::Lose,
                GameMoves::Scissors => RoundResult::Draw,
            },
        };

        round_result.get_round_result_score() + self.p1_my_move.get_chosen_move_score()
    }

    fn calculate_game_score_part2(&self) -> i32 {
        let chosen_my_move = match self.p2_needed_round_end {
            RoundResult::Win => match self.adv_move {
                GameMoves::Rock => GameMoves::Paper,
                GameMoves::Paper => GameMoves::Scissors,
                GameMoves::Scissors => GameMoves::Rock,
            },

            RoundResult::Lose => match self.adv_move {
                GameMoves::Rock => GameMoves::Scissors,
                GameMoves::Paper => GameMoves::Rock,
                GameMoves::Scissors => GameMoves::Paper,
            },

            RoundResult::Draw => self.adv_move,
        };

        self.p2_needed_round_end.get_round_result_score() + chosen_my_move.get_chosen_move_score()
    }
}

fn main() -> anyhow::Result<()> {
    let filename = utils::get_file_from_argv("files/day2/test.txt");
    let buffer = io::BufReader::new(fs::File::open(filename.as_ref())?);
    let moves = buffer
        .lines()
        .map(Result::unwrap)
        .map(|line| {
            let moves = line.split_whitespace().collect::<Vec<_>>();

            let adv_move = *moves.first().expect("No adversary move present");
            let my_move_or_needed_round_end = *moves.get(1).expect("No own move present");

            RoundLine {
                adv_move: GameMoves::from_str(adv_move).expect("Invalid move for adversary"),
                p2_needed_round_end: RoundResult::from_str(my_move_or_needed_round_end).expect("Invalid needed round end"),
                p1_my_move: GameMoves::from_str(my_move_or_needed_round_end).expect("Invalid move for my move"),
            }
        })
        .collect::<Vec<_>>();

    // Part 1: Total score if everything goes to the provided plan
    let score_part1 = moves
        .iter()
        .map(|line| line.calculate_game_score_part1())
        .sum::<i32>();

    let score_part2 = moves
        .iter()
        .map(|line| line.calculate_game_score_part2())
        .sum::<i32>();

    println!("Part 1: score if everything goes to plan: {}", score_part1);
    println!("Part 2: score if everything goes to plan: {}", score_part2);

    Ok(())
}
