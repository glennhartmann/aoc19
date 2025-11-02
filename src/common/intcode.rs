pub struct IntcodeComputer {
    memory: Vec<usize>,
    instr: usize,
}

impl IntcodeComputer {
    pub fn new(memory: Vec<usize>) -> IntcodeComputer {
        IntcodeComputer { memory, instr: 0 }
    }

    pub fn set_input(&mut self, noun: usize, verb: usize) {
        self.memory[1] = noun;
        self.memory[2] = verb;
    }

    pub fn get_output(&self) -> usize {
        self.memory[0]
    }

    pub fn run(&mut self, verbose: bool) {
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

