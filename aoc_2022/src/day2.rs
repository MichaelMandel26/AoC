use anyhow::Result;
use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum GameResult {
    Win = 6,
    Loose = 0,
    Draw = 3,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum PlayMove {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct StrategyGuideColumn {
    opponents_move: PlayMove,
    recommended_move: PlayMove,
    outcome: GameResult,
    desired_outcome: GameResult,
}

impl StrategyGuideColumn {
    fn calculate_own_score(self) -> i32 {
        let move_score: i32 = self.recommended_move as i32;
        let game_result_score = self.outcome as i32;
        move_score + game_result_score
    }

    fn get_best_move_by_outcome(self) -> PlayMove {
        match self.desired_outcome {
            GameResult::Win => match self.opponents_move {
                PlayMove::Rock => PlayMove::Paper,
                PlayMove::Paper => PlayMove::Scissors,
                PlayMove::Scissors => PlayMove::Rock,
            },
            GameResult::Loose => match self.opponents_move {
                PlayMove::Rock => PlayMove::Scissors,
                PlayMove::Paper => PlayMove::Rock,
                PlayMove::Scissors => PlayMove::Paper,
            },
            GameResult::Draw => self.opponents_move,
        }
    }
}

impl<T: ToString> From<T> for StrategyGuideColumn {
    fn from(value: T) -> Self {
        let string = value.to_string();
        let move_identifiers: Vec<char> = string
            .split(' ')
            .map(|c| c.chars().next().unwrap())
            .collect();

        let opponents_move = PlayMove::from(move_identifiers[0]);
        let recommended_move = PlayMove::from(move_identifiers[1]);
        let desired_outcome = GameResult::from(move_identifiers[1]);
        let outcome = match recommended_move {
            PlayMove::Rock => match opponents_move {
                PlayMove::Rock => GameResult::Draw,
                PlayMove::Paper => GameResult::Loose,
                PlayMove::Scissors => GameResult::Win,
            },
            PlayMove::Paper => match opponents_move {
                PlayMove::Rock => GameResult::Win,
                PlayMove::Paper => GameResult::Draw,
                PlayMove::Scissors => GameResult::Loose,
            },
            PlayMove::Scissors => match opponents_move {
                PlayMove::Rock => GameResult::Loose,
                PlayMove::Paper => GameResult::Win,
                PlayMove::Scissors => GameResult::Draw,
            },
        };

        StrategyGuideColumn {
            opponents_move,
            recommended_move,
            outcome,
            desired_outcome,
        }
    }
}

impl From<char> for PlayMove {
    fn from(move_identifier: char) -> Self {
        match move_identifier {
            'A' | 'X' => PlayMove::Rock,
            'B' | 'Y' => PlayMove::Paper,
            'C' | 'Z' => PlayMove::Scissors,
            _ => panic!("unexpected input"),
        }
    }
}

impl From<char> for GameResult {
    fn from(move_identifier: char) -> Self {
        match move_identifier {
            'X' => GameResult::Loose,
            'Y' => GameResult::Draw,
            'Z' => GameResult::Win,
            _ => panic!("unexpected input"),
        }
    }
}

#[aoc_generator(day2)]
fn parse_input_day2(input: &str) -> Vec<StrategyGuideColumn> {
    input.lines().map(StrategyGuideColumn::from).collect()
}

#[aoc(day2, part1)]
fn part1(strategy_guide: &[StrategyGuideColumn]) -> i32 {
    strategy_guide.iter().map(|c| c.calculate_own_score()).sum()
}

#[aoc(day2, part2)]
fn part2(strategy_guide: &[StrategyGuideColumn]) -> i32 {
    strategy_guide
        .iter()
        .map(|c| c.get_best_move_by_outcome() as i32 + c.desired_outcome as i32)
        .sum()
}
