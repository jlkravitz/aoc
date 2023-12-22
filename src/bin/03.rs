use std::collections::{HashMap, HashSet};

use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let matrix: Vec<Vec<char>> = input
        .split('\n')
        .map(|line| line.chars().collect())
        .collect();

    let next_to_symbol = |x: usize, y: usize| {
        (std::cmp::max(1, x) - 1..=x + 1).any(|i| {
            (std::cmp::max(1, y) - 1..=y + 1).any(|j| {
                !(i == x && j == y)
                    && i < matrix.len()
                    && j < matrix[i].len()
                    && !matrix[i][j].is_ascii_digit()
                    && matrix[i][j] != '.'
            })
        })
    };
    let regex = Regex::new(r"\d+").ok()?;

    Some(
        matrix
            .iter()
            .enumerate()
            .map(|(i, row)| {
                regex
                    .find_iter(&row.iter().collect::<String>())
                    .filter_map(|match_| {
                        (match_.start()..match_.end())
                            .any(|j| next_to_symbol(i, j))
                            .then_some(match_.as_str().parse::<u32>().ok()?)
                    })
                    .sum::<u32>()
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let matrix: Vec<Vec<char>> = input
        .split('\n')
        .map(|line| line.chars().collect())
        .collect();

    let regex = Regex::new(r"\d+").ok()?;
    let mut number_at_index = HashMap::new();

    matrix.iter().enumerate().for_each(|(i, row)| {
        regex
            .find_iter(&row.iter().collect::<String>())
            .for_each(|match_| {
                (match_.start()..match_.end()).for_each(|j| {
                    number_at_index.insert((i, j), match_.as_str().parse::<u32>().unwrap());
                });
            })
    });
    let mut sum = 0;

    (0..matrix.len()).for_each(|i| {
        for j in 0..matrix[i].len() {
            if matrix[i][j] == '*' {
                let num_index_ref = &number_at_index; // Create a reference to the hashmap

                let numbers: Vec<&u32> = (std::cmp::max(1, i) - 1..=i + 1)
                    .flat_map(|x| {
                        (std::cmp::max(1, j) - 1..=j + 1)
                            .filter_map(move |y| num_index_ref.get(&(x, y)))
                    })
                    .collect::<HashSet<&u32>>()
                    .into_iter()
                    .collect();
                if numbers.len() == 2 {
                    sum += numbers[0] * numbers[1];
                }
            }
        }
    });
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
