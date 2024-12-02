use std::collections::HashMap;
use log;


fn part1(input: &str) -> String {

    let mut first_list: Vec<u32> = Vec::new();
    let mut second_list: Vec<u32> = Vec::new();

    // split the input by lines
    for line in input.lines(){
        log::debug!("{}", line);
        let mut first: String = String::new();
        let mut second: String = String::new();
        let mut first_done: bool = false;
        for i in line.chars() {
            if i == ' ' {
                first_done = true;
            } else if first_done {
                second.push(i);
            } else {
                first.push(i);
            }
        }
        first_list.push(first.parse::<u32>().unwrap());
        second_list.push(second.parse::<u32>().unwrap());
    }

    first_list.sort();
    second_list.sort();

    let mut result: u32 = 0;
    let mut i = 0;

    while i < first_list.len() {
        result += first_list[i].abs_diff(second_list[i]);
        i = i + 1;
    }


    format!("{}", result)
}


fn part2(input: &str) -> String {

    let mut first_list: HashMap<u32, u32> = HashMap::new();
    let mut second_list: HashMap<u32, u32> = HashMap::new();

    // split the input by lines
    for line in input.lines(){
        log::debug!("{}", line);
        let mut first: String = String::new();
        let mut second: String = String::new();
        let mut first_done: bool = false;
        for i in line.chars() {
            if i == ' ' {
                first_done = true;
            } else if first_done {
                second.push(i);
            } else {
                first.push(i);
            }
        }
        let first_number = first.parse::<u32>().unwrap();
        let second_number = second.parse::<u32>().unwrap();

        *first_list.entry(first_number).or_insert(0) += 1;
        *second_list.entry(second_number).or_insert(0) += 1;
    }

    let mut result: u32 = 0;

    for n in first_list.keys() {
        result += *n * first_list[n] * *second_list.entry(*n).or_insert(0);
    }

    format!("{}", result)
}

fn main() {
    env_logger::init();
    let input = std::fs::read_to_string("day1.txt").expect("Unable to read input file");

    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
}


#[cfg(test)]
mod tests {
    use crate::part1;
    use crate::part2;

    #[test]
    fn test_part1() {
        let example1 = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;
        let expected1 = "11".to_string();

        assert_eq!(part1(example1), expected1);
    }

    #[test]
    fn test_part2() {
        let example1 = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;
        let expected1 = "31".to_string();

        assert_eq!(part2(example1), expected1);
    }
}
