fn main() {
    let mut i = 0;

    let mut code = get_codes();

    loop {
        match code[i] {
            1 => {
                code[i + 3] = code[i + 1] + code[i + 2];
            }
            2 => {
                code[i + 3] = code[i + 1] * code[i + 2];
            }
            99 => break,
            x => eprintln!("Illegal opcode {}", x),
        }

        i += 4;
    }

    println!("{:?}", code);
}

fn get_codes() -> Vec<usize> {
    std::fs::read_to_string("input/input.txt")
        .unwrap()
        .split(",")
        .map(|s| s.trim().parse::<usize>().unwrap())
        .collect()
}
