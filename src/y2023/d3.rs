use std::{
    cmp::{max, min},
    collections::HashMap,
};

use crate::{problem::Solution, solution};

solution!(2023, 3);

impl Solution for Problem {
    fn part_a(&self) -> anyhow::Result<String> {
        let engine = Engine::new(&self.input);

        let mut total: usize = 0;
        for ((x, y), number) in engine.numbers {
            let from_x = x.saturating_sub(1);
            let to_x = x + number.len();
            let from_y = y.saturating_sub(1);
            let to_y = min(y + 1, self.input.lines().count() - 1);
            println!(
                "x: {}..{}; y: {}..{} for number {}",
                from_x, to_x, from_y, to_y, number
            );
            'outer: for test_x in from_x..=to_x {
                for test_y in from_y..=to_y {
                    if let Some(_) = engine.symbols.get(&(test_x, test_y)) {
                        println!(
                            "found symbol near number {} at {}, {}",
                            number, test_x, test_y
                        );
                        total += number.parse::<usize>()?;
                        break 'outer;
                    }
                }
            }
        }

        Ok(total.to_string())
    }

    fn part_b(&self) -> anyhow::Result<String> {
        let engine = Engine::new(&self.input);

        let mut number_positions: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
        for ((x, y), number) in engine.numbers.iter() {
            for xx in *x..=(*x + number.len() - 1) {
                number_positions.insert((xx, *y), (*x, *y));
            }
        }

        let gears = engine.symbols.iter().filter(|(_, c)| **c == '*');

        let mut total = 0;
        for ((x, y), _) in gears {
            let from_x = x.saturating_sub(1);
            let to_x = x + 1;
            let from_y = y.saturating_sub(1);
            let to_y = y + 1;

            let mut numbers = Vec::new();
            for test_x in from_x..=to_x {
                for test_y in from_y..=to_y {
                    if let Some((num_x, num_y)) = number_positions.get(&(test_x, test_y)) {
                        if numbers.iter().any(|(pos, _)| *pos == (*num_x, *num_y)) {
                            continue;
                        }
                        println!(
                            "found number at ({}, {}) for gear at ({}, {})",
                            num_x, num_y, x, y
                        );

                        let num = engine
                            .numbers
                            .get(&(*num_x, *num_y))
                            .unwrap()
                            .parse::<usize>()?;

                        numbers.push(((*num_x, *num_y), num));
                    }
                }
            }
            if numbers.len() == 2 {
                println!("found gear at ({}, {})", x, y);
                total += numbers[0].1 * numbers[1].1;
            }
        }

        Ok(total.to_string())
    }
}

struct Engine {
    symbols: HashMap<(usize, usize), char>,
    numbers: HashMap<(usize, usize), String>,
}

impl Engine {
    fn new(input: &str) -> Self {
        let lines = input.lines();

        // create a symbol table
        let mut symbols = HashMap::new();
        for (y, line) in lines.enumerate() {
            for (x, c) in line.chars().enumerate() {
                if is_symbol(c) {
                    symbols.insert((x, y), c);
                }
            }
        }
        dbg!(&symbols);

        // create a table of numbers and their positions
        let mut numbers = HashMap::new();
        for (y, line) in input.lines().enumerate() {
            let mut curr_number = String::new();
            for (x, c) in line.chars().enumerate() {
                if !c.is_digit(10) {
                    if !curr_number.is_empty() {
                        numbers.insert((x - curr_number.len(), y), curr_number.clone());
                        curr_number.clear();
                    }
                } else {
                    curr_number.push(c);
                }
            }
            if !curr_number.is_empty() {
                numbers.insert((line.len() - curr_number.len(), y), curr_number.clone());
            }
        }

        Engine { symbols, numbers }
    }
}

fn is_symbol(c: char) -> bool {
    match c {
        '.' | '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => false,
        _ => true,
    }
}

#[cfg(test)]
mod tests {
    use crate::problem::Solution;

    #[test]
    fn part_a() {
        let input = "
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";

        let expected = "4361";

        let problem = super::Problem::with_input(input);

        assert_eq!(expected, problem.part_a().unwrap());
    }

    #[test]
    fn part_a_edge_cases() {
        let input = "
1.......2
.*.....+.
";

        let expected = "3";

        let problem = super::Problem::with_input(input);

        assert_eq!(expected, problem.part_a().unwrap());
    }

    #[test]
    fn part_b() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";
        let expected = "467835";

        let problem = super::Problem::with_input(input);

        assert_eq!(expected, problem.part_b().unwrap());
    }

    #[test]
    fn saturating_sub() {
        let a: usize = 0;
        assert_eq!(0, a.saturating_sub(1));
    }
}
