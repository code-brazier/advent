use std::collections::HashSet;

fn char_to_int(c: char) -> i32 {
    c as i32 - 0x30
}

struct Position {
    row: usize,
    col: usize,
}

fn get_trail_heads(map: &Vec<Vec<i32>>) -> Vec<Position> {
    let mut trail_heads: Vec<Position> = Vec::new();

    for row in 0..map.len() {
        for col in 0..map.len() {
            if map[row][col] == 0 {
                trail_heads.push(Position { row, col });
            }
        }
    }

    trail_heads
}

fn get_destinations(map: &Vec<Vec<i32>>, search_map: &mut Vec<Vec<Option<HashSet<i32>>>>, row: usize, col: usize, height: i32) -> HashSet<i32> {
    if height == 9 {
        return vec![(row * 100 + col) as i32].into_iter().collect()
    }

    match &search_map[row][col] {
        None => {
            let mut destinations: HashSet<i32> = HashSet::new();

            if row != 0 && map[row - 1][col] == height + 1 {
                destinations.extend(get_destinations(map, search_map, row - 1, col, height + 1));
            }

            if col != 0 && map[row][col - 1] == height + 1 {
                destinations.extend(get_destinations(map, search_map, row, col - 1, height + 1));
            }

            if row + 1 != map.len() && map[row + 1][col] == height + 1 {
                destinations.extend(get_destinations(map, search_map, row + 1, col, height + 1));
            }

            if col + 1 != map.len() && map[row][col + 1] == height + 1 {
                destinations.extend(get_destinations(map, search_map, row, col + 1, height + 1));
            }

            search_map[row][col] = Some(destinations.clone());
            destinations
        },
        Some(destinations) => destinations.clone()
    }
}

fn part1(input: &String) -> String {
    let size = input.lines().count();
    let mut total_score = 0;
    let map: Vec<Vec<i32>> = input.lines().map(|line| line.chars().map(char_to_int).collect()).collect();
    let mut search_map: Vec<Vec<Option<HashSet<i32>>>> = vec![vec![None; size]; size];

    for trail_head in get_trail_heads(&map) {
        total_score += get_destinations(&map, &mut search_map, trail_head.row, trail_head.col, 0).len();
    }

    format!("{}", total_score)
}

fn get_trails(map: &Vec<Vec<i32>>, search_map: &mut Vec<Vec<i32>>, row: usize, col: usize, height: i32) -> i32 {
    if height == 9 {
        return 1
    }

    match search_map[row][col] {
        -1 => {
            let mut trail_count: i32 = 0;

            if row != 0 && map[row - 1][col] == height + 1 {
                trail_count += get_trails(map, search_map, row - 1, col, height + 1);
            }

            if col != 0 && map[row][col - 1] == height + 1 {
                trail_count += get_trails(map, search_map, row, col - 1, height + 1);
            }

            if row + 1 != map.len() && map[row + 1][col] == height + 1 {
                trail_count += get_trails(map, search_map, row + 1, col, height + 1);
            }

            if col + 1 != map.len() && map[row][col + 1] == height + 1 {
                trail_count += get_trails(map, search_map, row, col + 1, height + 1);
            }

            search_map[row][col] = trail_count;
            trail_count
        },
        trail_count => trail_count
    }
}

fn part2(input: &String) -> String {
    let size = input.lines().count();
    let mut total_score = 0;
    let map: Vec<Vec<i32>> = input.lines().map(|line| line.chars().map(char_to_int).collect()).collect();
    let mut search_map: Vec<Vec<i32>> = vec![vec![-1; size]; size];

    for trail_head in get_trail_heads(&map) {
        total_score += get_trails(&map, &mut search_map, trail_head.row, trail_head.col, 0);
    }

    format!("{}", total_score)
}

fn main() {
    env_logger::init();
    let input1 = &std::fs::read_to_string("day10.txt").expect("Unable to read input file");
    println!("Part1: {}", part1(input1));

    println!("Part2: {}", part2(input1));
}


#[cfg(test)]
mod tests {
    use crate::part1;
    use crate::part2;

    #[test]
    fn test_part1() {
        let example1 = r#"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"#;
        let expected1 = "36".to_string();

        assert_eq!(part1(&example1.to_string()), expected1);
    }

    #[test]
    fn test_part2() {
        let example1 = r#"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"#;
        let expected1 = "81".to_string();

        assert_eq!(part2(&example1.to_string()), expected1);
    }
}
