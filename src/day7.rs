struct Equation {
    result: i64,
    operands: Vec<i64>
}

enum Operation {
    Add,
    Multiply
}

fn get_equations(input: &String) -> Vec<Equation> {
    input.lines().map(|line| {
        let result_length = line.find(":").unwrap();
        let result = (&line[0..result_length]).parse::<i64>().unwrap();
        let operands: Vec<i64> = line[result_length + 2..].split(" ").map(|operand| operand.parse::<i64>().unwrap()).collect();
        Equation { result, operands }
    }).collect()
}

fn get_first_valid_equation(result: i64, operands: &Vec<i64>, position: usize) -> Option<Vec<Operation>> {
    if position == 0 {
        return if result == 0 { Some(Vec::new()) } else { None }
    }
    let operand = operands[position - 1];
    if operand > result {
        return None
    }
    if result % operand == 0 {
        let equation_assuming_multiplication = get_first_valid_equation(result / operand, &operands, position - 1);
        if equation_assuming_multiplication.is_some() {
            return equation_assuming_multiplication
        }
    }
    get_first_valid_equation(result - operand, &operands, position - 1)
}

fn get_valid_results(equations: Vec<Equation>) -> Vec<i64> {
    equations.iter().filter(|e| get_first_valid_equation(e.result, &e.operands, e.operands.len()).is_some()).map(|equation| equation.result).collect()
}

fn part1(input: &String) -> String {
    format!("{}", get_valid_results(get_equations(input)).iter().sum::<i64>())
}

fn get_first_valid_equation_with_concatenation(result: i64, operands: &Vec<i64>, position: usize) -> Option<Vec<Operation>> {
    if position == 0 {
        return if result == 0 { Some(Vec::new()) } else { None }
    }
    let operand = operands[position - 1];
    if operand > result {
        return None
    }
    if result % operand == 0 {
        let equation_assuming_multiplication = get_first_valid_equation_with_concatenation(result / operand, &operands, position - 1);
        if equation_assuming_multiplication.is_some() {
            return equation_assuming_multiplication
        }
    }
    let power_of_ten = i64::pow(10, operand.to_string().len() as u32);
    if result % power_of_ten == operand {
        let equation_assuming_concatenation = get_first_valid_equation_with_concatenation(result / power_of_ten, &operands, position - 1);
        if equation_assuming_concatenation.is_some() {
            return equation_assuming_concatenation
        }
    }
    get_first_valid_equation_with_concatenation(result - operand, &operands, position - 1)
}

fn get_valid_results_with_concatenation(equations: Vec<Equation>) -> Vec<i64> {
    equations.iter().filter(|e| get_first_valid_equation_with_concatenation(e.result, &e.operands, e.operands.len()).is_some()).map(|equation| equation.result).collect()
}

fn part2(input: &String) -> String {
    format!("{}", get_valid_results_with_concatenation(get_equations(input)).iter().sum::<i64>())
}

fn main() {
    env_logger::init();
    let input1 = &std::fs::read_to_string("day7.txt").expect("Unable to read input file");
    println!("Part1: {}", part1(input1));

    println!("Part2: {}", part2(input1));
}


#[cfg(test)]
mod tests {
    use crate::part1;
    use crate::part2;

    #[test]
    fn test_part1() {
        let example1 = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#;
        let expected1 = "3749".to_string();

        assert_eq!(part1(&example1.to_string()), expected1);
    }

    #[test]
    fn test_part1_real() {
        let input1 = std::fs::read_to_string("day7.txt").expect("Unable to read input file");
        let expected1 = "12839601725877".to_string();

        assert_eq!(part1(&input1), expected1);
    }

    #[test]
    fn test_part2() {
        let example1 = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#;
        let expected1 = "11387".to_string();

        assert_eq!(part2(&example1.to_string()), expected1);
    }
}
