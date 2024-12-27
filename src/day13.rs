use std::str::FromStr;

struct InputData {
    x: i64,
    y: i64
}

impl FromStr for InputData {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(InputData {
            x: input[input.find('X').unwrap() + 2..input.find(',').unwrap()].parse::<i64>().unwrap(),
            y: input[input.find('Y').unwrap() + 2..].parse::<i64>().unwrap(),
        })
    }
}

fn get_score(input: &String, prize_offset: i64) -> String {
    let mut total_score = 0;
    let mut lines = input.lines();

    loop {
        let first_line = match lines.next() {
            None => break,
            Some(line) => line
        };
        let button_a = (if first_line.is_empty() { lines.next().unwrap() } else { first_line }).parse::<InputData>().unwrap();
        let button_b = lines.next().unwrap().parse::<InputData>().unwrap();

        let mut prize = lines.next().unwrap().parse::<InputData>().unwrap();
        prize.x += prize_offset;
        prize.y += prize_offset;

        if (button_a.x * prize.y - button_a.y * prize.x) % (button_a.x * button_b.y - button_a.y * button_b.x) != 0 {
            continue;
        }

        let b_presses = (button_a.x * prize.y - button_a.y * prize.x) / (button_a.x * button_b.y - button_a.y * button_b.x);

        if (prize.x - button_b.x * b_presses) % button_a.x != 0 {
            continue;
        }

        let a_presses = (prize.x - button_b.x * b_presses) / button_a.x;

        total_score += a_presses * 3 + b_presses;
    }

    format!("{}", total_score)
}

fn part1(input: &String) -> String {
    get_score(input, 0)
}

fn part2(input: &String) -> String {
    get_score(input, 10000000000000)
}

fn main() {
    env_logger::init();
    let input1 = &std::fs::read_to_string("day13.txt").expect("Unable to read input file");
    println!("Part1: {}", part1(input1));

    println!("Part2: {}", part2(input1));
}


#[cfg(test)]
mod tests {
    use crate::part1;
    use crate::part2;

    #[test]
    fn test_part1_small() {
        let example1 = r#"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279"#;
        let expected1 = "480".to_string();

        assert_eq!(part1(&example1.to_string()), expected1);
    }
}
