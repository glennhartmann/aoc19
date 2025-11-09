use crate::common::intcode::IntcodeComputer;

use std::io::{BufWriter, Write};

use aoclib_rs::{prep_io, printwriteln, split_and_parse};

pub fn run() {
    let mut contents = String::new();
    let (mut writer, contents) = prep_io(&mut contents, 2).unwrap();
    let memory: Vec<i64> = split_and_parse(contents[0], ",").unwrap();

    part1(&mut writer, memory.clone());
    part2(&mut writer, memory);
}

fn part1<W: Write>(writer: &mut BufWriter<W>, memory: Vec<i64>) {
    let mut c = IntcodeComputer::new(memory.clone());
    c.set_day2_input(12, 2);
    c.run(true /* verbose */);
    printwriteln!(writer, "{}", c.get_day2_output()).unwrap();
}

fn part2<W: Write>(writer: &mut BufWriter<W>, memory: Vec<i64>) {
    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut c = IntcodeComputer::new(memory.clone());
            c.set_day2_input(noun, verb);
            c.run(false /* verbose */);

            if c.get_day2_output() == 19690720 {
                printwriteln!(writer, "100 * {} + {} = {}", noun, verb, 100 * noun + verb).unwrap();
                return;
            }
        }
    }
}
