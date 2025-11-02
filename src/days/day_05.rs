use crate::common::intcode::IntcodeComputer;

use std::io::{BufWriter, Write};

use aoclib_rs::{prep_io, printwriteln, split_and_parse};

pub fn run() {
    let mut contents = String::new();
    let (mut writer, contents) = prep_io(&mut contents, 5).unwrap();
    let memory: Vec<i32> = split_and_parse(contents[0], ",").unwrap();

    part1(&mut writer, memory.clone());
    part2(&mut writer, memory);
}

fn part1<W: Write>(writer: &mut BufWriter<W>, memory: Vec<i32>) {
    let mut output = Vec::new();
    let mut c = IntcodeComputer::new_with_io(memory, || 1, |i| output.push(i));
    c.run(true /* verbose */);

    for o in &output[..(output.len() - 1)] {
        if *o != 0 {
            panic!("found non-zero error code");
        }
    }

    printwriteln!(writer, "{}", output[output.len() - 1]).unwrap();
}

fn part2<W: Write>(writer: &mut BufWriter<W>, memory: Vec<i32>) {
    let mut output = Vec::new();
    let mut c = IntcodeComputer::new_with_io(memory, || 5, |i| output.push(i));
    c.run(true /* verbose */);

    if output.len() != 1 {
        panic!("got {} outputs, expected 1", output.len());
    }

    printwriteln!(writer, "{}", output[0]).unwrap();
}
