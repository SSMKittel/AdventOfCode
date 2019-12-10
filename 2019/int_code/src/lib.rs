
pub use std::sync::mpsc::{Sender, Receiver, channel};
use crate::Parameter::*;
use crate::Operation::*;
use std::borrow::{BorrowMut};
use std::num::{ParseIntError};
use std::fmt;
use std::error::Error as StdError;
use std::sync::mpsc::TryRecvError;
use std::convert::TryInto;
use std::collections::HashMap;

pub type Word = i64;
pub struct Machine {
    memory: Memory,
    pc: Word,
    input: Receiver<Word>,
    output: Sender<Word>,
    waiting_input: bool
}

struct Memory {
    memory: HashMap<usize, Box<[Word; 1024]>>
}
struct AccessViolation(Word);

impl Memory {
    fn new(init: &[Word]) -> Memory {
        let mut m = Memory{ memory: HashMap::with_capacity((init.len() + 1023) / 1024) };
        let mut i = 0;
        for chunk in init.chunks(1024) {
            let mut tmp = Box::new([0; 1024]);
            tmp[..chunk.len()].copy_from_slice(chunk);
            if m.memory.insert(i, tmp).is_some() {
                panic!("Overwrote memory on initialisation");
            }
            i += 1;
        }
        m
    }

    fn read(&self, address: Parameter) -> Result<Word, AccessViolation> {
        match address {
            Immediate(val) => Ok(val),
            Position(addr) => {
                let (chunk_id, sub_chunk_index) = Memory::address(addr)?;
                match self.memory.get(&chunk_id) {
                    None => Ok(0),
                    Some(chunk) => Ok(chunk[sub_chunk_index]),
                }
            }
        }
    }
    fn read_position(&self, addr: Word) -> Result<Word, AccessViolation> {
        let (chunk_id, sub_chunk_index) = Memory::address(addr)?;
        match self.memory.get(&chunk_id) {
            None => Ok(0),
            Some(chunk) => Ok(chunk[sub_chunk_index]),
        }
    }
    fn write(&mut self, OutputParameter(addr): OutputParameter, new_val: Word) -> Result<(), AccessViolation> {
        let (chunk_id, sub_chunk_index) = Memory::address(addr)?;

        self.get_chunk_mut(chunk_id)[sub_chunk_index] = new_val;
        Ok(())
    }

    fn get_chunk_mut(&mut self, chunk_id: usize) -> &mut Box<[Word; 1024]> {
        self.memory
            .entry(chunk_id)
            .or_insert_with(|| Box::new([0; 1024]))
    }

    fn address(addr: Word) -> Result<(usize, usize), AccessViolation> {
        if addr < 0 {
            return Err(AccessViolation(addr));
        }
        let rslt: Result<usize, _> = addr.try_into();
        match rslt {
            Ok(addr_u) => Ok((addr_u / 1024, addr_u % 1024)),
            Err(_) => Err(AccessViolation(addr))
        }
    }
}

#[derive(Eq, PartialEq, Debug)]
pub enum ExecuteError {
    InputRequired,
    InputError,
    OutputError,
    NoProgress,
    ArithmeticOverflow,
    ExecutionLimitReached,
    UnrecognisedOpcode(Word),
    MemoryAccessViolation(Word)
}

pub fn parse_csv(csv: &str) -> Result<Vec<Word>, ParseIntError> {
    csv.split(',')
        .map(|x| x.parse::<Word>())
        .collect()
}

impl fmt::Display for ExecuteError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ExecuteError::OutputError => f.write_str("OutputError"),
            ExecuteError::InputError => f.write_str("InputError"),
            ExecuteError::InputRequired => f.write_str("InputRequired"),
            ExecuteError::NoProgress => f.write_str("NoProgress"),
            ExecuteError::ArithmeticOverflow => f.write_str("ArithmeticOverflow"),
            ExecuteError::ExecutionLimitReached => f.write_str("ExecutionLimitReached"),
            ExecuteError::MemoryAccessViolation(address) => write!(f, "MemoryAccessViolation({})", address),
            ExecuteError::UnrecognisedOpcode(op) => write!(f, "UnrecognisedOpcode({})", op),
        }
    }
}
impl StdError for ExecuteError {
    fn description(&self) -> &str {
        match *self {
            ExecuteError::OutputError => "Output Error",
            ExecuteError::InputError => "Input Error",
            ExecuteError::InputRequired => "Input Required",
            ExecuteError::NoProgress => "No Progress",
            ExecuteError::ArithmeticOverflow => "Arithmetic Overflow",
            ExecuteError::ExecutionLimitReached => "Execution Limit Reached",
            ExecuteError::MemoryAccessViolation(_) => "Memory Access Violation",
            ExecuteError::UnrecognisedOpcode(_) => "Unrecognised Opcode",
        }
    }
}

impl Machine {
    pub fn with_channels(memory: &[Word], input: Receiver<Word>, output: Sender<Word>) -> Machine {
        Machine{ memory: Memory::new(memory), pc: 0, input, output, waiting_input: false }
    }

    pub fn new(memory: &[Word]) -> (Machine, Sender<Word>, Receiver<Word>) {
        let (input_write, input): (Sender<Word>, Receiver<Word>) = channel();
        let (output, output_read): (Sender<Word>, Receiver<Word>) = channel();

        (Machine{ memory: Memory::new(memory), pc: 0, input, output, waiting_input: false },
        input_write, output_read)
    }

    pub fn execute(&mut self, limit: u32) -> Result<(), ExecuteError> {
        let mut lim = limit;
        loop {
            if lim == 0 {
                return Err(ExecuteError::ExecutionLimitReached);
            }

            let op = Operation::decode(&self.memory, self.pc)?;
            match self.step(op)? {
                StepResult::Executed => (),
                StepResult::Halt => return Ok(())
            }
            lim -= 1;
        }
    }

    fn step(&mut self, op: Operation) -> Result<StepResult, StepError> {
        let memory = self.memory.borrow_mut();
        match op {
            Add(a, b, out) => {
                let a_val = memory.read(a)?;
                let b_val = memory.read(b)?;
                memory.write(out, a_val.checked_add(b_val).ok_or(StepError::ArithmeticOverflow)?)?;
                self.pc += 4;
            },
            Multiply(a, b, out) => {
                let a_val = memory.read(a)?;
                let b_val = memory.read(b)?;
                memory.write(out, a_val.checked_mul(b_val).ok_or(StepError::ArithmeticOverflow)?)?;
                self.pc += 4;
            },
            Input(out) => {
                let readval = self.input.try_recv();
                match readval {
                    Ok(rslt) => memory.write(out, rslt)?,
                    Err(a) => {
                        if self.waiting_input {
                            return Err(StepError::NoProgress)
                        }
                        else if let TryRecvError::Empty = a {
                            self.waiting_input = true;
                            return Err(StepError::InputRequired)
                        }
                        else {
                            return Err(StepError::InputError)
                        }
                    }
                }
                self.pc += 2;
            },
            Output(a) => {
                match self.output.send(memory.read(a)?) {
                    Err(_) => {
                        self.waiting_input = false;
                        return Err(StepError::OutputError)
                    },
                    _ => ()
                }
                self.pc += 2;
            },
            JumpIfTrue(a, new_pc) => {
                if memory.read(a)? != 0 {
                    self.pc = memory.read(new_pc)?;
                }
                else {
                    self.pc += 3;
                }
            },
            JumpIfFalse(a, new_pc) => {
                if memory.read(a)? == 0 {
                    self.pc = memory.read(new_pc)?;
                }
                else {
                    self.pc += 3;
                }
            },
            LessThan(a, b, out) => {
                if memory.read(a)? < memory.read(b)? {
                    memory.write(out, 1)?;
                }
                else {
                    memory.write(out, 0)?;
                }
                self.pc += 4;
            },
            Equals(a, b, out) => {
                if memory.read(a)? == memory.read(b)? {
                    memory.write(out, 1)?;
                }
                else {
                    memory.write(out, 0)?;
                }
                self.pc += 4;
            },
            Halt => {
                self.waiting_input = false;
                return Ok(StepResult::Halt)
            }
        }
        self.waiting_input = false;
        Ok(StepResult::Executed)
    }
}

impl From<AccessViolation> for StepError {
    fn from(AccessViolation(a): AccessViolation) -> Self {
        StepError::MemoryAccessViolation(a)
    }
}

impl From<AccessViolation> for DecodeError {
    fn from(AccessViolation(a): AccessViolation) -> Self {
        DecodeError::AccessViolation(a)
    }
}

enum StepResult {
    Executed,
    Halt
}
enum StepError {
    InputRequired,
    InputError,
    OutputError,
    NoProgress,
    ArithmeticOverflow,
    MemoryAccessViolation(Word)
}

struct OutputParameter(Word);
enum Parameter {
    Immediate(Word),
    Position(Word)
}

enum Operation {
    Add(Parameter, Parameter, OutputParameter),
    Multiply(Parameter, Parameter, OutputParameter),
    Input(OutputParameter),
    Output(Parameter),
    JumpIfTrue(Parameter, Parameter),
    JumpIfFalse(Parameter, Parameter),
    LessThan(Parameter, Parameter, OutputParameter),
    Equals(Parameter, Parameter, OutputParameter),
    Halt
}

enum DecodeError {
    InvalidOpcode(Word),
    AccessViolation(Word)
}

impl From<DecodeError> for ExecuteError {
    fn from(a: DecodeError) -> Self {
        match a {
            DecodeError::InvalidOpcode(op) => ExecuteError::UnrecognisedOpcode(op),
            DecodeError::AccessViolation(av) => ExecuteError::MemoryAccessViolation(av),
        }
    }
}
impl From<StepError> for ExecuteError {
    fn from(a: StepError) -> Self {
        match a {
            StepError::MemoryAccessViolation(addr) => ExecuteError::MemoryAccessViolation(addr),
            StepError::ArithmeticOverflow => ExecuteError::ArithmeticOverflow,
            StepError::InputError => ExecuteError::InputError,
            StepError::InputRequired => ExecuteError::InputRequired,
            StepError::OutputError => ExecuteError::OutputError,
            StepError::NoProgress => ExecuteError::NoProgress
        }
    }
}

impl Operation {
    fn decode(memory: &Memory, pc: Word) -> Result<Operation, DecodeError> {
        let full_opcode = memory.read_position(pc)?;
        let opcode = full_opcode % 100;
        let params = full_opcode / 100;
        match opcode {
            1 | 2 | 7 | 8 => {
                let pos1 = memory.read_position(pc + 1)?;
                let pos2 = memory.read_position(pc + 2)?;
                let pos3 = memory.read_position(pc + 3)?;
                let p1 = match params % 10 {
                    0 => Position (pos1),
                    1 => Immediate(pos1),
                    _ => return Err(DecodeError::InvalidOpcode(full_opcode))
                };
                let p2 = match (params / 10) % 10 {
                    0 => Position (pos2),
                    1 => Immediate(pos2),
                    _ => return Err(DecodeError::InvalidOpcode(full_opcode))
                };
                let pout = match params / 100 {
                    0 => OutputParameter(pos3),
                    _ => return Err(DecodeError::InvalidOpcode(full_opcode))
                };
                match opcode {
                    1 => Ok(Add(p1, p2, pout)),
                    2 => Ok(Multiply(p1, p2, pout)),
                    7 => Ok(LessThan(p1, p2, pout)),
                    8 => Ok(Equals(p1, p2, pout)),
                    _ => return Err(DecodeError::InvalidOpcode(full_opcode))
                }
            },
            3 => {
                let pos1 = memory.read_position(pc + 1)?;
                match params {
                    0 => Ok(Input(OutputParameter(pos1))),
                    _ => return Err(DecodeError::InvalidOpcode(full_opcode))
                }
            },
            4 => {
                let pos1 = memory.read_position(pc + 1)?;
                match params {
                    0 => Ok(Output(Position (pos1))),
                    1 => Ok(Output(Immediate(pos1))),
                    _ => return Err(DecodeError::InvalidOpcode(full_opcode))
                }
            },
            5 | 6 => {
                let pos1 = memory.read_position(pc + 1)?;
                let pos2 = memory.read_position(pc + 2)?;
                let p1 = match params % 10 {
                    0 => Position (pos1),
                    1 => Immediate(pos1),
                    _ => return Err(DecodeError::InvalidOpcode(full_opcode))
                };
                let p2 = match (params / 10) % 10 {
                    0 => Position (pos2),
                    1 => Immediate(pos2),
                    _ => return Err(DecodeError::InvalidOpcode(full_opcode))
                };
                match opcode {
                    5 => Ok(JumpIfTrue(p1, p2)),
                    6 => Ok(JumpIfFalse(p1, p2)),
                    _ => return Err(DecodeError::InvalidOpcode(full_opcode))
                }
            },
            99 => {
                if full_opcode == opcode {
                    Ok(Halt)
                }
                else {
                    Err(DecodeError::InvalidOpcode(full_opcode))
                }
            }
            _ => Err(DecodeError::InvalidOpcode(full_opcode))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ExecuteError::*;

    #[test]
    fn test_program() {
        let input_mem = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
        let memory = parse_csv(input_mem).unwrap();

        {
            let (mut machine_7, input_write, out_read) = Machine::new(&memory);
            input_write.send(7).unwrap();
            assert_eq!(Ok(()), machine_7.execute(1000));
            let vals = out_read.try_iter().collect::<Vec<_>>();
            assert_eq!(vec![999], vals);
        }
        {
            let (mut machine_8, input_write, out_read) = Machine::new(&memory);
            input_write.send(8).unwrap();
            assert_eq!(Ok(()), machine_8.execute(1000));
            let vals = out_read.try_iter().collect::<Vec<_>>();
            assert_eq!(vec![1000], vals);
        }
        {
            let (mut machine_9, input_write, out_read) = Machine::new(&memory);
            input_write.send(9).unwrap();
            assert_eq!(Ok(()), machine_9.execute(1000));
            let vals = out_read.try_iter().collect::<Vec<_>>();
            assert_eq!(vec![1001], vals);
        }
    }

    #[test]
    fn test_infinite_loop() {
        let input_mem = "1106,0,0";
        let memory = parse_csv(input_mem).unwrap();

        let (mut machine, _, _) = Machine::new(&memory);
        assert_eq!(Err(ExecutionLimitReached), machine.execute(10));
    }

    #[test]
    fn test_input() {
        let input_mem = "3,2,0";
        let memory = parse_csv(input_mem).unwrap();

        let (mut machine, _inp, _) = Machine::new(&memory);
        assert_eq!(Err(InputRequired), machine.execute(10));
        // If input is still required on a second call, but it is still not available,
        // we expect a NoProgress error as the machine was unable to do any work
        assert_eq!(Err(NoProgress), machine.execute(10));

        let (mut machine, _, _) = Machine::new(&memory);
        // use of _ parameter for input-source causes it to be released, and the corresponding receiver to be closed
        // Complete failure when trying to get input
        assert_eq!(Err(InputError), machine.execute(10));
    }

    #[test]
    fn test_output() {
        let input_mem = "104,1,99";
        let memory = parse_csv(input_mem).unwrap();

        let (mut machine, _, out) = Machine::new(&memory);
        assert_eq!(Ok(()), machine.execute(10));
        assert_eq!(vec![1], out.try_iter().collect::<Vec<Word>>());

        let (mut machine, _, _) = Machine::new(&memory);
        // use of _ parameter for output-target causes it to be released, and the corresponding sender to be closed
        // Complete failure when trying to write output
        assert_eq!(Err(OutputError), machine.execute(10));
    }

    #[test]
    fn test_read_past_end() {
        let input_mem = "1,0,0,0";
        let memory = parse_csv(input_mem).unwrap();

        let (mut machine, _, _) = Machine::new(&memory);
        assert_eq!(Err(ExecuteError::UnrecognisedOpcode(0)), machine.execute(10));
    }

    #[test]
    fn test_pc_access_violation() {
        let input_mem = "1,0,0,-1";
        let memory = parse_csv(input_mem).unwrap();

        let (mut machine, _, _) = Machine::new(&memory);
        assert_eq!(Err(ExecuteError::MemoryAccessViolation(-1)), machine.execute(10));
    }
}
