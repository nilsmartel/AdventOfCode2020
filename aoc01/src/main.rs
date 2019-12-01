fn main() {
    let buffer = std::fs::read_to_string("input/modules").unwrap();

    let result: i64 = buffer
        .lines()
        .map(|l| l.parse::<i64>().unwrap())
        .map(get_fuel_for_mass)
        .sum();

    println!("{}", result);
}

fn get_fuel_for_mass(mass: i64) -> i64 {
    mass / 3 - 2
}
