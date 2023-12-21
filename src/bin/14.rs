use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
};

use itertools::Itertools;

advent_of_code::solution!(14);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Platform(Vec<Vec<char>>);
enum Direction {
    North,
    South,
    West,
    East,
}

impl Display for Platform {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in &self.0 {
            for c in row {
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Platform {
    fn parse(input: &str) -> Self {
        Self(input.lines().map(|line| line.chars().collect()).collect())
    }

    fn transpose(&self) -> Self {
        let mut columns = vec![vec![]; self.0[0].len()];
        for row in &self.0 {
            for (i, c) in row.iter().enumerate() {
                columns[i].push(*c);
            }
        }
        Self(columns)
    }

    fn reverse_rows(&self) -> Self {
        let mut rows = vec![];
        for row in &self.0 {
            rows.push(row.iter().rev().cloned().collect_vec());
        }
        Self(rows)
    }

    fn move_right(&self, row: &[char], i: usize) -> Vec<char> {
        assert!(row[i] == 'O');
        let mut row = row.to_owned();
        let mut j = i + 1;
        while j < row.len() && row[j] == '.' {
            row[j - 1] = '.';
            row[j] = 'O';
            j += 1;
        }
        row
    }

    fn score(&self) -> u32 {
        self.0
            .iter()
            .rev()
            .enumerate()
            .map(|(i, row)| row.iter().filter(|&c| *c == 'O').count() * (i + 1))
            .sum::<usize>() as u32
    }

    fn cycle(&self) -> Self {
        self.tilt(Direction::North)
            .tilt(Direction::West)
            .tilt(Direction::South)
            .tilt(Direction::East)
    }

    fn tilt(&self, direction: Direction) -> Self {
        match direction {
            Direction::North => self
                .transpose()
                .reverse_rows()
                .tilt(Direction::East)
                .reverse_rows()
                .transpose(),
            Direction::South => self.transpose().tilt(Direction::East).transpose(),
            Direction::West => self.reverse_rows().tilt(Direction::East).reverse_rows(),
            Direction::East => Platform(
                self.0
                    .iter()
                    .map(|row| {
                        let mut row = row.clone();
                        let row_len = row.len();
                        for (i, &c) in row.clone().iter().enumerate().rev() {
                            if c == 'O' {
                                row = self.move_right(&row, i)
                            }
                        }
                        assert!(row.len() == row_len);
                        row
                    })
                    .collect_vec(),
            ),
        }
    }
}
//     if let Some(first_square_rock, _) = row.iter().find_position(|&c| *c == '#') {
//         row.clone()
//             .into_iter()
//             .enumerate()
//             .filter_map(|(i, c)| (c == '#').then_some(i))
//             .tuple_windows()
//             .flat_map(move |(i, j)| {
//                 let num_circle_rocks =
//                     row[i + 1..j].iter().filter(|&c| *c == 'O').count();
//                 [
//                     vec!['O'; num_circle_rocks],
//                     vec!['.'; j - i - 1 - num_circle_rocks],
//                     vec!['#'],
//                 ]
//                 .concat()
//             })
//             .chain(row[j..].iter().cloned())
//             .collect_vec()
//     }
// })
// .collect_vec(),

pub fn part_one(input: &str) -> Option<u32> {
    let mut platform = Platform::parse(input);
    platform = platform.tilt(Direction::North);
    Some(platform.score())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut platform = Platform::parse(input);
    let mut cycles = HashMap::from([(platform.clone(), 0)]);
    let mut cur_cycle = 1;
    loop {
        platform = platform.cycle();
        if cycles.keys().contains(&platform) {
            break;
        }
        cycles.insert(platform.clone(), cur_cycle);
        cur_cycle += 1;
    }
    let cycle_start = cycles[&platform];

    let cycle_length = cur_cycle - cycles[&platform];
    let platform_cycle = cycles
        .into_iter()
        .filter(|(_, i)| (cycle_start..cur_cycle).contains(i))
        .sorted_by(|(_, i), (_, j)| i.cmp(j))
        .map(|(v, _)| v)
        .collect_vec();

    Some(platform_cycle[(1_000_000_000 - cycle_start) % cycle_length].score())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }
}
