use std::collections::{HashMap, VecDeque};

use itertools::Itertools;

advent_of_code::solution!(20);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Pulse {
    High,
    Low,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Response {
    Send(Vec<String>, Pulse),
    Noop,
}
#[derive(Debug)]
struct ModuleManager {
    modules: HashMap<String, ModuleType>,
}

impl ModuleManager {
    // write function that parses modules from string input
    // each row lists the module on the left hand side followed by a right-hand arow (->) followed by the modules output modules.
    // modules are either named "broadcaster" or prefixed with a % (flip flop module) or & (conjunction module)
    fn parse(input: &str) -> Self {
        let mut modules = HashMap::new();
        for line in input.lines() {
            let parts = line.split_once(" -> ");
            let left = parts.unwrap().0;
            let right = parts.unwrap().1;
            let destinations = right.split(", ").map(|s| s.trim().to_owned()).collect();
            let module = if left.starts_with("broadcaster") {
                ModuleType::Broadcast(BroadcastModule::new(destinations))
            } else if left.starts_with('%') {
                ModuleType::FlipFlop(FlipFlopModule::new(destinations))
            } else if left.starts_with('&') {
                ModuleType::Conjunction(ConjunctionModule::new(destinations))
            } else {
                panic!("invalid module type")
            };
            // strip prefix % or & if not broadcaster
            let left = if left.starts_with('%') || left.starts_with('&') {
                left[1..].to_owned()
            } else {
                left.to_owned()
            };
            modules.insert(left.to_owned(), module);
        }
        let module_inputs = modules
            .keys()
            .map(|name| {
                (
                    name.to_owned(),
                    modules
                        .iter()
                        .filter(|(_, m)| ((m.destinations()).contains(name)))
                        .map(|(n, _)| n.to_owned())
                        .collect_vec(),
                )
            })
            .collect_vec();
        for (name, module) in module_inputs.iter() {
            if let ModuleType::Conjunction(conjunction_module) = modules.get_mut(name).unwrap() {
                conjunction_module.set_inputs(module.clone());
            }
        }
        ModuleManager { modules }
    }

    fn press_button(&mut self, i: usize) -> (usize, usize) {
        let (mut low, mut high) = (0, 0);
        let mut queue = VecDeque::from([(
            "button".to_owned(),
            Response::Send(Vec::from(["broadcaster".to_owned()]), Pulse::Low),
        )]);
        while !queue.is_empty() {
            let (from, response) = queue.pop_front().unwrap();
            match response {
                Response::Send(destinations, signal) => {
                    if signal == Pulse::High {
                        high += destinations.len();
                    } else {
                        low += destinations.len();
                    }
                    for destination in destinations {
                        if let Some(module) = self.modules.get_mut(&destination.clone()) {
                            let response = module.receive(from.clone(), signal);
                            queue.push_back((destination.clone(), response));
                        }
                        // for part 2
                        if destination.clone() == "zh" && signal == Pulse::High {
                            println!("{} activated zh after {} presses", from, i);
                        }
                    }
                }
                Response::Noop => {}
            }
        }
        (low, high)
    }
}

#[derive(Debug)]
enum ModuleType {
    FlipFlop(FlipFlopModule),
    Conjunction(ConjunctionModule),
    Broadcast(BroadcastModule),
}

impl ModuleType {
    fn destinations(&self) -> Vec<String> {
        match self {
            ModuleType::FlipFlop(m) => m.destinations.clone(),
            ModuleType::Conjunction(m) => m.destinations.clone(),
            ModuleType::Broadcast(m) => m.destinations.clone(),
        }
    }

    fn receive(&mut self, from: String, signal: Pulse) -> Response {
        match self {
            ModuleType::FlipFlop(m) => m.receive(from, signal),
            ModuleType::Conjunction(m) => m.receive(from, signal),
            ModuleType::Broadcast(m) => m.receive(from, signal),
        }
    }
}

trait Module {
    fn receive(&mut self, from: String, signal: Pulse) -> Response;
}

#[derive(Debug)]
struct FlipFlopModule {
    state: bool,
    destinations: Vec<String>,
}
#[derive(Debug)]
struct ConjunctionModule {
    memory: HashMap<String, Pulse>,
    destinations: Vec<String>,
}
#[derive(Debug)]
struct BroadcastModule {
    destinations: Vec<String>,
}

impl FlipFlopModule {
    fn new(destinations: Vec<String>) -> Self {
        Self {
            state: false,
            destinations,
        }
    }
}

impl Module for FlipFlopModule {
    fn receive(&mut self, _from: String, signal: Pulse) -> Response {
        if signal == Pulse::Low {
            self.state = !self.state;
            return Response::Send(
                self.destinations.clone(),
                if self.state { Pulse::High } else { Pulse::Low },
            );
        }
        Response::Noop
    }
}

impl ConjunctionModule {
    fn new(destinations: Vec<String>) -> Self {
        Self {
            memory: HashMap::new(),
            //HashMap::from_iter(inputs.iter().map(|module| (module.clone(), Pulse::Low))),
            destinations,
        }
    }
    fn set_inputs(&mut self, inputs: Vec<String>) {
        self.memory = HashMap::from_iter(inputs.iter().map(|module| (module.clone(), Pulse::Low)));
    }
}

impl Module for ConjunctionModule {
    fn receive(&mut self, from: String, signal: Pulse) -> Response {
        self.memory.insert(from, signal);
        if self.memory.values().filter(|&v| *v == Pulse::Low).count() == 0 {
            Response::Send(self.destinations.clone(), Pulse::Low)
        } else {
            Response::Send(self.destinations.clone(), Pulse::High)
        }
    }
}

impl BroadcastModule {
    fn new(destinations: Vec<String>) -> Self {
        Self { destinations }
    }
}

impl Module for BroadcastModule {
    fn receive(&mut self, _from: String, signal: Pulse) -> Response {
        Response::Send(self.destinations.clone(), signal)
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut modules = ModuleManager::parse(input);

    let (low, high) = (0..1000).fold((0, 0), |total, i| {
        let (l, h) = modules.press_button(i + 1);
        (total.0 + l, total.1 + h)
    });
    Some(low * high)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut modules = ModuleManager::parse(input);
    None

    // find the number of presses required to activate the inputs to zh,
    // then manually find the LCM of the number of presses required to activate each input.
    // let n = (0..)
    //     .take_while(|i| {
    //         modules.press_button(i + 1);
    //         true
    //     })
    //     .count();
    // Some(n + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11687500));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
