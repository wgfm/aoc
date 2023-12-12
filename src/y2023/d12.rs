use std::collections::BTreeMap;

use crate::{problem::Solution, solution};

solution!(2023, 12);

impl Solution for Problem {
    fn part_a(&self) -> anyhow::Result<String> {
        let lines: Vec<Line> = self.input.lines().map(Line::parse).collect();

        let mut answer = 0;
        for line in lines {
            let mut cache = Cache::new();
            answer += cache.solve(
                line.map.chars().collect::<Vec<_>>().as_slice(),
                line.damaged_group_sizes.as_slice(),
                0,
                0,
                0,
            );
        }

        Ok(answer.to_string())
    }

    fn part_b(&self) -> anyhow::Result<String> {
        let lines: Vec<Line> = self.input.lines().map(Line::parse2).collect();

        let mut answer = 0;
        for line in lines {
            let mut cache = Cache::new();
            answer += cache.solve(
                line.map.chars().collect::<Vec<_>>().as_slice(),
                line.damaged_group_sizes.as_slice(),
                0,
                0,
                0,
            );
        }

        Ok(answer.to_string())
    }
}

struct Cache {
    cache: BTreeMap<(usize, usize, usize), usize>,
}

impl Cache {
    fn new() -> Self {
        Self {
            cache: BTreeMap::new(),
        }
    }

    fn solve(
        &mut self,
        map: &[char],
        groups: &[usize],
        map_i: usize,
        group_i: usize,
        current: usize,
    ) -> usize {
        let key = (map_i, group_i, current);
        if let Some(answer) = self.cache.get(&key) {
            return *answer;
        }

        if map.len() == map_i {
            if groups.len() == group_i && current == 0 {
                return 1;
            }

            if groups.len() == group_i + 1 && groups[group_i] == current {
                return 1;
            }

            return 0;
        }

        let mut answer = 0;
        for char in &['.', '#'] {
            if map[map_i] == *char || map[map_i] == '?' {
                match (*char, current) {
                    ('.', 0) => {
                        answer += self.solve(map, groups, map_i + 1, group_i, 0);
                    }
                    ('.', _) => {
                        if group_i < groups.len() && groups[group_i] == current {
                            answer += self.solve(map, groups, map_i + 1, group_i + 1, 0);
                        }
                    }
                    ('#', _) => {
                        answer += self.solve(map, groups, map_i + 1, group_i, current + 1);
                    }
                    _ => {}
                }
            }
        }

        self.cache.insert(key, answer);

        answer
    }
}

struct Line {
    map: String,
    damaged_group_sizes: Vec<usize>,
}

impl Line {
    fn parse(input: &str) -> Self {
        let (map, groups) = input.split_once(' ').unwrap();
        let damaged_group_sizes = groups.split(',').map(|s| s.parse().unwrap()).collect();

        Self {
            map: map.to_string(),
            damaged_group_sizes,
        }
    }

    fn parse2(input: &str) -> Self {
        let (map, groups) = input.split_once(' ').unwrap();
        let groups: Vec<usize> = groups.split(',').map(|s| s.parse().unwrap()).collect();

        let map = &[map, map, map, map, map].join("?");
        let groups = groups.repeat(5);

        Self {
            map: map.to_string(),
            damaged_group_sizes: groups,
        }
    }
}
