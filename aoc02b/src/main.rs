fn main() {
    let goal = 19690720;

    let code = get_codes();
    for noun in 0..=100 {
        for verb in 0..=100 {
            if calculate(code.clone(), noun, verb) == goal {
                println!("{}", 100 * noun + verb);
                std::process::exit(1);
            }
        }
    }
}

fn calculate(mut code: Vec<usize>, noun: usize, verb: usize) -> usize {
    let mut i = 0;

    // replace values to restore state
    code[1] = noun;
    code[2] = verb;

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

    code[0]
}

fn get_codes() -> Vec<usize> {
    std::fs::read_to_string("input/input.txt")
        .unwrap()
        .split(",")
        .map(|s| s.trim().parse::<usize>().unwrap())
        .collect()
}
