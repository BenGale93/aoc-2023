use std::{collections::HashMap, path::Path};

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
        todo!()
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
        let (input, (category, comparison, value, _, destination)) =
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

fn parse_puzzle(input: impl AsRef<Path>) -> (HashMap<String, Workflow>, Vec<Part>) {
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
}
