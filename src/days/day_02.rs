use std::io::{BufWriter, Write};

use aoclib_rs::{prep_io, printwriteln, split_and_parse};

struct IntcodeComputer {
    memory: Vec<usize>,
    instr: usize,
}

impl IntcodeComputer {
    fn new(memory: Vec<usize>) -> IntcodeComputer {
        IntcodeComputer { memory, instr: 0 }
    }

    fn set_input(&mut self, noun: usize, verb: usize) {
        self.memory[1] = noun;
        self.memory[2] = verb;
    }

    fn get_output(&self) -> usize {
        self.memory[0]
    }

    fn run(&mut self, verbose: bool) {
        loop {
            let op = Opcode::try_from(self.memory[self.instr]).unwrap();
            match op {
                Opcode::Add | Opcode::Multiply => {
                    let (src1, src2) = (self.memory[self.instr + 1], self.memory[self.instr + 2]);
                    let dst = self.memory[self.instr + 3];
                    match op {
                        Opcode::Add => {
                            let result = self.memory[src1] + self.memory[src2];
                            if verbose {
                                println!(
                                    "${} = ${} + ${} = {} + {} = {}",
                                    dst, src1, src2, self.memory[src1], self.memory[src2], result
                                );
                            }
                            self.memory[dst] = result;
                        }
                        Opcode::Multiply => {
                            let result = self.memory[src1] * self.memory[src2];
                            if verbose {
                                println!(
                                    "${} = ${} * ${} = {} * {} = {}",
                                    dst, src1, src2, self.memory[src1], self.memory[src2], result
                                );
                            }
                            self.memory[dst] = result;
                        }
                        _ => panic!("impossible"),
                    }
                    self.instr += 4;
                }
                Opcode::Terminate => return,
            }
        }
    }
}

enum Opcode {
    Add,
    Multiply,
    Terminate,
}

impl TryFrom<usize> for Opcode {
    type Error = String;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Opcode::Add),
            2 => Ok(Opcode::Multiply),
            99 => Ok(Opcode::Terminate),
            _ => Err(format!("Invalid opcode {}", value)),
        }
    }
}

pub fn run() {
    let mut contents = String::new();
    let (mut writer, contents) = prep_io(&mut contents, 2).unwrap();
    let memory: Vec<usize> = split_and_parse(contents[0], ",").unwrap();

    part1(&mut writer, memory.clone());
    part2(&mut writer, memory);
}

fn part1<W: Write>(writer: &mut BufWriter<W>, memory: Vec<usize>) {
    let mut c = IntcodeComputer::new(memory);
    c.set_input(12, 2);
    c.run(true /* verbose */);
    printwriteln!(writer, "{}", c.get_output()).unwrap();
}

fn part2<W: Write>(writer: &mut BufWriter<W>, memory: Vec<usize>) {
    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut c = IntcodeComputer::new(memory.clone());
            c.set_input(noun, verb);
            c.run(false /* verbose */);

            if c.get_output() == 19690720 {
                printwriteln!(writer, "100 * {} + {} = {}", noun, verb, 100 * noun + verb).unwrap();
                return;
            }
        }
    }
}
