
extern crate int_code;
use int_code::*;

fn main() {
    let input_mem = include_str!("memory.txt");
    let memory = input_mem.split(',')
        .map(|x| x.parse::<Word>().unwrap())
        .collect::<Vec<_>>();

    {
        let mut machine_1 = Machine::new(memory.to_vec(), vec![1]);
        let output = machine_1.execute();
        println!("System 1: {:?}", output);
    }
    {
        let mut machine_5 = Machine::new(memory.to_vec(), vec![5]);
        let output = machine_5.execute();
        println!("System 5: {:?}", output);
    }
}
