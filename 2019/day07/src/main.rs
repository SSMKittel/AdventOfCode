
extern crate int_code;
use int_code::*;

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

    let mut max: Word = 0;
    for phases in phase_permutations() {
        let mut bank = AmpBank::new(memory.clone(), phases[0] + 5, phases[1] + 5, phases[2] + 5, phases[3] + 5, phases[4] + 5);
        let tmp = bank.execute();
        if tmp > max {
            max = tmp;
        }
    }
    println!("Feedback Max: {}", max);
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
    input: Sender<Word>,
    output: Receiver<Word>
}

impl AmpBank {
    fn new(memory: Vec<Word>,
                  phase1: Word, phase2: Word,
                  phase3: Word, phase4: Word,
                  phase5: Word) -> AmpBank {
        let (input_1_source, input_1_read): (Sender<Word>, Receiver<Word>) = channel();
        let (input_2_source, input_2_read): (Sender<Word>, Receiver<Word>) = channel();
        let (input_3_source, input_3_read): (Sender<Word>, Receiver<Word>) = channel();
        let (input_4_source, input_4_read): (Sender<Word>, Receiver<Word>) = channel();
        let (input_5_source, input_5_read): (Sender<Word>, Receiver<Word>) = channel();
        let (output_source, output): (Sender<Word>, Receiver<Word>) = channel();

        input_1_source.send(phase1).unwrap();
        input_2_source.send(phase2).unwrap();
        input_3_source.send(phase3).unwrap();
        input_4_source.send(phase4).unwrap();
        input_5_source.send(phase5).unwrap();

        AmpBank {
            amps: [
                Machine::with_channels(&memory, input_1_read, input_2_source),
                Machine::with_channels(&memory, input_2_read, input_3_source),
                Machine::with_channels(&memory, input_3_read, input_4_source),
                Machine::with_channels(&memory, input_4_read, input_5_source),
                Machine::with_channels(&memory, input_5_read, output_source)
            ],
            input: input_1_source,
            output
        }
    }

    fn execute(&mut self) -> Word {
        let mut halting = false;
        let mut feedback = 0;

        loop {
            self.input.send(feedback).unwrap();
            for m in self.amps.iter_mut() {
                match m.execute(100) {
                    Ok(_) => halting = true,
                    Err(ExecuteError::InputRequired) => (),
                    Err(a) => panic!("Error {}", a),
                }
            }

            let out = self.output.try_iter().collect::<Vec<Word>>();
            if out.len() != 1 {
                panic!("unexpected output {:?}", out);
            }

            if halting {
                return out[0]
            }
            else {
                feedback = out[0];
            }
        }
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

    #[test]
    fn test_139629729() {
        let input_mem = "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5";
        let memory = parse_csv(input_mem).unwrap();
        let mut bank = AmpBank::new(memory, 9, 8, 7, 6, 5);
        assert_eq!(139629729, bank.execute());
    }

    #[test]
    fn test_18216() {
        let input_mem = "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10";
        let memory = parse_csv(input_mem).unwrap();
        let mut bank = AmpBank::new(memory, 9, 7, 8, 5, 6);
        assert_eq!(18216, bank.execute());
    }
}
