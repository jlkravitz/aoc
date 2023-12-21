use std::collections::HashSet;

use itertools::Itertools;

advent_of_code::solution!(16);

#[derive(Debug, Clone)]
struct LightGrid {
    map: Vec<Vec<GridItem>>,
}

#[derive(Debug, Clone, Copy)]
enum GridItem {
    HorizontalPipe,
    VerticalPipe,
    RightAngledMirror,
    LeftAngledMirror,
    Nothing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct LightBeam {
    row: usize,
    col: usize,
    direction: Direction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn_left(&self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        }
    }
    fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

impl LightBeam {
    fn set_direction(&self, direction: &Direction) -> Self {
        Self {
            row: self.row,
            col: self.col,
            direction: *direction,
        }
    }

    fn step_in(&self, grid: &LightGrid) -> Vec<Self> {
        let dirs = match (grid.map[self.row][self.col], self.direction) {
            (GridItem::HorizontalPipe, Direction::Up | Direction::Down) => {
                vec![Direction::Left, Direction::Right]
            }
            (GridItem::VerticalPipe, Direction::Left | Direction::Right) => {
                vec![Direction::Up, Direction::Down]
            }
            (GridItem::RightAngledMirror, Direction::Up | Direction::Down)
            | (GridItem::LeftAngledMirror, Direction::Right | Direction::Left) => {
                vec![self.direction.turn_right()]
            }
            (GridItem::RightAngledMirror, Direction::Right | Direction::Left)
            | (GridItem::LeftAngledMirror, Direction::Up | Direction::Down) => {
                vec![self.direction.turn_left()]
            }
            _ => vec![self.direction],
        };

        dirs.iter()
            .filter_map(|dir| self.clone().set_direction(dir).move_one_in(grid))
            .collect()
    }

    fn move_in(&self, drow: i32, dcol: i32, grid: &LightGrid) -> Option<Self> {
        if (self.row == 0 && drow < 0)
            || (self.row == grid.map.len() - 1 && drow > 0)
            || (self.col == 0 && dcol < 0)
            || (self.col == grid.map[0].len() - 1 && dcol > 0)
        {
            return None;
        }

        Some(Self {
            row: (self.row as i32 + drow) as usize,
            col: (self.col as i32 + dcol) as usize,
            direction: self.direction,
        })
    }

    fn move_one_in(&self, grid: &LightGrid) -> Option<Self> {
        match self.direction {
            Direction::Up => self.move_in(-1, 0, grid),
            Direction::Down => self.move_in(1, 0, grid),
            Direction::Left => self.move_in(0, -1, grid),
            Direction::Right => self.move_in(0, 1, grid),
        }
    }
}
impl LightGrid {
    fn parse(input: &str) -> Self {
        let mut grid = Vec::new();
        for line in input.lines() {
            let mut row = Vec::new();
            for c in line.chars() {
                row.push(match c {
                    '-' => GridItem::HorizontalPipe,
                    '|' => GridItem::VerticalPipe,
                    '/' => GridItem::RightAngledMirror,
                    '\\' => GridItem::LeftAngledMirror,
                    '.' => GridItem::Nothing,
                    _ => panic!("Invalid character in input"),
                });
            }
            grid.push(row);
        }
        Self { map: grid }
    }

    fn launch_beam(&self, row: usize, col: usize, direction: Direction) -> usize {
        let beam = LightBeam {
            row,
            col,
            direction,
        };

        let mut beam_path = HashSet::from([beam]);
        let mut active_beams = vec![beam];

        while !active_beams.is_empty() {
            active_beams = active_beams
                .iter()
                .flat_map(|b| b.step_in(self))
                .filter(|b| !beam_path.contains(b))
                .collect::<Vec<_>>();
            beam_path.extend(active_beams.iter());
        }
        let energized_tiles = beam_path
            .into_iter()
            .sorted_by(|a, b| a.row.cmp(&b.row).then(a.col.cmp(&b.col)))
            .dedup_by(|a, b| a.row == b.row && a.col == b.col)
            .collect::<Vec<_>>();

        energized_tiles.len()
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = LightGrid::parse(input);
    Some(grid.launch_beam(0, 0, Direction::Right) as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = LightGrid::parse(input);
    let maximum_energized_tiles = (0..grid.map.len())
        .flat_map(|row| {
            [
                grid.launch_beam(row, 0, Direction::Right),
                grid.launch_beam(row, grid.map[row].len() - 1, Direction::Left),
            ]
        })
        .chain((0..grid.map[0].len()).flat_map(|col| {
            [
                grid.launch_beam(0, col, Direction::Down),
                grid.launch_beam(grid.map.len() - 1, col, Direction::Up),
            ]
        }))
        .max()
        .unwrap_or_default();

    Some(maximum_energized_tiles as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
