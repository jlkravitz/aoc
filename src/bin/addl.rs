use core::panic;
use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    error,
    fs::File,
    io::{self, BufRead},
    ops::Range,
};

use intervaltree::IntervalTree;
use regex::Regex;

const CARD_STRENGTHS: &str = "J23456789TQKA";
// AKQJT98765432

fn main() {
    #[allow(unused_variables)]
    let input_example = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
    #[allow(unused_variables)]
    let input = &std::fs::read_to_string("input.txt").unwrap();
    match day8(input) {
        Ok(total) => println!("Total: {}", total),
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn day8(input: &str) -> Result<isize, Box<dyn error::Error>> {
    Ok(input
        .split('\n')
        .map(|line| {
            let readings = line
                .split_whitespace()
                .filter_map(|x| x.parse::<isize>().ok())
                .collect::<Vec<_>>();
            predict_previous(&readings)
        })
        .sum::<isize>())
}

fn predict_previous(readings: &[isize]) -> isize {
    if readings.iter().sum::<isize>() == 0 {
        return 0;
    }

    readings.iter().next().unwrap()
        - predict_previous(
            &readings
                .iter()
                .zip(readings.iter().skip(1))
                .map(|(x, y)| y - x)
                .collect::<Vec<_>>(),
        )
}

fn predict_next(readings: &[isize]) -> isize {
    if readings.iter().sum::<isize>() == 0 {
        return 0;
    }

    readings.last().unwrap()
        + predict_next(
            &readings
                .iter()
                .zip(readings.iter().skip(1))
                .map(|(x, y)| y - x)
                .collect::<Vec<_>>(),
        )
}

fn day7(input: &str) -> Result<usize, Box<dyn error::Error>> {
    let mut lines = input.split('\n');
    let instructions_raw = lines.next().unwrap_or("").chars();
    let instructions = instructions_raw.clone().cycle();

    lines.next();
    let edges = lines
        .flat_map(|line| {
            line.split_once(" = ").map(|(from, to)| {
                (
                    from,
                    to.strip_prefix('(')
                        .unwrap_or_default()
                        .strip_suffix(')')
                        .unwrap_or_default()
                        .split_once(", ")
                        .unwrap(),
                )
            })
        })
        .collect::<HashMap<_, _>>();

    let mut nodes = edges
        .keys()
        .filter(|node| node.ends_with('A'))
        .copied()
        .collect::<Vec<_>>();
    let mut cycle_steps = Vec::with_capacity(nodes.len());
    for (i, node) in nodes.iter_mut().enumerate() {
        for (step, instruction) in instructions.clone().enumerate() {
            match instruction {
                'R' => {
                    *node = edges.get(node).unwrap().1;
                }
                'L' => {
                    *node = edges.get(node).unwrap().0;
                }
                _ => panic!("Invalid instruction"),
            }
            if node.ends_with('Z') {
                cycle_steps.push(step + 1);
                break;
            }
        }
    }
    cycle_steps.sort();

    // let divisor = cycle_steps
    //     .into_iter()
    //     .reduce(|a, b| (a * b) / gcd(a, b))
    //     .unwrap();

    println!("{:?}", cycle_steps);
    let lcm = cycle_steps.iter().fold(1, |lcm, &item| {
        println!("{} {} {}", lcm, item, gcd(lcm, item));
        lcm * item / gcd(lcm, item)
    });

    Ok(lcm)
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandStrength {
    Nothing,
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn get_hand_strength(hand: &str) -> HandStrength {
    let mut counts = [0u32; CARD_STRENGTHS.len()];
    for card in hand.chars() {
        if let Some(index) = CARD_STRENGTHS.find(card) {
            counts[index] += 1;
        } else {
            return HandStrength::Nothing; // Invalid card case
        }
    }

    let iter = counts
        .iter()
        .enumerate()
        .filter(|(i, c)| **c != 0 && CARD_STRENGTHS.chars().nth(*i).unwrap() != 'J')
        .map(|(_, c)| c);

    match (
        iter.clone().max().unwrap_or(&0),
        iter.clone().min().unwrap_or(&0),
        counts[0],
    ) {
        (x, _, z) if x + z == 5 => HandStrength::FiveOfAKind,
        (x, _, z) if x + z == 4 => HandStrength::FourOfAKind,
        (x, y, z) if (x + y + z == 5) => HandStrength::FullHouse,
        (x, _, z) if x + z >= 3 => HandStrength::ThreeOfAKind,
        (x, _, z) if x + z >= 2 => {
            if (iter.clone().filter(|&&c| c == 2).count() + z as usize) >= 2 {
                HandStrength::TwoPair
            } else {
                HandStrength::OnePair
            }
        }
        (1, 1, _) => HandStrength::HighCard,
        _ => HandStrength::Nothing,
    }
}

fn compare_card_strengths(hand1: &str, hand2: &str) -> Ordering {
    // println!("Compare card strengths {} {}", hand1, hand2);
    if hand1 == hand2 {
        return Ordering::Equal;
    }

    let hand1_strength = CARD_STRENGTHS.find(hand1.chars().next().unwrap()).unwrap();
    let hand2_strength = CARD_STRENGTHS.find(hand2.chars().next().unwrap()).unwrap();
    // println!("{} {} {} {}", hand1, hand2, hand1_strength, hand2_strength);
    match hand1_strength.cmp(&hand2_strength) {
        Ordering::Equal => {
            // println!("EQUAL");
            compare_card_strengths(hand1.split_at(1).1, hand2.split_at(1).1)
        }
        ordering => ordering,
    }
}

fn compare_hands(hand1: &str, hand2: &str) -> Ordering {
    if hand1 == hand2 {
        return Ordering::Equal;
    }

    match get_hand_strength(hand1).cmp(&get_hand_strength(hand2)) {
        Ordering::Equal => {
            // println!("{:?}", compare_card_strengths(hand1, hand2));
            compare_card_strengths(hand1, hand2)
        }
        ordering => ordering,
    }
}

fn day6(input: &str) -> Result<u32, Box<dyn error::Error>> {
    let mut hands = input
        .split('\n')
        .map(|line| {
            line.split_once(' ')
                .map(|(hand, bid)| (hand, bid.parse::<u32>().unwrap()))
                .unwrap()
        })
        .collect::<Vec<_>>();
    hands.sort_by(|(hand1, _), (hand2, _)| compare_hands(hand1, hand2));
    // println!("{:?}", hands);
    Ok(hands
        .iter()
        .enumerate()
        .map(|(i, (hand, bid))| {
            println!(
                "Hand {}, Type {:?}, Bid {}, Rank {}, Add {}",
                hand,
                get_hand_strength(hand),
                bid,
                i + 1,
                bid * (i as u32 + 1)
            );
            bid * (i as u32 + 1)
        })
        .sum())
}

#[allow(dead_code)]
fn day5() -> Result<u64, Box<dyn error::Error>> {
    let file = std::fs::read_to_string("input.txt")?;
    let mut lines = file.split('\n');

    let seeds: Vec<_> = lines
        .next()
        .ok_or("First line is missing.")?
        .split(' ')
        .skip(1)
        .map(|num| num.parse::<u64>().unwrap())
        .collect();

    let seeds = seeds
        .chunks(2)
        .flat_map(|chunk| chunk[0]..(chunk[0] + chunk[1]));

    let maps: Vec<IntervalTree<u64, u64>> = lines
        .map(|line| {
            line.split(' ')
                .filter_map(|num| num.parse::<u64>().ok())
                .collect::<Vec<_>>()
        })
        .fold(vec![], |mut acc: Vec<HashMap<Range<u64>, u64>>, row| {
            if !row.is_empty() {
                acc.last_mut()
                    .unwrap()
                    .insert(row[1]..(row[1] + row[2]), row[0]);
            } else if acc.is_empty() || !acc.last_mut().unwrap().is_empty() {
                acc.push(HashMap::new());
            }
            acc
        })
        .into_iter()
        .map(IntervalTree::from_iter)
        .collect();

    Ok(seeds
        .map(|seed| {
            maps.iter()
                .fold(seed, |key, map| match map.query_point(key).next() {
                    Some(node) => node.value + (key - node.range.start),
                    None => key,
                })
        })
        .min()
        .unwrap())
}

fn day4() -> Result<u32, Box<dyn error::Error>> {
    let file = File::open("input.txt")?;
    let sum = io::BufReader::new(file)
        .lines()
        .map(|card| {
            card.unwrap()
                .split_once(':')
                .and_then(|(_, numbers)| {
                    let card_numbers = numbers
                        .trim()
                        .split('|')
                        .map(|line| {
                            line.split_whitespace()
                                .filter_map(|num| num.parse::<u32>().ok())
                                .collect::<HashSet<_>>()
                        })
                        .collect::<Vec<_>>();

                    card_numbers.get(0).and_then(|first_set| {
                        card_numbers
                            .get(1)
                            .map(|second_set| first_set.intersection(second_set).count())
                    })
                })
                .unwrap_or(0)
        })
        .map(|num_winning_numbers| {
            if num_winning_numbers > 0 {
                2u32.pow(num_winning_numbers as u32 - 1)
            } else {
                0
            }
        })
        .sum();
    Ok(sum)
}

#[allow(dead_code)]
fn day3_part2() -> Result<u32, Box<dyn error::Error>> {
    let file = File::open("input.txt")?;
    //     let matrix: Vec<Vec<char>> = "467..114..
    // ...*......
    // ..35..633.
    // ......#...
    // 617*......
    // .....+.58.
    // ..592.....
    // ......755.
    // ...$.*....
    // .664.598.."
    //         .split("\n")
    //         .map(|line| line.chars().collect())
    //         .collect();

    let matrix: Vec<Vec<char>> = io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();

    let regex = Regex::new(r"\d+")?;
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
    println!("{:?}", number_at_index);
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
                println!("{:?}", numbers);
                if numbers.len() == 2 {
                    sum += numbers[0] * numbers[1];
                }
            }
        }
    });
    Ok(sum)
}

#[allow(dead_code)]
fn day3_part1() -> Result<u32, Box<dyn error::Error>> {
    let file = File::open("input.txt")?;
    let matrix: Vec<Vec<char>> = io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap().chars().collect())
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
    let regex = Regex::new(r"\d+")?;

    Ok(matrix
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
        .sum())
}

#[allow(dead_code)]
fn day1() -> i32 {
    let re = Regex::new(r"one|two|three|four|five|six|seven|eight|nine|\d").unwrap();
    let re_bwd = Regex::new(r"eno|owt|eerht|ruof|evif|xis|neves|thgie|enin|\d").unwrap();

    let to_number = |word: &str| match word {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => word.parse::<i32>().unwrap(),
    };

    std::fs::read_to_string("input.txt")
        .unwrap()
        .split('\n')
        .map(|line| {
            let first = re.find_iter(line).next().unwrap().as_str();
            let second = re_bwd
                .find_iter(line.chars().rev().collect::<String>().as_str())
                .next()
                .unwrap()
                .as_str()
                .chars()
                .rev()
                .collect::<String>();

            format!("{}{}", to_number(first), to_number(second.as_str()))
                .parse::<i32>()
                .unwrap()
        })
        .sum::<i32>()
}
