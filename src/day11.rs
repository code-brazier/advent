use std::collections::HashMap;

fn add_stone(stone_counts: &mut HashMap<String, i64>, stone: String, count: i64) {
    *stone_counts.entry(stone).or_insert(0) += count;
}

fn drop_leading_zeros(number: &str) -> &str {
    let mut start_index = 0;
    for digit in number.chars() {
        if digit != '0' {
            return &number[start_index..]
        }
        start_index += 1
    }
    "0"
}

fn blink(stone_counts: &HashMap<String, i64>) -> HashMap<String, i64> {
    let mut updated_stone_counts: HashMap<String, i64> = HashMap::new();
    for (stone, &count) in stone_counts.iter() {
        if stone == "0" {
            add_stone(&mut updated_stone_counts, "1".to_string(), count);
        } else if stone.len() % 2 == 0 {
            add_stone(&mut updated_stone_counts, stone[..stone.len() / 2].to_string(), count);
            add_stone(&mut updated_stone_counts, drop_leading_zeros(&stone[stone.len() / 2..]).to_string(), count);
        } else {
            add_stone(&mut updated_stone_counts, format!("{}", stone.parse::<i64>().unwrap() * 2024), count);
        }
    }
    updated_stone_counts
}

fn part1(input: &String) -> String {
    let mut stone_counts: HashMap<String, i64> = HashMap::new();

    for stone in input.split_whitespace() {
        add_stone(&mut stone_counts, stone.to_string(), 1)
    }

    for _ in 0..25 {
        stone_counts = blink(&stone_counts);
    }

    format!("{}", stone_counts.values().sum::<i64>())
}

fn part2(input: &String) -> String {
    let mut stone_counts: HashMap<String, i64> = HashMap::new();

    for stone in input.split_whitespace() {
        add_stone(&mut stone_counts, stone.to_string(), 1)
    }

    for _ in 0..75 {
        stone_counts = blink(&stone_counts);
    }

    format!("{}", stone_counts.values().sum::<i64>())
}

fn main() {
    env_logger::init();
    let input1 = &std::fs::read_to_string("day11.txt").expect("Unable to read input file");
    println!("Part1: {}", part1(input1));

    println!("Part2: {}", part2(input1));
}


#[cfg(test)]
mod tests {
    use crate::part1;

    #[test]
    fn test_part1() {
        let example1 = r#"125 17"#;
        let expected1 = "55312".to_string();

        assert_eq!(part1(&example1.to_string()), expected1);
    }
}
