use itertools::{iproduct, Itertools};
use petgraph::algo::dijkstra;
use petgraph::graph::{DiGraph, NodeIndex};
use std::collections::HashMap;

advent_of_code::solution!(17);

type Position = (usize, usize);
type CrucibleState = (Position, Direction, usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}
impl Direction {
    fn turn_left(&self) -> Self {
        match self {
            Self::Up => Self::Left,
            Self::Left => Self::Down,
            Self::Down => Self::Right,
            Self::Right => Self::Up,
        }
    }

    fn turn_right(&self) -> Self {
        self.turn_left().turn_left().turn_left()
    }
}
struct Map(Vec<Vec<u32>>);

impl Map {
    fn parse(input: &str) -> Self {
        Map(input
            .split('\n')
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap_or_default())
                    .collect_vec()
            })
            .collect_vec())
    }

    fn crucible_states(
        &self,
        max_forward_steps: usize,
    ) -> impl Iterator<Item = CrucibleState> + '_ {
        let directions = [
            Direction::Down,
            Direction::Up,
            Direction::Left,
            Direction::Right,
        ];
        iproduct!(
            (0..self.nrows()).cartesian_product(0..self.ncols()),
            directions,
            1..=max_forward_steps
        )
    }
    fn to_graph(
        &self,
        min_forward_steps: usize,
        max_forward_steps: usize,
    ) -> (
        DiGraph<CrucibleState, u32>,
        HashMap<CrucibleState, NodeIndex>,
    ) {
        let mut graph = DiGraph::new();

        let node_indices = self
            .crucible_states(max_forward_steps)
            .map(|((row, col), direction, steps)| {
                let key = ((row, col), direction, steps);
                (key, graph.add_node(key))
            })
            .collect::<HashMap<_, _>>();

        for state in self.crucible_states(max_forward_steps) {
            self.add_edges_from(
                state,
                &mut graph,
                &node_indices,
                min_forward_steps,
                max_forward_steps,
            );
        }
        (graph, node_indices)
    }

    fn add_edges_from(
        &self,
        state: CrucibleState,
        graph: &mut DiGraph<CrucibleState, u32>,
        node_indices: &HashMap<CrucibleState, NodeIndex>,
        min_forward_steps: usize,
        max_forward_steps: usize,
    ) {
        let start_node = node_indices[&state];
        if let Some(next_position) = self.move_one(state.0, state.1) {
            if state.2 < max_forward_steps {
                graph.add_edge(
                    start_node,
                    node_indices[&(next_position, state.1, state.2 + 1)],
                    self.0[next_position.0][next_position.1],
                );
            }
        }
        if state.2 < min_forward_steps {
            return;
        }
        if let Some(next_position) = self.move_one(state.0, state.1.turn_left()) {
            graph.add_edge(
                start_node,
                node_indices[&(next_position, state.1.turn_left(), 1)],
                self.0[next_position.0][next_position.1],
            );
        }
        if let Some(next_position) = self.move_one(state.0, state.1.turn_right()) {
            graph.add_edge(
                start_node,
                node_indices[&(next_position, state.1.turn_right(), 1)],
                self.0[next_position.0][next_position.1],
            );
        }
    }

    fn lowest_heat_loss(&self, min_forward_steps: usize, max_forward_steps: usize) -> Option<u32> {
        let (graph, node_indices) = self.to_graph(min_forward_steps, max_forward_steps);

        let smallest_from = |direction: Direction| {
            let distances = dijkstra(&graph, node_indices[&((0, 0), direction, 1)], None, |e| {
                *e.weight()
            });
            node_indices
                .iter()
                .filter(|(&(position, _, _), _)| (position == (self.nrows() - 1, self.ncols() - 1)))
                .filter_map(|(_, &node_index)| distances.get(&node_index))
                .min()
                .copied()
                .unwrap_or(u32::MAX)
        };

        Some(smallest_from(Direction::Down).min(smallest_from(Direction::Right)))
    }

    fn move_one(&self, position: (usize, usize), direction: Direction) -> Option<(usize, usize)> {
        let (row, col) = position;
        match direction {
            Direction::Left => {
                if col < 1 {
                    None
                } else {
                    Some((row, col - 1))
                }
            }
            Direction::Right => {
                if col >= self.0[0].len() - 1 {
                    None
                } else {
                    Some((row, col + 1))
                }
            }
            Direction::Up => {
                if row < 1 {
                    None
                } else {
                    Some((row - 1, col))
                }
            }
            Direction::Down => {
                if row >= self.0.len() - 1 {
                    None
                } else {
                    Some((row + 1, col))
                }
            }
        }
    }

    fn nrows(&self) -> usize {
        self.0.len()
    }

    fn ncols(&self) -> usize {
        self.0[0].len()
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let map = Map::parse(input);
    map.lowest_heat_loss(0, 3)
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = Map::parse(input);
    map.lowest_heat_loss(4, 10)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(102));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(94));
    }
}
