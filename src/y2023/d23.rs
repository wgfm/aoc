use std::{
    collections::{HashMap, HashSet},
    fmt::Formatter,
};

use crate::{problem::Solution, solution};

solution!(2023, 23);

impl Solution for Problem {
    fn part_a(&self) -> anyhow::Result<String> {
        let map = parse(&self.input);
        let start = (1, 0);
        let end = (map[0].len() - 2, map.len() - 1);

        let graph = build_graph(&map, start.into(), end.into());

        dbg!(graph);

        todo!()
    }

    fn part_b(&self) -> anyhow::Result<String> {
        todo!()
    }
}

struct Walker {
    steps: usize,
    location: Point,
    origin: Point,
}

impl Walker {
    fn new(origin: Point) -> Self {
        Self {
            steps: 0,
            location: origin,
            origin,
        }
    }
}

#[derive(Hash, Eq, PartialEq)]
struct Node {
    location: Point,
}

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn translate(&self, direction: (isize, isize)) -> Self {
        Self {
            // Actually unsafe, but makes boundary checking easy
            x: (self.x as isize + direction.0) as usize,
            y: (self.y as isize + direction.1) as usize,
        }
    }
}

impl From<(usize, usize)> for Point {
    fn from((x, y): (usize, usize)) -> Self {
        Self { x, y }
    }
}

impl std::fmt::Debug for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn build_graph(map: &[Vec<Tile>], start: Point, end: Point) -> HashMap<Point, Vec<(Point, usize)>> {
    let mut visited: HashSet<Point> = HashSet::new();
    let mut adjacencies: HashMap<Point, Vec<(Point, usize)>> = HashMap::new();
    adjacencies.insert(start, vec![]);

    walk(map, &mut adjacencies, &mut visited, start, end);

    adjacencies
}

fn walk(
    map: &[Vec<Tile>],
    adjacencies: &mut HashMap<Point, Vec<(Point, usize)>>,
    visited: &mut HashSet<Point>,
    start: Point,
    end: Point,
) {
    let mut walker = Walker::new(start);

    let mut possibilities = vec![];
    let directions = vec![(0, -1), (0, 1), (-1, 0), (1, 0)];
    loop {
        visited.insert(walker.location);

        if walker.location == end {
            break;
        }

        for direction in &directions {
            let p = walker.location.translate(*direction);
            if p.y >= map.len() || p.x >= map[0].len() {
                continue;
            }

            if map[p.y][p.x].can_walk(*direction) && !visited.contains(&p) {
                possibilities.push(p);
            }

            if visited.contains(&p) && adjacencies.contains_key(&p) && p != walker.origin {
                adjacencies
                    .get_mut(&walker.origin)
                    .unwrap()
                    .push((p, walker.steps));
                return;
            }
        }

        if possibilities.len() > 1 {
            break;
        }

        if let Some(l) = possibilities.pop() {
            walker.location = l
        } else {
            panic!("dead end at {:?}", walker.location);
        }
    }

    adjacencies.insert(walker.location, vec![]);
    adjacencies
        .get_mut(&walker.location)
        .unwrap()
        .push((walker.location, walker.steps));

    for p in possibilities {
        walk(map, adjacencies, visited, p, end);
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Tile {
    Path,
    Forest,
    Slope(i8, i8),
}

impl Tile {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Path,
            '#' => Self::Forest,
            '^' => Self::Slope(0, -1),
            'v' => Self::Slope(0, 1),
            '>' => Self::Slope(1, 0),
            '<' => Self::Slope(-1, 0),
            _ => panic!("Invalid tile"),
        }
    }

    fn can_walk(&self, direction: (isize, isize)) -> bool {
        match self {
            Tile::Forest => false,
            Tile::Path => true,
            Tile::Slope(dx, dy) => (*dx as isize, *dy as isize) == direction,
        }
    }
}

fn parse(input: &str) -> Vec<Vec<Tile>> {
    input
        .lines()
        .map(|line| line.chars().map(Tile::from_char).collect())
        .collect()
}
