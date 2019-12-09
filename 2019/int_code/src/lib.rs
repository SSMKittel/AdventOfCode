
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;

pub type Word = i64;
pub struct Machine {
    memory: Vec<Word>,
    pc: usize,
    input: Receiver<Word>,
    output: Sender<Word>
}

#[derive(Eq, PartialEq, Debug)]
pub enum ExecuteError {
    InputRequired,
    OutputError,
}

use std::fmt;
use std::error::Error as StdError;

impl fmt::Display for ExecuteError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ExecuteError::OutputError => f.write_str("OutputError"),
            ExecuteError::InputRequired => f.write_str("InputRequired"),
        }
    }
}
impl StdError for ExecuteError {
    fn description(&self) -> &str {
        match *self {
            ExecuteError::OutputError => "Output Error",
            ExecuteError::InputRequired => "Input Required",
        }
    }
}

impl Machine {
    pub fn with_channels(memory: Vec<Word>, input: Receiver<Word>, output: Sender<Word>) -> Machine {
        Machine{ memory, pc: 0, input, output }
    }

    pub fn new(memory: Vec<Word>) -> (Machine, Sender<Word>, Receiver<Word>) {
        let (input_write, input): (Sender<Word>, Receiver<Word>) = mpsc::channel();
        let (output, output_read): (Sender<Word>, Receiver<Word>) = mpsc::channel();

        (Machine{ memory, pc: 0, input, output },
        input_write, output_read)
    }

    pub fn execute(&mut self) -> Result<(), ExecuteError> {
        loop {
            let op = Operation::decode(self.memory.borrow(), self.pc);

            match self.step(op) {
                Ok(StepResult::Executed) => (),
                Ok(StepResult::Halt) => return Ok(()),
                Err(StepError::InputRequired) => return Err(ExecuteError::InputRequired),
                Err(StepError::OutputError) => return Err(ExecuteError::OutputError)
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
                    Err(_) => return Err(StepError::InputRequired)
                }
                self.pc += oplen;
            },
            Output(a) => {
                match self.output.send(a.read(memory)) {
                    Err(_) => return Err(StepError::OutputError),
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
            Halt => return Ok(StepResult::Halt)
        }
        Ok(StepResult::Executed)
    }
}

enum StepResult {
    Executed,
    Halt
}
enum StepError {
    InputRequired,
    OutputError
}


enum Parameter {
    Immediate(Word),
    Position(usize)
}

use crate::Parameter::*;
use crate::Operation::*;
use std::borrow::{BorrowMut, Borrow};

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

    fn decode(memory: &[Word], pc: usize) -> Operation {
        let full_opcode = memory[pc];
        let opcode = full_opcode % 100;
        let params = full_opcode / 100;
        match opcode {
            1 | 2 | 7 | 8 => {
                let p1 = match params % 10 {
                    0 => Position (memory[pc + 1] as usize),
                    1 => Immediate(memory[pc + 1]),
                    _ => panic!("Unrecognised opcode {}", full_opcode)
                };
                let p2 = match (params / 10) % 10 {
                    0 => Position (memory[pc + 2] as usize),
                    1 => Immediate(memory[pc + 2]),
                    _ => panic!("Unrecognised opcode {}", full_opcode)
                };
                let pout = match params / 100 {
                    0 => Immediate (memory[pc + 3]),
                    1 => Position(memory[pc + 3] as usize),
                    _ => panic!("Unrecognised opcode {}", full_opcode)
                };
                match opcode {
                    1 => Add(p1, p2, pout),
                    2 => Multiply(p1, p2, pout),
                    7 => LessThan(p1, p2, pout),
                    8 => Equals(p1, p2, pout),
                    _ => panic!("Unrecognised opcode {}", full_opcode)
                }
            },
            3 => {
                match params {
                    0 => Input(Immediate(memory[pc + 1])),
                    1 => Input(Position (memory[pc + 1] as usize)),
                    _ => panic!("Unrecognised opcode {}", full_opcode)
                }
            },
            4 => {
                match params {
                    0 => Output(Position (memory[pc + 1] as usize)),
                    1 => Output(Immediate(memory[pc + 1])),
                    _ => panic!("Unrecognised opcode {}", full_opcode)
                }
            },
            5 | 6 => {
                let p1 = match params % 10 {
                    0 => Position (memory[pc + 1] as usize),
                    1 => Immediate(memory[pc + 1]),
                    _ => panic!("Unrecognised opcode {}", full_opcode)
                };
                let p2 = match (params / 10) % 10 {
                    0 => Position (memory[pc + 2] as usize),
                    1 => Immediate(memory[pc + 2]),
                    _ => panic!("Unrecognised opcode {}", full_opcode)
                };
                match opcode {
                    5 => JumpIfTrue(p1, p2),
                    6 => JumpIfFalse(p1, p2),
                    _ => panic!("Unrecognised opcode {}", full_opcode)
                }
            },
            99 => {
                if full_opcode != opcode {
                    panic!("Unrecognised opcode {}", full_opcode)
                }
                Halt
            }
            _ => panic!("Unrecognised opcode {}", full_opcode)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_program() {
        let input_mem = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
        let memory = input_mem.split(',')
            .map(|x| x.parse::<Word>().unwrap())
            .collect::<Vec<_>>();

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
