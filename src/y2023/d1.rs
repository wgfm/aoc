use crate::{problem::Solution, solution};

solution!(2023, 1);

impl Solution for Problem {
    fn part_a(&self) -> anyhow::Result<String> {
        let answer: u32 = self
            .input
            .lines()
            .map(|l| {
                let first_digit = l.chars().find(|c| c.is_digit(10)).unwrap();
                let last_digit = l.chars().rev().find(|c| c.is_digit(10)).unwrap();
                first_digit.to_digit(10).unwrap() * 10 + last_digit.to_digit(10).unwrap()
            })
            .sum();
        Ok(format!("{}", answer))
    }

    // buh...
    fn part_b(&self) -> anyhow::Result<String> {
        let rx = regex::Regex::new(r"one|two|three|four|five|six|seven|eight|nine|[0-9]")?;
        let rx_rev = regex::Regex::new(r"eno|owt|eerht|ruof|evif|xis|neves|thgie|enin|[0-9]")?;
        let answer: u32 = self
            .input
            .lines()
            .map(|l| {
                let first = rx.find(l);
                let first_digit = first.unwrap().as_str();

                let rev = l.chars().rev().collect::<String>();
                let last = rx_rev.find(&rev);

                let last_digit: String = last.unwrap().as_str().chars().rev().collect();

                10 * to_digit(first_digit) + to_digit(&last_digit)
            })
            .sum();

        Ok(format!("{}", answer))
    }
}

fn to_digit(s: &str) -> u32 {
    match s {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => u32::from_str_radix(s, 10).expect(&format!("s should be a number: {}", s)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_b() {
        let problem = Problem::with_input(
            "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen",
        );

        assert_eq!(problem.part_b().unwrap(), "281");
    }
}
