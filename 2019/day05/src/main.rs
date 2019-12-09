
extern crate int_code;
use int_code::*;

fn main() {
    let input_mem = include_str!("memory.txt");
    let memory = parse_csv(input_mem).unwrap();

    {
        let (mut machine_1, input_write, output_read) = Machine::new(memory.to_vec());
        input_write.send(1).unwrap();
        machine_1.execute(1000).unwrap();
        let output = output_read.try_iter().collect::<Vec<_>>();
        println!("System 1: {:?}", output);
    }
    {
        let (mut machine_5, input_write, output_read) = Machine::new(memory.to_vec());
        input_write.send(5).unwrap();
        machine_5.execute(1000).unwrap();
        let output = output_read.try_iter().collect::<Vec<_>>();
        println!("System 5: {:?}", output);
    }
}
