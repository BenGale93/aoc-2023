use std::{
    collections::{HashMap, VecDeque},
    path::Path,
};

use aoc_utils::{puzzle_input_lines, Cli};

fn main() {
    let part_two = Cli::parse_args().part_two;

    let result = if part_two {
        todo!()
    } else {
        pulse_multiple("input")
    };
    println!("Puzzle result: {result}");
}

fn pulse_multiple(input: impl AsRef<Path>) -> usize {
    let mut module_map = parse_puzzle(input);
    let mut pulse_count = PulseCounter { low: 0, high: 0 };
    for _ in 0..1000 {
        let mut queue = VecDeque::new();
        queue.push_back(("broadcaster".to_owned(), "button".to_owned(), Pulse::Low));

        while let Some((destination, incoming, pulse)) = queue.pop_front() {
            pulse_count.increment(pulse);
            let module = module_map.get_mut(&destination);

            let module = match module {
                Some(m) => m,
                None => continue,
            };

            match &mut module.type_ {
                ModuleType::Broadcaster => {
                    for next_dest in &module.destinations {
                        queue.push_back((next_dest.to_owned(), destination.to_owned(), pulse));
                    }
                }
                ModuleType::Conjunction(ref mut c) => {
                    let output = c.process(&incoming, pulse);
                    for next_dest in &module.destinations {
                        queue.push_back((next_dest.to_owned(), destination.to_owned(), output));
                    }
                }
                ModuleType::FlipFlop(ref mut f) => {
                    let output = f.process(pulse);
                    if let Some(p) = output {
                        for next_dest in &module.destinations {
                            queue.push_back((next_dest.to_owned(), destination.to_owned(), p));
                        }
                    }
                }
            };
        }
    }

    pulse_count.pulse_multiple()
}

type Name = String;
type Destinations = Vec<Name>;

#[derive(Debug, Clone, PartialEq, Eq)]
struct FlipFlop {
    on: bool,
}

impl FlipFlop {
    fn new() -> Self {
        Self { on: false }
    }

    fn process(&mut self, in_pulse: Pulse) -> Option<Pulse> {
        match in_pulse {
            Pulse::High => None,
            Pulse::Low => {
                if self.on {
                    self.on = false;
                    Some(Pulse::Low)
                } else {
                    self.on = true;
                    Some(Pulse::High)
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Conjunction {
    incoming: HashMap<String, Pulse>,
}

impl Conjunction {
    fn new() -> Self {
        Self {
            incoming: HashMap::new(),
        }
    }

    fn process(&mut self, incoming: &str, in_pulse: Pulse) -> Pulse {
        self.incoming.insert(incoming.to_owned(), in_pulse);

        if self.incoming.values().all(|p| matches!(p, Pulse::High)) {
            Pulse::Low
        } else {
            Pulse::High
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ModuleType {
    Broadcaster,
    FlipFlop(FlipFlop),
    Conjunction(Conjunction),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Module {
    type_: ModuleType,
    destinations: Destinations,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Pulse {
    High,
    Low,
}

#[derive(Debug, Copy, Clone)]
struct PulseCounter {
    low: usize,
    high: usize,
}

impl PulseCounter {
    fn increment(&mut self, pulse: Pulse) {
        match pulse {
            Pulse::High => self.high += 1,
            Pulse::Low => self.low += 1,
        };
    }

    fn pulse_multiple(&self) -> usize {
        self.low * self.high
    }
}

fn parse_puzzle(input: impl AsRef<Path>) -> HashMap<Name, Module> {
    let puzzle_lines = puzzle_input_lines(input);
    let mut module_map = HashMap::new();
    for line in puzzle_lines {
        let line = line.unwrap();
        let config: Vec<_> = line.split(" -> ").collect();
        let destinations: Vec<String> = config
            .last()
            .unwrap()
            .split(", ")
            .map(|s| s.to_string())
            .collect();
        let module_name = config.first().unwrap();
        let (name, module) = if module_name == &"broadcaster" {
            (
                module_name.to_string(),
                Module {
                    type_: ModuleType::Broadcaster,
                    destinations,
                },
            )
        } else if module_name.contains('%') {
            (
                module_name.strip_prefix('%').unwrap().to_string(),
                Module {
                    type_: ModuleType::FlipFlop(FlipFlop::new()),
                    destinations,
                },
            )
        } else {
            (
                module_name.strip_prefix('&').unwrap().to_string(),
                Module {
                    type_: ModuleType::Conjunction(Conjunction::new()),
                    destinations,
                },
            )
        };
        module_map.insert(name, module);
    }

    for (name, module) in &module_map.clone() {
        for destination in &module.destinations {
            if let Some(dest_module) = module_map.get_mut(destination) {
                if let ModuleType::Conjunction(c) = &mut dest_module.type_ {
                    c.incoming.insert(name.to_owned(), Pulse::Low);
                };
            };
        }
    }

    module_map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example_one() {
        let result = pulse_multiple("test1_part1");
        assert_eq!(result, 32000000);
    }

    #[test]
    fn part_one_example_two() {
        let result = pulse_multiple("test2_part1");
        assert_eq!(result, 11687500);
    }
}
