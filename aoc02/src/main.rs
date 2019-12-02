fn main() {
    dbg!(get_codes());
}

fn get_codes() -> Vec<usize> {
    std::fs::read_to_string("input/input.txt")
        .unwrap()
        .split(",")
        .map(|s| s.trim().parse::<usize>().unwrap())
        .collect()
}
