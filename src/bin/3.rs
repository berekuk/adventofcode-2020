use std::io::{self, BufRead};

type MapValue = bool;

struct Map(Vec<Vec<MapValue>>);

struct Slope {
    dx: u64,
    dy: u64,
}

fn load_map(input: &mut dyn BufRead) -> Map {
    let mut map: Map = Map(Vec::new());
    for line in input.lines() {
        let s = line.unwrap();
        let mut line = Vec::new();
        for c in s.trim().chars() {
            let v = c == '#';
            line.push(v);
        }
        map.push(line);
    }
    map
}

impl Map {
    fn lookup(&self, x: u64, y: u64) -> MapValue {
        self.0[y as usize][(x as usize) % self.columns()]
    }
    fn push(&mut self, v: Vec<MapValue>) {
        self.0.push(v);
    }
    fn rows(&self) -> usize {
        self.0.len()
    }
    fn columns(&self) -> usize {
        self.0[0].len()
    }
}

fn slide(map: &Map, slope: &Slope) -> u64 {
    let mut x: u64 = 0;
    let mut y: u64 = 0;
    let mut trees: u64 = 0;

    while (y as usize) < map.rows() {
        if map.lookup(x, y) {
            trees += 1;
        }
        x += slope.dx;
        y += slope.dy;
    }
    trees
}

fn main() {
    let stdin = io::stdin();
    let map = load_map(&mut stdin.lock());
    let slopes = [
        Slope { dx: 1, dy: 1 },
        Slope { dx: 3, dy: 1 },
        Slope { dx: 5, dy: 1 },
        Slope { dx: 7, dy: 1 },
        Slope { dx: 1, dy: 2 },
    ];

    let mut total: u64 = 1;
    for slope in slopes.iter() {
        let trees = slide(&map, &slope);
        total *= trees;
    }
    println!("{}", total);
}
