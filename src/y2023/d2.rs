use nom::{
    bytes::complete::tag,
    combinator::{all_consuming, map},
    multi::separated_list1,
    sequence::delimited,
    IResult, Parser,
};

use crate::{problem::Solution, solution};

solution!(2023, 2);

impl Solution for Problem {
    fn part_a(&self) -> anyhow::Result<String> {
        let games: Vec<Game> = self
            .input
            .lines()
            .map(Game::parse)
            .map(Result::unwrap)
            .collect();

        let answer: u64 = games
            .iter()
            .filter(|Game { reveals, .. }| {
                reveals.iter().all(
                    |Reveal {
                         blues,
                         reds,
                         greens,
                     }| { *reds <= 12 && *greens <= 13 && *blues <= 14 },
                )
            })
            .map(|Game { id, .. }| id)
            .sum();

        Ok(answer.to_string())
    }

    fn part_b(&self) -> anyhow::Result<String> {
        let games: Vec<Game> = self
            .input
            .lines()
            .map(Game::parse)
            .map(Result::unwrap)
            .collect();

        let answer: u64 = games
            .iter()
            .map(|Game { reveals, .. }| {
                let min_reds = reveals
                    .iter()
                    .map(|Reveal { reds, .. }| reds)
                    .max()
                    .unwrap();
                let min_blues = reveals
                    .iter()
                    .map(|Reveal { blues, .. }| blues)
                    .max()
                    .unwrap();
                let min_greens = reveals
                    .iter()
                    .map(|Reveal { greens, .. }| greens)
                    .max()
                    .unwrap();
                (min_reds * min_blues * min_greens) as u64
            })
            .sum();

        Ok(answer.to_string())
    }
}

struct Game {
    id: u64,
    reveals: Vec<Reveal>,
}

struct Reveal {
    blues: usize,
    reds: usize,
    greens: usize,
}

impl Game {
    fn parse(input: &str) -> anyhow::Result<Self> {
        parse_game(input)
            .map(|(_, game)| game)
            .map_err(|e| anyhow::anyhow!("Failed to parse game: {}", e))
    }
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    // Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    let (input, id) = parse_game_id(input)?;

    let (input, reveals) = separated_list1(tag("; "), parse_reveal)(input)?;

    assert!(input.is_empty());

    Ok((input, Game { id, reveals }))
}

fn parse_game_id(input: &str) -> IResult<&str, u64> {
    delimited(tag("Game "), nom::character::complete::u64, tag(": "))(input)
}

fn parse_reveal(input: &str) -> IResult<&str, Reveal> {
    let (input, reveals) = separated_list1(tag(", "), parse_color)(input)?;
    let blues = find_reveal(&reveals, "blue");
    let reds = find_reveal(&reveals, "red");
    let greens = find_reveal(&reveals, "green");

    Ok((
        input,
        Reveal {
            blues,
            reds,
            greens,
        },
    ))
}

fn find_reveal(reveals: &[(usize, &str)], color: &str) -> usize {
    reveals
        .iter()
        .find(|(_, c)| *c == color)
        .map(|(count, _)| *count)
        .unwrap_or(0)
}

fn parse_color(input: &str) -> IResult<&str, (usize, &str)> {
    let (input, count) = nom::character::complete::digit1(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, color) = nom::character::complete::alpha1(input)?;
    Ok((input, (count.parse().unwrap(), color)))
}
