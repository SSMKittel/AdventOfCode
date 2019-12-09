
extern crate int_code;
use int_code::*;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;

fn main() {
    let memory = parse_csv(include_str!("input.txt")).unwrap();

    let mut max: Word = 0;
    for phases in phase_permutations() {
        let mut bank = AmpBank::new(memory.clone(), phases[0], phases[1], phases[2], phases[3], phases[4]);
        let tmp = bank.execute();
        if tmp > max {
            max = tmp;
        }
    }
    println!("Max: {}", max);
}

fn phase_permutations() -> Vec<[Word; 5]> {
    let mut results = Vec::with_capacity(5 * 4 * 3 * 2);
    for i1 in 0..=4 {
        for i2 in 0..=4 {
            for i3 in 0..=4 {
                for i4 in 0..=4 {
                    for i5 in 0..=4 {
                        let tmp = [i1, i2, i3, i4, i5];
                        if is_valid(&tmp) {
                            results.push(tmp);
                        }
                    }
                }
            }
        }
    }
    results
}

fn is_valid(phases: &[Word; 5]) -> bool {
    let mut tmp = [0u32; 5];
    for i in phases {
        tmp[*i as usize] += 1;
    }
    tmp.iter().all(|x| *x == 1u32)
}

struct AmpBank {
    amps: [Machine; 5],
    output: Receiver<Word>
}

impl AmpBank {
    fn new(memory: Vec<Word>,
                  phase1: Word, phase2: Word,
                  phase3: Word, phase4: Word,
                  phase5: Word) -> AmpBank {
        let (input, input_1): (Sender<Word>, Receiver<Word>) = mpsc::channel();
        let (output_1, input_2): (Sender<Word>, Receiver<Word>) = mpsc::channel();
        let (output_2, input_3): (Sender<Word>, Receiver<Word>) = mpsc::channel();
        let (output_3, input_4): (Sender<Word>, Receiver<Word>) = mpsc::channel();
        let (output_4, input_5): (Sender<Word>, Receiver<Word>) = mpsc::channel();
        let (output_5, output): (Sender<Word>, Receiver<Word>) = mpsc::channel();

        input.send(phase1).unwrap();
        input.send(0).unwrap();
        output_1.send(phase2).unwrap();
        output_2.send(phase3).unwrap();
        output_3.send(phase4).unwrap();
        output_4.send(phase5).unwrap();

        AmpBank {
            amps: [
                Machine::with_channels(memory.clone(), input_1, output_1),
                Machine::with_channels(memory.clone(), input_2, output_2),
                Machine::with_channels(memory.clone(), input_3, output_3),
                Machine::with_channels(memory.clone(), input_4, output_4),
                Machine::with_channels(memory, input_5, output_5)
            ],
            output
        }
    }

    fn execute(&mut self) -> Word {
        for m in self.amps.iter_mut() {
            match m.execute() {
                Ok(_) => {},
                Err(a) => panic!("Error {}", a),
            }
        }
        let out = self.output.try_iter().collect::<Vec<Word>>();
        if out.len() != 1 {
            panic!("unexpected output {:?}", out);
        }
        out[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_43210() {
        let input_mem = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0";
        let memory = parse_csv(input_mem).unwrap();
        let mut bank = AmpBank::new(memory, 4, 3, 2, 1, 0);
        assert_eq!(43210, bank.execute());
    }

    #[test]
    fn test_54321() {
        let input_mem = "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0";
        let memory = parse_csv(input_mem).unwrap();
        let mut bank = AmpBank::new(memory, 0, 1, 2, 3, 4);
        assert_eq!(54321, bank.execute());
    }

    #[test]
    fn test_65210() {
        let input_mem = "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0";
        let memory = parse_csv(input_mem).unwrap();
        let mut bank = AmpBank::new(memory, 1, 0, 4, 3, 2);
        assert_eq!(65210, bank.execute());
    }
}
