use std::collections::HashMap;

use anyhow::Result;
use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
fn parse_input_day1(input: &str) -> Result<(Vec<i32>, Vec<i32>)> {
    let lines: (Vec<i32>, Vec<i32>) = input
        .lines()
        .map(|l| {
            let numbers: Vec<&str> = l.split_whitespace().collect();
            (
                numbers[0].parse::<i32>().unwrap(),
                numbers[1].parse::<i32>().unwrap(),
            )
        })
        .collect();

    Ok(lines)
}

#[aoc(day1, part1)]
fn part1(numbers: &(Vec<i32>, Vec<i32>)) -> i32 {
    let (mut left, mut right) = numbers.clone();
    left.sort_unstable();
    right.sort_unstable();

    left.iter()
        .zip(right.iter())
        .map(|(a, b)| (a - b).abs())
        .sum()
}

#[aoc(day1, part2)]
fn part2(numbers: &(Vec<i32>, Vec<i32>)) -> i32 {
    let (left, right) = numbers;
    let mut right_counts = HashMap::new();

    for &num in right {
        *right_counts.entry(num).or_insert(0) += 1;
    }

    left.iter()
        .map(|&num| num * right_counts.get(&num).cloned().unwrap_or(0))
        .sum()
}

mod tests {
    #[test]
    fn test_part1() {
        let input = r#"3   4
4   3
2   5
1   3
3   9
3   3
"#;
        let numbers = super::parse_input_day1(input).unwrap();
        assert_eq!(super::part1(&numbers), 11);
    }

    #[test]
    fn test_part2() {
        let input = r#"3   4
4   3
2   5
1   3
3   9
3   3
"#;
        let numbers = super::parse_input_day1(input).unwrap();
        assert_eq!(super::part2(&numbers), 31);
    }
}
