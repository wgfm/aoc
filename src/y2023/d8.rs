use num::integer::lcm;
use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::alpha1,
    combinator::map,
    multi::many0,
    sequence::{delimited, preceded, separated_pair, tuple},
    IResult, Parser,
};

use crate::{problem::Solution, solution};

solution!(2023, 8);

impl Solution for Problem {
    fn part_a(&self) -> anyhow::Result<String> {
        let graph = Graph::parse(&self.input);

        let mut directions = graph.directions.iter().cycle();

        let mut answer = 0;

        let mut curr = "AAA";
        while curr != "ZZZ" {
            answer += 1;
            let (left, right) = graph.adjacency_map.get(curr).unwrap();
            let next = if directions.next().unwrap() == &'L' {
                left
            } else {
                right
            };
            curr = next;
        }

        Ok(answer.to_string())
    }

    fn part_b(&self) -> anyhow::Result<String> {
        let graph = Graph::parse(&self.input);

        let starting_points: Vec<String> = graph
            .adjacency_map
            .keys()
            .filter(|k| k.ends_with("A"))
            .cloned()
            .collect();

        let mut loop_lengths: Vec<u64> = vec![];
        for starting_point in starting_points {
            let mut visited = HashMap::new();

            let mut directions = graph.directions.iter().enumerate().cycle();

            let mut curr: &str = &starting_point;
            let mut loop_len = 0;
            while !curr.ends_with('Z') {
                loop_len += 1;

                let (i, dir) = directions.next().unwrap();

                visited.insert((curr, i), loop_len);

                let (left, right) = graph.adjacency_map.get(curr).unwrap();
                curr = if dir == &'L' { left } else { right };
            }

            loop_lengths.push(loop_len);
        }

        dbg!(&loop_lengths);

        let answer = loop_lengths.iter().fold(1, |acc, x| lcm(acc, *x));

        Ok(answer.to_string())
    }
}

struct Graph {
    directions: Vec<char>,
    adjacency_map: HashMap<String, (String, String)>,
}

impl Graph {
    fn parse(input: &str) -> Self {
        let mut lines = input.lines();
        let directions = parse_directions(lines.next().unwrap()).unwrap().1;
        lines.next().unwrap();

        let adjacencies = lines.map(|l| parse_adjacency(l).unwrap().1).fold(
            HashMap::new(),
            |mut acc, (from, (left, right))| {
                acc.insert(from, (left, right));
                acc
            },
        );

        Self {
            directions,
            adjacency_map: adjacencies,
        }
    }
}

fn parse_adjacency(input: &str) -> IResult<&str, (String, (String, String))> {
    map(
        tuple((
            alpha1,
            preceded(
                tag(" = "),
                delimited(
                    tag("("),
                    separated_pair(alpha1, tag(", "), alpha1),
                    tag(")"),
                ),
            ),
        )),
        |(from, (left, right)): (&str, (&str, &str))| (from.into(), (left.into(), right.into())),
    )(input)
}

fn parse_directions(input: &str) -> IResult<&str, Vec<char>> {
    map(alpha1, |s: &str| s.chars().collect())(input)
}
