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

#[derive(Debug, Clone)]
struct GameLine {
    adv_move: GameMoves,
    my_move: GameMoves,
}

impl GameLine {
    fn calculate_chosen_move_score(&self) -> i32 {
        match self.my_move {
            GameMoves::Rock => 1,
            GameMoves::Paper => 2,
            GameMoves::Scissors => 3,
        }
    }

    fn calculate_game_score_part1(&self) -> i32 {
        if self.my_move == self.adv_move {
            return 3;
        }

        match self.my_move {
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
            let my_move = *moves.get(1).expect("No own move present");

            GameLine {
                adv_move: GameMoves::from_str(adv_move).expect("Invalid move for adversary"),
                my_move: GameMoves::from_str(my_move).expect("Invalid move for my move"),
            }
        })
        .collect::<Vec<_>>();

    // Part 1: Total score if everything goes to the provided plan
    let score_part1 = moves
        .iter()
        .map(|line| line.calculate_chosen_move_score() + line.calculate_game_score_part1())
        .sum::<i32>();

    println!("Part 1: score if everything goes to plan: {}", score_part1);

    Ok(())
}
