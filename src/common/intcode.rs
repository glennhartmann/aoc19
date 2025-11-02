pub struct IntcodeComputer<FIF, POF>
where
    FIF: Fn() -> i32,
    POF: FnMut(i32),
{
    memory: Vec<i32>,
    instr: usize,
    fetch_input: FIF,
    provide_output: POF,
    opcode: Opcode,
    pmodes: Vec<ParameterMode>,
}

impl IntcodeComputer<fn() -> i32, fn(i32)> {
    pub fn new(memory: Vec<i32>) -> Self {
        IntcodeComputer {
            memory,
            instr: 0,
            fetch_input: || panic!("tried to fetch from uninitialized input"),
            provide_output: |_| panic!("tried to write to uninitialized output"),
            opcode: Opcode::Uninitialized,
            pmodes: Vec::new(),
        }
    }
}

impl<FIF, POF> IntcodeComputer<FIF, POF>
where
    FIF: Fn() -> i32,
    POF: FnMut(i32),
{
    pub fn new_with_io(memory: Vec<i32>, fetch_input: FIF, provide_output: POF) -> Self {
        IntcodeComputer {
            memory,
            instr: 0,
            fetch_input,
            provide_output,
            opcode: Opcode::Uninitialized,
            pmodes: Vec::new(),
        }
    }

    pub fn set_day2_input(&mut self, noun: i32, verb: i32) {
        self.memory[1] = noun;
        self.memory[2] = verb;
    }

    pub fn get_day2_output(&self) -> i32 {
        self.memory[0]
    }

    pub fn run(&mut self, verbose: bool) {
        loop {
            self.read_op();
            match self.opcode {
                Opcode::Add | Opcode::Multiply => {
                    let (p1, p2) = (self.get_param(1), self.get_param(2));
                    let dst = self.memory[self.instr + 3];
                    match self.opcode {
                        Opcode::Add => {
                            let result = p1 + p2;
                            if verbose {
                                println!("${} = {} + {} = {}", dst, p1, p2, result);
                            }
                            self.set_mem(dst, result);
                        }
                        Opcode::Multiply => {
                            let result = p1 * p2;
                            if verbose {
                                println!("${} = {} * {} = {}", dst, p1, p2, result);
                            }
                            self.set_mem(dst, result);
                        }
                        _ => panic!("impossible"),
                    }
                    self.instr += 4;
                }
                Opcode::Input => {
                    let dst = self.memory[self.instr + 1];
                    self.set_mem(dst, (self.fetch_input)());
                    if verbose {
                        println!("${} = $input = {}", dst, self.get_mem(dst));
                    }
                    self.instr += 2;
                }
                Opcode::Output => {
                    let p = self.get_param(1);
                    if verbose {
                        println!("$output = {}", p);
                    }
                    (self.provide_output)(p);
                    self.instr += 2;
                }
                Opcode::JumpIfTrue => {
                    let (p, dst) = (self.get_param(1), self.get_param(2));
                    if p != 0 {
                        self.instr = unsafe_i32_to_usize(dst);
                        if verbose {
                            println!("$ip = {}", dst);
                        }
                    } else {
                        self.instr += 3;
                        if verbose {
                            println!("no jump");
                        }
                    }
                }
                Opcode::JumpIfFalse => {
                    let (p, dst) = (self.get_param(1), self.get_param(2));
                    if p == 0 {
                        self.instr = unsafe_i32_to_usize(dst);
                        if verbose {
                            println!("$ip = {}", dst);
                        }
                    } else {
                        self.instr += 3;
                        if verbose {
                            println!("no jump");
                        }
                    }
                }
                Opcode::LessThan => {
                    let (p1, p2) = (self.get_param(1), self.get_param(2));
                    let dst = self.memory[self.instr + 3];
                    if p1 < p2 {
                        self.set_mem(dst, 1);
                        if verbose {
                            println!("${} = 1", dst);
                        }
                    } else {
                        self.set_mem(dst, 0);
                        if verbose {
                            println!("${} = 0", dst);
                        }
                    }
                    self.instr += 4;
                }
                Opcode::Equals => {
                    let (p1, p2) = (self.get_param(1), self.get_param(2));
                    let dst = self.memory[self.instr + 3];
                    if p1 == p2 {
                        self.set_mem(dst, 1);
                        if verbose {
                            println!("${} = 1", dst);
                        }
                    } else {
                        self.set_mem(dst, 0);
                        if verbose {
                            println!("${} = 0", dst);
                        }
                    }
                    self.instr += 4;
                }
                Opcode::Terminate => return,
                Opcode::Uninitialized => panic!("opcode uninitialized (never ran self.read_op()?)"),
            }
        }
    }

    fn get_mem(&self, src: i32) -> i32 {
        self.memory[unsafe_i32_to_usize(src)]
    }

    fn set_mem(&mut self, dst: i32, i: i32) {
        self.memory[unsafe_i32_to_usize(dst)] = i
    }

    fn read_op(&mut self) {
        let mut op = self.memory[self.instr];
        self.opcode = Opcode::try_from(op % 100).unwrap();
        op /= 100;

        self.pmodes = Vec::new();
        while op > 0 {
            self.pmodes.push(ParameterMode::try_from(op % 10).unwrap());
            op /= 10;
        }
    }

    fn get_param(&self, i: i32) -> i32 {
        let iusize = unsafe_i32_to_usize(i);
        let pmode = if self.pmodes.len() >= iusize {
            self.pmodes[iusize - 1]
        } else {
            ParameterMode::Position
        };
        let immediate = self.memory[self.instr + iusize];
        match pmode {
            ParameterMode::Position => self.get_mem(immediate),
            ParameterMode::Immediate => immediate,
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Opcode {
    Add,
    Multiply,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    Terminate,
    Uninitialized,
}

impl TryFrom<i32> for Opcode {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Opcode::Add),
            2 => Ok(Opcode::Multiply),
            3 => Ok(Opcode::Input),
            4 => Ok(Opcode::Output),
            5 => Ok(Opcode::JumpIfTrue),
            6 => Ok(Opcode::JumpIfFalse),
            7 => Ok(Opcode::LessThan),
            8 => Ok(Opcode::Equals),
            99 => Ok(Opcode::Terminate),
            _ => Err(format!("Invalid opcode {}", value)),
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum ParameterMode {
    Position,
    Immediate,
}

impl TryFrom<i32> for ParameterMode {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ParameterMode::Position),
            1 => Ok(ParameterMode::Immediate),
            _ => Err(format!("Invalid parameter mode {}", value)),
        }
    }
}

fn unsafe_i32_to_usize(i: i32) -> usize {
    usize::try_from(i).unwrap()
}
