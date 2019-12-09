
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;

pub type Word = i64;
pub struct Machine {
    memory: Vec<Word>,
    pc: usize,
    input: Receiver<Word>,
    output: Sender<Word>
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

    pub fn execute(&mut self) {
        loop {
            let op = Operation::decode(self.memory.borrow(), self.pc);

            if let Some(new_pc) = op.execute(self) {
                self.pc = new_pc;
            }
            else {
                break;
            }
        }
    }
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
    fn execute(&self, machine: &mut Machine) -> Option<usize> {
        let memory = machine.memory.borrow_mut();
        let pc = machine.pc;
        match self {
            Add(a, b, out) => {
                out.write(memory, a.read(memory) + b.read(memory));
                Some(pc + self.length())
            },
            Multiply(a, b, out) => {
                out.write(memory, a.read(memory) * b.read(memory));
                Some(pc + self.length())
            },
            Input(out) => {
                out.write(memory, machine.input.recv().unwrap());
                Some(pc + self.length())
            },
            Output(a) => {
                machine.output.send(a.read(memory));
                Some(pc + self.length())
            },
            JumpIfTrue(a, new_pc) => {
                if a.read(memory) != 0 {
                    Some(new_pc.read(memory) as usize)
                }
                else {
                    Some(pc + self.length())
                }
            },
            JumpIfFalse(a, new_pc) => {
                if a.read(memory) == 0 {
                    Some(new_pc.read(memory) as usize)
                }
                else {
                    Some(pc + self.length())
                }
            },
            LessThan(a, b, out) => {
                if a.read(memory) < b.read(memory) {
                    out.write(memory, 1);
                }
                else {
                    out.write(memory, 0);
                }
                Some(pc + self.length())
            },
            Equals(a, b, out) => {
                if a.read(memory) == b.read(memory) {
                    out.write(memory, 1);
                }
                else {
                    out.write(memory, 0);
                }
                Some(pc + self.length())
            },
            Halt => None
        }
    }

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
            machine_7.execute();
            let vals = out_read.try_iter().collect::<Vec<_>>();
            assert_eq!(vec![999], vals);
        }
        {
            let (mut machine_8, input_write, out_read) = Machine::new(memory.to_vec());
            input_write.send(8).unwrap();
            machine_8.execute();
            let vals = out_read.try_iter().collect::<Vec<_>>();
            assert_eq!(vec![1000], vals);
        }
        {
            let (mut machine_9, input_write, out_read) = Machine::new(memory.to_vec());
            input_write.send(9).unwrap();
            machine_9.execute();
            let vals = out_read.try_iter().collect::<Vec<_>>();
            assert_eq!(vec![1001], vals);
        }
    }
}
