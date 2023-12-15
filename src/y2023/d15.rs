use crate::{problem::Solution, solution};

solution!(2023, 15);

impl Solution for Problem {
    fn part_a(&self) -> anyhow::Result<String> {
        let steps = self.input.trim().split(',');
        let mut answer = 0;

        for step in steps {
            let mut hash = 0;
            for ch in step.chars() {
                hash = hash + ch as u64;
                hash = hash * 17;
                hash = hash % 256;
            }

            answer += hash;
        }

        Ok(answer.to_string())
    }

    fn part_b(&self) -> anyhow::Result<String> {
        let steps = self.input.trim().split(',');

        let mut boxes = vec![vec![]; 256];

        for step in steps {
            let (label, op) = parse_step(step);
            let hash = hash(label);

            let contents = &mut boxes[hash];

            match op {
                Op::Remove => {
                    if let Some((i, _)) = contents
                        .iter()
                        .enumerate()
                        .find(|(_, (lbl, _))| *lbl == label)
                    {
                        contents.remove(i);
                    }
                }
                Op::Insert(focal) => {
                    if let Some((i, _)) = contents
                        .iter()
                        .enumerate()
                        .find(|(_, (lbl, _))| *lbl == label)
                    {
                        contents[i].1 = focal;
                    } else {
                        contents.push((label, focal));
                    }
                }
            }
        }

        let mut answer = 0;
        for (bi, b) in boxes.iter().enumerate() {
            for (li, (_, focal)) in b.iter().enumerate() {
                answer += (bi + 1) * (li + 1) * *focal as usize;
            }
        }

        Ok(answer.to_string())
    }
}

enum Op {
    Remove,
    Insert(u8),
}

fn parse_step(step: &str) -> (&str, Op) {
    if let Some((label, focal)) = step.split_once("=") {
        return (label, Op::Insert(focal.parse().unwrap()));
    }

    (step.trim_end_matches('-'), Op::Remove)
}

fn hash(s: &str) -> usize {
    let mut hash = 0;
    for ch in s.chars() {
        hash = hash + ch as usize;
        hash = hash * 17;
        hash = hash % 256;
    }

    hash
}
