use std::collections::{HashMap, VecDeque};

advent_of_code::solution!(15);

fn hash(input: &str) -> usize {
    input
        .bytes()
        .fold(0, |cur_value, c| ((cur_value + c as usize) * 17) % 256)
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(input.replace('\n', "").split(',').map(hash).sum::<usize>())
}

#[derive(Debug, Clone)]
struct Lens<'a> {
    label: &'a str,
    focal_length: u32,
}

#[derive(Default, Debug, Clone)]
struct LensBox<'a>(VecDeque<Lens<'a>>);

impl<'a> LensBox<'a> {
    fn replace_or_add_lens(&mut self, new_lens: &Lens<'a>) {
        if let Some(index) = self.0.iter().position(|lens| lens.label == new_lens.label) {
            self.0[index] = new_lens.clone();
        } else {
            self.0.push_back(new_lens.clone());
        }
    }

    fn drop_lens(&mut self, label: &str) {
        self.0.retain(|lens| lens.label != label);
    }

    fn focusing_power(&self, box_id: usize) -> u32 {
        self.0
            .iter()
            .enumerate()
            .map(|(i, lens)| (box_id + 1) as u32 * (i + 1) as u32 * lens.focal_length)
            .sum::<u32>()
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut boxes = HashMap::<usize, LensBox>::new();
    let input = input.replace('\n', "");

    for step in input.split(',') {
        if let Some(label) = step.strip_suffix('-') {
            let box_id = hash(label);
            boxes
                .entry(box_id)
                .and_modify(|lens_box| lens_box.drop_lens(label));
        } else if let Some((label, focal_length)) = step.split_once('=') {
            let box_id = hash(label);
            let focal_length = focal_length.trim().parse::<u32>().unwrap();
            boxes.entry(box_id).or_default().replace_or_add_lens(&Lens {
                label,
                focal_length,
            });
        }
    }

    Some(
        boxes
            .into_iter()
            .map(|(id, lens_box)| lens_box.focusing_power(id))
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(145));
    }
}
