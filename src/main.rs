fn parse_second_number(input: &str, position: usize, use_conditionals: bool, first_number_raw: &str, output: &mut String) -> i32 {
    let mut i = 0;
    loop {
        if position + i >= input.len() {
            break;
        }
        if &input[position + i..position + i + 1] == ")" {
            if i == 0 {
                break;
            }
            let first_number = first_number_raw.parse::<i32>().unwrap();
            let second_number = input[position..position + i].parse::<i32>().unwrap();
            output.push_str(format!("mul({},{})", first_number_raw, &input[position..position + i]).as_str());
            return first_number * second_number + parse_mul(input, position + i + 1, use_conditionals, output);
        }
        if !input[position + i..position + i + 1].parse::<i32>().is_ok() {
            break;
        }
        i += 1;
    }

    parse_mul(input, position + i + 1, use_conditionals, output)
}

fn parse_first_number(input: &str, position: usize, use_conditionals: bool, output: &mut String) -> i32 {
    let mut i = 0;
    loop {
        if position + i >= input.len() {
            break;
        }
        if &input[position + i..position + i + 1] == "," {
            if i == 0 {
                break;
            }
            return parse_second_number(input, position + i + 1, use_conditionals, &input[position..position + i], output);
        }
        if !input[position + i..position + i + 1].parse::<i32>().is_ok() {
            break;
        }
        i += 1;
    }

    parse_mul(input, position + i + 1, use_conditionals, output)
}

// fn parse_do(input: &str, position: usize) -> i32 {
//     let mut i = 0;
//     loop {
//         if position + i + 7 >= input.len() {
//             return 0
//         }
//         if &input[position + i..position + i + 7] == "don't()" {
//             return parse_mul(input, position + i + 7, true)
//         }
//         i += 1;
//     }
// }
//
// fn parse_mul(input: &str, position: usize, use_conditionals: bool) -> i32 {
//     let mut i = 0;
//     loop {
//         if position + i + 7 >= input.len() {
//             return 0;
//         }
//         if &input[position + i..position + i + 4] == "mul(" {
//             return parse_first_number(input, position + i + 4, use_conditionals)
//         }
//         if use_conditionals && &input[position + i..position + i + 4] == "do()" {
//             return parse_do(input, position + i + 4);
//         }
//         i += 1;
//     }
// }

fn parse_do(input: &str, position: usize, output: &mut String) -> i32 {
    let mut i = 0;
    loop {
        if position + i + 4 >= input.len() {
            return 0
        }
        if &input[position + i..position + i + 4] == "do()" {
            output.push_str("do()");
            return parse_mul(input, position + i + 4, true, output)
        }
        i += 1;
    }
}

fn parse_mul(input: &str, position: usize, use_conditionals: bool, output: &mut String) -> i32 {
    let mut i = 0;
    loop {
        if position + i + 7 >= input.len() {
            return 0;
        }
        if &input[position + i..position + i + 4] == "mul(" {
            return parse_first_number(input, position + i + 4, use_conditionals, output)
        }
        if use_conditionals && &input[position + i..position + i + 7] == "don't()" {
            output.push_str("don't()");
            return parse_do(input, position + i + 7, output)
        }
        i += 1;
    }
}

fn part1(input: &str) -> String {

    let mut mul_count = 0;

    mul_count += parse_mul(input, 0, false, &mut String::new());

    format!("{}", mul_count)
}

fn part2(input: &str) -> String {

    let mut mul_count = 0;
    let mut output = String::new();

    mul_count += parse_mul(input, 0, true, &mut output);

    // std::fs::write("out.txt", output).expect("Unable to write output file");

    format!("{}", mul_count)
}

fn main() {
    env_logger::init();
    let input = std::fs::read_to_string("day3.txt").expect("Unable to read input file");

    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
}


#[cfg(test)]
mod tests {
    use crate::part1;
    use crate::part2;
    use crate::parse_do;

    #[test]
    fn test_part1() {
        let example1 = r#"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"#;
        let expected1 = "161".to_string();

        assert_eq!(part1(example1), expected1);
    }

    #[test]
    fn test_part2() {
        let example1 = r#"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"#;
        let expected1 = "48".to_string();

        assert_eq!(part2(example1), expected1);
    }
}
