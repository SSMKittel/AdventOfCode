
pub use std::sync::mpsc::{Sender, Receiver, channel};
use crate::Parameter::*;
use crate::Operation::*;
use std::borrow::{BorrowMut, Borrow};
use std::num::ParseIntError;


pub type Word = i64;
pub struct Machine {
    memory: Vec<Word>,
    pc: usize,
    input: Receiver<Word>,
    output: Sender<Word>,
    waiting_input: bool
}

#[derive(Eq, PartialEq, Debug)]
pub enum ExecuteError {
    InputRequired,
    OutputError,
    NoProgress,
    UnrecognisedOpcode(Word)
}

pub fn parse_csv(csv: &str) -> Result<Vec<Word>, ParseIntError> {
    csv.split(',')
        .map(|x| x.parse::<Word>())
        .collect()
}

use std::fmt;
use std::error::Error as StdError;

impl fmt::Display for ExecuteError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ExecuteError::OutputError => f.write_str("OutputError"),
            ExecuteError::InputRequired => f.write_str("InputRequired"),
            ExecuteError::NoProgress => f.write_str("NoProgress"),
            ExecuteError::UnrecognisedOpcode(op) => write!(f, "UnrecognisedOpcode({})", op),
        }
    }
}
impl StdError for ExecuteError {
    fn description(&self) -> &str {
        match *self {
            ExecuteError::OutputError => "Output Error",
            ExecuteError::InputRequired => "Input Required",
            ExecuteError::NoProgress => "No Progress",
            ExecuteError::UnrecognisedOpcode(_) => "Unrecognised Opcode",
        }
    }
}

impl Machine {
    pub fn with_channels(memory: Vec<Word>, input: Receiver<Word>, output: Sender<Word>) -> Machine {
        Machine{ memory, pc: 0, input, output, waiting_input: false }
    }

    pub fn new(memory: Vec<Word>) -> (Machine, Sender<Word>, Receiver<Word>) {
        let (input_write, input): (Sender<Word>, Receiver<Word>) = channel();
        let (output, output_read): (Sender<Word>, Receiver<Word>) = channel();

        (Machine{ memory, pc: 0, input, output, waiting_input: false },
        input_write, output_read)
    }

    pub fn execute(&mut self) -> Result<(), ExecuteError> {
        loop {
            match Operation::decode(self.memory.borrow(), self.pc) {
                Err(op) => return Err(ExecuteError::UnrecognisedOpcode(op)),
                Ok(op) => {
                    match self.step(op) {
                        Ok(StepResult::Executed) => (),
                        Ok(StepResult::Halt) => return Ok(()),
                        Err(StepError::InputRequired) => return Err(ExecuteError::InputRequired),
                        Err(StepError::OutputError) => return Err(ExecuteError::OutputError),
                        Err(StepError::NoProgress) => return Err(ExecuteError::NoProgress)
                    }
                },
            }

        }
    }

    fn step(&mut self, op: Operation) -> Result<StepResult, StepError> {
        let memory = self.memory.borrow_mut();
        let oplen = op.length();
        match op {
            Add(a, b, out) => {
                out.write(memory, a.read(memory) + b.read(memory));
                self.pc += oplen;
            },
            Multiply(a, b, out) => {
                out.write(memory, a.read(memory) * b.read(memory));
                self.pc += oplen;
            },
            Input(out) => {
                let readval = self.input.recv();
                match readval {
                    Ok(rslt) => out.write(memory, rslt),
                    Err(_) => {
                        if self.waiting_input {
                            return Err(StepError::NoProgress)
                        }
                        else {
                            self.waiting_input = true;
                            return Err(StepError::InputRequired)
                        }
                    }
                }
                self.pc += oplen;
            },
            Output(a) => {
                match self.output.send(a.read(memory)) {
                    Err(_) => {
                        self.waiting_input = false;
                        return Err(StepError::OutputError)
                    },
                    _ => ()
                }
                self.pc += oplen;
            },
            JumpIfTrue(a, new_pc) => {
                if a.read(memory) != 0 {
                    self.pc = new_pc.read(memory) as usize;
                }
                else {
                    self.pc += oplen;
                }
            },
            JumpIfFalse(a, new_pc) => {
                if a.read(memory) == 0 {
                    self.pc = new_pc.read(memory) as usize;
                }
                else {
                    self.pc += oplen;
                }
            },
            LessThan(a, b, out) => {
                if a.read(memory) < b.read(memory) {
                    out.write(memory, 1);
                }
                else {
                    out.write(memory, 0);
                }
                self.pc += oplen;
            },
            Equals(a, b, out) => {
                if a.read(memory) == b.read(memory) {
                    out.write(memory, 1);
                }
                else {
                    out.write(memory, 0);
                }
                self.pc += oplen;
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

enum StepResult {
    Executed,
    Halt
}
enum StepError {
    InputRequired,
    OutputError,
    NoProgress
}


enum Parameter {
    Immediate(Word),
    Position(usize)
}

impl Parameter {
    fn read(&self, memory: &[Word]) -> Word {
        match self {
            Immediate(val) => *val,
            Position(addr) => memory[*addr]
        }
    }
    fn write(&self, memory: &mut [Word], new_val: Word) {
        match self {
            Immediate(direct_addr) => memory[*direct_addr as usize] = new_val,
            Position(indirect_addr) => memory[memory[*indirect_addr] as usize] = new_val
        }
    }
}

enum Operation {
    Add(Parameter, Parameter, Parameter),
    Multiply(Parameter, Parameter, Parameter),
    Input(Parameter),
    Output(Parameter),
    JumpIfTrue(Parameter, Parameter),
    JumpIfFalse(Parameter, Parameter),
    LessThan(Parameter, Parameter, Parameter),
    Equals(Parameter, Parameter, Parameter),
    Halt
}

impl Operation {
    fn length(&self) -> usize {
        match self {
            Add(_, _, _) => 4,
            Multiply(_, _, _) => 4,
            Input(_) => 2,
            Output(_) => 2,
            JumpIfTrue(_, _) => 3,
            JumpIfFalse(_, _) => 3,
            LessThan(_, _, _) => 4,
            Equals(_, _, _) => 4,
            Halt => 1
        }
    }

    fn decode(memory: &[Word], pc: usize) -> Result<Operation, Word> {
        let full_opcode = memory[pc];
        let opcode = full_opcode % 100;
        let params = full_opcode / 100;
        match opcode {
            1 | 2 | 7 | 8 => {
                let p1 = match params % 10 {
                    0 => Position (memory[pc + 1] as usize),
                    1 => Immediate(memory[pc + 1]),
                    _ => return Err(full_opcode)
                };
                let p2 = match (params / 10) % 10 {
                    0 => Position (memory[pc + 2] as usize),
                    1 => Immediate(memory[pc + 2]),
                    _ => return Err(full_opcode)
                };
                let pout = match params / 100 {
                    0 => Immediate (memory[pc + 3]),
                    1 => Position(memory[pc + 3] as usize),
                    _ => return Err(full_opcode)
                };
                match opcode {
                    1 => Ok(Add(p1, p2, pout)),
                    2 => Ok(Multiply(p1, p2, pout)),
                    7 => Ok(LessThan(p1, p2, pout)),
                    8 => Ok(Equals(p1, p2, pout)),
                    _ => return Err(full_opcode)
                }
            },
            3 => {
                match params {
                    0 => Ok(Input(Immediate(memory[pc + 1]))),
                    1 => Ok(Input(Position (memory[pc + 1] as usize))),
                    _ => return Err(full_opcode)
                }
            },
            4 => {
                match params {
                    0 => Ok(Output(Position (memory[pc + 1] as usize))),
                    1 => Ok(Output(Immediate(memory[pc + 1]))),
                    _ => return Err(full_opcode)
                }
            },
            5 | 6 => {
                let p1 = match params % 10 {
                    0 => Position (memory[pc + 1] as usize),
                    1 => Immediate(memory[pc + 1]),
                    _ => return Err(full_opcode)
                };
                let p2 = match (params / 10) % 10 {
                    0 => Position (memory[pc + 2] as usize),
                    1 => Immediate(memory[pc + 2]),
                    _ => return Err(full_opcode)
                };
                match opcode {
                    5 => Ok(JumpIfTrue(p1, p2)),
                    6 => Ok(JumpIfFalse(p1, p2)),
                    _ => return Err(full_opcode)
                }
            },
            99 => {
                if full_opcode == opcode {
                    Ok(Halt)
                }
                else {
                    Err(full_opcode)
                }
            }
            _ => Err(full_opcode)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_program() {
        let input_mem = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
        let memory = parse_csv(input_mem).unwrap();

        {
            let (mut machine_7, input_write, out_read) = Machine::new(memory.to_vec());
            input_write.send(7).unwrap();
            assert_eq!(Ok(()), machine_7.execute());
            let vals = out_read.try_iter().collect::<Vec<_>>();
            assert_eq!(vec![999], vals);
        }
        {
            let (mut machine_8, input_write, out_read) = Machine::new(memory.to_vec());
            input_write.send(8).unwrap();
            assert_eq!(Ok(()), machine_8.execute());
            let vals = out_read.try_iter().collect::<Vec<_>>();
            assert_eq!(vec![1000], vals);
        }
        {
            let (mut machine_9, input_write, out_read) = Machine::new(memory.to_vec());
            input_write.send(9).unwrap();
            assert_eq!(Ok(()), machine_9.execute());
            let vals = out_read.try_iter().collect::<Vec<_>>();
            assert_eq!(vec![1001], vals);
        }
    }
}
