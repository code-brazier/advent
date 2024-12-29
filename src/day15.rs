use std::cmp::PartialEq;

#[derive(PartialEq, Clone)]
enum Entity {
    Wall,
    Box,
    Space,
    BoxLeft,
    BoxRight
}

fn push(map: &mut Vec<Vec<Entity>>, map_width: i32, position_x: i32, position_y: i32, direction_x: i32, direction_y: i32, entity: Entity) -> bool {
    let destination_x = position_x + direction_x;
    let destination_y = position_y + direction_y;
    if destination_x < 0 || destination_y < 0 || destination_x > map_width || destination_y > map.len() as i32 {
        return false
    }
    let success = match map[destination_y as usize][destination_x as usize] {
        Entity::Wall => false,
        Entity::Box => {
            push(map, map_width, destination_x, destination_y, direction_x, direction_y, Entity::Box)
        },
        Entity::BoxLeft => {
            push(map, map_width, destination_x, destination_y, direction_x, direction_y, Entity::BoxLeft)
        },
        Entity::BoxRight => {
            push(map, map_width, destination_x, destination_y, direction_x, direction_y, Entity::BoxRight)
        },
        Entity::Space => {
            true
        },
    };
    if success {
        map[destination_y as usize][destination_x as usize] = entity;
    }
    success
}

fn part1(input: &String) -> String {
    let mut map: Vec<Vec<Entity>> = Vec::new();
    let mut robot_x: i32 = 0;
    let mut robot_y: i32 = 0;

    let mut row_index = 0;
    let mut lines = input.lines();
    loop {
        let mut col_index = 0;
        let line = lines.next().unwrap();
        if line.is_empty() {
            break;
        }
        let mut row: Vec<Entity> = Vec::new();
        for entity in line.chars() {
            row.push(match entity {
                '#' => Entity::Wall,
                'O' => Entity::Box,
                '@' => {
                    robot_x = col_index;
                    robot_y = row_index;
                    Entity::Space
                },
                _ => Entity::Space,
            });
            col_index += 1;
        }
        map.push(row);

        row_index += 1;
    }
    let size = map.len();

    for instruction in lines.collect::<String>().chars() {
        let mut direction_x: i32 = 0;
        let mut direction_y: i32 = 0;
        match instruction {
            '<' => { direction_x = -1; }
            '>' => { direction_x = 1; }
            '^' => { direction_y = -1; }
            'v' => { direction_y = 1; }
            _ => {}
        }
        if push(&mut map, size as i32, robot_x, robot_y, direction_x, direction_y, Entity::Space) {
            robot_x += direction_x;
            robot_y += direction_y;
        }
    }

    let mut coordinate_sum = 0;
    for height in 0..size {
        for width in 0..size {
            if map[height][width] == Entity::Box {
                coordinate_sum += height * 100 + width;
            }
        }
    }

    format!("{}", coordinate_sum)
}

fn push_wide(map: &mut Vec<Vec<Entity>>, position_x: i32, position_y: i32, direction_y: i32, entity: Entity, dry_run: bool) -> bool {
    let destination_y = position_y + direction_y;
    if destination_y < 0 || destination_y > map.len() as i32 {
        return false
    }
    let success = match map[destination_y as usize][position_x as usize] {
        Entity::Wall => false,
        Entity::BoxLeft => {
            push_wide(map, position_x, destination_y, direction_y, Entity::BoxLeft, dry_run) && push_wide(map, position_x + 1, destination_y, direction_y, Entity::BoxRight, dry_run)
        },
        Entity::BoxRight => {
            push_wide(map, position_x, destination_y, direction_y, Entity::BoxRight, dry_run) && push_wide(map, position_x - 1, destination_y, direction_y, Entity::BoxLeft, dry_run)
        }
        Entity::Space => {
            true
        },
        _ => false,
    };
    if success && !dry_run {
        map[destination_y as usize][position_x as usize] = entity;
        match map[destination_y as usize][position_x as usize] {
            Entity::BoxLeft => {
                map[position_y as usize][position_x as usize] = Entity::Space;
                map[position_y as usize][position_x as usize + 1] = Entity::Space;
            },
            Entity::BoxRight => {
                map[position_y as usize][position_x as usize] = Entity::Space;
                map[position_y as usize][position_x as usize - 1] = Entity::Space;
            }
            _ => {},
        }

    }
    success
}

fn part2(input: &String) -> String {
    let mut map: Vec<Vec<Entity>> = Vec::new();
    let mut robot_x: i32 = 0;
    let mut robot_y: i32 = 0;

    let mut row_index = 0;
    let mut lines = input.lines();
    loop {
        let mut col_index = 0;
        let line = lines.next().unwrap();
        if line.is_empty() {
            break;
        }
        let mut row: Vec<Entity> = Vec::new();
        for entity in line.chars() {
            match entity {
                '#' => {
                    row.push(Entity::Wall);
                    row.push(Entity::Wall);
                },
                'O' => {
                    row.push(Entity::BoxLeft);
                    row.push(Entity::BoxRight);
                },
                '@' => {
                    robot_x = col_index * 2;
                    robot_y = row_index;
                    row.push(Entity::Space);
                    row.push(Entity::Space);
                },
                _ => {
                    row.push(Entity::Space);
                    row.push(Entity::Space);
                },
            }
            col_index += 1;
        }
        map.push(row);

        row_index += 1;
    }
    let size = map.len();

    for instruction in lines.collect::<String>().chars() {
        let mut direction_x: i32 = 0;
        let mut direction_y: i32 = 0;
        match instruction {
            '<' => { direction_x = -1; }
            '>' => { direction_x = 1; }
            '^' => { direction_y = -1; }
            'v' => { direction_y = 1; }
            _ => {}
        }
        if direction_y == 0 {
            if push(&mut map, size as i32 * 2, robot_x, robot_y, direction_x, direction_y, Entity::Space) {
                robot_x += direction_x;
            }
        } else {
            if push_wide(&mut map, robot_x, robot_y, direction_y, Entity::Space, true) {
                push_wide(&mut map, robot_x, robot_y, direction_y, Entity::Space, false);
                robot_y += direction_y;
            }
        }
    }

    let mut coordinate_sum = 0;
    for row in 0..size {
        for col in 0..size * 2 {
            if map[row][col] == Entity::BoxLeft {
                // let height = usize::min(row, size - row - 1);
                let height = row;
                let width = col;
                coordinate_sum += height * 100 + width;
            }
        }
    }

    format!("{}", coordinate_sum)
}

fn main() {
    env_logger::init();
    let input1 = &std::fs::read_to_string("day15.txt").expect("Unable to read input file");
    println!("Part1: {}", part1(input1));
    println!("Part2: {}", part2(input1));
}


#[cfg(test)]
mod tests {
    use crate::part1;
    use crate::part2;

    #[test]
    fn test_part1_small() {
        let example1 = r#"########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<"#;
        let expected1 = "2028".to_string();

        assert_eq!(part1(&example1.to_string()), expected1);
    }

    #[test]
    fn test_part1_large() {
        let example1 = r#"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^"#;
        let expected1 = "10092".to_string();

        assert_eq!(part1(&example1.to_string()), expected1);
    }

    #[test]
    fn test_part2_small() {
        let example1 = r#"#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^"#;

        part2(&example1.to_string());
    }

    #[test]
    fn test_part2_large() {
        let example1 = r#"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^"#;
        let expected1 = "9021".to_string();

        assert_eq!(part2(&example1.to_string()), expected1);
    }
}
