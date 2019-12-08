
fn main() {
    let input_mem = include_str!("memory.txt");
    let memory = input_mem.split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    {
        let mut memory_1 = memory.to_vec();
        let output = execute(&mut memory_1, vec![1]);
        println!("System 1: {:?}", output);
    }
    {
        let mut memory_5 = memory.to_vec();
        let output = execute(&mut memory_5, vec![5]);
        println!("System 5: {:?}", output);
    }
}

fn execute(memory: &mut [i32], mut input: Vec<i32>) -> Vec<i32> {
    let mut output = Vec::<i32>::new();
    let mut pc = 0;
    loop {
        let op = Operation::decode(&memory, pc);

        if let Some(new_pc) = op.execute(memory, &pc, &mut input, &mut output) {
            pc = new_pc;
        }
        else {
            break;
        }
    }
    output
}

enum Parameter {
    Immediate(i32),
    Position(usize)
}

use crate::Parameter::*;
use crate::Operation::*;

impl Parameter {
    fn read(&self, memory: &[i32]) -> i32 {
        match self {
            Immediate(val) => *val,
            Position(addr) => memory[*addr]
        }
    }
    fn write(&self, memory: &mut [i32], new_val: i32) {
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
    fn execute(&self, memory: &mut [i32], pc: &usize,
               input: &mut Vec<i32>, output: &mut Vec<i32>) -> Option<usize> {
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
                out.write(memory, input.pop().unwrap());
                Some(pc + self.length())
            },
            Output(a) => {
                output.push(a.read(memory));
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

    fn decode(memory: &[i32], pc: usize) -> Operation {
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
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        {
            let mut mem_7 = memory.to_vec();
            let output = execute(&mut mem_7, vec![7]);
            assert_eq!(vec![999], output);
        }
        {
            let mut mem_8 = memory.to_vec();
            let output = execute(&mut mem_8, vec![8]);
            assert_eq!(vec![1000], output);
        }
        {
            let mut mem_9 = memory.to_vec();
            let output = execute(&mut mem_9, vec![9]);
            assert_eq!(vec![1001], output);
        }
    }
}