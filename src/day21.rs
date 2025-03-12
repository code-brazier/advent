use std::collections::HashMap;
use std::fs;

#[derive(Clone)]
struct Key {
    label: char,
    row: i32,
    col: i32,
}

impl Key {
    fn new(label: char, row: i32, col: i32) -> Self {
        Key { label, row, col }
    }
}

trait Keyboard {
    fn press(&mut self, movement: char) -> String;
    fn derive_movements(&mut self, keys: &String, arrow_move_counts: &mut HashMap<&str, ArrowMove>);
}

impl BasicKeyboard {
    fn new() -> Self {
        let mut keys = Vec::new();
        keys.push(Key::new('7', 0, 0));
        keys.push(Key::new('8', 0, 1));
        keys.push(Key::new('9', 0, 2));
        keys.push(Key::new('4', 1, 0));
        keys.push(Key::new('5', 1, 1));
        keys.push(Key::new('6', 1, 2));
        keys.push(Key::new('1', 2, 0));
        keys.push(Key::new('2', 2, 1));
        keys.push(Key::new('3', 2, 2));
        keys.push(Key::new('0', 3, 1));
        keys.push(Key::new('A', 3, 2));

        BasicKeyboard { selected_key: 10, keys, dead_cell_row: 3 }
    }

    fn select_key_at(&mut self, row: i32, col: i32) {
        let mut key_index = 0;
        for key in self.keys.iter() {
            if key.row == row && key.col == col {
                self.selected_key = key_index;
                return
            }
            key_index += 1;
        }
        panic!("Key not found at location ({}, {})", col, row);
    }

    fn get_key(&self, label: char) -> Key {
        for key in self.keys.iter() {
            if key.label == label {
                return key.clone()
            }
        }
        panic!("Key not found with label {}", label);
    }
}

struct BasicKeyboard {
    selected_key: usize,
    keys: Vec<Key>,
    dead_cell_row: i32,
}

impl Keyboard for BasicKeyboard {
    fn press(&mut self, movement: char) -> String {
        let mut output = "".to_owned();
        let key = &self.keys[self.selected_key];
        match movement {
            '<' => {
                self.select_key_at(key.row, key.col - 1)
            }
            '>' => {
                self.select_key_at(key.row, key.col + 1)
            }
            '^' => {
                self.select_key_at(key.row - 1, key.col)
            }
            'v' => {
                self.select_key_at(key.row + 1, key.col)
            }
            'A' => {
                output.push_str(key.label.to_string().as_str());
            }
            _ => {}
        }
        output
    }

    fn derive_movements(&mut self, keys: &String, arrow_move_counts: &mut HashMap<&str, ArrowMove>) {
        let mut output = String::new();
        output.push('A');
        for label in keys.chars() {
            let key_from = &self.keys[self.selected_key];
            let key_to = self.get_key(label);
            let mut movements = String::new();
            if key_to.col == 0 && key_from.row == self.dead_cell_row {
                for _ in 0..key_from.row.abs_diff(key_to.row) {
                    movements.push(if key_from.row > key_to.row { '^' } else { 'v' });
                }
                for _ in 0..key_from.col.abs_diff(key_to.col) {
                    movements.push(if key_from.col > key_to.col { '<' } else { '>' });
                }
            } else if (key_from.col == 0 && key_to.row == self.dead_cell_row) || key_to.col < key_from.col {
                for _ in 0..key_from.col.abs_diff(key_to.col) {
                    movements.push(if key_from.col > key_to.col { '<' } else { '>' });
                }
                for _ in 0..key_from.row.abs_diff(key_to.row) {
                    movements.push(if key_from.row > key_to.row { '^' } else { 'v' });
                }
            } else {
                for _ in 0..key_from.row.abs_diff(key_to.row) {
                    movements.push(if key_from.row > key_to.row { '^' } else { 'v' });
                }
                for _ in 0..key_from.col.abs_diff(key_to.col) {
                    movements.push(if key_from.col > key_to.col { '<' } else { '>' });
                }
            }
            movements.push('A');

            self.select_key_at(key_to.row, key_to.col);
            output.push_str(movements.as_str());
        }
        for i in 1..output.len() {
            let key_move = format!("{}{}", output.chars().nth(i - 1).unwrap(), output.chars().nth(i).unwrap());
            arrow_move_counts.get_mut(key_move.as_str()).unwrap().count += 1;
        }
    }
}

struct DelegatingKeyboard {
    keyboard: BasicKeyboard,
    delegate: Box<dyn Keyboard>,
}

impl DelegatingKeyboard {
    fn arrow_keyboard(delegate: Box<dyn Keyboard>) -> Self {
        let mut keys = Vec::new();
        keys.push(Key::new('^', 0, 1));
        keys.push(Key::new('A', 0, 2));
        keys.push(Key::new('<', 1, 0));
        keys.push(Key::new('v', 1, 1));
        keys.push(Key::new('>', 1, 2));

        DelegatingKeyboard { keyboard: BasicKeyboard { selected_key: 1, keys, dead_cell_row: 0 }, delegate }
    }
}

impl Keyboard for DelegatingKeyboard {
    fn press(&mut self, movement: char) -> String {
        match self.keyboard.press(movement).as_str() {
            "" => "".to_string(),
            key => self.delegate.press(key.chars().nth(0).unwrap())
        }
    }

    fn derive_movements(&mut self, keys: &String, arrow_move_counts: &mut HashMap<&str, ArrowMove>) {
        self.delegate.derive_movements(keys, arrow_move_counts);
        let mut new_counts: HashMap<&str, i64> = HashMap::new();
        arrow_move_counts.iter().for_each(|(arrow_move, _)| { new_counts.insert(arrow_move, 0); });

        arrow_move_counts.iter().for_each(|(_, arrow_move)| {
            if arrow_move.count == 0 {
                return
            }
            for i in 1..arrow_move.moves.len() {
                let key_move = format!("{}{}", arrow_move.moves.chars().nth(i - 1).unwrap(), arrow_move.moves.chars().nth(i).unwrap());
                *new_counts.get_mut(key_move.as_str()).unwrap() += arrow_move.count;
            }
        });

        new_counts.iter().for_each(|(arrow_move, &count)| {
            arrow_move_counts.get_mut(arrow_move).unwrap().count = count;
        });
    }
}

fn create_keyboard_chain(keyboard_count: i32) -> Box<dyn Keyboard> {
    if keyboard_count == 0 {
        Box::new(BasicKeyboard::new())
    } else {
        Box::new(DelegatingKeyboard::arrow_keyboard(create_keyboard_chain(keyboard_count - 1)))
    }
}

struct ArrowMove {
    count: i64,
    moves: String,
}

fn find_shortest(code: &String, directional_robot_count: i32) -> i64 {
    let mut arrow_move_counts = HashMap::new();
    arrow_move_counts.insert("A>", ArrowMove { count: 0, moves: "AvA".to_string() });
    arrow_move_counts.insert("A<", ArrowMove { count: 0, moves: "Av<<A".to_string() });
    arrow_move_counts.insert("A^", ArrowMove { count: 0, moves: "A<A".to_string() });
    arrow_move_counts.insert("Av", ArrowMove { count: 0, moves: "A<vA".to_string() });
    arrow_move_counts.insert(">A", ArrowMove { count: 0, moves: "A^A".to_string() });
    arrow_move_counts.insert("<A", ArrowMove { count: 0, moves: "A>>^A".to_string() });
    arrow_move_counts.insert("^A", ArrowMove { count: 0, moves: "A>A".to_string() });
    arrow_move_counts.insert("vA", ArrowMove { count: 0, moves: "A^>A".to_string() });
    arrow_move_counts.insert("v>", ArrowMove { count: 0, moves: "A>A".to_string() });
    arrow_move_counts.insert(">v", ArrowMove { count: 0, moves: "A<A".to_string() });
    arrow_move_counts.insert("v<", ArrowMove { count: 0, moves: "A<A".to_string() });
    arrow_move_counts.insert("<v", ArrowMove { count: 0, moves: "A>A".to_string() });
    arrow_move_counts.insert("^>", ArrowMove { count: 0, moves: "Av>A".to_string() });
    arrow_move_counts.insert(">^", ArrowMove { count: 0, moves: "A<^A".to_string() });
    arrow_move_counts.insert("^<", ArrowMove { count: 0, moves: "Av<A".to_string() });
    arrow_move_counts.insert("<^", ArrowMove { count: 0, moves: "A>^A".to_string() });
    arrow_move_counts.insert("<<", ArrowMove { count: 0, moves: "AA".to_string() });
    arrow_move_counts.insert(">>", ArrowMove { count: 0, moves: "AA".to_string() });
    arrow_move_counts.insert("^^", ArrowMove { count: 0, moves: "AA".to_string() });
    arrow_move_counts.insert("vv", ArrowMove { count: 0, moves: "AA".to_string() });
    arrow_move_counts.insert("AA", ArrowMove { count: 0, moves: "AA".to_string() });
    create_keyboard_chain(directional_robot_count).derive_movements(code, &mut arrow_move_counts);
    arrow_move_counts.iter().map(|(_, arrow_move)| arrow_move.count).sum::<i64>()
}

fn part1(input: &String) -> String {
    let mut complexity_sum = 0;

    for line in input.lines() {
        let numeric_part = line[0..3].parse::<i64>().unwrap();
        let shortest_sequence = find_shortest(&line.to_string(), 2);
        complexity_sum += numeric_part * shortest_sequence;
    }

    format!("{}", complexity_sum)
}

fn part2(input: &String) -> String {
    let mut complexity_sum = 0;

    for line in input.lines() {
        let numeric_part = line[0..3].parse::<i64>().unwrap();
        let shortest_sequence = find_shortest(&line.to_string(), 25);
        complexity_sum += numeric_part * shortest_sequence;
    }

    format!("{}", complexity_sum)
}

fn main() {
    env_logger::init();
    let input1 = &fs::read_to_string("day21.txt").expect("Unable to read input file");
    println!("Part1: {}", part1(&input1.to_string()));
    println!("Part2: {}", part2(&input1.to_string()));
}

#[cfg(test)]
mod tests {
    use crate::{part1, DelegatingKeyboard, BasicKeyboard, Keyboard, find_shortest};

    #[test]
    fn test_numeric_keyboard() {
        let mut keyboard = BasicKeyboard::new();

        let mut output = "".to_owned();
        for movement in "<A^A>^^AvvvA".chars() {
            output.push_str(keyboard.press(movement).as_str());
        }
        assert_eq!(output, "029A");

        let mut output = "".to_owned();
        for movement in "<A^A^>^AvvvA".chars() {
            output.push_str(keyboard.press(movement).as_str());
        }
        assert_eq!(output, "029A");

        let mut output = "".to_owned();
        for movement in "<A^A^^>AvvvA".chars() {
            output.push_str(keyboard.press(movement).as_str());
        }
        assert_eq!(output, "029A");
    }

    #[test]
    fn test_single_arrow_keyboard() {
        let numenor = BasicKeyboard::new();
        let mut keyboard = DelegatingKeyboard::arrow_keyboard(Box::new(numenor));

        let mut output = "".to_owned();
        for movement in "v<<A>>^A<A>AvA<^AA>A<vAAA>^A".chars() {
            output.push_str(keyboard.press(movement).as_str());
        }
        assert_eq!(output, "029A");
    }

    #[test]
    fn test_double_arrow_keyboard() {
        let numenor = BasicKeyboard::new();
        let radioactive_arrows = DelegatingKeyboard::arrow_keyboard(Box::new(numenor));
        let mut angled_arrows = DelegatingKeyboard::arrow_keyboard(Box::new(radioactive_arrows));

        let mut output = "".to_owned();
        for movement in "v<A<AA>>^AvAA^<A>Av<<A>>^AvA^Av<<A>>^AAv<A>A^A<A>Av<A<A>>^AAA<Av>A^A".chars() {
            output.push_str(angled_arrows.press(movement).as_str());
        }
        assert_eq!(output, "029A");
    }

    #[test]
    fn test_shortest() {
        assert_eq!(find_shortest(&"029A".to_owned(), 2) as usize, "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A".len());
        assert_eq!(find_shortest(&"980A".to_owned(), 2) as usize, "<v<A>>^AAAvA^A<vA<AA>>^AvAA<^A>A<v<A>A>^AAAvA<^A>A<vA>^A<A>A".len());
        assert_eq!(find_shortest(&"179A".to_owned(), 2) as usize, "<v<A>>^A<vA<A>>^AAvAA<^A>A<v<A>>^AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A".len());
        assert_eq!(find_shortest(&"456A".to_owned(), 2) as usize, "<v<A>>^AA<vA<A>>^AAvAA<^A>A<vA>^A<A>A<vA>^A<A>A<v<A>A>^AAvA<^A>A".len());
        assert_eq!(find_shortest(&"379A".to_owned(), 2) as usize, "<v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A".len());
    }

    #[test]
    fn test_part1() {
        let example1 = r#"029A
980A
179A
456A
379A"#;

        assert_eq!(part1(&example1.to_string()), "126384");
    }

    #[test]
    fn test_part1_actual() {
        let example1 = r#"169A
279A
540A
869A
789A"#;

        assert_eq!(part1(&example1.to_string()), "184716");
    }
}
