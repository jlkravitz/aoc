use std::collections::HashSet;

use itertools::Itertools;

advent_of_code::solution!(13);

struct Terrain(Vec<Vec<char>>);

impl Terrain {
    fn parse(input: &str) -> Self {
        let rows = input
            .lines()
            .map(|line| line.chars().collect_vec())
            .collect_vec();

        Self(rows)
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

    fn unsmudge(&mut self, i: usize, j: usize) {
        self.0[i][j] = if self.0[i][j] == '#' { '.' } else { '#' };
    }

    fn get_mirrors(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        (0..self.0.len() - 1).filter_map(|i| self.is_mirrored_at((i, i + 1)).then_some((i, i + 1)))
    }

    fn compute_mirror_score(
        row_mirrors: impl Iterator<Item = (usize, usize)>,
        col_mirrors: impl Iterator<Item = (usize, usize)>,
    ) -> usize {
        row_mirrors.map(|(_, j)| j).sum::<usize>() * 100
            + col_mirrors.map(|(_, j)| j).sum::<usize>()
    }

    fn is_mirrored_at(&self, split_at: (usize, usize)) -> bool {
        let mut left = split_at.0;
        let mut right = split_at.1;

        while right < self.0.len() {
            if self.0[left] != self.0[right] {
                return false;
            }
            match (left.checked_sub(1), right.checked_add(1)) {
                (Some(l), Some(r)) => {
                    left = l;
                    right = r;
                }
                _ => break,
            }
        }

        true
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let terrains = input.split("\n\n").map(Terrain::parse);

    let mirror_scores = terrains.map(|terrain| {
        Terrain::compute_mirror_score(terrain.get_mirrors(), terrain.transpose().get_mirrors())
    });

    Some(mirror_scores.sum::<usize>() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let terrains = input.split("\n\n").map(Terrain::parse);

    let mirror_scores = terrains.filter_map(|mut terrain| {
        let (original_row_mirrors, original_col_mirrors) = (
            terrain.get_mirrors().collect(),
            terrain.transpose().get_mirrors().collect(),
        );
        terrain.0.clone().iter().enumerate().find_map(|(i, row)| {
            row.iter().enumerate().find_map(|(j, _)| {
                terrain.unsmudge(i, j);
                let score = Terrain::compute_mirror_score(
                    (&terrain.get_mirrors().collect::<HashSet<_>>() - &original_row_mirrors)
                        .into_iter(),
                    (&terrain.transpose().get_mirrors().collect::<HashSet<_>>()
                        - &original_col_mirrors)
                        .into_iter(),
                );
                if score > 0 {
                    return Some(score as u32);
                }
                terrain.unsmudge(i, j);
                None
            })
        })
    });

    Some(mirror_scores.sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
