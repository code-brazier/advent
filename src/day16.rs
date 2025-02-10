use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::hash_map::Entry;
use std::collections::VecDeque;

struct Position {
    row: usize,
    col: usize
}

struct CostDirection {
    cost: i32,
    row: i32,
    col: i32,
}

struct PathNode {
    position: Position,
    inputs: Vec<CostDirection>,
    outputs: Vec<CostDirection>
}

struct NodeVector {
    node_index: usize,
    row_delta: i32,
    col_delta: i32
}

impl PathNode {
    fn new(row: usize, col: usize, row_delta: i32, col_delta: i32, cost: i32) -> PathNode {
        let mut node = PathNode { position: Position { row, col }, inputs: Vec::new(), outputs: Vec::new() };
        node.inputs.push(CostDirection { cost, row: row_delta, col: col_delta });
        node
    }

    fn add_input(&mut self, row_delta: i32, col_delta: i32, cost: i32) {
        for input in &mut self.inputs {
            if input.row == row_delta && input.col == col_delta {
                input.cost = cost;
                return
            }
        }
        self.inputs.push(CostDirection { cost, row: row_delta, col: col_delta })
    }
}

struct Graph {
    nodes: HashMap<usize, PathNode>
}

impl Graph {
    fn add(&mut self, index: usize, node: PathNode) {
        self.nodes.insert(index, node);
    }

    fn add_input(&mut self, row: usize, col: usize, row_delta: i32, col_delta: i32, cost: i32) {
        let node_index = row * 1000 + col;
        match self.nodes.entry(node_index) {
            Entry::Occupied(n) => {
                let node = n.into_mut();
                node.add_input(row_delta, col_delta, cost);
            }
            Entry::Vacant(v) => {
                let node = PathNode::new(row, col, row_delta, col_delta, cost);
                v.insert(node);
            }
        }
    }

    fn add_output(&mut self, node_index: usize, row_delta: i32, col_delta: i32) -> Option<i32> {
        let node = &mut self.nodes.get_mut(&node_index).unwrap();
        for output in &mut node.outputs {
            if output.row == row_delta && output.col == col_delta {
                let mut updated = false;
                for input in &node.inputs {
                    if input.row == row_delta && input.col == col_delta && output.cost > input.cost + 1 {
                        updated = true;
                        output.cost = input.cost + 1;
                    } else if output.cost > input.cost + 1001 {
                        updated = true;
                        output.cost = input.cost + 1001;
                    }
                }
                return if updated { Some(output.cost) } else { None }
            }
        }

        let mut cost = i32::MAX;
        for input in &node.inputs {
            if input.row == row_delta && input.col == col_delta && cost > input.cost + 1 {
                cost = input.cost + 1;
            } else if cost > input.cost + 1001 {
                cost = input.cost + 1001;
            }
        }
        node.outputs.push(CostDirection { cost, row: row_delta, col: col_delta });

        Some(cost)
    }

    fn get_inputs_for_output(&self, node_index: usize, row_delta: i32, col_delta: i32) -> Vec<NodeVector> {
        let mut output_cost = 0;
        for output in &self.nodes[&node_index].outputs {
            if output.row == row_delta && output.col == col_delta {
                output_cost = output.cost;
            }
        }
        let mut min_inputs: Vec<NodeVector> = Vec::new();
        for input in &self.nodes[&node_index].inputs {
            let is_valid_straight = input.row == row_delta && input.col == col_delta && input.cost + 1 == output_cost;
            let is_valid_turn = input.row != row_delta && input.col != col_delta && input.cost + 1001 == output_cost;
            if is_valid_straight || is_valid_turn {
                let next_node_index = (node_index as i32 / 1000 - input.row) * 1000 + node_index as i32 % 1000 - input.col;
                min_inputs.push(NodeVector { node_index: next_node_index as usize, row_delta: input.row, col_delta: input.col });
            }
        }

        min_inputs
    }

    fn get_min_cost(&self, node_index: &usize) -> i32 {
        let mut cost = i32::MAX;
        for input in &self.nodes[node_index].inputs {
            if input.cost < cost {
                cost = input.cost;
            }
        }
        cost
    }

    fn get_end_vector(&self, node_index: usize) -> NodeVector {
        let mut candidate: Option<&CostDirection> = None;
        for input in &self.nodes[&node_index].inputs {
            if candidate.is_none() || input.cost < candidate.unwrap().cost {
                candidate = Some(input);
            }
        }

        let input_with_min_cost = candidate.unwrap();
        let next_node_index = (node_index as i32 / 1000 - input_with_min_cost.row) * 1000 + node_index as i32 % 1000 - input_with_min_cost.col;
        NodeVector { node_index: next_node_index as usize, row_delta: input_with_min_cost.row, col_delta: input_with_min_cost.col }
    }

    fn get_position(&self, node_index: &usize) -> Position {
        let position = &self.nodes[node_index].position;
        Position { row: position.row, col: position.col }
    }
}

fn part1(input: &String) -> String {
    let size = input.lines().count();
    let mut walls = vec!(vec!(false; size); size);

    let mut start_x = 0;
    let mut start_y = 0;
    let mut end_x = 0;
    let mut end_y = 0;

    let mut row_index = 0;
    for line in input.lines() {
        let mut col_index = 0;
        for entity in line.chars() {
            match entity {
                '#' => { walls[row_index][col_index] = true; },
                'S' => {
                    start_x = col_index;
                    start_y = row_index;
                },
                'E' => {
                    end_x = col_index;
                    end_y = row_index;
                },
                _ => {},
            }
            col_index += 1;
        }

        row_index += 1;
    }

    let mut nodes = Graph { nodes: HashMap::new() };
    let mut alternatives = VecDeque::new();
    let start_node = PathNode::new(start_y, start_x, 0, 1, 0);
    let start_node_index = start_y * 1000 + start_x;
    alternatives.push_back(start_node_index);
    nodes.add(start_node_index, start_node);

    loop {
        match alternatives.pop_front() {
            None => {
                break;
            }
            Some(node_index) => {
                move_along_direction(&walls, &mut nodes, &mut alternatives, node_index, 0, -1);
                move_along_direction(&walls, &mut nodes, &mut alternatives, node_index, 0, 1);
                move_along_direction(&walls, &mut nodes, &mut alternatives, node_index, -1, 0);
                move_along_direction(&walls, &mut nodes, &mut alternatives, node_index, 1, 0);
            }
        }
    }

    format!("{}", nodes.get_min_cost(&(end_y * 1000 + end_x)))
}

fn move_along_direction(walls: &Vec<Vec<bool>>, nodes: &mut Graph, alternatives: &mut VecDeque<usize>, node_index: usize, direction_y: i32, direction_x: i32) {
    if node_index == 1000 + walls.len() - 2 {
        return
    }
    let position = &nodes.get_position(&node_index);
    let target_row: usize = (position.row as i32 + direction_y) as usize;
    let target_col: usize = (position.col as i32 + direction_x) as usize;
    if !walls[target_row][target_col] {
        let cost = nodes.add_output(node_index, direction_y, direction_x);
        if cost.is_some() {
            alternatives.push_back(target_row * 1000 + target_col);
            nodes.add_input(target_row, target_col, direction_y, direction_x, cost.unwrap());
        }
    }
}

fn part2(input: &String) -> String {
    let size = input.lines().count();
    let mut walls = vec!(vec!(false; size); size);

    let mut start_x = 0;
    let mut start_y = 0;
    let mut end_x = 0;
    let mut end_y = 0;

    let mut row_index = 0;
    for line in input.lines() {
        let mut col_index = 0;
        for entity in line.chars() {
            match entity {
                '#' => { walls[row_index][col_index] = true; },
                'S' => {
                    start_x = col_index;
                    start_y = row_index;
                },
                'E' => {
                    end_x = col_index;
                    end_y = row_index;
                },
                _ => {},
            }
            col_index += 1;
        }

        row_index += 1;
    }

    let mut nodes = Graph { nodes: HashMap::new() };
    let mut alternatives = VecDeque::new();
    let start_node = PathNode::new(start_y, start_x, 0, 1, 0);
    let start_node_index = start_y * 1000 + start_x;
    alternatives.push_back(start_node_index);
    nodes.add(start_node_index, start_node);

    loop {
        match alternatives.pop_front() {
            None => {
                break;
            }
            Some(node_index) => {
                move_along_direction(&walls, &mut nodes, &mut alternatives, node_index, 0, -1);
                move_along_direction(&walls, &mut nodes, &mut alternatives, node_index, 0, 1);
                move_along_direction(&walls, &mut nodes, &mut alternatives, node_index, -1, 0);
                move_along_direction(&walls, &mut nodes, &mut alternatives, node_index, 1, 0);
            }
        }
    }

    let mut hot_nodes = HashSet::new();
    let end_index = end_y * 1000 + end_x;
    hot_nodes.insert(end_index);

    let end_vector = nodes.get_end_vector(end_index);
    hot_nodes.insert(end_vector.node_index);
    work_backwards(&nodes, &mut hot_nodes, end_vector.node_index, end_vector.row_delta, end_vector.col_delta);

    format!("{}", hot_nodes.len())
}

fn work_backwards(nodes: &Graph, hot_nodes: &mut HashSet<usize>, node_index: usize, row_delta: i32, col_delta: i32) {
    for next_node in nodes.get_inputs_for_output(node_index, row_delta, col_delta) {
        if !hot_nodes.contains(&next_node.node_index) && next_node.node_index % 1000 != 0 {
            hot_nodes.insert(next_node.node_index);
            work_backwards(nodes, hot_nodes, next_node.node_index, next_node.row_delta, next_node.col_delta);
        }
    }
}

fn main() {
    env_logger::init();
    let input1 = &fs::read_to_string("day16.txt").expect("Unable to read input file");
    println!("Part1: {}", part1(input1));
    println!("Part2: {}", part2(input1));
}


#[cfg(test)]
mod tests {
    use std::fs;
    use crate::part1;
    use crate::part2;

    #[test]
    fn test_part1_small() {
        let example1 = r#"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############"#;
        let expected1 = "7036".to_string();

        assert_eq!(part1(&example1.to_string()), expected1);
    }

    #[test]
    fn test_part1_large() {
        let example1 = r#"#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################"#;
        let expected1 = "11048".to_string();

        assert_eq!(part1(&example1.to_string()), expected1);
    }

    #[test]
    fn test_part1_actual() {
        let example1 = &fs::read_to_string("day16.txt").expect("Unable to read input file");
        let expected1 = "94444".to_string();

        assert_eq!(part1(&example1.to_string()), expected1);
    }

    #[test]
    fn test_part2_small() {
        let example1 = r#"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############"#;
        let expected1 = "45".to_string();

        assert_eq!(part2(&example1.to_string()), expected1);
    }

    #[test]
    fn test_part2_large() {
        let example1 = r#"#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################"#;
        let expected1 = "64".to_string();

        assert_eq!(part2(&example1.to_string()), expected1);
    }
}
