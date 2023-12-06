use std::{collections::HashSet, ops::Range};

use nom::{multi::separated_list1, IResult};

use crate::{problem::Solution, solution};

solution!(2023, 5);

impl Solution for Problem {
    fn part_a(&self) -> anyhow::Result<String> {
        let farm = Farm::parse(&self.input)?;

        let answer = farm
            .seeds
            .iter()
            .map(|seed| {
                let mut ptr = *seed;
                for step in farm.steps.iter() {
                    ptr = step.jump(ptr);
                }

                ptr
            })
            .min()
            .unwrap();

        Ok(answer.to_string())
    }

    // We need to consider ranges to be 'streams' or 'bands' that we can split. The set of bands we
    // end up with will be much smaller than the total set of seeds.
    //
    // With each step, we split the range into smaller ranges, forming a tree.
    //
    // For sanity's sake, we will sort the farm's maps by source_start when we parse the farm.
    fn part_b(&self) -> anyhow::Result<String> {
        let farm = Farm::parse(&self.input)?;

        let seed_ranges: Vec<Range<u64>> = farm
            .seeds
            .chunks(2)
            .map(|chunk| {
                let from_seed = chunk[0];
                let length = chunk[1];
                from_seed..from_seed + length
            })
            .collect();

        let mut to_map = seed_ranges.clone();

        for step in farm.steps {
            let mut next_stage = vec![];
            for (source_range, destination_range) in step.map.iter() {
                let mut next_to_map = vec![];

                for current in to_map.iter() {
                    if current.is_empty() {
                        continue;
                    }

                    let (overlap, not_overlapping) =
                        overlap_ranges(current.clone(), source_range.clone());

                    if let Some(overlap) = overlap {
                        let from = overlap.start + destination_range.start - source_range.start;
                        let to = overlap.end + destination_range.start - source_range.start;
                        next_stage.push(from..to);
                    }

                    next_to_map.extend_from_slice(&not_overlapping);
                }

                to_map = next_to_map;
            }
            next_stage.extend_from_slice(&to_map);
            to_map = next_stage;
        }

        dbg!(&to_map);
        to_map.sort_by_key(|range| range.start);
        let answer = to_map[0].start;
        Ok(answer.to_string())
    }

    /*
    // This naive implementation is too slow.
    fn part_b(&self) -> anyhow::Result<String> {
        let farm = Farm::parse(&self.input)?;

        let seeds = farm.seeds.chunks(2).map(|chunk| {
            let from_seed = chunk[0];
            let length = chunk[1];
            from_seed..from_seed + length
        });

        let answer = seeds
            .map(|range| {
                range
                    .map(|seed| {
                        let mut ptr = seed;
                        for step in farm.steps.iter() {
                            ptr = step.jump(ptr);
                        }

                        ptr
                    })
                    .min()
                    .unwrap()
            })
            .min()
            .unwrap();

        Ok(answer.to_string())
    }
    */
}

fn overlap_ranges(upper: Range<u64>, lower: Range<u64>) -> (Option<Range<u64>>, Vec<Range<u64>>) {
    if lower.start > upper.end {
        (None, vec![upper.clone()])
    } else if upper.start > lower.end {
        (None, vec![upper.clone()])
    } else {
        let mut non_overlapping = vec![];
        if upper.start < lower.start {
            non_overlapping.push(upper.start..lower.start);
        }

        let middle =
            Some(std::cmp::max(lower.start, upper.start)..std::cmp::min(lower.end, upper.end));

        if upper.end > lower.end {
            non_overlapping.push(lower.end..upper.end);
        }

        (middle, non_overlapping)
    }
}

struct Map {
    map: Vec<(Range<u64>, Range<u64>)>,
}

impl Map {
    fn jump(&self, from: u64) -> u64 {
        for (from_range, to_range) in self.map.iter() {
            if from_range.contains(&from) {
                return to_range.start + (from - from_range.start);
            }
        }

        from
    }
}

struct Farm {
    seeds: Vec<u64>,
    steps: Vec<Map>,
}

impl Farm {
    fn parse(input: &str) -> anyhow::Result<Self> {
        parse_farm(input)
            .map(|(_, farm)| farm)
            .map_err(|e| anyhow::anyhow!("Could not parse farm: {}", e))
    }
}

/*
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
 */
fn parse_farm(input: &str) -> IResult<&str, Farm> {
    let (input, _) = nom::bytes::complete::tag("seeds: ")(input)?;
    let (input, seeds) = separated_list1(
        nom::character::complete::space1,
        nom::character::complete::u64,
    )(input)?;
    let (input, _) = nom::character::complete::multispace1(input)?;

    let mut maps = input
        .split("\n\n")
        .map(|section| {
            let mut ranges = vec![];
            for line in section.lines().skip(1) {
                let mut parts = line.split_whitespace();
                let destination_start = parts.next().unwrap().parse::<u64>().unwrap();
                let source_start = parts.next().unwrap().parse::<u64>().unwrap();
                let length = parts.next().unwrap().parse::<u64>().unwrap();

                ranges.push((
                    source_start..source_start + length,
                    destination_start..destination_start + length,
                ));
            }

            Map { map: ranges }
        })
        .collect::<Vec<_>>();

    maps.iter_mut()
        .for_each(|map| map.map.sort_by_key(|(from, _)| from.start));

    Ok(("", Farm { seeds, steps: maps }))
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::problem::Solution;

    #[test]
    fn range_set() {
        let mut set = HashSet::new();
        set.insert(1..3);

        assert!(set.contains(&(1..3)));
    }

    #[test]
    fn part_b() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";

        let expected = "46";

        let problem = super::Problem::with_input(input);

        assert_eq!(expected, problem.part_b().unwrap());
    }

    #[test]
    fn overlap_ranges_left() {
        let (overlap, not_overlapping) = super::overlap_ranges(1..3, 2..4);
        assert_eq!(overlap, Some(2..3));
        assert_eq!(not_overlapping, vec![1..2]);
    }

    #[test]
    fn overlap_ranges_right() {
        let (overlap, not_overlapping) = super::overlap_ranges(2..4, 1..3);
        assert_eq!(overlap, Some(2..3));
        assert_eq!(not_overlapping, vec![3..4]);
    }

    #[test]
    fn overlapp_ranges_full() {
        let (overlap, not_overlapping) = super::overlap_ranges(2..3, 1..4);
        assert_eq!(overlap, Some(2..3));
        assert_eq!(not_overlapping, vec![]);
    }

    #[test]
    fn overlap_ranges_small_lower() {
        let (overlap, not_overlapping) = super::overlap_ranges(1..4, 2..3);
        assert_eq!(overlap, Some(2..3));
        assert_eq!(not_overlapping, vec![1..2, 3..4]);
    }
}
