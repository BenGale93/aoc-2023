use std::{
    collections::{HashMap, VecDeque},
    ops::Range,
    path::Path,
};

use aoc_utils::Cli;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, digit1, one_of},
    sequence::tuple,
    IResult,
};

fn main() {
    let part_two = Cli::parse_args().part_two;

    let result = if part_two {
        rating_combinations("input")
    } else {
        rating_sum("input")
    };
    println!("Puzzle result: {result}");
}

fn rating_sum(input: impl AsRef<Path>) -> usize {
    let (workflows, parts) = parse_puzzle(input);
    let mut accepted_sum: usize = 0;
    for part in &parts {
        let mut destination = "in".to_string();
        loop {
            let workflow = workflows.get(&destination).unwrap();
            destination = evaluate_workflow(workflow, part);
            if &destination == "A" {
                accepted_sum += part.total_rating();
                break;
            } else if &destination == "R" {
                break;
            }
        }
    }

    accepted_sum
}

fn rating_combinations(input: impl AsRef<Path>) -> usize {
    let (workflows, _) = parse_puzzle(input);
    let start_range = PartRange::new();
    let mut queue = VecDeque::new();
    queue.push_back(("in".to_owned(), start_range));
    let mut accepted_ranges = vec![];
    while let Some((workflow, mut range)) = queue.pop_front() {
        if workflow == "A" {
            accepted_ranges.push(range);
            continue;
        } else if workflow == "R" {
            continue;
        }

        let workflow = workflows.get(&workflow).unwrap();
        for rule in workflow {
            match rule {
                Rule::Destination(d) => {
                    queue.push_back((d.to_owned(), range));
                    break;
                }
                Rule::Comparison(c) => {
                    let result = c.trim_range(&range);
                    queue.push_back(result);
                    c.bad_trim(&mut range);
                }
            }
        }
    }
    accepted_ranges.iter().map(|p| p.total_ratings()).sum()
}

#[derive(Debug, Clone, Copy)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Part {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, (_, x, _)) = tuple((tag("{x="), digit1, tag(",")))(input)?;
        let (input, (_, m, _)) = tuple((tag("m="), digit1, tag(",")))(input)?;
        let (input, (_, a, _)) = tuple((tag("a="), digit1, tag(",")))(input)?;
        let (input, (_, s, _)) = tuple((tag("s="), digit1, tag("}")))(input)?;

        Ok((
            input,
            Part {
                x: x.parse().unwrap(),
                m: m.parse().unwrap(),
                a: a.parse().unwrap(),
                s: s.parse().unwrap(),
            },
        ))
    }

    fn total_rating(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
}

#[derive(Debug, Clone)]
struct PartRange {
    x: Range<usize>,
    m: Range<usize>,
    a: Range<usize>,
    s: Range<usize>,
}

impl PartRange {
    fn new() -> Self {
        let max_range = 1..4001;
        Self {
            x: max_range.clone(),
            m: max_range.clone(),
            a: max_range.clone(),
            s: max_range.clone(),
        }
    }

    fn total_ratings(&self) -> usize {
        self.x.len() * self.m.len() * self.a.len() * self.s.len()
    }
}

#[derive(Debug, Clone, Copy)]
enum Category {
    X,
    M,
    A,
    S,
}

#[derive(Debug, Clone, Copy)]
enum Comparison {
    LessThan,
    GreaterThan,
}

#[derive(Debug, Clone)]
struct ComparisonRule {
    category: Category,
    comparison: Comparison,
    value: usize,
    destination: String,
}

impl ComparisonRule {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (_, (category, comparison, value, _, destination)) =
            tuple((one_of("xmas"), one_of("<>"), digit1, tag(":"), alpha1))(input)?;
        let category = match category {
            'x' => Category::X,
            'm' => Category::M,
            'a' => Category::A,
            's' => Category::S,
            _ => panic!("unrecognised category"),
        };

        let comparison = match comparison {
            '<' => Comparison::LessThan,
            '>' => Comparison::GreaterThan,
            _ => panic!("unrecognised comparison"),
        };

        Ok((
            "",
            Self {
                category,
                comparison,
                value: value.parse().unwrap(),
                destination: destination.to_string(),
            },
        ))
    }

    fn evaluate(&self, part: &Part) -> Option<String> {
        let category = match self.category {
            Category::A => part.a,
            Category::M => part.m,
            Category::S => part.s,
            Category::X => part.x,
        };

        let condition = match self.comparison {
            Comparison::GreaterThan => category > self.value,
            Comparison::LessThan => category < self.value,
        };

        if condition {
            Some(self.destination.clone())
        } else {
            None
        }
    }

    fn trim_range(&self, part_range: &PartRange) -> (String, PartRange) {
        let mut new_range = part_range.clone();
        let category = match self.category {
            Category::A => &mut new_range.a,
            Category::M => &mut new_range.m,
            Category::S => &mut new_range.s,
            Category::X => &mut new_range.x,
        };

        match self.comparison {
            Comparison::GreaterThan => {
                category.start = (self.value + 1).max(category.start);
            }
            Comparison::LessThan => {
                category.end = (self.value).min(category.end);
            }
        };

        (self.destination.to_owned(), new_range)
    }

    fn bad_trim(&self, part_range: &mut PartRange) {
        let category = match self.category {
            Category::A => &mut part_range.a,
            Category::M => &mut part_range.m,
            Category::S => &mut part_range.s,
            Category::X => &mut part_range.x,
        };

        match self.comparison {
            Comparison::LessThan => {
                category.start = (self.value).max(category.start);
            }
            Comparison::GreaterThan => {
                category.end = (self.value + 1).min(category.end);
            }
        };
    }
}

#[derive(Debug, Clone)]
enum Rule {
    Comparison(ComparisonRule),
    Destination(String),
}

impl Rule {
    fn parse(input: &str) -> IResult<&str, Self> {
        if !input.contains(':') {
            return Ok(("", Self::Destination(input.to_string())));
        }
        Ok((
            "",
            Self::Comparison(ComparisonRule::parse(input).unwrap().1),
        ))
    }

    fn evaluate(&self, part: &Part) -> Option<String> {
        match self {
            Self::Comparison(r) => r.evaluate(part),
            Self::Destination(d) => Some(d.clone()),
        }
    }
}

type Workflow = Vec<Rule>;
type Workflows = HashMap<String, Workflow>;

fn evaluate_workflow(workflow: &Workflow, part: &Part) -> String {
    for rule in workflow {
        let destination = rule.evaluate(part);
        if let Some(destination) = destination {
            return destination;
        }
    }
    "".to_owned()
}

fn parse_workflow(input: &str) -> IResult<&str, (String, Workflow)> {
    let (workflow, (destination, _)) = tuple((alpha1, tag("{")))(input)?;
    let raw_rules: Vec<_> = workflow.strip_suffix('}').unwrap().split(',').collect();
    let rules: Vec<Rule> = raw_rules
        .iter()
        .map(|r| Rule::parse(r).unwrap().1)
        .collect();
    Ok(("", (destination.to_string(), rules)))
}

fn parse_puzzle(input: impl AsRef<Path>) -> (Workflows, Vec<Part>) {
    let input = std::fs::read_to_string(input).unwrap();

    let input: Vec<_> = input.trim().split("\n\n").collect();
    let raw_workflows: Vec<_> = input.first().unwrap().split('\n').collect();
    let raw_parts: Vec<_> = input.last().unwrap().split('\n').collect();

    let workflow_map = raw_workflows
        .iter()
        .map(|w| parse_workflow(w).unwrap().1)
        .collect();
    let parts = raw_parts
        .iter()
        .map(|p| Part::parse(p).unwrap().1)
        .collect();

    (workflow_map, parts)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        let result = rating_sum("test_part1");
        assert_eq!(result, 19114);
    }

    #[test]
    fn part_two() {
        let result = rating_combinations("test_part1");
        assert_eq!(result, 167409079868000);
    }
}
