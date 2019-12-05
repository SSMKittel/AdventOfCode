fn main() {
    let min = 372037u32;
    let max = 905157u32;

    let mut data = permute_increasing(6, min, max);
    data.retain(has_doubles);
    println!("Count with doubles {}", data.len());
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