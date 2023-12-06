use crate::{problem::Solution, solution};

solution!(2023, 6);

impl Solution for Problem {
    fn part_a(&self) -> anyhow::Result<String> {
        let races = input_a();

        let mut answer = 1;
        for race in races {
            let mut ways_to_win = 0;
            for t in 0..race.time {
                let v = t;
                let remaining_time = race.time - t;

                if remaining_time * v > race.distance {
                    ways_to_win += 1;
                }
            }

            answer *= ways_to_win;
        }

        Ok(answer.to_string())
    }

    fn part_b(&self) -> anyhow::Result<String> {
        let race = input_b();
        let mut ways_to_win = 0;
        for t in 0..race.time {
            let v = t;
            let remaining_time = race.time - t;

            if remaining_time * v > race.distance {
                ways_to_win += 1;
            }
        }

        Ok(ways_to_win.to_string())
    }
}

struct Race {
    time: u64,
    distance: u64,
}

fn input_b() -> Race {
    Race {
        time: 40828492,
        distance: 233101111101487,
    }
}

// Time:        40     82     84     92
// Distance:   233   1011   1110   1487
fn input_a() -> Vec<Race> {
    vec![
        Race {
            time: 40,
            distance: 233,
        },
        Race {
            time: 82,
            distance: 1011,
        },
        Race {
            time: 84,
            distance: 1110,
        },
        Race {
            time: 92,
            distance: 1487,
        },
    ]
}
