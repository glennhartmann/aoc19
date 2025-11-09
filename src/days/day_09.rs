use crate::common::intcode::IntcodeComputer;

use std::io::{BufWriter, Write};

use aoclib_rs::{prep_io, printwriteln, split_and_parse};

pub fn run() {
    let mut contents = String::new();
    let (mut writer, contents) = prep_io(&mut contents, 9).unwrap();
    let memory: Vec<i64> = split_and_parse(contents[0], ",").unwrap();

    part1(&mut writer, memory.clone());
    part2(&mut writer, memory);
}

fn part1<W: Write>(writer: &mut BufWriter<W>, memory: Vec<i64>) {
    let mut output = Vec::new();
    let mut c = IntcodeComputer::new_with_io(memory, || 1, |i| output.push(i));
    c.run(true /* verbose */);

    if output.len() > 1 {
        for o in &output[..(output.len() - 1)] {
            println!("bad opcode: {}", o);
        }
        panic!();
    }

    printwriteln!(writer, "{}", output[0]).unwrap();
}

fn part2<W: Write>(writer: &mut BufWriter<W>, memory: Vec<i64>) {
    let mut c =
        IntcodeComputer::new_with_io(memory, || 2, |i| printwriteln!(writer, "{}", i).unwrap());
    c.run(true /* verbose */);
}
