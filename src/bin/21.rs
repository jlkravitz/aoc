use core::panic;
use std::{
    collections::{HashSet, VecDeque},
    ops::Neg,
};

use itertools::Itertools;

advent_of_code::solution!(21);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Item {
    GardenPlot,
    Rock,
}

#[derive(Debug)]
struct Map {
    map: Vec<Vec<Item>>,
}

impl Map {
    fn parse(input: &str) -> (Self, (isize, isize)) {
        let mut maybe_start = None;
        let map = input
            .lines()
            .enumerate()
            .map(|(row, line)| {
                line.chars()
                    .enumerate()
                    .map(|(col, c)| match c {
                        '.' => Item::GardenPlot,
                        '#' => Item::Rock,
                        'S' => {
                            maybe_start = Some((row as isize, col as isize));
                            Item::GardenPlot
                        }
                        _ => panic!("Invalid character"),
                    })
                    .collect_vec()
            })
            .collect_vec();
        if let Some(start) = maybe_start {
            (Self { map }, start)
        } else {
            panic!("No start found");
        }
    }

    fn neighbors(&self, row: isize, col: isize) -> Vec<(isize, isize)> {
        let row = row as usize;
        let col = col as usize;
        let mut neighbors = Vec::new();
        if row > 0 {
            neighbors.push((row as isize - 1, col as isize));
        }
        if col > 0 {
            neighbors.push((row as isize, col as isize - 1));
        }
        if row < self.map.len() - 1 {
            neighbors.push((row as isize + 1, col as isize));
        }
        if col < self.map[0].len() - 1 {
            neighbors.push((row as isize, col as isize + 1));
        }
        neighbors
    }

    fn neighbors_infinite_scroll(&self, row: isize, col: isize) -> Vec<(isize, isize)> {
        let mut v = vec![];
        if row > (self.map.len() as isize).neg() {
            v.push((row - 1, col));
        }
        if col > (self.map[0].len() as isize).neg() {
            v.push((row, col - 1));
        }
        if row < (self.map.len() as isize * 2) - 1 {
            v.push((row + 1, col));
        }
        if col < (self.map[0].len() as isize * 2) - 1 {
            v.push((row, col + 1));
        }
        v
    }

    fn at_infinite_scroll(&self, row: isize, col: isize) -> Item {
        let (mut adj_row, mut adj_col) = (
            row % self.map.len() as isize,
            col % self.map[0].len() as isize,
        );
        if adj_row < 0 {
            adj_row += self.map.len() as isize;
        }
        if adj_col < 0 {
            adj_col += self.map[0].len() as isize;
        }
        self.map[adj_row as usize][adj_col as usize]
    }

    fn shortest_path_to_gardens(
        &self,
        start: (isize, isize),
        infinite: bool,
    ) -> HashSet<((isize, isize), isize)> {
        let mut queue = VecDeque::new();
        queue.push_back((start, 0));
        let mut gardens = HashSet::new();
        let mut visited = HashSet::new();

        while let Some(((row, col), steps)) = queue.pop_front() {
            let item = if infinite {
                self.at_infinite_scroll(row, col)
            } else {
                self.map[row as usize][col as usize]
            };
            if item == Item::Rock || visited.contains(&(row, col)) {
                continue;
            }
            gardens.insert(((row, col), steps));
            visited.insert((row, col));
            for (row, col) in {
                if infinite {
                    self.neighbors_infinite_scroll(row, col)
                } else {
                    self.neighbors(row, col)
                }
            } {
                queue.push_back(((row, col), steps + 1));
            }
        }
        gardens
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (map, start) = Map::parse(input);
    let n = map
        .shortest_path_to_gardens(start, false)
        .into_iter()
        .filter(|&(_, steps)| steps <= 64 && steps % 2 == 0)
        .count();
    Some(n as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (map, start) = Map::parse(input);
    let shortest_paths = map
        .shortest_path_to_gardens(start, true)
        .into_iter()
        .filter(|&(_, steps)| steps % 2 == 0)
        .count();
    Some(shortest_paths as u32)
    // dbg!(map.gardens_reachable_in(26501365));
    // Some(map.gardens_reachable_in(26501365))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16733044));
    }
}
