struct Position {
    row: i32,
    col: i32,
}

struct Perimeter {
    length: i32,
    sides: i32
}

fn is_in_region(map: &Vec<Vec<char>>, row: i32, col: i32, region: char) -> bool {
    row != -1 && col != -1 && row != map.len() as i32 && col != map.len() as i32 && map[row as usize][col as usize] == region
}

fn evaluate_corner(neighbour1: bool, neighbour2: bool, neighbour3: bool) -> bool {
    (!neighbour1 && !neighbour2) || (neighbour1 && neighbour2 && !neighbour3)
}

fn evaluate_corners(map: &Vec<Vec<char>>, row: i32, col: i32, region: char, perimeter: &mut Perimeter) {
    if evaluate_corner(is_in_region(map, row + 1, col, region), is_in_region(map, row, col + 1, region), is_in_region(map, row + 1, col + 1, region)) {
        perimeter.sides += 1;
    }

    if evaluate_corner(is_in_region(map, row + 1, col, region), is_in_region(map, row, col - 1, region), is_in_region(map, row + 1, col - 1, region)) {
        perimeter.sides += 1;
    }

    if evaluate_corner(is_in_region(map, row - 1, col, region), is_in_region(map, row, col - 1, region), is_in_region(map, row - 1, col - 1, region)) {
        perimeter.sides += 1;
    }

    if evaluate_corner(is_in_region(map, row - 1, col, region), is_in_region(map, row, col + 1, region), is_in_region(map, row - 1, col + 1, region)) {
        perimeter.sides += 1;
    }
}

fn evaluate_position(map: &Vec<Vec<char>>, search_map: &mut Vec<Vec<bool>>, row: i32, col: i32, region: char, queue: &mut Vec<Position>, perimeter: &mut Perimeter) {
    if !is_in_region(map, row, col, region) {
        perimeter.length += 1;
    } else if !search_map[row as usize][col as usize] {
        search_map[row as usize][col as usize] = true;
        queue.push(Position { row, col });
        evaluate_corners(map, row, col, region, perimeter);
    }
}

fn get_region_area(map: &Vec<Vec<char>>, search_map: &mut Vec<Vec<bool>>, row: i32, col: i32, perimeter: &mut Perimeter) -> i32 {
    let mut area = 0;
    let region = map[row as usize][col as usize];

    let mut queue: Vec<Position> = Vec::new();
    queue.push(Position {row, col});
    search_map[row as usize][col as usize] = true;
    evaluate_corners(map, row, col, region, perimeter);

    loop {
        match queue.pop() {
            None => break,
            Some(position) => {
                area += 1;

                evaluate_position(map, search_map, position.row + 1, position.col, region, &mut queue, perimeter);
                evaluate_position(map, search_map, position.row, position.col + 1, region, &mut queue, perimeter);
                evaluate_position(map, search_map, position.row - 1, position.col, region, &mut queue, perimeter);
                evaluate_position(map, search_map, position.row, position.col - 1, region, &mut queue, perimeter);
            }
        }
    }

    area
}

fn part1(input: &String) -> String {
    let size = input.lines().count();
    let mut total_score = 0;
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut search_map: Vec<Vec<bool>> = vec![vec![false; size]; size];

    for row in 0..map.len() {
        for col in 0..map.len() {
            if !search_map[row][col] {
                let mut perimeter = Perimeter { length: 0, sides: 0 };
                total_score += get_region_area(&map, &mut search_map, row as i32, col as i32, &mut perimeter) * perimeter.length;
            }
        }
    }

    format!("{}", total_score)
}

fn part2(input: &String) -> String {
    let size = input.lines().count();
    let mut total_score = 0;
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut search_map: Vec<Vec<bool>> = vec![vec![false; size]; size];

    for row in 0..map.len() {
        for col in 0..map.len() {
            if !search_map[row][col] {
                let mut perimeter = Perimeter { length: 0, sides: 0 };
                let area = get_region_area(&map, &mut search_map, row as i32, col as i32, &mut perimeter);
                total_score += area * perimeter.sides;
            }
        }
    }

    format!("{}", total_score)
}

fn main() {
    env_logger::init();
    let input1 = &std::fs::read_to_string("day12.txt").expect("Unable to read input file");
    println!("Part1: {}", part1(input1));

    println!("Part2: {}", part2(input1));
    // 846196 - too low
}


#[cfg(test)]
mod tests {
    use crate::part1;
    use crate::part2;

    #[test]
    fn test_part1_small() {
        let example1 = r#"AAAA
BBCD
BBCC
EEEC"#;
        let expected1 = "140".to_string();

        assert_eq!(part1(&example1.to_string()), expected1);
    }

    #[test]
    fn test_part1_medium() {
        let example1 = r#"OOOOO
OXOXO
OOOOO
OXOXO
OOOOO"#;
        let expected1 = "772".to_string();

        assert_eq!(part1(&example1.to_string()), expected1);
    }

    #[test]
    fn test_part1_large() {
        let example1 = r#"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"#;
        let expected1 = "1930".to_string();

        assert_eq!(part1(&example1.to_string()), expected1);
    }

    #[test]
    fn test_part2_small() {
        let example1 = r#"AAAA
BBCD
BBCC
EEEC"#;
        let expected1 = "80".to_string();

        assert_eq!(part2(&example1.to_string()), expected1);
    }

    #[test]
    fn test_part2_e() {
        let example1 = r#"EEEEE
EXXXX
EEEEE
EXXXX
EEEEE"#;
        let expected1 = "236".to_string();

        assert_eq!(part2(&example1.to_string()), expected1);
    }

    #[test]
    fn test_part2_medium() {
        let example1 = r#"OOOOO
OXOXO
OOOOO
OXOXO
OOOOO"#;
        let expected1 = "436".to_string();

        assert_eq!(part2(&example1.to_string()), expected1);
    }

    #[test]
    fn test_part2_a_b() {
        let example1 = r#"AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA
"#;
        let expected1 = "368".to_string();

        assert_eq!(part2(&example1.to_string()), expected1);
    }

    #[test]
    fn test_part2_large() {
        let example1 = r#"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"#;
        let expected1 = "1206".to_string();

        assert_eq!(part2(&example1.to_string()), expected1);
    }
}
