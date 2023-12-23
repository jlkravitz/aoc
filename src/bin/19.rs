use core::panic;
use std::collections::{HashMap, VecDeque};

use regex::Regex;

advent_of_code::solution!(19);

type Constraint = (u64, u64);
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct PartRange {
    x: Constraint,
    m: Constraint,
    a: Constraint,
    s: Constraint,
}

impl PartRange {
    fn new((min, max): Constraint) -> Self {
        PartRange {
            x: (min, max),
            m: (min, max),
            a: (min, max),
            s: (min, max),
        }
    }
    fn count_possible_parts(&self) -> u64 {
        (self.x.1 - self.x.0 + 1)
            * (self.m.1 - self.m.0 + 1)
            * (self.a.1 - self.a.0 + 1)
            * (self.s.1 - self.s.0 + 1)
    }

    fn get_rating(&self, category: char) -> Constraint {
        match category {
            'x' => self.x,
            'm' => self.m,
            'a' => self.a,
            's' => self.s,
            _ => panic!("Unknown category {}", category),
        }
    }

    fn with(&self, category: char, constraint: Constraint) -> Self {
        match category {
            'x' => PartRange {
                x: constraint,
                m: self.m,
                a: self.a,
                s: self.s,
            },
            'm' => PartRange {
                x: self.x,
                m: constraint,
                a: self.a,
                s: self.s,
            },
            'a' => PartRange {
                x: self.x,
                m: self.m,
                a: constraint,
                s: self.s,
            },
            's' => PartRange {
                x: self.x,
                m: self.m,
                a: self.a,
                s: constraint,
            },
            _ => panic!("Unknown category {}", category),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Part {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}

impl Part {
    fn parse(input: &str) -> Option<Self> {
        let re = Regex::new(r"\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)\}").unwrap();
        re.captures(input).map(|caps| Part {
            x: caps[1].parse().unwrap(),
            m: caps[2].parse().unwrap(),
            a: caps[3].parse().unwrap(),
            s: caps[4].parse().unwrap(),
        })
    }
    fn rating(&self, category: char) -> u64 {
        match category {
            'x' => self.x,
            'm' => self.m,
            'a' => self.a,
            's' => self.s,
            _ => panic!("Unknown category {}", category),
        }
    }
}

type State = String;

// px{a<2006:qkq,m>2090:A,rfg}
// pv{a>1716:R,A}
// lnx{m>1548:A,A}
// impl State {
//     fn parse(input: &str) {
//         let re = Regex::new(r"(?<from>\w+)\{(?<transitions>((?<category>x|m|a|s)(?<comparison>>|<)(?<number>\d+):(?<to>\w+),)*)(?<else>\w+)\}").unwrap();

//         let caps = re.captures_iter(input).collect::<Vec<_>>();
//         println!("{:?}", caps);
//     }
// }

#[derive(Debug, Clone)]
struct Transition {
    category: char,
    comparison: char,
    number: u64,
    to: State,
}

impl Transition {
    fn try_transition(&self, part: Part) -> Option<State> {
        match part.rating(self.category).cmp(&self.number) {
            std::cmp::Ordering::Less if self.comparison == '<' => Some(self.to.clone()),
            std::cmp::Ordering::Greater if self.comparison == '>' => Some(self.to.clone()),
            _ => None,
        }
    }

    fn transition_part_range(&self, part_range: PartRange) -> (PartRange, PartRange) {
        let (cur_min, cur_max) = part_range.get_rating(self.category);
        if self.comparison == '<' {
            (
                part_range.with(self.category, (cur_min, cur_max.min(self.number - 1))),
                part_range.with(self.category, (cur_min.max(self.number), cur_max)),
            )
        } else if self.comparison == '>' {
            (
                part_range.with(self.category, (cur_min.max(self.number + 1), cur_max)),
                part_range.with(self.category, (cur_min, cur_max.min(self.number))),
            )
        } else {
            panic!("Unknown comparison {}", self.comparison);
        }
    }

    fn parse(input: &str) -> Option<Self> {
        let re = Regex::new(r"(?P<category>x|m|a|s)(?P<comparison>>|<)(?P<number>\d+):(?P<to>\w+)")
            .unwrap();
        re.captures(input).map(|caps| Transition {
            category: caps["category"].chars().next().unwrap(),
            comparison: caps["comparison"].chars().next().unwrap(),
            number: caps["number"].parse().unwrap(),
            to: caps["to"].to_string(),
        })
    }
}

struct StateMachine {
    transitions: HashMap<State, Vec<Transition>>,
    defaults: HashMap<State, State>,
}

impl StateMachine {
    fn parse(input: &str) -> Self {
        let re = Regex::new(r"(?<from>\w+)\{(?<transitions>.+,)*(?<else>\w+)\}").unwrap();
        let (transitions, defaults): (HashMap<_, _>, HashMap<_, _>) = re
            .captures_iter(input)
            .map(|caps| {
                let from = caps.name("from").unwrap().as_str().to_owned();
                let else_state = caps.name("else").unwrap().as_str().to_owned();
                let transitions = caps
                    .name("transitions")
                    .unwrap()
                    .as_str()
                    .split(',')
                    .filter_map(Transition::parse)
                    .collect::<Vec<_>>();
                ((from.clone(), transitions), (from, else_state))
            })
            .unzip();

        StateMachine {
            transitions,
            defaults,
        }
    }

    fn transition_part(&self, from: State, part: Part) -> State {
        self.transitions[&from]
            .iter()
            .find_map(|t| t.try_transition(part))
            .unwrap_or(self.defaults[&from].clone())
    }

    fn transition_part_range(&self, from: State, part_range: PartRange) -> Vec<(State, PartRange)> {
        let (mut all_part_ranges, part_range_on_fail) = self.transitions[&from].iter().fold(
            (vec![], part_range),
            |(mut all_part_ranges, part_range_on_fail), t| {
                let (success, fail) = t.transition_part_range(part_range_on_fail);
                all_part_ranges.push((t.to.clone(), success));
                (all_part_ranges, fail)
            },
        );
        all_part_ranges.push((self.defaults[&from].clone(), part_range_on_fail));
        all_part_ranges
    }

    fn consume(&self, part: Part) -> bool {
        let mut state = "in".to_owned();
        while state != "A" && state != "R" {
            state = self.transition_part(state, part);
        }
        state == "A"
    }

    fn count_accepted_states(&self) -> u64 {
        let mut queue = VecDeque::from([("in".to_owned(), PartRange::new((1, 4000)))]);
        let mut count = 0;
        while let Some((state, part_range)) = queue.pop_front() {
            if state == "A" {
                count += part_range.count_possible_parts();
            } else if state != "R" {
                queue.extend(self.transition_part_range(state, part_range));
            }
        }
        count
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let (states_input, parts_input) = input.split_once("\n\n").unwrap();
    let machine = StateMachine::parse(states_input);
    Some(
        parts_input
            .split('\n')
            .filter_map(Part::parse)
            .filter(|&p| machine.consume(p))
            .map(|p| p.x + p.m + p.a + p.s)
            .sum::<u64>(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let (states_input, _) = input.split_once("\n\n").unwrap();
    let machine = StateMachine::parse(states_input);
    Some(machine.count_accepted_states())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(19114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(167409079868000));
    }
}
