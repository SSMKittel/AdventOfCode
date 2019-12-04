fn main() {
    let input_mem = include_str!("input.txt");
    let mut memory = input_mem.split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    memory[1] = 12;
    memory[2] = 2;

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

    dump(&memory);
}

fn dump(mem:&Vec<i32>) {
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
