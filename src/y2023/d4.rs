use std::collections::HashSet;

use nom::{bytes::complete::tag, character::complete::space1, multi::separated_list1, IResult};

use crate::{problem::Solution, solution};

solution!(2023, 4);

impl Solution for Problem {
    fn part_a(&self) -> anyhow::Result<String> {
        let cards: Vec<Card> = self
            .input
            .lines()
            .map(Card::parse)
            .map(Result::unwrap)
            .collect();

        let answer: u64 = cards
            .iter()
            .map(|card| {
                let have = HashSet::<u64>::from_iter(card.have_numbers.iter().copied());
                let winning = HashSet::<u64>::from_iter(card.winning_numbers.iter().copied());

                let num_winnings = have.intersection(&winning).count();
                if num_winnings == 0 {
                    0
                } else {
                    1 << (num_winnings - 1)
                }
            })
            .sum();

        Ok(answer.to_string())
    }

    fn part_b(&self) -> anyhow::Result<String> {
        let cards: Vec<Card> = self
            .input
            .lines()
            .map(Card::parse)
            .map(Result::unwrap)
            .collect();

        let mut counts = vec![1; cards.len()];
        for (i, card) in cards.iter().enumerate() {
            let have = HashSet::<u64>::from_iter(card.have_numbers.iter().copied());
            let winning = HashSet::<u64>::from_iter(card.winning_numbers.iter().copied());

            let num_winnings = have.intersection(&winning).count();
            for j in 0..num_winnings {
                counts[i + j + 1] += counts[i];
            }
        }

        dbg!(&counts[0..10]);

        let answer: usize = counts.iter().sum();

        Ok(answer.to_string())
    }
}

#[derive(Debug)]
struct Card {
    number: u64,
    winning_numbers: Vec<u64>,
    have_numbers: Vec<u64>,
}

impl Card {
    fn parse(line: &str) -> anyhow::Result<Self> {
        let (_, card) =
            parse_card(line).map_err(|e| anyhow::anyhow!("Failed to parse card: {}", e))?;
        Ok(card)
    }
}

fn parse_card(line: &str) -> IResult<&str, Card> {
    let (line, _) = tag("Card")(line)?;
    let (line, _) = space1(line)?;
    let (line, number) = nom::character::complete::u64(line)?;
    let (line, _) = tag(":")(line)?;
    let (line, _) = space1(line)?;
    let (line, winning_numbers) = separated_list1(space1, nom::character::complete::u64)(line)?;
    let (line, _) = tag(" |")(line)?;
    let (line, _) = space1(line)?;
    let (line, have_numbers) = separated_list1(space1, nom::character::complete::u64)(line)?;

    Ok((
        line,
        Card {
            number,
            winning_numbers,
            have_numbers,
        },
    ))
}
