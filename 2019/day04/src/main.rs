fn main() {
    let min = 372037u32;
    let max = 905157u32;

    let data = permute_increasing(6, min, max);

    let mut data_simple = data.to_vec();
    data_simple.retain(has_doubles);
    println!("Count with doubles {}", data_simple.len());

    let mut data_exact = data.to_vec();
    data_exact.retain(has_doubles_exact);
    println!("Count with doubles (exact) {}", data_exact.len());
}

fn permute_increasing(digits: u32, min: u32, max: u32) -> Vec<u32> {
    let factor = 10u32.pow(digits - 1);
    let mut data: Vec<u32> = Vec::new();
    permute_increasing_main(0, digits, min / factor, &mut data);
    data.retain(|x| x >= &min && x <= &max);
    data
}

fn permute_increasing_main(total: u32, digits: u32, min: u32, data: &mut Vec<u32>){
    if digits == 0 {
        data.push(total);
        return;
    }

    let factor = 10u32.pow(digits - 1);
    for i in min..=9 {
        permute_increasing_main(total + i * factor, digits - 1, i, data)
    }
}

fn has_doubles(val: &u32) -> bool {
    let mut tmp = *val;
    let mut prev = 0u32;
    while tmp > 0 {
        let current = tmp % 10;
        if current == prev {
            return true;
        }
        prev = current;
        tmp /= 10;
    }
    false
}

// Not a general-purpose function;
// only works when numbers are in distinct groups like we get with the increase-only permutation
fn has_doubles_exact(val: &u32) -> bool {
    let mut tmp = *val;
    let mut counts: [u32; 10] = [0; 10];
    while tmp > 0 {
        counts[(tmp % 10) as usize] += 1;
        tmp /= 10;
    }
    counts.contains(&2u32)
}