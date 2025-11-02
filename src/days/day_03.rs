use std::{
    error::Error,
    fmt,
    io::{BufWriter, Write},
    str::FromStr,
};

use aoclib_rs::{
    dir::{Dir4, Direction},
    prep_io, printwriteln, split_and_parse,
};

struct Path {
    d: Dir4,
    l: usize,
}

impl FromStr for Path {
    type Err = PathErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let l = usize::from_str(&s[1..]).map_err(|e| PathErr::new(format!("{}", e)))?;
        let b: Vec<_> = s.bytes().collect();
        let d = match b[0] {
            b'U' => Dir4::Up,
            b'D' => Dir4::Down,
            b'L' => Dir4::Left,
            b'R' => Dir4::Right,
            _ => return Err(PathErr::new(format!("invalid direction: {}", b[0]))),
        };

        Ok(Path { d, l })
    }
}

#[derive(Debug)]
struct PathErr {
    err: String,
}

impl PathErr {
    fn new(err: String) -> Self {
        PathErr { err }
    }
}

impl fmt::Display for PathErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.err)
    }
}

impl Error for PathErr {}

pub fn run() {
    let mut contents = String::new();
    let (mut writer, contents) = prep_io(&mut contents, 3).unwrap();
    let input: Vec<Vec<Path>> = contents
        .iter()
        .map(|line| split_and_parse(line, ",").unwrap())
        .collect();

    part1(&mut writer, &input);
    part2(&mut writer, &input);
}

fn part1<W: Write>(writer: &mut BufWriter<W>, input: &[Vec<Path>]) {
    let origin = (20000, 20000);
    let intersections = find_intersections(input, origin);
    printwriteln!(
        writer,
        "{}",
        intersections
            .iter()
            .map(|i| manhattan_dist(origin, *i))
            .min()
            .unwrap()
    )
    .unwrap();
}

fn manhattan_dist(a: (usize, usize), b: (usize, usize)) -> usize {
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
}

fn part2<W: Write>(writer: &mut BufWriter<W>, input: &[Vec<Path>]) {
    let origin = (20000, 20000);
    let intersections = find_intersections(input, origin);
    printwriteln!(
        writer,
        "{}",
        intersections
            .iter()
            .map(|i| signal_delay(origin, *i, input))
            .min()
            .unwrap()
    )
    .unwrap();
}

fn signal_delay(
    origin: (usize, usize),
    intersection: (usize, usize),
    input: &[Vec<Path>],
) -> usize {
    let mut total: usize = 0;
    for wire in input {
        let mut curr = origin;
        let mut dist = 0;
        'outer: for p in wire {
            for _ in 0..p.l {
                curr = p.d.apply_delta_to_usizes(curr);
                dist += 1;
                if curr == intersection {
                    total += dist;
                    continue 'outer;
                }
            }
        }
    }
    total
}

fn find_intersections(input: &[Vec<Path>], origin: (usize, usize)) -> Vec<(usize, usize)> {
    let mut map = vec![vec![b'.'; 40000]; 40000];
    map[origin.0][origin.1] = b'o';

    let mut intersections: Vec<(usize, usize)> = Vec::new();

    for (i, wire) in input.iter().enumerate() {
        let mut curr = origin;
        for p in wire {
            for _ in 0..p.l {
                curr = p.d.apply_delta_to_usizes(curr);
                match map[curr.0][curr.1] {
                    b'1' => {
                        if i == 1 {
                            map[curr.0][curr.1] = b'X';
                            intersections.push(curr);
                        }
                    }
                    b'X' => (),
                    _ => {
                        map[curr.0][curr.1] = if i == 0 { b'1' } else { b'2' };
                    }
                };
            }
        }
    }

    for i in &intersections {
        println!("{:?}", *i);
    }

    intersections
}
