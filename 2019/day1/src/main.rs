fn main() {
    let input_mass = include_str!("input.txt");
    let module_masses = input_mass.split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    let simple_fuel_mass = module_masses.iter()
        .map(calc_fuel_simple)
        .sum::<u32>();

    let real_fuel_mass = module_masses.iter()
        .map(calc_fuel)
        .sum::<u32>();

    println!("Total Fuel Required (Simple): {}", simple_fuel_mass);
    println!("Total Fuel Required (Including Fuel Mass): {}", real_fuel_mass);
}
fn calc_fuel_simple(mass:&u32) -> u32 {
  mass / 3 - 2
}
fn calc_fuel(mass:&u32) -> u32 {
    let tmp = mass / 3;
    if tmp <= 2 {
        return 0;
    }
    let tmp = tmp - 2;
    tmp + calc_fuel(&tmp)
}
