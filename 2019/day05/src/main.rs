
fn main() {
    let input_mem = include_str!("memory.txt");
    let mut memory = input_mem.split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let mut pc = 0;
    let mut input = vec![1];
    let mut output = Vec::<i32>::new();

    loop {
        let op = Operation::decode(&memory, pc);
        if let Halt = op {
            break;
        }

        op.execute(&mut memory, &mut input, &mut output);

        pc += op.length();
    }

    println!("Output: {:?}", output);
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
    Halt
}

impl Operation {
    fn execute(&self, memory: &mut [i32],
               input: &mut Vec<i32>, output: &mut Vec<i32>) {
        match self {
            Add(a, b, out) => out.write(memory, a.read(memory) + b.read(memory)),
            Multiply(a, b, out) => out.write(memory, a.read(memory) * b.read(memory)),
            Input(out) => out.write(memory, input.pop().unwrap()),
            Output(a) => output.push(a.read(memory)),
            Halt => ()
        }
    }

    fn length(&self) -> usize {
        match self {
            Add(_, _, _) => 4,
            Multiply(_, _, _) => 4,
            Input(_) => 2,
            Output(_) => 2,
            Halt => 1
        }
    }

    fn decode(memory: &[i32], pc: usize) -> Operation {
        let full_opcode = memory[pc];
        match full_opcode {
            00001 => Add(Position (memory[pc + 1] as usize), Position (memory[pc + 2] as usize), Immediate(memory[pc + 3])),
            00101 => Add(Immediate(memory[pc + 1]), Position (memory[pc + 2] as usize), Immediate(memory[pc + 3])),
            01001 => Add(Position (memory[pc + 1] as usize), Immediate(memory[pc + 2]), Immediate(memory[pc + 3])),
            01101 => Add(Immediate(memory[pc + 1]), Immediate(memory[pc + 2]), Immediate(memory[pc + 3])),
            10001 => Add(Position (memory[pc + 1] as usize), Position (memory[pc + 2] as usize), Position (memory[pc + 3] as usize)),
            10101 => Add(Immediate(memory[pc + 1]), Position (memory[pc + 2] as usize), Position (memory[pc + 3] as usize)),
            11001 => Add(Position (memory[pc + 1] as usize), Immediate(memory[pc + 2]), Position (memory[pc + 3] as usize)),
            11101 => Add(Immediate(memory[pc + 1]), Immediate(memory[pc + 2]), Position (memory[pc + 3] as usize)),

            00002 => Multiply(Position (memory[pc + 1] as usize), Position (memory[pc + 2] as usize), Immediate(memory[pc + 3])),
            00102 => Multiply(Immediate(memory[pc + 1]), Position (memory[pc + 2] as usize), Immediate(memory[pc + 3])),
            01002 => Multiply(Position (memory[pc + 1] as usize), Immediate(memory[pc + 2]), Immediate(memory[pc + 3])),
            01102 => Multiply(Immediate(memory[pc + 1]), Immediate(memory[pc + 2]), Immediate(memory[pc + 3])),
            10002 => Multiply(Position (memory[pc + 1] as usize), Position (memory[pc + 2] as usize), Position (memory[pc + 3] as usize)),
            10102 => Multiply(Immediate(memory[pc + 1]), Position (memory[pc + 2] as usize), Position (memory[pc + 3] as usize)),
            11002 => Multiply(Position (memory[pc + 1] as usize), Immediate(memory[pc + 2]), Position (memory[pc + 3] as usize)),
            11102 => Multiply(Immediate(memory[pc + 1]), Immediate(memory[pc + 2]), Position (memory[pc + 3] as usize)),

            003 => Input(Immediate(memory[pc + 1])),
            103 => Input(Position (memory[pc + 1] as usize)),

            004 => Output(Position (memory[pc + 1] as usize)),
            104 => Output(Immediate(memory[pc + 1])),

            99 => Halt,
            _ => panic!("Unrecognised opcode {}", full_opcode)
        }
    }
}