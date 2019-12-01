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
    match mass {
        x if x <= 0 => 0,
        mass => {
            let fuel_mass = mass / 3 - 2;

            fuel_mass + get_fuel_for_mass(fuel_mass)
        }
    }
}

#[cfg(test)]
mod test {
    use super::get_fuel_for_mass;

    #[test]
    fn first() {
        assert_eq!(get_fuel_for_mass(14), 2);
    }

    #[test]
    fn second() {
        assert_eq!(get_fuel_for_mass(14), 2);
    }
}
