use regex::Regex;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"\d").unwrap();
    Some(
        input
            .split('\n')
            .map(|line| {
                let mut it = re.find_iter(line);
                let first = it.next().unwrap();
                format!("{}{}", first.as_str(), it.last().unwrap_or(first).as_str())
                    .parse::<u32>()
                    .unwrap()
            })
            .sum::<u32>(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
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

    Some(
        input
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
                    .parse::<u32>()
                    .unwrap()
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
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
