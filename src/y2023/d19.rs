use std::collections::HashMap;

use crate::{problem::Solution, solution};

solution!(2023, 19);

type Range = (u64, u64);

impl Solution for Problem {
    fn part_a(&self) -> anyhow::Result<String> {
        let (workflows, parts) = parse(&self.input);

        let mut accepted_parts = vec![];
        for part in parts {
            let mut curr_workflow = "in";
            while curr_workflow != "A" && curr_workflow != "R" {
                let workflow = workflows.get(curr_workflow).unwrap();
                for (condition, target) in workflow.rules.iter() {
                    if condition.apply(&part) {
                        curr_workflow = target;
                        break;
                    }
                }
            }

            if curr_workflow == "A" {
                accepted_parts.push(part);
            }
        }

        let answer = accepted_parts
            .iter()
            .map(|part| part.ratings.iter().map(|(_, val)| val).sum::<u64>())
            .sum::<u64>();

        Ok(answer.to_string())
    }

    fn part_b(&self) -> anyhow::Result<String> {
        let (workflows, _) = parse(&self.input);
        let answer = count(
            &workflows,
            "in",
            &[(1, 4000), (1, 4000), (1, 4000), (1, 4000)],
        );

        Ok(answer.to_string())
    }
}

fn parse(input: &str) -> (HashMap<String, Workflow>, Vec<Part>) {
    let (workflows, parts) = input.split_once("\n\n").unwrap();
    let workflows = workflows
        .lines()
        .map(|line| Workflow::parse(line))
        .map(|workflow| (workflow.name.clone(), workflow))
        .collect();

    let parts = parts
        .lines()
        .map(|line| Part::parse(line))
        .collect::<Vec<_>>();

    (workflows, parts)
}

#[derive(Debug)]
enum Condition {
    GreaterThan(char, u64),
    LessThan(char, u64),
    Default,
}

impl Condition {
    fn parse(input: &str) -> Self {
        if let Some((lhs, rhs)) = input.split_once('<') {
            Condition::LessThan(lhs.chars().next().unwrap(), rhs.parse().unwrap())
        } else if let Some((lhs, rhs)) = input.split_once('>') {
            Condition::GreaterThan(lhs.chars().next().unwrap(), rhs.parse().unwrap())
        } else {
            Condition::Default
        }
    }

    fn apply(&self, part: &Part) -> bool {
        match self {
            Condition::Default => true,
            Condition::GreaterThan(ch, rating) => part.rating(*ch) > *rating,
            Condition::LessThan(ch, rating) => part.rating(*ch) < *rating,
        }
    }
}

struct Workflow {
    name: String,
    rules: Vec<(Condition, String)>,
}

impl Workflow {
    fn parse(input: &str) -> Self {
        // px{a<2006:qkq,m>2090:A,rfg}
        let (name, rules) = input.split_once('{').unwrap();
        let rules = rules.trim_end_matches('}');

        let rules = rules
            .split(',')
            .map(|rule| {
                if let Some((condition, target)) = rule.split_once(':') {
                    (Condition::parse(condition), target.to_string())
                } else {
                    // Default
                    (Condition::Default, rule.to_string())
                }
            })
            .collect();

        Workflow {
            name: name.to_string(),
            rules,
        }
    }
}

#[derive(Debug)]
struct Part {
    ratings: Vec<(char, u64)>,
}

impl Part {
    fn parse(input: &str) -> Self {
        let input = input.trim_matches(|c| c == '{' || c == '}');

        let ratings = input
            .split(',')
            .map(|rating| {
                let (name, rating) = rating.split_once('=').unwrap();
                (name.chars().next().unwrap(), rating.parse().unwrap())
            })
            .collect();

        Self { ratings }
    }

    fn rating(&self, ch: char) -> u64 {
        self.ratings
            .iter()
            .find(|(c, _)| *c == ch)
            .map(|(_, r)| *r)
            .unwrap()
    }
}

fn count(workflows: &HashMap<String, Workflow>, workflow: &str, values: &[Range; 4]) -> u64 {
    if workflow == "R" {
        return 0;
    }

    if workflow == "A" {
        return values
            .iter()
            .fold(1, |acc, (from, to)| acc * (to - from + 1));
    }

    let mut values = values.clone();

    let mut total = 0;
    for (condition, target) in workflows.get(workflow).unwrap().rules.iter() {
        let category;
        let (overlap_range, non_overlap_range) = match condition {
            Condition::GreaterThan(ch, amt) => {
                let (from, to) = values[idx_from_category(*ch)];
                category = *ch;
                ((amt + 1, to), (from, *amt))
            }
            Condition::LessThan(ch, amt) => {
                let (from, to) = values[idx_from_category(*ch)];
                category = *ch;
                ((from, *amt - 1), (*amt, to))
            }
            Condition::Default => {
                total += count(workflows, target, &values);
                break;
            }
        };

        if overlap_range.0 <= overlap_range.1 {
            let mut new_values = values.clone();
            new_values[idx_from_category(category)] = overlap_range;
            total += count(workflows, target, &new_values);
        }

        if non_overlap_range.0 > non_overlap_range.1 {
            break;
        }

        values[idx_from_category(category)] = non_overlap_range;
    }

    total
}

fn idx_from_category(ch: char) -> usize {
    match ch {
        'x' => 0,
        'm' => 1,
        'a' => 2,
        's' => 3,
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

    #[test]
    fn part_a_testinput() {
        let p = Problem::with_input(INPUT);
        assert_eq!(p.part_a().unwrap(), "19114");
    }

    #[test]
    fn part_b_testinput() {
        let p = Problem::with_input(INPUT);
        assert_eq!(p.part_b().unwrap(), "167409079868000");
    }
}
