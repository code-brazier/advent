use std::collections::HashMap;
use std::fs;

fn part1(input: &String) -> String {
    let towels = input.lines().nth(0).unwrap();
    let mut map = HashMap::new();

    for towel in towels.split(", ") {
        map.entry(towel.chars().nth(0).unwrap()).or_insert(Vec::new()).push(towel);
    }

    let mut valid_count = 0;
    for design in input.lines().skip(2) {
        if check_design(design, &map, 0) {
            valid_count += 1;
        }
    }

    format!("{}", valid_count)
}

fn check_design(design: &str, towels: &HashMap<char, Vec<&str>>, index: usize) -> bool {
    if index == design.len() {
        return true
    }

    let next_char = design.chars().nth(index).unwrap();
    if !towels.contains_key(&next_char) {
        return false
    }

    towels[&next_char].iter().any(|towel| {
        if towel.len() + index <= design.len() && towel.to_string() == design[index..index + towel.len()] {
            if check_design(design, towels, index + towel.len()) {
                return true
            }
        }

        false
    })
}

fn part2(input: &String) -> String {
    let towels = input.lines().nth(0).unwrap();
    let mut map = HashMap::new();

    for towel in towels.split(", ") {
        map.entry(towel.chars().nth(0).unwrap()).or_insert(Vec::new()).push(towel);
    }

    let mut total = 0;
    for design in input.lines().skip(2) {
        let mut valid_count: HashMap<usize, u64> = HashMap::new();
        valid_count.insert(0, 1);

        for i in 0..design.len() {
            if !valid_count.contains_key(&i) {
                continue
            }

            let stripe = design.chars().nth(i).unwrap();
            let partial_valid_count = valid_count.get(&i).unwrap_or(&0).clone();

            if !map.contains_key(&stripe) {
                continue
            }

            map[&stripe].iter().for_each(|towel| {
                let next_stripe_index = towel.len() + i;
                if next_stripe_index <= design.len() && towel.to_string() == design[i..i + towel.len()] {
                    *valid_count.entry(next_stripe_index).or_insert(0) += partial_valid_count;
                }
            });
        }

        total += valid_count.get(&design.len()).unwrap_or(&0);
    }

    format!("{}", total)
}

fn main() {
    env_logger::init();
    let input1 = &fs::read_to_string("day19.txt").expect("Unable to read input file");
    println!("Part1: {}", part1(&input1.to_string()));
    println!("Part2: {}", part2(&input1.to_string()));
}

#[cfg(test)]
mod tests {
    use crate::part1;
    use crate::part2;

    #[test]
    fn test_part1() {
        let example1 = r#"r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb"#;
        let expected1 = "6".to_string();

        assert_eq!(part1(&example1.to_string()), expected1);
    }

    #[test]
    fn test_part2() {
        let example1 = r#"r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb"#;
        let expected1 = "16".to_string();

        assert_eq!(part2(&example1.to_string()), expected1);
    }
}
