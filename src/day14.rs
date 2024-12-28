use std::io::stdin;

struct Robot {
    position_x: i32,
    position_y: i32,
    velocity_x: i32,
    velocity_y: i32,
}

fn parse_velocity(line: &str, position_x: i32, position_y: i32) -> Robot {
    let comma = line.find(',').unwrap();
    let velocity_x = line[..comma].parse::<i32>().unwrap();
    let velocity_y = line[comma + 1..].parse::<i32>().unwrap();
    Robot { position_x, position_y, velocity_x, velocity_y }
}

fn parse_position(line: &str) -> Robot {
    let comma = line.find(',').unwrap();
    let space = line.find(' ').unwrap();
    let position_x = line[2..comma].parse::<i32>().unwrap();
    let position_y = line[comma + 1..space].parse::<i32>().unwrap();
    parse_velocity(&line[space + 3..], position_x, position_y)
}

fn calculate_final_position(initial_position: i32, velocity: i32, size: i32) -> i32 {
    let remainder = (initial_position + velocity * 100) % size;
    if remainder.is_negative() { remainder + size } else { remainder }
}

fn part1(input: &String, width: i32, height: i32) -> String {
    let mut safety_north_west = 0;
    let mut safety_north_east = 0;
    let mut safety_south_west = 0;
    let mut safety_south_east = 0;
    for line in input.lines() {
        let robot = parse_position(line);
        let final_x = calculate_final_position(robot.position_x, robot.velocity_x, width);
        let final_y = calculate_final_position(robot.position_y, robot.velocity_y, height);
        if final_x == width / 2 || final_y == height / 2 {
            continue;
        }
        if final_x < width / 2 {
            if final_y < height / 2 {
                safety_north_west += 1;
            } else {
                safety_south_west += 1;
            }
        } else {
            if final_y < height / 2 {
                safety_north_east += 1;
            } else {
                safety_south_east += 1;
            }
        }
    }

    format!("{}", safety_north_west * safety_north_east * safety_south_west * safety_south_east)
}

fn part2(input: &String, width: i32, height: i32) {
    let mut robots: Vec<Robot> = Vec::new();
    for line in input.lines() {
        robots.push(parse_position(line));
    }

    for seconds in 1..i32::MAX {
        let is_candidate = (seconds - 14) % 101 == 0;

        let mut display = Vec::new();
        if is_candidate {
            display = vec!(vec!(' '; width as usize); height as usize);
        }

        for robot in &mut robots {
            robot.position_x += robot.velocity_x;
            if robot.position_x < 0 {
                robot.position_x += width;
            } else if robot.position_x >= width {
                robot.position_x -= width;
            }
            robot.position_y += robot.velocity_y;
            if robot.position_y < 0 {
                robot.position_y += height;
            } else if robot.position_y >= height {
                robot.position_y -= height;
            }

            if is_candidate {
                display[robot.position_y as usize][robot.position_x as usize] = '#';
            }
        }

        if is_candidate {
            println!("\n{} seconds\n", seconds);
            for line in display.iter() {
                println!("{}", line.iter().collect::<String>())
            }

            let mut s = String::new();
            stdin().read_line(&mut s).expect("Did not enter a correct string");
        }
    }
}

fn main() {
    env_logger::init();
    let input1 = &std::fs::read_to_string("day14.txt").expect("Unable to read input file");
    println!("Part1: {}", part1(input1, 101, 103));

    part2(input1, 101, 103);
}


#[cfg(test)]
mod tests {
    use crate::part1;
    use crate::part2;

    #[test]
    fn test_part1() {
        let example1 = r#"p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3"#;
        let expected1 = "12".to_string();

        assert_eq!(part1(&example1.to_string(), 11, 7), expected1);
    }

    #[test]
    fn test_part2() {
        let example1 = r#"p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3"#;

        part2(&example1.to_string(), 11, 7);
    }
}
