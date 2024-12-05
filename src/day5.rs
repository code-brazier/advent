use std::collections::HashMap;
use std::collections::HashSet;

fn get_middle_page(line: &str, page_order: &HashMap<i32, Vec<i32>>) -> i32 {
    let mut illegal_pages: HashSet<i32> = HashSet::new();
    let mut middle_page = 0;

    for index in (0..line.len()).step_by(3) {
        let page = line[index..index + 2].parse::<i32>().unwrap();
        if illegal_pages.contains(&page) {
            return 0
        }
        if index == line.len() / 2 - 1 {
            middle_page = page;
        }
        if page_order.contains_key(&page) {
            for illegal_page in &page_order[&page] {
                illegal_pages.insert(illegal_page.clone());
            }
        }
    }

    middle_page
}

fn part1(input: String) -> String {

    let mut sum = 0;
    let mut loaded = false;
    let mut page_order: HashMap<i32, Vec<i32>> = HashMap::new();

    for line in input.lines() {
        if loaded {
            sum += get_middle_page(line, &page_order);
        } else if line.len() == 0 {
            loaded = true;
        } else {
            page_order.entry(line[3..5].parse::<i32>().unwrap()).or_insert(Vec::new()).push(line[0..2].parse::<i32>().unwrap());
        }
    }

    format!("{}", sum)
}

fn check_page_order(page1: &i32, page2: &i32, rules: &HashMap<i32, Vec<i32>>) -> bool {
    rules.contains_key(page1) && rules[page1].contains(page2)
}

fn get_sorted_middle_page(line: &str, forward_rules: &HashMap<i32, Vec<i32>>) -> i32 {
    let mut sorted: Vec<i32> = Vec::new();

    for index in (0..line.len()).step_by(3) {
        let page = line[index..index + 2].parse::<i32>().unwrap();
        let mut destination_index = 0;
        for sortedPage in &sorted {
            if check_page_order(&page, sortedPage, forward_rules) {
                break;
            }
            destination_index += 1;
        }
        sorted.insert(destination_index, page);
    }

    println!("{:?}", &sorted);
    sorted[sorted.len() / 2]
}

fn part2(input: &str) -> String {

    let mut sum = 0;
    let mut loaded = false;
    let mut backward_rules: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut forward_rules: HashMap<i32, Vec<i32>> = HashMap::new();

    for line in input.lines() {
        if loaded {
            if get_middle_page(line, &backward_rules) == 0 {
                sum += get_sorted_middle_page(line, &forward_rules)
            }
        } else if line.len() == 0 {
            loaded = true;
        } else {
            forward_rules.entry(line[0..2].parse::<i32>().unwrap()).or_insert(Vec::new()).push(line[3..5].parse::<i32>().unwrap());
            backward_rules.entry(line[3..5].parse::<i32>().unwrap()).or_insert(Vec::new()).push(line[0..2].parse::<i32>().unwrap());
        }
    }

    format!("{}", sum)
}

fn main() {
    env_logger::init();
    let input1 = std::fs::read_to_string("day5.txt").expect("Unable to read input file");
    println!("Part1: {}", part1(input1));

    let input2 = std::fs::read_to_string("day5.txt").expect("Unable to read input file");
    println!("Part2: {}", part2(&input2));
}


#[cfg(test)]
mod tests {
    use crate::part1;
    use crate::part2;

    #[test]
    fn test_part1() {
        let example1 = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;
        let expected1 = "143".to_string();

        assert_eq!(part1(example1.to_string()), expected1);
    }

    #[test]
    fn test_part2() {
        let example1 = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;
        let expected1 = "123".to_string();

        assert_eq!(part2(example1), expected1);
    }
}
