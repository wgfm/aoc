use crate::{problem::Solution, solution};

solution!(2023, 9);

impl Solution for Problem {
    // 1  3  6 10 15
    // 2  3  4  5
    // 1  1  1
    // 0  0
    fn part_a(&self) -> anyhow::Result<String> {
        let sequences = parse(&self.input);

        let mut sums: Vec<i64> = vec![];

        for mut sequence in sequences {
            let mut iteration = 0;

            loop {
                let mut has_non_zero = false;
                for i in 0..sequence.len() - iteration - 1 {
                    sequence[i] = sequence[i + 1] - sequence[i];
                    if sequence[i] != 0 {
                        has_non_zero = true;
                    }
                }

                if !has_non_zero {
                    break;
                }

                iteration += 1;
            }

            dbg!(&sequence);
            sums.push(sequence.iter().sum());
        }

        let answer = sums.iter().sum::<i64>();

        Ok(answer.to_string())
    }

    fn part_b(&self) -> anyhow::Result<String> {
        let sequences = parse(&self.input);

        let mut sums: Vec<i64> = vec![];

        for mut sequence in sequences {
            let mut iteration = 0;

            // 1  3  6 10 15
            //    2  3  4  5
            loop {
                let range = iteration + 1..sequence.len();
                if range.is_empty() {
                    break;
                }

                for i in range.rev() {
                    sequence[i] = sequence[i] - sequence[i - 1];
                }

                iteration += 1;
            }
            dbg!(&sequence);

            let prev =
                sequence.iter().enumerate().fold(
                    0,
                    |acc, (i, x)| {
                        if i % 2 == 0 {
                            acc + x
                        } else {
                            acc - x
                        }
                    },
                );

            sums.push(prev);
        }

        let answer = sums.iter().sum::<i64>();

        Ok(answer.to_string())
    }
}

fn parse(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|line| {
            line.split(" ")
                .map(|num| num.parse::<i64>().unwrap())
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_b() {
        let input = "10 13 16 21 30 45";
        let problem = Problem::with_input(input);

        let answer = problem.part_b();
        assert_eq!(answer.unwrap(), "5");
    }
}
