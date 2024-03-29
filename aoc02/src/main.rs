fn main() {
    let mut i = 0;

    let mut code = get_codes();

    // replace values to restore state
    code[1] = 12;
    code[2] = 2;

    loop {
        match code[i] {
            1 => {
                let (a, b, pos) = (code[i + 1], code[i + 2], code[i + 3]);
                code[pos] = code[a] + code[b];
            }
            2 => {
                let (a, b, pos) = (code[i + 1], code[i + 2], code[i + 3]);
                code[pos] = code[a] * code[b];
            }
            99 => break,
            x => eprintln!("Illegal opcode {}", x),
        }

        i += 4;
    }

    println!("{}", code[0]);
}

fn get_codes() -> Vec<usize> {
    std::fs::read_to_string("input/input.txt")
        .unwrap()
        .split(",")
        .map(|s| s.trim().parse::<usize>().unwrap())
        .collect()
}
