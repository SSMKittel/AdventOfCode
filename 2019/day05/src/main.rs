
extern crate int_code;
use int_code::*;

fn main() {
    let input_mem = include_str!("memory.txt");
    let memory = input_mem.split(',')
        .map(|x| x.parse::<Word>().unwrap())
        .collect::<Vec<_>>();

    {
        let (mut machine_1, input_write, output_read) = Machine::new(memory.to_vec());
        input_write.send(1).unwrap();
        machine_1.execute();
        let output = output_read.try_iter().collect::<Vec<_>>();
        println!("System 1: {:?}", output);
    }
    {
        let (mut machine_5, input_write, output_read) = Machine::new(memory.to_vec());
        input_write.send(5).unwrap();
        machine_5.execute();
        let output = output_read.try_iter().collect::<Vec<_>>();
        println!("System 5: {:?}", output);
    }
}
