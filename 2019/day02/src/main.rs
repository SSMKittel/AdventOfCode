fn main() {
    let input_mem = include_str!("input.txt");
    let memory = input_mem.split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    {
        let mut memory_1202 = memory.to_vec();
        memory_1202[1] = 12;
        memory_1202[2] = 2;
        execute(&mut memory_1202);
        println!("Repaired value: {}", memory_1202[0]);
    }
    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut memory_nv = memory.to_vec();
            memory_nv[1] = noun;
            memory_nv[2] = verb;
            execute(&mut memory_nv);
            if memory_nv[0] == 19690720 {
                println!("100 * {} + {} = {}", noun, verb, 100 * noun + verb);
            }
        }
    }
}

fn execute(memory:&mut [i32]) {
    for pc in (0..memory.len()).step_by(4) {
        let opcode = memory[pc];
        if opcode == 99 {
            break;
        }
        let val1_address = memory[pc + 1] as usize;
        let val2_address = memory[pc + 2] as usize;
        let storage_address = memory[pc + 3] as usize;

        let val1 = memory[val1_address];
        let val2 = memory[val2_address];
        if opcode == 1 {
            memory[storage_address] = val1 + val2;
        }
        else if opcode == 2 {
            memory[storage_address] = val1 * val2;
        }
        else {
            panic!("unrecognised opcode {} at address {}", opcode, pc);
        }
    }
}

#[allow(dead_code)]
fn dump(mem:&[i32]) {
    let mut i = 0;
    for memory_value in mem {
        if i == 0 {
            print!("{}",memory_value);
            i += 1;
        }
        else {
            print!("\t{}", memory_value);
            i += 1;
            if i == 4 {
                println!();
                i = 0;
            }
        }
    }
    if i == 1 {
        println!();
    }
}
