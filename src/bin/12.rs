use std::{borrow::BorrowMut, collections::HashMap};

use itertools::Itertools;

advent_of_code::solution!(12);

struct Field(Vec<Row>);

struct Row {
    springs: String,
    groups: Vec<usize>,
}

impl Row {
    fn parse(line: &str, repeat: usize) -> Option<Self> {
        let (springs, groups) = line.split_once(' ')?;
        Some(Self {
            springs: [springs].repeat(repeat).join("?"),
            groups: groups
                .split(',')
                .filter_map(|num| num.parse::<usize>().ok())
                .collect_vec()
                .repeat(repeat),
        })
    }

    fn count_arrangements(&self) -> u64 {
        let mut cache = HashMap::new();
        Self::count_arrangements_helper(&self.springs, &self.groups, false, 0, cache.borrow_mut())
    }

    fn count_arrangements_helper(
        springs: &str,
        groups: &Vec<usize>,
        in_group: bool,
        group_index: usize,
        cache: &mut HashMap<(String, Vec<usize>), u64>,
    ) -> u64 {
        let key = (springs.to_owned(), groups[group_index..].to_vec());
        if let Some(&count) = cache.get(&key) {
            return count;
        }
        let count = match springs.chars().next() {
            None if group_index == groups.len() => 1,
            None if in_group && groups[group_index] == 0 => {
                Self::count_arrangements_helper(springs, groups, in_group, group_index + 1, cache)
            }
            Some('.') if !in_group || group_index == groups.len() => {
                Self::count_arrangements_helper(&springs[1..], groups, false, group_index, cache)
            }
            Some('.') if in_group && groups[group_index] == 0 => Self::count_arrangements_helper(
                &springs[1..],
                groups,
                false,
                group_index + 1,
                cache,
            ),
            Some('#') if group_index < groups.len() && groups[group_index] > 0 => {
                let mut groups_adj = groups.clone();
                groups_adj[group_index] -= 1;
                Self::count_arrangements_helper(
                    &springs[1..],
                    &groups_adj,
                    true,
                    group_index,
                    cache,
                )
            }
            Some('?') => {
                Self::count_arrangements_helper(
                    ("#".to_owned() + &springs[1..]).as_str(),
                    groups,
                    in_group,
                    group_index,
                    cache,
                ) + Self::count_arrangements_helper(
                    (".".to_owned() + &springs[1..]).as_str(),
                    groups,
                    in_group,
                    group_index,
                    cache,
                )
            }
            _ => 0,
        };
        cache.insert(key, count);
        count
    }
}

impl Field {
    fn parse(input: &str, repeat: usize) -> Self {
        Self(
            input
                .split('\n')
                .filter_map(|line| Row::parse(line, repeat))
                .collect_vec(),
        )
    }

    fn count_arrangements(&self) -> u64 {
        self.0.iter().map(|row| row.count_arrangements()).sum()
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let field = Field::parse(input, 1);
    Some(field.count_arrangements())
}

pub fn part_two(input: &str) -> Option<u64> {
    let field = Field::parse(input, 5);
    Some(field.count_arrangements())
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
