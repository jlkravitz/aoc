use itertools::Itertools;
use std::collections::{HashSet, VecDeque};

advent_of_code::solution!(10);

type NodeList = HashSet<(usize, usize)>;

fn build_maze(input: &str) -> (Vec<&str>, (usize, usize), char) {
    let maze = input.split('\n').collect::<Vec<&str>>();
    let start_pos = maze
        .iter()
        .enumerate()
        .find_map(|(i, row)| row.chars().position(|c| c == 'S').map(|j| (i, j)))
        .expect("mouse not found");
    let above = maze
        .clone()
        .into_iter()
        .nth(start_pos.0 - 1)
        .and_then(|row| {
            row.chars()
                .nth(start_pos.1)
                .filter(|c| *c == '|' || *c == '7' || *c == 'F')
        })
        .is_some();
    let below = maze
        .clone()
        .into_iter()
        .nth(start_pos.0 + 1)
        .and_then(|row| {
            row.chars()
                .nth(start_pos.1)
                .filter(|c| *c == '|' || *c == 'L' || *c == 'J')
        })
        .is_some();

    let right = maze
        .clone()
        .into_iter()
        .nth(start_pos.0)
        .and_then(|row| {
            row.chars()
                .nth(start_pos.1 + 1)
                .filter(|c| *c == '-' || *c == '7' || *c == 'J')
        })
        .is_some();

    maze[start_pos.0].chars().nth(start_pos.1 + 1);
    let left = maze
        .clone()
        .into_iter()
        .nth(start_pos.0)
        .and_then(|row| {
            row.chars()
                .nth(start_pos.1 - 1)
                .filter(|c| *c == '-' || *c == 'L' || *c == 'F')
        })
        .is_some();

    let s_char = if left && above {
        'J'
    } else if left && below {
        '7'
    } else if right && above {
        'L'
    } else {
        'F'
    };

    (maze, start_pos, s_char)
}

fn get_cycle_nodes(input: &[&str], start_pos: (usize, usize), s_char: char) -> NodeList {
    let mut queue = VecDeque::from([start_pos]);
    let mut visited = HashSet::new();

    while !queue.is_empty() {
        let (row, col) = queue.pop_front().unwrap();
        if visited.contains(&(row, col)) {
            continue;
        }
        match input.get(row).and_then(|r| r.chars().nth(col)).map(
            |c| {
                if c == 'S' {
                    s_char
                } else {
                    c
                }
            },
        ) {
            // Some('S') => {
            //     if row != 0 {
            //         queue.push_back((row - 1, col));
            //     }
            //     queue.push_back((row + 1, col));
            //     if col != 0 {
            //         queue.push_back((row, col - 1));
            //     }
            //     queue.push_back((row, col + 1));
            // }
            Some('|') => {
                if row != 0 {
                    queue.push_back((row - 1, col));
                }
                queue.push_back((row + 1, col));
            }
            Some('-') => {
                if col != 0 {
                    queue.push_back((row, col - 1));
                }
                queue.push_back((row, col + 1));
            }
            Some('7') => {
                queue.push_back((row + 1, col));
                if col != 0 {
                    queue.push_back((row, col - 1));
                }
            }
            Some('F') => {
                queue.push_back((row + 1, col));
                queue.push_back((row, col + 1));
            }
            Some('J') => {
                if row != 0 {
                    queue.push_back((row - 1, col));
                }
                if col != 0 {
                    queue.push_back((row, col - 1));
                }
            }
            Some('L') => {
                if row != 0 {
                    queue.push_back((row - 1, col));
                }
                queue.push_back((row, col + 1));
            }
            _ => continue,
        }
        visited.insert((row, col));
    }
    visited
}

pub fn part_one(input: &str) -> Option<u32> {
    let (maze, start_pos, s_char) = build_maze(input);
    Some(get_cycle_nodes(&maze, start_pos, s_char).len() as u32 / 2)
}

fn is_piped_pair(left: char, right: char) -> bool {
    matches!((left, right), |('L', '7')| ('F', 'J'))
}
pub fn part_two(input: &str) -> Option<u32> {
    let (maze, start_pos, s_char) = build_maze(input);
    let cycle = get_cycle_nodes(&maze, start_pos, s_char);

    let n = maze
        .into_iter()
        .enumerate()
        .map(|(i, row)| {
            let mut inside = false;
            let mut inner_nodes = vec![];
            let mut last_char = ' ';
            for (j, c) in row.chars().enumerate() {
                if cycle.contains(&(i, j)) {
                    if c == '|' || is_piped_pair(last_char, if c == 'S' { s_char } else { c }) {
                        inside = !inside;
                    }
                    if c != '-' {
                        last_char = if c == 'S' { s_char } else { c };
                    }
                } else if !cycle.contains(&(i, j)) && inside {
                    inner_nodes.push((i, j));
                }
            }
            inner_nodes.len() as u32
        })
        .sum::<u32>();
    Some(n)

    // Some(nodes.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1));
    }
}
