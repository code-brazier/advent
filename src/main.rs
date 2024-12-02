use std::collections::HashMap;
use log;
use num_traits::sign::signum;


fn part1(input: &str) -> String {

    let mut safe_count = 0;

    // split the input by lines
    for line in input.lines(){

        let levels: Vec<i32> = line.split(' ').map(|level| level.parse::<i32>().unwrap()).collect();

        let mut index = 0;
        let mut offset = 0;

        loop {
            if levels[index] == levels[index + 1] + offset {
                if offset == 0 {
                    break;
                }

                index += 1;
                offset = 0;
                if levels.len() == index + 1 {
                    safe_count += 1;
                    break;
                }
            } else {
                offset += 1;
                if offset > 3 {
                    break;
                }
            }
        }

        index = 0;
        offset = 0;

        loop {
            if levels[index] == levels[index + 1] - offset {
                if offset == 0 {
                    break;
                }

                index += 1;
                offset = 0;
                if levels.len() == index + 1 {
                    safe_count += 1;
                    break;
                }
            } else {
                offset += 1;
                if offset > 3 {
                    break;
                }
            }
        }
    }

    format!("{}", safe_count)
}

fn pair_valid(a: i32, b: i32, direction: i32) -> bool {
    let mut offset = 0;
    loop {
        if a + offset * direction == b {
            if offset == 0 {
                println!("Not valid: {} {} {}", a, b, direction);
            } else {
                println!("Valid: {} {} {}", a, b, direction);
            }
            return offset != 0
        }
        offset += 1;
        if offset > 3 {
            println!("Not valid: {} {} {}", a, b, direction);
            return false
        }
    }
}

fn level_valid(levels: Vec<i32>, direction: i32) -> bool {
    let mut index = 0;
    let mut tolerated = false;

    loop {
        if index + 1 == levels.len() {
            return true;
        }
        if pair_valid(levels[index], levels[index + 1], direction) {
            index += 1;
        } else {
            if tolerated {
                return false;
            }
            tolerated = true;

            if index + 2 == levels.len() {
                return true
            }

            let can_remove_first = (index == 0 || pair_valid(levels[index - 1], levels[index + 1], direction)) && pair_valid(levels[index + 1], levels[index + 2], direction);
            let can_remove_second = pair_valid(levels[index], levels[index + 2], direction);
            if !can_remove_first && !can_remove_second {
                return false;
            }

            index += 2;
        }
    }
}


fn part2(input: &str) -> String {

    let mut safe_count = 0;

    // split the input by lines
    for line in input.lines() {

        let levels: Vec<i32> = line.split(' ').map(|level| level.parse::<i32>().unwrap()).collect();

        if level_valid(levels.clone(), 1) || level_valid(levels.clone(), -1) {
            safe_count += 1;
        }
    }

    format!("{}", safe_count)
}

fn main() {
    env_logger::init();
    let input = std::fs::read_to_string("day2.txt").expect("Unable to read input file");

    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
}


#[cfg(test)]
mod tests {
    use crate::part1;
    use crate::part2;

    #[test]
    fn test_part1() {
        let example1 = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;
        let expected1 = "2".to_string();

        assert_eq!(part1(example1), expected1);
    }

    #[test]
    fn test_part2() {
        let example1 = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;
        let expected1 = "4".to_string();

        assert_eq!(part2(example1), expected1);
    }

    #[test]
    fn test_part2_hard() {
        let example1 = r#"7 6 6 3 1"#;
        let expected1 = "1".to_string();

        assert_eq!(part2(example1), expected1);
    }
}
