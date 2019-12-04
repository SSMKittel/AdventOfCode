fn main() {
    let input_mass = include_str!("input.txt");
    let total_fuel = input_mass.split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .map(calc_fuel)
        .sum::<u32>();
    println!("Total Fuel Required: {}", total_fuel);
}
fn calc_fuel(mass:u32) -> u32 {
  mass / 3 - 2
}
