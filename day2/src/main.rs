use std::fs;
use std::io;
use std::io::BufRead;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
enum GameMoves {
    Rock,
    Paper,
    Scissors
}

#[derive(Debug, Clone)]
enum NeededRoundResult {
    Win,
    Lose,
    Draw
}

impl FromStr for GameMoves {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
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

impl FromStr for NeededRoundResult {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(NeededRoundResult::Lose),
            "Y" => Ok(NeededRoundResult::Draw),
            "Z" => Ok(NeededRoundResult::Win),
            _ => Err(format!("Invalid needed game result {}", s))
        }
    }
}

#[derive(Debug, Clone)]
struct RoundLine {
    adv_move: GameMoves,
    p2_needed_round_end: NeededRoundResult,
    p1_my_move: GameMoves,
}

impl RoundLine {
    fn calculate_game_score_part1(&self) -> i32 {
        if self.p1_my_move == self.adv_move {
            return 3;
        }

        match self.p1_my_move {
            GameMoves::Rock => match self.adv_move {
                GameMoves::Paper => 0,
                GameMoves::Scissors => 6,
                _ => unreachable!(),
            },
            GameMoves::Paper => match self.adv_move {
                GameMoves::Rock => 6,
                GameMoves::Scissors => 0,
                _ => unreachable!(),
            },
            GameMoves::Scissors => match self.adv_move {
                GameMoves::Paper => 6,
                GameMoves::Rock => 0,
                _ => unreachable!(),
            },
        }
    }

    fn calculate_game_score_part2(&self) -> i32 {
        todo!()
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

            let adv_move = *moves.get(0).expect("No adversary move present");
            let my_move_or_needed_round_end = *moves.get(1).expect("No own move present");

            RoundLine {
                adv_move: GameMoves::from_str(adv_move).expect("Invalid move for adversary"),
                p2_needed_round_end: NeededRoundResult::from_str(my_move_or_needed_round_end).expect("Invalid needed round end"),
                p1_my_move: GameMoves::from_str(my_move_or_needed_round_end).expect("Invalid move for my move"),
            }
        })
        .collect::<Vec<_>>();

    // Part 1: Total score if everything goes to the provided plan
    let score_part1 = moves
        .iter()
        .map(|line| line.p1_my_move.get_chosen_move_score() + line.calculate_game_score_part1())
        .sum::<i32>();

    let score_part2 = moves
        .iter()
        .map(|line| line.calculate_game_score_part2())
        .sum::<i32>();

    println!("Part 1: score if everything goes to plan: {}", score_part1);
    println!("Part 2: score if everything goes to plan: {}", score_part2);

    Ok(())
}
