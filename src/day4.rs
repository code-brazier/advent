fn rotate1(input: &String) -> String {
    let mut rows: Vec<String> = Vec::new();

    let mut start_index = 0;
    for line in input.lines(){
        let mut index = start_index;
        for char in line.chars().rev() {
            if index == rows.len() {
                rows.push(String::new());
            }
            rows[index].push(char);
            index += 1;
        }
        start_index += 1;
    }

    rows.join("\n")
}

fn rotate2(input: &String) -> String {
    let mut rows: Vec<String> = Vec::new();

    let mut start_index = 0;
    let mut previous_len = 0;
    for line in input.lines(){
        if line.len() < previous_len {
            start_index += 1;
        }
        previous_len = line.len();
        let mut index = start_index;
        for char in line.chars().rev() {
            if index == rows.len() {
                rows.push(String::new());
            }
            rows[index].push(char);
            index += 1;
        }
    }

    rows.join("\n")
}

fn count(input: &String) -> i32 {
    let mut xmas_count = 0;

    for line in input.lines(){
        let mut index = 0;
        loop {
            if index + 4 > line.len() {
                break;
            }
            if &line[index..index + 4] == "XMAS" {
                xmas_count += 1;
            }
            index += 1;
        }
    }

    xmas_count
}

fn part1(input: String) -> String {

    let mut xmas_count = 0;

    xmas_count += count(&input);
    let mut rotation = rotate1(&input);

    for _i in 0..3 {
        xmas_count += count(&rotation);
        rotation = rotate2(&rotation);
        xmas_count += count(&rotation);
        rotation = rotate1(&rotation);
    }

    xmas_count += count(&rotation);

    format!("{}", xmas_count)
}

fn contains_endpoints(a: char, b: char) -> bool {
    (a == 'M' && b == 'S') || (a == 'S' && b == 'M')
}

fn is_x_mas(candidate: &String) -> bool {
    candidate.chars().nth(4).unwrap() == 'A' && contains_endpoints(candidate.chars().nth(0).unwrap(), candidate.chars().nth(8).unwrap()) && contains_endpoints(candidate.chars().nth(2).unwrap(), candidate.chars().nth(6).unwrap())
}

fn part2(input: &str) -> String {

    let mut candidates: Vec<String> = Vec::new();

    let mut row = 0;
    for line in input.lines(){
        for i in 0..line.len() - 2 {
            if row < line.len() - 2 {
                candidates.push(line[i..i + 3].to_string());
            }
            if row > 0 && row < line.len() - 1 {
                candidates[(row - 1) * (line.len() - 2) + i].push_str(&line[i..i + 3])
            }
            if row > 1 {
                candidates[(row - 2) * (line.len() - 2) + i].push_str(&line[i..i + 3])
            }
        }
        row = row + 1;
    }

    format!("{}", candidates.iter().filter(|c| is_x_mas(c)).count())
}

fn main() {
    env_logger::init();
    let input1 = std::fs::read_to_string("day4.txt").expect("Unable to read input file");
    println!("Part1: {}", part1(input1));

    let input2 = std::fs::read_to_string("day4.txt").expect("Unable to read input file");
    println!("Part2: {}", part2(&input2));
}


#[cfg(test)]
mod tests {
    use crate::part1;
    use crate::part2;

    #[test]
    fn test_part1() {
        let example1 = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;
        let expected1 = "18".to_string();

        assert_eq!(part1(example1.to_string()), expected1);
    }

    #[test]
    fn test_part2() {
        let example1 = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;
        let expected1 = "9".to_string();

        assert_eq!(part2(example1), expected1);
    }
}
