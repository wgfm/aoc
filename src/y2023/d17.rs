use crate::{problem::Solution, solution};

solution!(2023, 17);

impl Solution for Problem {
    fn part_a(&self) -> anyhow::Result<String> {
        Ok(solve(&self.input, 1, 3).to_string())
    }

    fn part_b(&self) -> anyhow::Result<String> {
        Ok(solve(&self.input, 4, 10).to_string())
    }
}

#[derive(Eq, PartialEq, Debug)]
struct State {
    pos: (usize, usize),
    cost: u32,
    dir: (i8, i8),
    steps: u32,
    path: Vec<(usize, usize)>,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn parse(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| line.chars().map(|ch| ch.to_digit(10).unwrap()).collect())
        .collect()
}

fn solve(input: &str, min_step: u32, max_step: u32) -> usize {
    let city_blocks = parse(input);
    let mut visited = std::collections::HashSet::new();

    let bounding_box = (city_blocks[0].len(), city_blocks.len());

    let mut heap = std::collections::BinaryHeap::new();
    heap.push(State {
        pos: (0, 0),
        cost: 0,
        dir: (0, 0),
        steps: 1,
        path: vec![],
    });

    let target = (city_blocks[0].len() - 1, city_blocks.len() - 1);

    while let Some(state) = heap.pop() {
        let (x, y) = state.pos;
        if visited.contains(&(x, y, state.dir, state.steps)) {
            continue;
        }

        if state.pos == target {
            return state.cost as usize;
        }

        visited.insert((x, y, state.dir, state.steps));

        if let Some(next_state) = try_step(
            &city_blocks,
            &state,
            bounding_box,
            (0, 1),
            min_step,
            max_step,
        ) {
            heap.push(next_state);
        };
        if let Some(next_state) = try_step(
            &city_blocks,
            &state,
            bounding_box,
            (0, -1),
            min_step,
            max_step,
        ) {
            heap.push(next_state);
        };
        if let Some(next_state) = try_step(
            &city_blocks,
            &state,
            bounding_box,
            (1, 0),
            min_step,
            max_step,
        ) {
            heap.push(next_state);
        };
        if let Some(next_state) = try_step(
            &city_blocks,
            &state,
            bounding_box,
            (-1, 0),
            min_step,
            max_step,
        ) {
            heap.push(next_state);
        };
    }

    unreachable!()
}

fn try_step(
    city_blocks: &[Vec<u32>],
    curr_state: &State,
    bounding_box: (usize, usize),
    direction: (i8, i8),
    min_moves: u32,
    max_moves: u32,
) -> Option<State> {
    let (x, y) = curr_state.pos;
    if (x as i32 + direction.0 as i32) < 0
        || (y as i32 + direction.1 as i32) < 0
        || (x as i32 + direction.0 as i32) >= (bounding_box.0 as i32)
        || (y as i32 + direction.1 as i32) >= (bounding_box.1 as i32)
    {
        return None;
    }

    if curr_state.steps == max_moves && direction == curr_state.dir {
        return None;
    }

    if curr_state.steps < min_moves && direction != curr_state.dir && curr_state.dir != (0, 0) {
        return None;
    }

    if direction.0 == -curr_state.dir.0 && direction.1 == -curr_state.dir.1 {
        return None;
    }

    let steps = if direction == curr_state.dir {
        curr_state.steps + 1
    } else {
        1
    };

    let pos = (
        (x as i32 + direction.0 as i32) as usize,
        (y as i32 + direction.1 as i32) as usize,
    );

    let mut path = curr_state.path.clone();
    path.push(pos);

    Some(State {
        pos,
        cost: curr_state.cost + city_blocks[pos.1][pos.0],
        dir: direction,
        steps,
        path,
    })
}

#[cfg(test)]
mod tests {
    use crate::problem::Solution;

    use super::Problem;

    const INPUT: &'static str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

    #[test]
    fn part_a() {
        let p = Problem::with_input(INPUT);

        assert_eq!(p.part_a().unwrap(), "102");
    }

    #[test]
    fn part_b() {
        let p = Problem::with_input(INPUT);

        assert_eq!(p.part_b().unwrap(), "94");
    }

    #[test]
    fn minus_zero() {
        let a: i8 = 0;
        assert_eq!(a, -a);
    }
}
