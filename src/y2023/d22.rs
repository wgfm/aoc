use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet, VecDeque},
    ops::RangeInclusive,
};

use crate::{problem::Solution, solution};

solution!(2023, 22);

impl Solution for Problem {
    fn part_a(&self) -> anyhow::Result<String> {
        let mut bricks = parse(&self.input);

        drop_all(&mut bricks);

        let mut answer = 0;
        for i in 0..bricks.len() {
            let mut bricks_clone = bricks.clone();
            bricks_clone.remove(i);
            if !drop_all(&mut bricks_clone) {
                answer += 1;
            }
        }

        Ok(answer.to_string())
    }

    fn part_b(&self) -> anyhow::Result<String> {
        let mut bricks = parse(&self.input);
        drop_all(&mut bricks);

        dbg!(&bricks);

        let mut supports: HashMap<&Brick, Vec<&Brick>> = HashMap::new();
        let mut supported_bys: HashMap<&Brick, Vec<&Brick>> = HashMap::new();

        // 'Accidentally' quadratic
        // This works. Don't touch it.
        for i in 0..bricks.len() {
            for j in 0..bricks.len() {
                if i == j {
                    continue;
                }

                if bricks[i].supports(&bricks[j]) {
                    supports.entry(&bricks[i]).or_default().push(&bricks[j]);

                    supported_bys
                        .entry(&bricks[j])
                        .or_default()
                        .push(&bricks[i]);
                }
            }
        }

        let mut answer = 0;

        for brick in bricks.iter() {
            let mut collapsed = HashMap::new();
            let mut queue = VecDeque::new();
            if let Some(ss) = supports.get(&brick) {
                queue.extend(ss.iter().cloned());
            }
            collapsed.insert(brick, true);

            while let Some(curr) = queue.pop_front() {
                if collapsed.contains_key(curr) {
                    continue;
                }

                if let Some(supported_by) = supported_bys.get(&curr) {
                    if supported_by.len() > 1
                        && supported_by
                            .iter()
                            .any(|b| !collapsed.get(b).unwrap_or(&false))
                    {
                        collapsed.insert(curr, false);
                        continue;
                    }
                }

                answer += 1;
                collapsed.insert(curr, true);

                if let Some(ss) = supports.get(&curr) {
                    queue.extend(ss.iter().cloned());
                }
            }
        }

        Ok(answer.to_string())
    }
}

fn drop_all(bricks: &mut Vec<Brick>) -> bool {
    let num_bricks = bricks.len();

    let mut changed = true;
    let mut num_changes = 0;
    let mut supporting = vec![false; num_bricks];
    while changed {
        bricks.sort_by(|a, b| b.z.start().cmp(a.z.start()));
        changed = false;

        for i in (0..num_bricks).rev() {
            if bricks[i].z.start() == &1 {
                // Already at bottom
                supporting[i] = true;
                continue;
            }

            let mut phantom = bricks[i].clone();
            phantom.z = 1..=*phantom.z.end();
            let mut dist = usize::MAX;
            for j in i + 1..num_bricks {
                if bricks[j].collide(&phantom) {
                    dist = min(dist, bricks[i].vertical_distance(&bricks[j]));
                }
            }

            let curr_brick = &mut bricks[i];
            if dist == usize::MAX {
                // Fall to bottom
                dist = *curr_brick.z.start() - 1;
            }

            if dist == 0 {
                continue;
            }

            curr_brick.drop(dist);
            num_changes += 1;
            changed = true;
        }
    }

    num_changes > 0
}

fn partially_overlap(r1: &RangeInclusive<usize>, r2: &RangeInclusive<usize>) -> bool {
    *r1.start() <= *r2.end() && *r2.start() <= *r1.end()
}

#[derive(PartialEq, Eq, Clone, Hash)]
struct Brick {
    x: RangeInclusive<usize>,
    y: RangeInclusive<usize>,
    z: RangeInclusive<usize>,
}

impl Brick {
    fn supports(&self, other: &Brick) -> bool {
        partially_overlap(&self.x, &other.x)
            && partially_overlap(&self.y, &other.y)
            && *self.z.end() + 1 == *other.z.start()
    }

    fn drop(&mut self, amount: usize) -> bool {
        self.z = *self.z.start() - amount..=*self.z.end() - amount;
        assert!(self.z.start() >= &1);
        true
    }

    fn vertical_distance(&self, other: &Brick) -> usize {
        *self.z.start() - *other.z.end() - 1
    }

    fn collide(&self, other: &Brick) -> bool {
        partially_overlap(&self.x, &other.x)
            && partially_overlap(&self.y, &other.y)
            && partially_overlap(&self.z, &other.z)
    }
}

impl std::fmt::Debug for Brick {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Brick (x: {}..={}, y: {}..={}, z: {}..={})",
            self.x.start(),
            self.x.end(),
            self.y.start(),
            self.y.end(),
            self.z.start(),
            self.z.end()
        )
    }
}

fn parse(input: &str) -> Vec<Brick> {
    input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let (one, two) = line.split_once("~").unwrap();
            let (x1, y1, z1) = parse_coords(one);
            let (x2, y2, z2) = parse_coords(two);

            Brick {
                x: min(x1, x2)..=max(x1, x2),
                y: min(y1, y2)..=max(y1, y2),
                z: min(z1, z2)..=max(z1, z2),
            }
        })
        .collect()
}

fn parse_coords(input: &str) -> (usize, usize, usize) {
    let v: Vec<_> = input.splitn(3, ",").map(|s| s.parse().unwrap()).collect();
    (v[0], v[1], v[2])
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn range_inclusive() {
        let r = 2..=2;
        assert_eq!(r.contains(&2), true);
    }

    #[test]
    fn test_partially_overlap() {
        assert!(partially_overlap(&(1..=3), &(2..=4)));
        assert!(partially_overlap(&(2..=4), &(1..=3)));
        assert!(partially_overlap(&(1..=3), &(3..=5)));
        assert!(partially_overlap(&(3..=5), &(1..=3)));
        assert!(!partially_overlap(&(1..=3), &(4..=5)));
    }

    #[test]
    fn brick_vertical_distance() {
        let b1 = Brick {
            x: 1..=3,
            y: 1..=3,
            z: 1..=3,
        };
        let b2 = Brick {
            x: 1..=3,
            y: 1..=3,
            z: 4..=6,
        };
        assert_eq!(b2.vertical_distance(&b1), 0);
    }

    #[test]
    fn brick_dist_drop() {
        let mut top = Brick {
            x: 1..=1,
            y: 1..=4,
            z: 5..=5,
        };
        let bottom = Brick {
            x: 1..=1,
            y: 1..=4,
            z: 1..=1,
        };

        let dist = top.vertical_distance(&bottom);
        assert_eq!(dist, 3);

        top.drop(dist);
        assert_eq!(top.z, 2..=2);
    }

    #[test]
    fn brick_partially_overlap() {
        let b1 = Brick {
            x: 1..=3,
            y: 1..=3,
            z: 1..=3,
        };
        let b2 = Brick {
            x: 2..=4,
            y: 2..=4,
            z: 2..=4,
        };
        assert!(b1.collide(&b2));
        assert!(b2.collide(&b1));

        let b1 = Brick {
            x: 1..=1,
            y: 1..=4,
            z: 1..=5,
        };
        let b2 = Brick {
            x: 1..=1,
            y: 1..=4,
            z: 4..=4,
        };
        assert!(b1.collide(&b2));
        assert!(b2.collide(&b1));
    }

    const INPUT: &'static str = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";

    #[test]
    fn test_part_a() {
        let p = Problem::with_input(INPUT);
        assert_eq!(p.part_a().unwrap(), "5");
    }

    #[test]
    fn test_part_b() {
        let p = Problem::with_input(INPUT);
        assert_eq!(p.part_b().unwrap(), "7");
    }
}
