use std::collections::HashMap;

advent_of_code::solution!(2);

#[derive(Debug)]
struct Game {
    id: u32,
    rounds: Vec<Round>,
}
#[derive(Debug, Clone, Default)]
struct Round {
    red: u32,
    blue: u32,
    green: u32,
}

impl Round {
    fn max(self, other: Self) -> Self {
        Self {
            red: self.red.max(other.red),
            blue: self.blue.max(other.blue),
            green: self.green.max(other.green),
        }
    }

    fn power(self) -> u32 {
        self.red * self.blue * self.green
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .split('\n')
            .filter_map(parse_game)
            .filter_map(|game| {
                let min_cubes = game
                    .rounds
                    .into_iter()
                    .reduce(Round::max)
                    .unwrap_or_default();
                (min_cubes.blue <= 14 && min_cubes.red <= 12 && min_cubes.green <= 13)
                    .then_some(game.id)
            })
            .sum::<u32>(),
    )
}

fn parse_round(round: &str) -> Round {
    let round = round
        .trim()
        .split(", ")
        .filter_map(|color_str| {
            let mut parts = color_str.split_whitespace();
            let value = parts.next()?.parse().ok()?;
            let color = parts.next()?;
            Some((color, value))
        })
        .collect::<HashMap<&str, u32>>();

    Round {
        red: *round.get("red").unwrap_or(&0),
        blue: *round.get("blue").unwrap_or(&0),
        green: *round.get("green").unwrap_or(&0),
    }
}

fn parse_game(row: &str) -> Option<Game> {
    row.split_once(':').and_then(|(label, rounds)| {
        let id = label
            .split_once(' ')
            .and_then(|(_, id)| id.parse::<u32>().ok())?;
        let rounds = rounds
            .trim()
            .split(';')
            .map(parse_round)
            .collect::<Vec<_>>();
        Some(Game { id, rounds })
    })
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .split('\n')
            .filter_map(parse_game)
            .map(|game| {
                game.rounds
                    .into_iter()
                    .reduce(Round::max)
                    .unwrap_or_default()
                    .power()
            })
            .sum::<u32>(),
    )
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
        assert_eq!(result, Some(2286));
    }
}
