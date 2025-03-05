use std::fs;

#[derive(Clone)]
struct Cell {
    distance: i32,
    is_obstacle: bool,
    is_visited: bool,
}

impl Cell {
    fn new() -> Cell {
        Cell { distance: 0, is_obstacle: false, is_visited: false }
    }
}

struct Grid {
    grid: Vec<Vec<Cell>>,
    size: i32,
    min_cheat_distance: i32,
}

impl Grid {
    fn get_cell(&self, point: &Point) -> Cell {
        self.grid[point.y as usize][point.x as usize].clone()
    }

    fn add_obstacle(&mut self, x: i32, y: i32) {
        self.grid[y as usize][x as usize].is_obstacle = true;
    }

    fn visit(&mut self, x: i32, y: i32, distance: i32) {
        self.grid[y as usize][x as usize].is_visited = true;
        self.grid[y as usize][x as usize].distance = distance;
    }
}

#[derive(Clone, Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn part1(input: &String, min_cheat_distance: i32) -> String {
    let mut start = Point { x: 0, y: 0 };
    let mut grid = create_grid(input, &mut start, min_cheat_distance);
    number_grid(&mut grid, &mut Vec::new(), start.x, start.y, 0);
    format!("{}", count_cheats(&grid, start.x, start.y, 0))
}

fn create_grid(input: &String, start: &mut Point, min_cheat_distance: i32) -> Grid {
    let size = input.lines().nth(0).unwrap().len();
    let mut grid = Grid { grid: vec![vec![Cell::new(); size]; size], size: size as i32, min_cheat_distance };

    let mut y = 0;
    for line in input.lines() {
        let mut x = 0;
        for c in line.chars() {
            match c {
                '#' => grid.add_obstacle(x, y),
                'S' => {
                    start.x = x;
                    start.y = y;
                },
                _ => {}
            }
            x += 1;
        }
        y += 1;
    }

    grid
}

fn number_grid(grid: &mut Grid, race_track: &mut Vec<Point>, x: i32, y: i32, number: i32) {
    if x < 0 || y < 0 || x >= grid.size || y >= grid.size {
        return
    }

    let cell = grid.get_cell(&Point { x, y });
    if cell.is_obstacle || cell.is_visited {
        return
    }

    grid.visit(x, y, number);
    race_track.push(Point { x, y });

    number_grid(grid, race_track, x + 1, y, number + 1);
    number_grid(grid, race_track, x - 1, y, number + 1);
    number_grid(grid, race_track, x, y + 1, number + 1);
    number_grid(grid, race_track, x, y - 1, number + 1);
}

fn count_cheats(grid: &Grid, x: i32, y: i32, number: i32) -> i32 {
    if x < 0 || y < 0 || x >= grid.size || y >= grid.size {
        return 0
    }

    let cell = grid.get_cell(&Point { x, y });
    if cell.is_obstacle || cell.distance != number {
        return 0
    }

    let mut cheats = 0;
    cheats += cheat_value(grid, x + 2, y, cell.distance);
    cheats += cheat_value(grid, x - 2, y, cell.distance);
    cheats += cheat_value(grid, x, y + 2, cell.distance);
    cheats += cheat_value(grid, x, y - 2, cell.distance);

    cheats += count_cheats(grid, x + 1, y, number + 1);
    cheats += count_cheats(grid, x - 1, y, number + 1);
    cheats += count_cheats(grid, x, y + 1, number + 1);
    cheats += count_cheats(grid, x, y - 1, number + 1);

    cheats
}

fn cheat_value(grid: &Grid, x: i32, y: i32, cheat_start_distance: i32) -> i32 {
    if x < 0 || y < 0 || x >= grid.size || y >= grid.size {
        return 0
    }

    let cell = grid.get_cell(&Point { x, y });
    if cell.is_obstacle {
        return 0
    }

    if cell.distance >= cheat_start_distance + 2 + grid.min_cheat_distance {
        1
    } else {
        0
    }
}

fn part2(input: &String, min_cheat_distance: i32) -> String {
    let mut start = Point { x: 0, y: 0 };
    let mut grid = create_grid(input, &mut start, min_cheat_distance);
    let mut race_track = Vec::new();
    number_grid(&mut grid, &mut race_track, start.x, start.y, 0);

    let mut cheats = 0;
    for start_distance in 0..race_track.len() {
        let start_point = &race_track[start_distance];
        cheats += count_cheat_paths(&grid, start_point.x, start_point.y);
    }

    format!("{}", cheats)
}

fn count_cheat_paths(grid: &Grid, x: i32, y: i32) -> i32 {
    let mut current_search;
    let mut next_search = Vec::new();
    let mut seen = Vec::new();
    let start_distance = grid.get_cell(&Point{ x, y }).distance;

    next_search.push(Point { x, y });
    seen.push(x * 1000 + y);

    let mut iteration = 0;
    let mut count = 0;

    while iteration <= 20 && next_search.len() > 0 {
        current_search = next_search.clone();
        next_search.clear();

        let min_distance = start_distance + grid.min_cheat_distance + iteration;

        for point in current_search {
            let cell = grid.get_cell(&point);
            if cell.distance >= min_distance {
                count += 1;
            }

            maybe_search(grid, point.x + 1, point.y, &mut next_search, &mut seen);
            maybe_search(grid, point.x - 1, point.y, &mut next_search, &mut seen);
            maybe_search(grid, point.x, point.y + 1, &mut next_search, &mut seen);
            maybe_search(grid, point.x, point.y - 1, &mut next_search, &mut seen);
        }

        iteration += 1;
    }

    count
}

fn maybe_search(grid: &Grid, x: i32, y: i32, search: &mut Vec<Point>, seen: &mut Vec<i32>) {
    if x < 0 || y < 0 || x >= grid.size || y >= grid.size || seen.contains(&(x * 1000 + y)) {
        return
    }

    seen.push(x * 1000 + y);
    search.push(Point { x, y });
}

fn main() {
    env_logger::init();
    let input1 = &fs::read_to_string("day20.txt").expect("Unable to read input file");
    println!("Part1: {}", part1(&input1.to_string(), 100));

    println!("Part2: {}", part2(&input1.to_string(), 100));
}

#[cfg(test)]
mod tests {
    use std::fs;
    use crate::{create_grid, count_cheat_paths, number_grid, part1, part2, Point};

    #[test]
    fn test_part1() {
        let example1 = r#"###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############
"#;
        let expected1 = "4".to_string();

        assert_eq!(part1(&example1.to_string(), 36), expected1);
    }

    #[test]
    fn test_cheat_path() {
        let input1 = &fs::read_to_string("day20.txt").expect("Unable to read input file");

        let mut start = Point { x: 0, y: 0 };
        let mut grid = create_grid(input1, &mut start, 50);
        number_grid(&mut grid, &mut Vec::new(), start.x, start.y, 0);

        assert_eq!(count_cheat_paths(&grid, 77, 129), 1);
    }

    #[test]
    fn test_part2() {
        let example1 = r#"###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############
"#;
        let expected = 32 + 31 + 29 + 39 + 25 + 23 + 20 + 19 + 12 + 14 + 12 + 22 + 4 + 3;

        assert_eq!(part2(&example1.to_string(), 50), expected.to_string());
    }

    #[test]
    fn test_part2_max() {
        let example1 = r#"###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############
"#;
        assert_eq!(part2(&example1.to_string(), 76), 3.to_string());
    }
}
