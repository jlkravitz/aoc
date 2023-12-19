use std::fmt;

use itertools::Itertools;

advent_of_code::solution!(11);

#[derive(Debug, Clone)]
struct Image(Vec<Vec<char>>);

impl fmt::Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.0 {
            for c in row {
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Image {
    fn new(input: &str) -> Self {
        let data = input
            .split('\n')
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect();
        Image(data)
    }

    fn transpose(&self) -> Image {
        let mut columns = vec![vec![]; self.0[0].len()];
        for row in &self.0 {
            for (i, c) in row.iter().enumerate() {
                columns[i].push(*c);
            }
        }
        Image(columns)
    }

    fn get_empty_rows(&self) -> Vec<usize> {
        self.0
            .iter()
            .enumerate()
            .filter_map(|(i, row)| row.iter().all(|&c| c == '.').then_some(i))
            .collect::<Vec<_>>()
    }

    fn galaxies(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        self.0.iter().enumerate().flat_map(|(i, row)| {
            row.iter().enumerate().filter_map(
                move |(j, c)| {
                    if *c == '#' {
                        Some((i, j))
                    } else {
                        None
                    }
                },
            )
        })
    }

    fn shortest_paths(&self, expansion_factor: usize) -> impl Iterator<Item = u64> + '_ {
        let empty_rows = self.get_empty_rows();
        let empty_cols = self.transpose().get_empty_rows();

        self.galaxies().combinations(2).map(move |pair| {
            let (row_max, row_min) = (pair[0].0.max(pair[1].0), pair[0].0.min(pair[1].0));
            let (col_max, col_min) = (pair[0].1.max(pair[1].1), pair[0].1.min(pair[1].1));

            let expanded_rows = empty_rows
                .clone()
                .into_iter()
                .filter(|row| (row_min + 1..row_max).contains(row))
                .count();

            let expanded_cols = empty_cols
                .clone()
                .into_iter()
                .filter(|col| (col_min + 1..col_max).contains(col))
                .count();

            ((row_max - row_min)
                + (col_max - col_min)
                + expanded_rows * (expansion_factor - 1)
                + expanded_cols * (expansion_factor - 1)) as u64
        })
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let image = Image::new(input);
    Some(image.shortest_paths(2).sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(Image::new(input).shortest_paths(1000000).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
