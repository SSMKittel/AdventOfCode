fn main() {
    let input = include_str!("input.txt");
    let groups = input.split("\n\n")
        .filter(|&x| x.len() > 0)
        .map(|x| parse_group(x))
        .collect::<Vec<_>>();

    let totals_any: u32 = groups.iter()
        .map(|x| x.iter().fold(0, |a, &b| (a | b)).count_ones())
        .sum();

    let totals_all: u32 = groups.iter()
        .map(|x| x.iter().fold(!0, |a, &b| (a & b)).count_ones())
        .sum();

    println!("any: {}, all: {}", totals_any, totals_all);
}

fn parse_group(group: &str) -> Vec<u32> {
    let mut result: Vec<u32> = Vec::new();
    let mut combined = 0;

    for c in group.chars() {
        if c >= 'a' && c <= 'z' {
            combined = combined | (1 << (c as u32 - 'a' as u32));
        }
        else if c == '\n' {
            result.push(combined);
            combined = 0;
            continue;
        }
        else {
            panic!("unrecognised {}", c);
        }
    }
    if combined != 0 {
        result.push(combined);
    }
    result
}