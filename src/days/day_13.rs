use crate::common::intcode::{IntcodeComputer, State};

use std::{
    cmp::Ordering,
    collections::HashMap,
    io::{BufWriter, Write},
};

use aoclib_rs::{option_min_max::OptionMinMax, prep_io, printwriteln, split_and_parse};

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
enum Tile {
    Empty,
    Wall,
    Block,
    HorizontalPaddle,
    Ball,
}

impl From<i64> for Tile {
    fn from(i: i64) -> Tile {
        match i {
            0 => Tile::Empty,
            1 => Tile::Wall,
            2 => Tile::Block,
            3 => Tile::HorizontalPaddle,
            4 => Tile::Ball,
            _ => panic!("invalid tile"),
        }
    }
}

impl From<Tile> for char {
    fn from(t: Tile) -> char {
        match t {
            Tile::Empty => ' ',
            Tile::Wall => '█',
            Tile::Block => '#',
            Tile::HorizontalPaddle => '-',
            Tile::Ball => 'o',
        }
    }
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
enum Joystick {
    Left,
    Neutral,
    Right,
}

impl From<Joystick> for i64 {
    fn from(j: Joystick) -> i64 {
        match j {
            Joystick::Left => -1,
            Joystick::Neutral => 0,
            Joystick::Right => 1,
        }
    }
}

pub fn run() {
    let mut contents = String::new();
    let (mut writer, contents) = prep_io(&mut contents, 13).unwrap();
    let memory: Vec<i64> = split_and_parse(contents[0], ",").unwrap();

    part1(&mut writer, memory.clone());
    part2(&mut writer, memory);
}

fn part1<W: Write>(writer: &mut BufWriter<W>, memory: Vec<i64>) {
    const VERBOSE: bool = true;

    let mut screen: HashMap<(i64, i64), Tile> = HashMap::new();

    let mut c = IntcodeComputer::new(memory);
    c.run(VERBOSE);

    let (mut min_x, mut min_y, mut max_x, mut max_y) = (
        OptionMinMax(None),
        OptionMinMax(None),
        OptionMinMax(None),
        OptionMinMax(None),
    );
    loop {
        match c.get_state() {
            State::BlockedOnOutput => {
                let x = c.get_output(VERBOSE);
                c.run(VERBOSE);

                min_x = min_x.min(x);
                max_x = max_x.max(x);

                let y = c.get_output(VERBOSE);
                c.run(VERBOSE);

                min_y = min_y.min(y);
                max_y = max_y.max(y);

                let tile = Tile::from(c.get_output(VERBOSE));
                screen
                    .entry((x, y))
                    .and_modify(|e| *e = tile)
                    .or_insert(tile);

                c.run(VERBOSE);
            }
            State::Terminated => break,
            _ => panic!("invalid state"),
        }
    }

    // min_x: 0, min_y: 0, max_x: 39, max_y: 24
    println!(
        "min_x: {}, min_y: {}, max_x: {}, max_y: {}",
        min_x.0.expect(""),
        min_y.0.expect(""),
        max_x.0.expect(""),
        max_y.0.expect("")
    );

    let num_blocks = screen
        .values()
        .fold(0, |acc, e| acc + if *e == Tile::Block { 1 } else { 0 });
    printwriteln!(writer, "{}", num_blocks).unwrap();
}

fn part2<W: Write>(writer: &mut BufWriter<W>, mut memory: Vec<i64>) {
    const VERBOSE: bool = false;
    const HEIGHT: usize = 25;
    const WIDTH: usize = 40;
    const PRINT_EVERY: u32 = 50;

    let mut screen: Vec<Vec<Tile>> = vec![vec![Tile::Empty; WIDTH]; HEIGHT];

    memory[0] = 2;
    let mut c = IntcodeComputer::new(memory);
    c.run(VERBOSE);

    let mut score = 0;
    let mut ticks = 0;
    let mut ball_x: Option<i64> = None;
    let mut paddle_x: Option<i64> = None;
    loop {
        match c.get_state() {
            State::BlockedOnInput => {
                ticks += 1;
                if ticks == PRINT_EVERY {
                    println!("score: {}", score);
                    for row in &screen {
                        for tile in row {
                            print!("{}", char::from(*tile));
                        }
                        println!();
                    }
                    ticks = 0;
                }

                c.provide_input(
                    i64::from(match paddle_x.expect("").cmp(&ball_x.expect("")) {
                        Ordering::Less => Joystick::Right,
                        Ordering::Equal => Joystick::Neutral,
                        Ordering::Greater => Joystick::Left,
                    }),
                    VERBOSE,
                );

                c.run(VERBOSE);
            }
            State::BlockedOnOutput => {
                let x = c.get_output(VERBOSE);
                c.run(VERBOSE);

                let y = c.get_output(VERBOSE);
                c.run(VERBOSE);

                if x == -1 && y == 0 {
                    score = c.get_output(VERBOSE);
                } else {
                    let tile = Tile::from(c.get_output(VERBOSE));
                    screen[y as usize][x as usize] = tile;
                    match tile {
                        Tile::HorizontalPaddle => paddle_x = Some(x),
                        Tile::Ball => ball_x = Some(x),
                        _ => {}
                    }
                }

                c.run(VERBOSE);
            }
            State::Terminated => break,
            _ => panic!("invalid state"),
        }
    }

    printwriteln!(writer, "{}", score).unwrap();
}
