use std::collections::{HashMap, HashSet, VecDeque};

advent_of_code::solution!(10);

fn get_cycle_length(input: &str) -> u32 {
    let input = input.split('\n').collect::<Vec<&str>>();

    let (start_row, start_col) = input
        .clone()
        .into_iter()
        .enumerate()
        .find_map(|(i, row)| row.chars().position(|c| c == 'S').map(|j| (i, j)))
        .expect("mouse not found");

    let mut queue = VecDeque::from([(start_row, start_col)]);
    let mut visited = HashSet::new();

    while !queue.is_empty() {
        let (row, col) = queue.pop_front().unwrap();
        if visited.contains(&(row, col)) {
            continue;
        }
        match input.get(row).and_then(|r| r.chars().nth(col)) {
            Some('S') => {
                queue.push_back((row - 1, col));
                queue.push_back((row + 1, col));
                queue.push_back((row, col - 1));
                queue.push_back((row, col + 1));
            }
            Some('|') => {
                queue.push_back((row - 1, col));
                queue.push_back((row + 1, col));
            }
            Some('-') => {
                queue.push_back((row, col - 1));
                queue.push_back((row, col + 1));
            }
            Some('7') => {
                queue.push_back((row + 1, col));
                queue.push_back((row, col - 1));
            }
            Some('F') => {
                queue.push_back((row + 1, col));
                queue.push_back((row, col + 1));
            }
            Some('J') => {
                queue.push_back((row - 1, col));
                queue.push_back((row, col - 1));
            }
            Some('L') => {
                queue.push_back((row - 1, col));
                queue.push_back((row, col + 1));
            }
            _ => (),
        }
        visited.insert((row, col));
    }
    visited.len() as u32
}

pub fn part_one(input: &str) -> Option<u32> {
    //     let input = "7-F7-
    // .FJ|7
    // SJLL7
    // |F--J
    // LJ.LJ";
    //     let input = ".....
    // .S-7.
    // .|.|.
    // .L-J.
    // .....";
    Some(get_cycle_length(input) / 2)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
