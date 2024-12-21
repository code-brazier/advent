use std::collections::HashMap;
use std::collections::HashSet;

struct Location {
    row: i32,
    col: i32
}

fn get_frequencies(input: &String) -> HashMap<char, Vec<Location>> {
    let mut frequencies: HashMap<char, Vec<Location>> = HashMap::new();
    let mut row: i32 = 0;
    for line in input.lines() {
        let mut col: i32 = 0;
        for frequency in line.chars() {
            if frequency != '.' {
                frequencies.entry(frequency).or_insert(Vec::new()).push(Location { row, col });
            }
            col += 1;
        }
        row += 1;
    }
    frequencies
}

fn get_antinode(f1: &Location, f2: &Location, max: i32) -> Option<i32> {
    let antinode_row = f1.row - (f2.row - f1.row);
    if antinode_row < 0 || antinode_row >= max {
        return None
    }

    let antinode_col = f1.col - (f2.col - f1.col);
    if antinode_col < 0 || antinode_col >= max {
        return None
    }

    Some(antinode_row * 100 + antinode_col)
}

fn get_antinodes(frequencies: HashMap<char, Vec<Location>>, max: i32) -> HashSet<i32> {
    let mut antinodes: HashSet<i32> = HashSet::new();
    for locations in frequencies.values() {
        for f1 in 0..locations.len() {
            for f2 in f1 + 1..locations.len() {
                get_antinode(&locations[f1], &locations[f2], max).inspect(|&antinode| { antinodes.insert(antinode); });
                get_antinode(&locations[f2], &locations[f1], max).inspect(|&antinode| { antinodes.insert(antinode); });
            }
        }
    }
    antinodes
}

fn add_harmonic_antinodes_for_frequency(f1: &Location, f2: &Location, max: i32, antinodes: &mut HashSet<i32>) {
    let row_delta = f2.row - f1.row;
    let col_delta = f2.col - f1.col;
    let mut harmonic = 1;

    loop {
        let antinode_row = f1.row - row_delta * harmonic;
        if antinode_row < 0 || antinode_row >= max {
            break;
        }

        let antinode_col = f1.col - col_delta * harmonic;
        if antinode_col < 0 || antinode_col >= max {
            break;
        }

        antinodes.insert(antinode_row * 100 + antinode_col);

        harmonic += 1;
    }
}

fn get_harmonic_antinodes(frequencies: HashMap<char, Vec<Location>>, max: i32) -> HashSet<i32> {
    let mut antinodes: HashSet<i32> = HashSet::new();
    for locations in frequencies.values() {
        for f1 in 0..locations.len() {
            antinodes.insert(locations[f1].row * 100 + locations[f1].col);
            for f2 in f1 + 1..locations.len() {
                add_harmonic_antinodes_for_frequency(&locations[f1], &locations[f2], max, &mut antinodes);
                add_harmonic_antinodes_for_frequency(&locations[f2], &locations[f1], max, &mut antinodes);
            }
        }
    }
    antinodes
}

fn part1(input: &String) -> String {
    format!("{}", get_antinodes(get_frequencies(input), input.lines().count() as i32).len())
}

fn part2(input: &String) -> String {
    format!("{}", get_harmonic_antinodes(get_frequencies(input), input.lines().count() as i32).len())
}

fn main() {
    env_logger::init();
    let input1 = &std::fs::read_to_string("day8.txt").expect("Unable to read input file");
    println!("Part1: {}", part1(input1));

    println!("Part2: {}", part2(input1));
}


#[cfg(test)]
mod tests {
    use crate::part1;
    use crate::part2;

    #[test]
    fn test_part1() {
        let example1 = r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"#;
        let expected1 = "14".to_string();

        assert_eq!(part1(&example1.to_string()), expected1);
    }

    #[test]
    fn test_part2() {
        let example1 = r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"#;
        let expected1 = "34".to_string();

        assert_eq!(part2(&example1.to_string()), expected1);
    }
}
