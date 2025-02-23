use std::collections::VecDeque;
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

struct Point {
    x: i32,
    y: i32,
}

fn chart_distance(x: i32, y: i32, grid: &mut Grid, queue: &mut VecDeque<Point>, distance: i32) {
    if x < 0 || y < 0 || x >= grid.size || y >= grid.size {
        return
    }

    let cell = grid.get_cell(&Point{x, y});
    if cell.is_obstacle || (cell.is_visited && cell.distance <= distance) {
        return
    }

    grid.visit(x, y, distance);
    queue.push_back(Point{x, y});
}

fn traverse(grid: &mut Grid) {
    let queue = &mut VecDeque::new();
    queue.push_back(Point{x:0, y:0});
    while !queue.is_empty() {
        let point = queue.pop_front().unwrap();
        let cell = grid.get_cell(&point);
        chart_distance(point.x + 1, point.y, grid, queue, cell.distance + 1);
        chart_distance(point.x - 1, point.y, grid, queue, cell.distance + 1);
        chart_distance(point.x, point.y + 1, grid, queue, cell.distance + 1);
        chart_distance(point.x, point.y - 1, grid, queue, cell.distance + 1);
        chart_distance(point.x, point.y - 1, grid, queue, cell.distance + 1);
    }
}

fn part1(input: &String, time: i32, size: i32) -> String {
    let mut grid = Grid{grid: vec![vec![Cell::new(); size as usize]; size as usize], size};
    let mut byte = 0;
    for line in input.lines() {
        if byte == time {
            break;
        }

        let (x, y) = line.split_once(",").unwrap();
        grid.add_obstacle(x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap());

        byte += 1;
    }
    traverse(&mut grid);
    format!("{}", grid.get_cell(&Point{x: size - 1, y: size - 1}).distance)
}

fn part2(input: &String, mut time: i32, size: i32) -> String {
    loop {
        time = time + 1;
        if part1(input, time, size) == "0" {
            return input.lines().nth((time - 1) as usize).unwrap().to_string()
        }
    }
}

fn main() {
    env_logger::init();
    let input1 = &fs::read_to_string("day18.txt").expect("Unable to read input file");
    println!("Part1: {}", part1(&input1.to_string(), 1024, 71));
    println!("Part2: {}", part2(&input1.to_string(), 1024, 71));
}

#[cfg(test)]
mod tests {
    use crate::part1;
    use crate::part2;

    #[test]
    fn test_part1() {
        let example1 = r#"5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0"#;
        let expected1 = "22".to_string();

        assert_eq!(part1(&example1.to_string(), 12, 7), expected1);
    }

    #[test]
    fn test_part2() {
        let example1 = r#"5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0"#;
        let expected1 = "6,1".to_string();

        assert_eq!(part2(&example1.to_string(), 12, 7), expected1);
    }
}
