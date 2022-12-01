use anyhow::Result;
use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
fn parse_input_day1(input: &str) -> Result<Vec<i32>> {
    let mut result: Vec<i32> = vec![];

    let calorie_groups: Vec<&str> = input.split("\n\n").collect();

    for group in calorie_groups.iter() {
        let elve_calories: Vec<i32> = group.lines().map(|l| l.parse().unwrap()).collect();
        let total_calories: i32 = elve_calories.iter().sum();
        result.push(total_calories)
    }

    Ok(result)
}

#[aoc(day1, part1)]
fn part1(elves: &[i32]) -> i32 {
    *elves.iter().max().unwrap()
}

#[aoc(day1, part2)]
fn part2(elves: &[i32]) -> i32 {
    let mut elves: Vec<i32> = elves.to_vec();
    elves.sort();
    elves.iter().rev().take(3).sum()
}
