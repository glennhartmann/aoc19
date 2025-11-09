use crate::common::{intcode, intcode::IntcodeComputer};

use std::{
    collections::{HashMap, HashSet},
    io::{BufWriter, Write},
};

use aoclib_rs::{
    dir::{Dir4, Direction},
    option_min_max::OptionMinMax,
    prep_io, printwriteln, split_and_parse,
};

#[derive(Copy, Clone)]
enum Colour {
    Black,
    White,
}

impl From<i64> for Colour {
    fn from(i: i64) -> Colour {
        match i {
            0 => Colour::Black,
            1 => Colour::White,
            _ => panic!("bad colour"),
        }
    }
}

impl From<Colour> for i64 {
    fn from(c: Colour) -> i64 {
        match c {
            Colour::Black => 0,
            Colour::White => 1,
        }
    }
}

impl From<Colour> for char {
    fn from(c: Colour) -> char {
        match c {
            Colour::Black => ' ',
            Colour::White => '█',
        }
    }
}

#[derive(Copy, Clone)]
enum State {
    Paint,
    Move,
}

#[derive(Copy, Clone)]
enum Turn {
    Left,
    Right,
}

impl From<i64> for Turn {
    fn from(i: i64) -> Turn {
        match i {
            0 => Turn::Left,
            1 => Turn::Right,
            _ => panic!("bad turn"),
        }
    }
}

pub fn run() {
    let mut contents = String::new();
    let (mut writer, contents) = prep_io(&mut contents, 11).unwrap();
    let memory: Vec<i64> = split_and_parse(contents[0], ",").unwrap();

    part1(&mut writer, memory.clone());
    part2(&mut writer, memory);
}

fn part1<W: Write>(writer: &mut BufWriter<W>, memory: Vec<i64>) {
    let (painted_count, _, _, _) =
        get_paint_data(true /* part1 */, memory, true /* verbose */);
    printwriteln!(writer, "{}", painted_count).unwrap();
}

type Point = (i64, i64);
type Panels = HashMap<Point, Colour>;

fn get_paint_data(part1: bool, memory: Vec<i64>, verbose: bool) -> (i64, Panels, Point, Point) {
    let (mut x, mut y) = (0, 0);
    let mut panels = HashMap::<Point, Colour>::new();
    let mut painted = HashSet::<Point>::new();
    let mut state = State::Paint;
    let mut dir = Dir4::Up;

    let mut c = IntcodeComputer::new(memory);
    c.run(verbose);

    let (mut min_x, mut min_y, mut max_x, mut max_y) = (
        OptionMinMax(None),
        OptionMinMax(None),
        OptionMinMax(None),
        OptionMinMax(None),
    );
    let mut painted_count = 0;
    loop {
        min_x = min_x.min(x);
        max_x = max_x.max(x);
        min_y = min_y.min(y);
        max_y = max_y.max(y);

        match c.get_state() {
            intcode::State::BlockedOnInput => {
                c.provide_input(
                    (*panels
                        .entry((x, y))
                        .or_insert(if part1 || x != 0 || y != 0 {
                            Colour::Black
                        } else {
                            Colour::White
                        }))
                    .into(),
                    verbose,
                );
                c.run(verbose);
            }
            intcode::State::BlockedOnOutput => {
                let o = c.get_output(verbose);
                match state {
                    State::Paint => {
                        panels
                            .entry((x, y))
                            .and_modify(|e| *e = o.into())
                            .or_insert(o.into());
                        if !painted.contains(&(x, y)) {
                            painted_count += 1;
                        }
                        painted.insert((x, y));
                        state = State::Move;
                    }
                    State::Move => {
                        match Turn::from(o) {
                            Turn::Left => {
                                dir = dir.rotate_left();
                            }
                            Turn::Right => {
                                dir = dir.rotate_right();
                            }
                        }
                        let d = dir.delta();
                        (x, y) = (x + d.0 as i64, y + d.1 as i64);
                        state = State::Paint;
                    }
                }
                c.run(verbose);
            }
            intcode::State::Terminated => break,
            _ => panic!("invalid state"),
        }
    }

    (
        painted_count,
        panels,
        (min_x.0.expect(""), min_y.0.expect("")),
        (max_x.0.expect(""), max_y.0.expect("")),
    )
}

fn part2<W: Write>(writer: &mut BufWriter<W>, memory: Vec<i64>) {
    let (_, panels, (min_x, min_y), (max_x, max_y)) =
        get_paint_data(false /* part1 */, memory, true /* verbose */);
    let x_range = max_x - min_x + 1;
    let y_range = max_y - min_y + 1;

    for y in 0..y_range {
        for x in 0..x_range {
            print!(
                "{}",
                char::from(
                    *panels
                        .get(&(x + min_x, y + min_y))
                        .or(Some(&Colour::Black))
                        .expect("")
                )
            );
        }
        println!();
    }

    printwriteln!(writer, "HCZRUGAZ").unwrap();
}
