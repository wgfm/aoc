use std::{collections::HashMap, iter::Peekable};

use crate::{problem::Solution, solution};

solution!(2015, 5);

impl Solution for Problem {
    fn part_a(&self) -> anyhow::Result<String> {
        let mut num_nice_strings = 0;
        for line in self.input.lines() {
            let mut num_vowels = 0;
            let mut has_double_letter = false;
            let mut has_naughty_sequence = false;

            let mut iter = line.chars().peekable();
            while let Some(ch) = iter.next() {
                let p = iter.peek();
                if p == Some(&ch) {
                    has_double_letter = true;
                }

                match ch {
                    'a' => {
                        num_vowels += 1;
                        if p == Some(&'b') {
                            has_naughty_sequence = true;
                        }
                    }
                    'e' | 'i' | 'o' | 'u' => {
                        num_vowels += 1;
                    }
                    'c' => {
                        if p == Some(&'d') {
                            has_naughty_sequence = true;
                        }
                    }
                    'p' => {
                        if p == Some(&'q') {
                            has_naughty_sequence = true;
                        }
                    }
                    'x' => {
                        if p == Some(&'y') {
                            has_naughty_sequence = true;
                        }
                    }
                    _ => {}
                }
            }

            if num_vowels >= 3 && !has_naughty_sequence && has_double_letter {
                num_nice_strings += 1;
            }
        }

        Ok(format!("{}", num_nice_strings))
    }

    fn part_b(&self) -> anyhow::Result<String> {
        let num_nice_strings = self
            .input
            .lines()
            .filter(|l| line_has_pair(l))
            .filter(|l| line_has_repeat(l))
            .count();

        Ok(format!("{}", num_nice_strings))
    }
}

fn line_has_repeat(line: &str) -> bool {
    // check if there is a letter which repeats with exactly one letter between them
    line.chars().zip(line.chars().skip(2)).any(|(a, b)| a == b)
}

fn line_has_pair(line: &str) -> bool {
    // check if there is a pair of any two letters that appears at least twice in the
    // string without overlapping. e.g. xyxy (xy) or aabcdefgaa (aa), but not like aaa (aa,
    // but it overlaps).
    let mut pairs: HashMap<(char, char), Vec<usize>> = HashMap::new();
    let pair_iter = line.chars().zip(line.chars().skip(1));
    for (pos, pair) in pair_iter.enumerate() {
        pairs
            .entry(pair)
            .and_modify(|p| p.push(pos))
            .or_insert(vec![pos]);
    }

    let pairs = pairs.iter().filter(|(_, positions)| {
        positions
            .iter()
            .zip(positions.iter().skip(1))
            .any(|(a, b)| b - a > 1)
    });

    pairs.count() > 0
}

#[cfg(test)]
mod tests {
    #[test]
    fn two_positions() {
        let positions = vec![1, 8];
        let has_pair = positions
            .iter()
            .zip(positions.iter().skip(1))
            .any(|(a, b)| b - a > 1);
        assert!(has_pair);
    }

    #[test]
    fn line_has_repeat_finds_repeats_with_one_letter_between() {
        let cases = vec![
            ("abacdefg", true),
            ("abcdefgf", true),
            ("abcdcefg", true),
            ("aaa", true),
            ("xyx", true),
            ("abcdef", false),
            ("abbacdef", false),
        ];

        for (line, expected) in cases {
            assert_eq!(super::line_has_repeat(line), expected);
        }
    }

    #[test]
    fn line_has_pair_finds_non_overlapping_pairs() {
        let cases = vec![
            ("abaustnyeab", true),
            ("aaa", false),
            ("xyxy", true),
            ("rastoinerauft", true),
            ("artoienen", true),
            ("arststoien", true),
            ("ararararar", true),
        ];

        for (line, expected) in cases {
            assert_eq!(super::line_has_pair(line), expected);
        }
    }
}
