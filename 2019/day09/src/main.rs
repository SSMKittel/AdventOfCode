extern crate int_code;
use int_code::*;

fn main() {
    let mem = parse_csv(include_str!("input.txt")).unwrap();
    {
        let (mut machine, input, output) = Machine::new(&mem);
        input.send(1).unwrap();
        machine.execute(1000).unwrap();
        let result = output.try_iter().collect::<Vec<Word>>();
        println!("Part 1: {:?}", result);
    }
    {
        let (mut machine, input, output) = Machine::new(&mem);
        input.send(2).unwrap();
        machine.execute(1000000).unwrap();
        let result = output.try_iter().collect::<Vec<Word>>();
        println!("Part 2: {:?}", result);
    }
}
