use std::collections::HashSet;

fn rotate(rows: &Vec<String>) -> Vec<String> {
    let mut cols: Vec<String> = Vec::new();

    for char in rows[0].chars() {
        let mut col = String::new();
        col.push(char);
        cols.push(col);
    }

    for row in 1..rows.len() {
        let mut col = 0;
        for char in rows[row].chars() {
            cols[col].push(char);
            col += 1;
        }
    }

    cols
}

fn reverse_find(room: &String, guard_index: usize) -> i32 {
    let mut i: i32 = guard_index as i32 - 1;
    let mut iter = room.chars().rev().skip(room.len() - guard_index);
    loop {
        match iter.next() {
            None => {
                break;
            }
            Some(area) => {
                if area == '#' {
                    break;
                }
            }
        }
        i -= 1;
    }
    i
}

fn find(room: &String, guard_index: usize ) -> usize {
    match room[guard_index..].find('#') {
        None => room.len(),
        Some(obstacle_index) => obstacle_index + guard_index
    }
}

struct State {
    row: i32,
    col: i32
}

struct Grid {
    rows: Vec<String>,
    cols: Vec<String>
}

#[derive(PartialEq, Eq)]
enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT
}

fn walk_out(state: &mut State, rows: &Vec<String>, cols: &Vec<String>, found: &mut HashSet<i32>, direction: Direction) -> bool {
    match direction {
        Direction::UP => {
            let obstacle_index = reverse_find(&cols[state.col as usize], state.row as usize);
            for i in obstacle_index + 1..state.row {
                found.insert(1000 * i + state.col);
                if found.contains(&5007) {
                    println!("UP anomaly starting from ({}, {})", state.row, state.col);
                }
            }
            if obstacle_index == -1 {
                return true
            }
            state.row = obstacle_index + 1;
        }
        Direction::RIGHT => {
            let obstacle_index = find(&rows[state.row as usize], state.col as usize) as i32;
            for i in state.col + 1..obstacle_index {
                found.insert(1000 * state.row + i);
                if found.contains(&5007) {
                    println!("RIGHT anomaly starting from ({}, {})", state.row, state.col);
                }
            }
            if obstacle_index as usize == rows.len() {
                return true
            }
            state.col = obstacle_index - 1;
        }
        Direction::DOWN => {
            let obstacle_index = find(&cols[state.col as usize], state.row as usize) as i32;
            for i in state.row + 1..obstacle_index {
                found.insert(1000 * i + state.col);
                if found.contains(&5007) {
                    println!("DOWN anomaly starting from ({}, {})", state.row, state.col);
                }
            }
            if obstacle_index as usize == cols.len() {
                return true
            }
            state.row = obstacle_index - 1;
        }
        Direction::LEFT => {
            let obstacle_index = reverse_find(&rows[state.row as usize], state.col as usize);
            for i in obstacle_index + 1..state.col {
                found.insert(1000 * state.row + i);
                if found.contains(&5007) {
                    println!("LEFT anomaly starting from ({}, {})", state.row, state.col);
                }
            }
            if obstacle_index == -1 {
                return true
            }
            state.col = obstacle_index + 1;
        }
    }

    false
}

fn part1(input: String) -> String {
    format!("{}", find_path(&get_grid(input)).len())
}

fn find_path(grid: &Grid) -> HashSet<i32> {
    let mut found: HashSet<i32> = HashSet::new();

    let mut state = get_starting_position(&grid.rows);

    println!("Start position is ({}, {})", state.row, state.col);
    found.insert(1000 * state.row + state.col);

    loop {
        if walk_out(&mut state, &grid.rows, &grid.cols, &mut found, Direction::UP) {
            break;
        }

        if walk_out(&mut state, &grid.rows, &grid.cols, &mut found, Direction::RIGHT) {
            break;
        }

        if walk_out(&mut state, &grid.rows, &grid.cols, &mut found, Direction::DOWN) {
            break;
        }

        if walk_out(&mut state, &grid.rows, &grid.cols, &mut found, Direction::LEFT) {
            break;
        }
    }
    found
}

fn get_grid(input: String) -> Grid {
    let mut grid = Grid { rows: vec![], cols: vec![] };
    grid.rows = input.lines().map(|s| s.to_string()).collect();
    grid.cols = rotate(&grid.rows);
    grid
}

fn get_starting_position(rows: &Vec<String>) -> State {
    let mut state = State { row: 0, col: 0 };
    for row in rows {
        match row.find("^") {
            None => {
                state.row += 1;
            }
            Some(index) => {
                state.col = index as i32;
                break;
            }
        }
    }
    state
}

fn part2(input: String) -> String {
    let grid = get_grid(input);
    let start = get_starting_position(&grid.rows);

    let mut loop_count = 0;
    let mut found: HashSet<i32> = HashSet::new();

    for path_element in find_path(&grid) {
        let obstacle_row = path_element / 1000;
        let obstacle_col = path_element % 1000;
        if test_loop(&grid.rows, &grid.cols, start.row, start.col, obstacle_row, obstacle_col) {
            found.insert(path_element);
            loop_count += 1;
        }
    }

    format!("{}", found.len())
}

fn test_loop(original_rows: &Vec<String>, original_cols: &Vec<String>, start_row: i32, start_col: i32, obstacle_row: i32, obstacle_col: i32) -> bool {
    let rows = add_obstacle(original_rows, obstacle_row, obstacle_col);
    let cols = add_obstacle(original_cols, obstacle_col, obstacle_row);
    let mut row = start_row;
    let mut col = start_col;
    let mut found: HashSet<i32> = HashSet::new();

    loop {
        // UP
        let position = row * 1000 + col;
        if found.contains(&position) {
            return true
        }
        found.insert(position);
        let obstacle_index = reverse_find(&cols[col as usize], row as usize);
        if obstacle_index == -1 {
            break;
        }
        row = obstacle_index + 1;

        // RIGHT
        let position = 1000000 + row * 1000 + col;
        if found.contains(&position) {
            return true
        }
        found.insert(position);
        let obstacle_index = find(&rows[row as usize], col as usize) as i32;
        if obstacle_index as usize == rows.len() {
            break;
        }
        col = obstacle_index - 1;

        // DOWN
        let position = 2000000 + row * 1000 + col;
        if found.contains(&position) {
            return true
        }
        found.insert(position);
        let obstacle_index = find(&cols[col as usize], row as usize) as i32;
        if obstacle_index as usize == cols.len() {
            break;
        }
        row = obstacle_index - 1;

        // LEFT
        let position = 3000000 + row * 1000 + col;
        if found.contains(&position) {
            return true
        }
        found.insert(position);
        let obstacle_index = reverse_find(&rows[row as usize], col as usize);
        if obstacle_index == -1 {
            break;
        }
        col = obstacle_index + 1;

        // for row in 0..rows.len() {
        //     for col in 0..cols.len() {
        //         if found.contains(&((1000 * row + col) as i32)) {
        //             print!("X");
        //         } else {
        //             print!("{}", &rows[row].chars().nth(col).unwrap());
        //         }
        //     }
        //     println!();
        // }
        //
        // let mut s = String::new();
        // stdin().read_line(&mut s);
    }

    false
}

fn add_obstacle(original_map: &Vec<String>, map_index: i32, string_index: i32) -> Vec<String> {
    let mut map: Vec<String> = Vec::new();

    let mut i: i32 = 0;
    loop {
        if i == original_map.len() as i32 {
            return map
        }
        if i == map_index {
            let mut j: i32 = 0;
            let mut original_chars = original_map[i as usize].chars();
            let mut line = String::new();
            loop {
                if j == original_map.len() as i32 {
                    break;
                }
                let next_char = original_chars.next().unwrap();
                line.push(if j == string_index { '#' } else {next_char});
                j += 1;
            }
            map.push(line);
        } else {
            map.push(original_map[i as usize].clone());
        }
        i += 1;
    }
}

fn main() {
    env_logger::init();
    let input1 = std::fs::read_to_string("day6.txt").expect("Unable to read input file");
    println!("Part1: {}", part1(input1));

    let input2 = std::fs::read_to_string("day6.txt").expect("Unable to read input file");
    println!("Part2: {}", part2(input2));
}


#[cfg(test)]
mod tests {
    use crate::part1;
    use crate::part2;

    #[test]
    fn test_part1() {
        let example1 = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;
        let expected1 = "41".to_string();

        assert_eq!(part1(example1.to_string()), expected1);
    }

    #[test]
    fn test_part1_real() {
        let input1 = std::fs::read_to_string("day6.txt").expect("Unable to read input file");
        let expected1 = "4580".to_string();

        assert_eq!(part1(input1), expected1);
    }

    #[test]
    fn test_part2() {
        let example1 = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;
        let expected1 = "6".to_string();

        assert_eq!(part2(example1.to_string()), expected1);
    }
}
