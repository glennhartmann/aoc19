use crate::common::intcode::{IntcodeComputer, State};

use std::{
    cmp,
    cmp::Ord,
    io::{BufWriter, Write},
};

use aoclib_rs::{prep_io, printwriteln, split_and_parse};

struct NoneMax<T>(Option<T>);

impl<T> NoneMax<T>
where
    T: Ord + Copy,
{
    fn max(&self, other: T) -> Self {
        Self(Some(match self.0 {
            None => other,
            Some(max) => cmp::max(other, max),
        }))
    }
}

pub fn run() {
    let mut contents = String::new();
    let (mut writer, contents) = prep_io(&mut contents, 7).unwrap();
    let memory: Vec<i64> = split_and_parse(contents[0], ",").unwrap();

    part1(&mut writer, memory.clone());
    part2(&mut writer, memory);
}

fn part1<W: Write>(writer: &mut BufWriter<W>, memory: Vec<i64>) {
    let mut initial = Vec::new();
    let mut rest = vec![0, 1, 2, 3, 4];
    let max = get_max(memory.clone(), &mut initial, &mut rest, try_phase_part1);
    printwriteln!(writer, "{}", max).unwrap();
}

fn get_max(
    memory: Vec<i64>,
    perm: &mut Vec<i64>,
    rest: &mut Vec<i64>,
    try_phase: fn(Vec<i64>, &Vec<i64>) -> i64,
) -> i64 {
    if rest.is_empty() {
        let t = try_phase(memory.clone(), perm);
        println!("{:?}: {}", perm, t);
        return t;
    }

    let mut max = NoneMax(None);
    for i in 0..rest.len() {
        perm.push(rest.remove(i));

        max = max.max(get_max(memory.clone(), perm, rest, try_phase));

        rest.insert(i, perm.pop().expect("push/pop asymmetry"));
    }

    max.0.expect("no loop iterations - impossible")
}

fn try_phase_part1(memory: Vec<i64>, phase: &Vec<i64>) -> i64 {
    let mut signal = 0;
    for p in phase {
        let input = [*p, signal];
        let mut input_it = input.iter();
        let mut c = IntcodeComputer::new_with_io(
            memory.clone(),
            || *input_it.next().unwrap(),
            |i| signal = i,
        );
        c.run(false /* verbose */);
    }
    signal
}

fn part2<W: Write>(writer: &mut BufWriter<W>, memory: Vec<i64>) {
    let mut initial = Vec::new();
    let mut rest = vec![5, 6, 7, 8, 9];
    let max = get_max(memory.clone(), &mut initial, &mut rest, try_phase_part2);
    printwriteln!(writer, "{}", max).unwrap();
}

fn try_phase_part2(memory: Vec<i64>, phase: &Vec<i64>) -> i64 {
    const VERBOSE: bool = false;

    let mut computers = Vec::<IntcodeComputer<fn() -> i64, fn(i64)>>::new();
    for _ in phase {
        computers.push(IntcodeComputer::new(memory.clone()));
    }

    // first initialization - provide phase
    for (i, computer) in computers.iter_mut().enumerate() {
        computer.run(VERBOSE);
        if computer.get_state() != State::BlockedOnInput {
            panic!("unexpected state: {:?}", computer.get_state());
        }
        computer.provide_input(phase[i], VERBOSE);
    }

    let mut i = 0;
    let mut signal = 0;
    loop {
        match computers[i].get_state() {
            State::WaitingToRun => {}
            State::BlockedOnInput => {
                computers[i].provide_input(signal, VERBOSE);
            }
            _ => panic!("unexpected state: {:?}", computers[i].get_state()),
        }

        computers[i].run(VERBOSE);

        match computers[i].get_state() {
            State::BlockedOnInput => {
                i = (i + 1) % computers.len();
                continue;
            }
            State::BlockedOnOutput => {
                signal = computers[i].get_output(VERBOSE);
                continue;
            }
            State::Terminated => {
                if i == computers.len() - 1 {
                    break;
                }
                i = (i + 1) % computers.len();
                continue;
            }
            _ => panic!("unexpected state: {:?}", computers[i].get_state()),
        }
    }

    signal
}
