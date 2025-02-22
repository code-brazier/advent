fn run(mut a: u64, mut b: u64, mut c: u64, program: &Vec<&str>) -> Option<String> {
    let mut output: Vec<u64> = Vec::new();

    let mut pc = 0;

    while pc < program.len() {
        let instruction = program[pc];
        let raw_operand = program[pc + 1].parse::<u64>().unwrap();
        let operand = match instruction {
            "1" | "3" | "4" => raw_operand,
            _ => {
                match raw_operand {
                    0 | 1 | 2 | 3 => raw_operand,
                    4 => a,
                    5 => b,
                    6 => c,
                    _ => return None
                }
            }
        };
        match instruction {
            "0" => {
                let denominator = 2_u64.pow(operand as u32);
                a = a / denominator;
                pc += 2;
            },
            "1" => {
                b = operand ^ b;
                pc += 2;
            },
            "2" => {
                b = operand % 8;
                pc += 2;
            },
            "3" => {
                pc = if a == 0 { pc + 2 } else { operand as usize };
            }
            "4" => {
                b = b ^ c;
                pc += 2;
            },
            "5" => {
                output.push(operand % 8);
                pc += 2;
            },
            "6" => {
                let denominator = 2_u64.pow(operand as u32);
                b = a / denominator;
                pc += 2;
            },
            "7" => {
                let denominator = 2_u64.pow(operand as u32);
                c = a / denominator;
                pc += 2;
            },
            _ => {
                return None
            }
        }
    }

    Some(format!("{}", output.iter().map(ToString::to_string).collect::<Vec<String>>().join(",")))
}

fn part1(input: &String) -> String {
    let a = input.lines().nth(0).unwrap().split(": ").nth(1).unwrap().parse::<u64>().unwrap();
    let b = input.lines().nth(1).unwrap().split(": ").nth(1).unwrap().parse::<u64>().unwrap();
    let c = input.lines().nth(2).unwrap().split(": ").nth(1).unwrap().parse::<u64>().unwrap();
    let program = input.lines().nth(4).unwrap().split(": ").nth(1).unwrap().split(",").collect::<Vec<&str>>();

    run(a, b, c, &program).unwrap()
}

fn solve(a: u64, program: &Vec<&str>, depth: usize) -> Option<u64> {
    let solution_at_depth = program[program.len() - depth..program.len()].join(",");

    for guess in a..a + 8 {
        if guess == 0 {
            continue
        }

        if run(guess, 0, 0, &program) == Some(solution_at_depth.clone()) {
            if depth == program.len() {
                return Some(guess)
            } else {
                let candidate = solve(guess * 8, program, depth + 1);
                if candidate.is_some() {
                    return candidate;
                }
            }
        }
    }

    None
}

fn part2(input: &String) -> String {
    let original_program = input.lines().nth(4).unwrap();
    let program = original_program.split(": ").nth(1).unwrap().split(",").collect::<Vec<&str>>();

    format!("{}", solve(0, &program, 1).unwrap())
}

fn main() {
    env_logger::init();
    let input = r#"Register A: 61156655
Register B: 0
Register C: 0

Program: 2,4,1,5,7,5,4,3,1,6,0,3,5,5,3,0"#;

    println!("Part1: {}", part1(&input.to_string()));
    println!("Part2: {}", part2(&input.to_string()));
}

#[cfg(test)]
mod tests {
    use crate::part1;

    #[test]
    fn test_part1() {
        let example1 = r#"Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0"#;
        let expected1 = "4,6,3,5,6,3,5,2,1,0".to_string();

        assert_eq!(part1(&example1.to_string()), expected1);
    }

    #[test]
    fn test_part1_actual() {
        let example1 = r#"Register A: 61156655
Register B: 0
Register C: 0

Program: 2,4,1,5,7,5,4,3,1,6,0,3,5,5,3,0"#;
        let expected1 = "7,3,5,7,5,7,4,3,0".to_string();

        assert_eq!(part1(&example1.to_string()), expected1);
    }
}
