use std::iter::repeat_n;

pub struct IntcodeComputer<FIF, POF>
where
    FIF: FnMut() -> i64,
    POF: FnMut(i64),
{
    memory: Vec<i64>,
    instr: usize,
    fetch_input: FIF,
    provide_output: POF,
    opcode: Opcode,
    pmodes: Vec<ParameterMode>,
    state: State,
    blocking_io: bool,
    relative_base: i64,
}

impl IntcodeComputer<fn() -> i64, fn(i64)> {
    pub fn new(memory: Vec<i64>) -> Self {
        IntcodeComputer {
            memory,
            instr: 0,
            fetch_input: || panic!("tried to fetch from uninitialized input"),
            provide_output: |_| panic!("tried to write to uninitialized output"),
            opcode: Opcode::Uninitialized,
            pmodes: Vec::new(),
            state: State::WaitingToRun,
            blocking_io: true,
            relative_base: 0,
        }
    }
}

impl<FIF, POF> IntcodeComputer<FIF, POF>
where
    FIF: FnMut() -> i64,
    POF: FnMut(i64),
{
    pub fn new_with_io(memory: Vec<i64>, fetch_input: FIF, provide_output: POF) -> Self {
        IntcodeComputer {
            memory,
            instr: 0,
            fetch_input,
            provide_output,
            opcode: Opcode::Uninitialized,
            pmodes: Vec::new(),
            state: State::WaitingToRun,
            blocking_io: false,
            relative_base: 0,
        }
    }

    pub fn set_day2_input(&mut self, noun: i64, verb: i64) {
        self.memory[1] = noun;
        self.memory[2] = verb;
    }

    pub fn get_day2_output(&self) -> i64 {
        self.memory[0]
    }

    pub fn run(&mut self, verbose: bool) {
        loop {
            self.read_op();
            match self.opcode {
                Opcode::Add | Opcode::Multiply => {
                    let (p1, p2) = (self.get_src_param(1), self.get_src_param(2));
                    let dst = self.get_dst_param(3);
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
                    if self.blocking_io {
                        self.state = State::BlockedOnInput;
                        return;
                    }

                    let dst = self.get_dst_param(1);
                    let input = (self.fetch_input)();
                    self.set_mem(dst, input);
                    if verbose {
                        println!("${} = $input = {}", dst, self.get_mem(dst));
                    }
                    self.instr += 2;
                }
                Opcode::Output => {
                    if self.blocking_io {
                        self.state = State::BlockedOnOutput;
                        return;
                    }

                    let p = self.get_src_param(1);
                    if verbose {
                        println!("$output = {}", p);
                    }
                    (self.provide_output)(p);
                    self.instr += 2;
                }
                Opcode::JumpIfTrue => {
                    let (p, dst) = (self.get_src_param(1), self.get_src_param(2));
                    if p != 0 {
                        self.instr = unsafe_i64_to_usize(dst);
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
                    let (p, dst) = (self.get_src_param(1), self.get_src_param(2));
                    if p == 0 {
                        self.instr = unsafe_i64_to_usize(dst);
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
                    let (p1, p2) = (self.get_src_param(1), self.get_src_param(2));
                    let dst = self.get_dst_param(3);
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
                    let (p1, p2) = (self.get_src_param(1), self.get_src_param(2));
                    let dst = self.get_dst_param(3);
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
                Opcode::RelativeBaseOffset => {
                    let param = self.get_src_param(1);
                    if verbose {
                        println!(
                            "$relative_base += ({}) = {}",
                            param,
                            self.relative_base + param
                        );
                    }
                    self.relative_base += param;
                    self.instr += 2;
                }
                Opcode::Terminate => {
                    self.state = State::Terminated;
                    return;
                }
                Opcode::Uninitialized => panic!("opcode uninitialized (never ran self.read_op()?)"),
            }
        }
    }

    pub fn get_state(&self) -> State {
        self.state
    }

    pub fn provide_input(&mut self, i: i64, verbose: bool) {
        let dst = self.get_dst_param(1);
        self.set_mem(dst, i);
        if verbose {
            println!("${} = $input = {}", dst, self.get_mem(dst));
        }
        self.instr += 2;
        self.state = State::WaitingToRun;
    }

    pub fn get_output(&mut self, verbose: bool) -> i64 {
        let p = self.get_src_param(1);
        if verbose {
            println!("$output = {}", p);
        }
        self.instr += 2;
        self.state = State::WaitingToRun;
        p
    }

    fn get_mem(&self, src: i64) -> i64 {
        let src_usize = unsafe_i64_to_usize(src);
        if src_usize >= self.memory.len() {
            0
        } else {
            self.memory[src_usize]
        }
    }

    fn set_mem(&mut self, dst: i64, i: i64) {
        let dst_usize = unsafe_i64_to_usize(dst);
        if dst_usize >= self.memory.len() {
            self.memory
                .extend(repeat_n(0, dst_usize - self.memory.len() + 1));
        }
        self.memory[dst_usize] = i;
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

    fn get_src_param(&self, i: i64) -> i64 {
        let (pmode, immediate) = self.get_pmode_and_immediate(i);
        match pmode {
            ParameterMode::Position => self.get_mem(immediate),
            ParameterMode::Immediate => immediate,
            ParameterMode::Relative => self.get_mem(immediate + self.relative_base),
        }
    }

    fn get_dst_param(&self, i: i64) -> i64 {
        let (pmode, immediate) = self.get_pmode_and_immediate(i);
        match pmode {
            ParameterMode::Position => immediate,
            ParameterMode::Immediate => panic!("immediate mode for write param"),
            ParameterMode::Relative => immediate + self.relative_base,
        }
    }

    fn get_pmode_and_immediate(&self, i: i64) -> (ParameterMode, i64) {
        let iusize = unsafe_i64_to_usize(i);
        let pmode = if self.pmodes.len() >= iusize {
            self.pmodes[iusize - 1]
        } else {
            ParameterMode::Position
        };
        let immediate = self.memory[self.instr + iusize];
        (pmode, immediate)
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
    RelativeBaseOffset,
    Terminate,
    Uninitialized,
}

impl TryFrom<i64> for Opcode {
    type Error = String;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Opcode::Add),
            2 => Ok(Opcode::Multiply),
            3 => Ok(Opcode::Input),
            4 => Ok(Opcode::Output),
            5 => Ok(Opcode::JumpIfTrue),
            6 => Ok(Opcode::JumpIfFalse),
            7 => Ok(Opcode::LessThan),
            8 => Ok(Opcode::Equals),
            9 => Ok(Opcode::RelativeBaseOffset),
            99 => Ok(Opcode::Terminate),
            _ => Err(format!("Invalid opcode {}", value)),
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum ParameterMode {
    Position,
    Immediate,
    Relative,
}

impl TryFrom<i64> for ParameterMode {
    type Error = String;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ParameterMode::Position),
            1 => Ok(ParameterMode::Immediate),
            2 => Ok(ParameterMode::Relative),
            _ => Err(format!("Invalid parameter mode {}", value)),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum State {
    WaitingToRun,
    BlockedOnInput,
    BlockedOnOutput,
    Terminated,
}

fn unsafe_i64_to_usize(i: i64) -> usize {
    usize::try_from(i).unwrap()
}
