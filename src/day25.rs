use std::fs;

fn main() {
    let input = &fs::read_to_string("day25.txt").expect("Unable to read input file");
    println!("part1: {}", part1(input));
}

struct Schematics {
    keys: Vec<Key>,
    locks: Vec<Lock>
}

struct Key {
    heights: [i32; 5]
}

struct Lock {
    heights: [i32; 5]
}

fn part1(input: &String) -> String {
    let schematics = get_schematics(input);
    let mut victory_count = 0;
    for lock in &schematics.locks {
        for key in schematics.keys.iter() {
            let mut the_key_fits = true;
            for i in 0..5 {
                if lock.heights[i] + key.heights[i] > 5 {
                    the_key_fits = false;
                }
            }
            if the_key_fits {
                victory_count += 1;
            }
        }
    }
    format!("{}", victory_count)
}

fn get_schematics(input: &String) -> Schematics {
    let mut schematics = Schematics { keys: vec![], locks: vec![] };
    let mut lines = input.lines();
    loop {
        if lines.next().unwrap().starts_with("#") {
            let mut lock = Lock { heights: [0; 5] };
            for _row in 0..5 {
                let mut col = 0;
                for c in lines.next().unwrap().chars() {
                    if c == '#' {
                        lock.heights[col] += 1;
                    }
                    col += 1;
                }
            }
            schematics.locks.push(lock);
        } else {
            let mut key = Key { heights: [5; 5] };
            for _row in 0..5 {
                let mut col = 0;
                for c in lines.next().unwrap().chars() {
                    if c == '.' {
                        key.heights[col] -= 1;
                    }
                    col += 1;
                }
            }
            schematics.keys.push(key);
        }
        lines.next(); // Last row of schematic
        let line = lines.next(); // New line or EOF
        if line.is_none() {
            break;
        }
    }
    schematics
}

#[cfg(test)]
mod tests {
    use std::fs;
    use crate::part1;

    #[test]
    fn test_part1() {
        let input = &fs::read_to_string("day25_test.txt").expect("Unable to read input file");
        assert_eq!(part1(input), "3");
    }
}
