use std::collections::{HashMap, HashSet};

advent_of_code::solution!(4);

struct Card {
    id: usize,
    numbers: HashSet<u32>,
    winners: HashSet<u32>,
}

impl Card {
    fn parse_numbers_list(numbers: &str) -> HashSet<u32> {
        numbers
            .split_whitespace()
            .filter_map(|num| num.parse::<u32>().ok())
            .collect::<HashSet<_>>()
    }

    fn parse(card: &str) -> Option<Self> {
        card.split_once(':').and_then(|(id_str, numbers)| {
            let id = id_str.split_whitespace().last()?.parse::<usize>().ok()?;
            numbers.trim().split_once('|').map(|(x, y)| Self {
                id,
                numbers: Card::parse_numbers_list(x),
                winners: Card::parse_numbers_list(y),
            })
        })
    }

    fn num_matching_numbers(&self) -> usize {
        self.numbers.intersection(&self.winners).count()
    }
    fn score(&self) -> u32 {
        let num_matching = self.num_matching_numbers();
        if num_matching > 0 {
            2u32.pow(num_matching as u32 - 1)
        } else {
            0
        }
    }
}

struct Cards(Vec<Card>);

impl Cards {
    fn parse(input: &str) -> Option<Self> {
        Some(Self(
            input
                .split('\n')
                .filter_map(Card::parse)
                .collect::<Vec<_>>(),
        ))
    }

    fn score(&self) -> u32 {
        self.0.iter().map(Card::score).sum()
    }

    fn count(&self) -> u32 {
        let mut card_copies = HashMap::new();
        for card in self.0.iter() {
            let num_copies = card_copies.get(&card.id).unwrap_or(&0) + 1;
            let num_matching = card.num_matching_numbers();
            for j in 1..=num_matching {
                card_copies
                    .entry(card.id + j)
                    .and_modify(|x| *x += num_copies)
                    .or_insert(num_copies);
            }
        }
        card_copies.values().sum::<u32>() + self.0.len() as u32
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let cards = Cards::parse(input)?;
    Some(cards.score())
}

pub fn part_two(input: &str) -> Option<u32> {
    let cards = Cards::parse(input)?;
    Some(cards.count())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
