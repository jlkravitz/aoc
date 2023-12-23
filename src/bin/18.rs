use regex::Regex;

advent_of_code::solution!(18);

struct DigPlan {
    vertices: Vec<(i64, i64)>,
    length: usize,
}

struct Action {
    dir: char,
    n: i64,
}
impl DigPlan {
    fn action_from_raw(dir: &str, n: &str, _: &str) -> Action {
        let dir = dir.chars().next().unwrap();
        let n = n.parse::<i64>().unwrap();
        Action { dir, n }
    }

    fn action_from_color(_: &str, _: &str, color: &str) -> Action {
        let dir = match color.chars().nth(5).unwrap() {
            '0' => 'R',
            '1' => 'D',
            '2' => 'L',
            '3' => 'U',
            _ => panic!("Invalid direction"),
        };
        let n = i64::from_str_radix(&color[..5], 16).unwrap();
        Action { dir, n }
    }

    fn parse(input: &str, get_action: fn(&str, &str, &str) -> Action) -> Self {
        let re: Regex = Regex::new(r"(R|D|U|L) (\d+) \(#(......)\)").unwrap();
        let mut vertices = Vec::from([(0, 0)]);
        let mut length = 0;

        for c in re.captures_iter(input).map(|c| c.extract::<3>()) {
            let Action { n, dir } = get_action(c.1[0], c.1[1], c.1[2]);
            length += n as usize;
            let cur_vertex = vertices.last().unwrap();
            vertices.push(match dir {
                'R' => (cur_vertex.0 + n, cur_vertex.1),
                'D' => (cur_vertex.0, cur_vertex.1 - n),
                'U' => (cur_vertex.0, cur_vertex.1 + n),
                'L' => (cur_vertex.0 - n, cur_vertex.1),
                _ => panic!("Invalid direction"),
            });
        }

        vertices.pop(); // Remove the last vertex, which is the same as the first

        Self { vertices, length }
    }

    /**
     * Shoelace formula with Pick's Theorem.
     * See: https://en.wikipedia.org/wiki/Shoelace_formula
     *      https://en.wikipedia.org/wiki/Pick%27s_theorem
     *      https://www.mathed.page/geometry-labs/pick/#:~:text=Pick's%20formula%20for%20the%20area,the%20area%20should%20be%208.5.
     */
    fn area(&self) -> u64 {
        let (x, y): (Vec<_>, Vec<_>) = self.vertices.clone().into_iter().unzip();
        let sum1 = x
            .iter()
            .zip(y.iter().cycle().skip(1))
            .take(self.vertices.len())
            .map(|(x, y)| x * y)
            .sum::<i64>();

        let sum2 = y
            .iter()
            .zip(x.iter().cycle().skip(1))
            .take(self.vertices.len())
            .map(|(y, x)| y * x)
            .sum::<i64>();

        sum2.abs_diff(sum1) / 2 + 1 + self.length as u64 / 2
    }
}
pub fn part_one(input: &str) -> Option<u64> {
    let plan = DigPlan::parse(input, DigPlan::action_from_raw);
    Some(plan.area())
}

pub fn part_two(input: &str) -> Option<u64> {
    let plan = DigPlan::parse(input, DigPlan::action_from_color);
    Some(plan.area())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(62));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(952408144115));
    }
}
