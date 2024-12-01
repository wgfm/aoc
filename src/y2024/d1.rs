use crate::{problem::Solution, solution};
use std::collections::HashMap;

solution!(2024, 1);

impl Solution for Problem {
    fn part_a(&self) -> anyhow::Result<String> {
        let mut list1: Vec<i64> = vec![];
        let mut list2: Vec<i64> = vec![];
        self.input.lines().for_each(|line| {
            let mut parts = line.split_whitespace();
            let first = parts.next().unwrap().parse::<i64>().unwrap();
            let second = parts.next().unwrap().parse::<i64>().unwrap();
            list1.push(first);
            list2.push(second);
        });

        list1.sort();
        list2.sort();

        let answer: i64 = list1
            .iter()
            .zip(list2.iter())
            .map(|(one, two)| (one - two).abs())
            .sum();

        Ok(answer.to_string())
    }

    fn part_b(&self) -> anyhow::Result<String> {
        let mut list1: Vec<i64> = vec![];
        let mut map: HashMap<i64, i64> = HashMap::new();
        self.input.lines().for_each(|line| {
            let mut parts = line.split_whitespace();
            let first = parts.next().unwrap().parse::<i64>().unwrap();
            let second = parts.next().unwrap().parse::<i64>().unwrap();
            list1.push(first);
            map.entry(second).and_modify(|v| *v += 1).or_insert(1);
        });

        let mut answer = 0;
        for num in list1 {
            if let Some(count) = map.get(&num) {
                answer += num * *count;
            }
        }

        Ok(answer.to_string())
    }
}
