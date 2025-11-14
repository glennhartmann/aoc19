use crate::common::intcode::{IntcodeComputer, IntcodeComputerDefault, State};

use std::{
    collections::HashMap,
    io::{BufWriter, Write},
    iter,
};

use aoclib_rs::{
    dijkstra::{Dijkstrable, PqElement},
    dir::{Dir4, Direction},
    option_min_max::OptionMinMax,
    prep_io, printwriteln, split_and_parse,
};

const START: (i64, i64) = (0, 0);

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
enum Status {
    HitWall,
    Moved,
    MovedAndOxygen,
}

impl From<i64> for Status {
    fn from(i: i64) -> Status {
        match i {
            0 => Status::HitWall,
            1 => Status::Moved,
            2 => Status::MovedAndOxygen,
            _ => panic!("invalid status"),
        }
    }
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
enum Tile {
    Empty,
    Wall,
    Oxygen,
    Unknown,
}

impl From<Tile> for char {
    fn from(t: Tile) -> char {
        match t {
            Tile::Empty => '.',
            Tile::Wall => '#',
            Tile::Oxygen => 'O',
            Tile::Unknown => '?',
        }
    }
}

struct StackFrame {
    dir_iter: Box<dyn Iterator<Item = Dir4>>,
    loc: (i64, i64),
    curr_dir: Option<Dir4>,
}

impl StackFrame {
    fn new() -> Self {
        Self::new_with_loc(START)
    }

    fn new_with_loc(loc: (i64, i64)) -> Self {
        Self {
            dir_iter: Box::new(Dir4::iter()),
            loc,
            curr_dir: None,
        }
    }
}

struct Map {
    m: HashMap<(i64, i64), Tile>,
    dists: HashMap<(i64, i64), Option<i64>>,
}

impl Map {
    fn new() -> Self {
        Self {
            m: HashMap::new(),
            dists: HashMap::new(),
        }
    }
}

impl Dijkstrable for Map {
    type Point = (i64, i64);
    type Bounds = ((i64, i64), (i64, i64));
    type Dist = i64;
    type PQE = PqElement<(i64, i64), i64>;

    fn neighbours(
        p: Self::Point,
        _: Self::Bounds,
    ) -> impl Iterator<Item = (Self::Point, Self::Dist)> {
        let mut dirs = Dir4::iter();
        iter::from_fn(move || {
            if let Some(d) = dirs.next() {
                let delta = d.delta();
                return Some(((p.0 + delta.0 as i64, p.1 + delta.1 as i64), 1));
            }
            None
        })
    }

    fn is_impossible(&self, p: Self::Point) -> bool {
        let t = self.m.get(&p).expect("");
        *t == Tile::Wall
    }

    fn dist(&self, p: Self::Point) -> Option<Self::Dist> {
        *self.dists.get(&p)?
    }

    fn set_dist(&mut self, p: Self::Point, d: Option<Self::Dist>) {
        self.dists.insert(p, d);
    }
}

pub fn run() {
    let mut contents = String::new();
    let (mut writer, contents) = prep_io(&mut contents, 15).unwrap();
    let memory: Vec<i64> = split_and_parse(contents[0], ",").unwrap();

    let m = part1(&mut writer, memory);
    part2(&mut writer, m);
}

fn part1<W: Write>(writer: &mut BufWriter<W>, memory: Vec<i64>) -> Map {
    let (mut m, oxygen, min_x, max_x, min_y, max_y) = depth_first_flood_fill(memory);
    print_map(&m, min_x, max_x, min_y, max_y);
    m.dijkstra(START, 0, ((min_x, min_y), (max_x, max_y)));
    printwriteln!(writer, "{}", m.dists.get(&oxygen).expect("").expect("")).unwrap();
    m
}

fn depth_first_flood_fill(memory: Vec<i64>) -> (Map, (i64, i64), i64, i64, i64, i64) {
    const VERBOSE: bool = false;

    let mut c = IntcodeComputer::new(memory);
    c.run(VERBOSE);

    let mut m = Map::new();
    m.m.insert(START, Tile::Empty);

    let (mut min_x, mut min_y, mut max_x, mut max_y) = (
        OptionMinMax(None),
        OptionMinMax(None),
        OptionMinMax(None),
        OptionMinMax(None),
    );
    let mut stack: Vec<StackFrame> = Vec::new();
    let mut frame = StackFrame::new();
    let mut oxygen: Option<(i64, i64)> = None;
    loop {
        println!("stack frame {}", stack.len());
        println!("  loc: {:?}", frame.loc);

        min_x = min_x.min(frame.loc.0);
        max_x = max_x.max(frame.loc.0);
        min_y = min_y.min(frame.loc.1);
        max_y = max_y.max(frame.loc.1);

        let Some(next_dir) = frame.dir_iter.next() else {
            frame = match stack.pop() {
                None => break,
                Some(f) => f,
            };

            let back_dir = frame.curr_dir.expect("").opposite();
            let s = try_move(&mut c, back_dir, VERBOSE);
            if s == Status::HitWall {
                panic!("hit wall returning to previous location");
            }

            continue;
        };

        println!("  next dir: {:?}", next_dir);

        let delta = next_dir.delta();
        let new_loc = (frame.loc.0 + delta.0 as i64, frame.loc.1 + delta.1 as i64);

        println!("  new loc: {:?}", new_loc);

        if m.m.contains_key(&new_loc) {
            println!("    already visited; skipping");
            continue;
        }

        match try_move(&mut c, next_dir, VERBOSE) {
            Status::HitWall => {
                println!("  hit wall");
                m.m.insert(new_loc, Tile::Wall);
                min_x = min_x.min(new_loc.0);
                max_x = max_x.max(new_loc.0);
                min_y = min_y.min(new_loc.1);
                max_y = max_y.max(new_loc.1);
                continue;
            }
            Status::Moved => {
                println!("  moved");
                m.m.insert(new_loc, Tile::Empty);
            }
            Status::MovedAndOxygen => {
                println!("  moved and found oxygen");
                oxygen = Some(new_loc);
                m.m.insert(new_loc, Tile::Oxygen);
            }
        }

        frame.curr_dir = Some(next_dir);
        stack.push(frame);
        frame = StackFrame::new_with_loc(new_loc);
    }

    (
        m,
        oxygen.expect(""),
        min_x.0.expect(""),
        max_x.0.expect(""),
        min_y.0.expect(""),
        max_y.0.expect(""),
    )
}

fn i64_from_dir4(d: Dir4) -> i64 {
    match d {
        Dir4::Up => 1,
        Dir4::Down => 2,
        Dir4::Left => 3,
        Dir4::Right => 4,
    }
}

fn print_map(m: &Map, min_x: i64, max_x: i64, min_y: i64, max_y: i64) {
    let x_range = max_x - min_x + 1;
    let y_range = max_y - min_y + 1;

    for y in 0..y_range {
        for x in 0..x_range {
            print!(
                "{}",
                char::from(
                    *m.m.get(&(x + min_x, y + min_y))
                        .or(Some(&Tile::Unknown))
                        .expect("")
                )
            );
        }
        println!();
    }
}

fn try_move(c: &mut IntcodeComputerDefault, next_dir: Dir4, verbose: bool) -> Status {
    if c.get_state() != State::BlockedOnInput {
        panic!("should be expecting input");
    }
    c.provide_input(i64_from_dir4(next_dir), verbose);
    c.run(verbose);

    if c.get_state() != State::BlockedOnOutput {
        panic!("should be providing output");
    }
    let s = Status::from(c.get_output(verbose));
    c.run(verbose);

    s
}

fn part2<W: Write>(writer: &mut BufWriter<W>, mut m: Map) {
    let mut minutes = 0;
    while !is_saturated(&m) {
        minutes += 1;
        let mut m2 = m.m.clone();
        for (k, v) in &m.m {
            match *v {
                Tile::Empty | Tile::Wall | Tile::Unknown => {
                    if !m2.contains_key(k) {
                        m2.insert(*k, *v);
                    }
                }
                Tile::Oxygen => {
                    m2.insert(*k, *v);
                    for d in Dir4::iter() {
                        let delta = d.delta();
                        let neighbour = (k.0 + delta.0 as i64, k.1 + delta.1 as i64);
                        if let Some(t) = m.m.get(&neighbour)
                            && *t != Tile::Wall
                        {
                            m2.insert(neighbour, *v);
                        }
                    }
                }
            }
        }
        m.m = m2;
    }
    printwriteln!(writer, "{}", minutes).unwrap();
}

fn is_saturated(m: &Map) -> bool {
    let f: Vec<_> = m.m.iter().filter(|(_, v)| **v == Tile::Empty).collect();
    f.is_empty()
}
