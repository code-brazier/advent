use log;
use std::collections::HashMap;

fn part1(input: &str) -> String {
    let mut first_list: Vec<u32> = Vec::new();
    let mut second_list: Vec<u32> = Vec::new();

    // split the input by lines
    for line in input.lines() {
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
    for line in input.lines() {
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

        //    *first_list.entry(first_number).or_insert(0) += 1;
        //    *second_list.entry(second_number).or_insert(0) += 1;

        // A common pattern in rust is to use a match statement to switch based on if a value is present or not.
        // BTW: Your old way is more efficient and almost certainly better especially if the value is not a simple integer.
        // It looks like references are common when dealing with map entries in rust, I just haven't needed to use them too much...
        let current_first = match first_list.get(&first_number) {
            Some(n) => n.to_owned(),
            None => 0,
        };
        first_list.insert(first_number, current_first + 1);

        // Alternatively, short hand for the same thing...
        // unwrap_or is a common way to deal with Option types in rust
        let current_second = second_list.get(&second_number).copied().unwrap_or(0);
        second_list.insert(second_number, current_second + 1);
    }

    let mut result: u32 = 0;

    for n in first_list.keys() {
        // result += *n * first_list[n] * *second_list.entry(*n).or_insert(0);
        // This is where I think the dereferences are a little bit confusing with the multiplier operator intermingled.

        // I think this is nicer, it seems rust is happy to do arithmetic on a borrowed value and return a non-borrowed one (for simple types only)...
        result += n * first_list[n] * second_list.get(n).unwrap_or(&0); // Note: This also avoids modifying the map, which we don't need do to in this situation.

        // let n = n.to_owned(); // Fine in this case as it's a u32 (BTW: overriding a variable with the same name but different type is common in rust!!!)
        // result += n * first_list[&n] * second_list.get(&n).copied().unwrap_or(0);

        // If you wanted to keep the reference, but don't want to have * everywhere you use it...
        // let n = *n;
        // result += n * first_list[&n] * second_list.get(&n).copied().unwrap_or(0);
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
