use std::cmp::max;
use std::collections::{HashMap, HashSet};

fn char_to_int(c: char) -> i64 {
    c as i64 - 0x30
}

fn part1(input: &String) -> String {
    let mut forward_iter = input.chars();
    let mut backward_iter = input.chars().rev();
    let mut forward_pos = 0;
    let mut backward_pos: i64 = input.len() as i64 - 1;
    let mut checksum = 0;
    let mut disk_pos = 0;
    let mut size_of_last_file = 0;
    
    loop {
        let file_size = forward_iter.next().map(char_to_int).unwrap();
        checksum += (disk_pos..disk_pos + file_size).sum::<i64>() * (forward_pos / 2);
        disk_pos += file_size;
        forward_pos += 1;

        if forward_pos >= backward_pos {
            if size_of_last_file != 0 {
                checksum += (disk_pos..disk_pos + size_of_last_file).sum::<i64>() * ((forward_pos + 1) / 2);
            }
            break;
        }

        let mut space_size = forward_iter.next().map(char_to_int).unwrap();
        forward_pos += 1;

        loop {
            if space_size == 0 {
                break;
            }
            if size_of_last_file == 0 {
                size_of_last_file = backward_iter.next().map(char_to_int).unwrap();
                backward_iter.next().unwrap();
                backward_pos -= 2;
            }
            let amount_to_copy = space_size.min(size_of_last_file);
            space_size -= amount_to_copy;
            size_of_last_file -= amount_to_copy;
            checksum += (disk_pos..disk_pos + amount_to_copy).sum::<i64>() * ((backward_pos + 2) / 2);
            disk_pos += amount_to_copy;
        }

        if forward_pos >= backward_pos {
            if size_of_last_file != 0 {
                checksum += (disk_pos..disk_pos + size_of_last_file).sum::<i64>() * ((backward_pos + 2) / 2);
            }
            break;
        }
    }

    format!("{}", checksum)
}

fn part2(input: &String) -> String {
    let mut checksum = 0;

    let mut size_to_ids: HashMap<i64, Vec<i64>> = HashMap::new();
    let mut pos: i64 = input.len() as i64;
    let mut space_next = false;

    for block in input.chars().rev() {
        pos -= 1;
        space_next = !space_next;
        if space_next {
            size_to_ids.entry(char_to_int(block)).or_insert(Vec::new()).push(pos / 2);
        }
    }

    let mut disk_pos = 0;
    space_next = false;
    let mut used: HashSet<i64> = HashSet::new();

    pos = 0;
    for block in input.chars() {
        space_next = !space_next;

        let mut block_size = char_to_int(block);
        if space_next {
            let id = pos / 2;
            if used.insert(id) {
                checksum += (disk_pos..disk_pos + block_size).sum::<i64>() * id;
            }
            disk_pos += block_size;
        } else {
            loop {
                let mut max_id = None;
                let mut file_size = 0;
                for i in 1..block_size + 1 {
                    match size_to_ids.get(&i) {
                        None => {},
                        Some(&ref ids) => {
                            if ids.len() == 0 || used.contains(&ids[0]) {
                                size_to_ids.remove(&i);
                            } else {
                                let id = ids[0];
                                match max_id {
                                    None => {
                                        max_id = Some(id);
                                        file_size = i;
                                    },
                                    Some(current_max) => {
                                        if id > current_max {
                                            max_id = Some(id);
                                            file_size = i;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                match max_id {
                    None => {
                        disk_pos += block_size;
                        break;
                    },
                    Some(id) => {
                        used.insert(id);
                        size_to_ids.entry(file_size).and_modify(|ids| { ids.remove(0); });
                        checksum += (disk_pos..disk_pos + file_size).sum::<i64>() * id;
                        disk_pos += file_size;

                        block_size -= file_size;
                        if block_size == 0 {
                            break;
                        }
                    }
                }
            }
        }

        pos += 1;
    }

    format!("{}", checksum)
}

fn main() {
    env_logger::init();
    let input1 = &std::fs::read_to_string("day9.txt").expect("Unable to read input file");
    println!("Part1: {}", part1(input1));

    println!("Part2: {}", part2(input1));
}


#[cfg(test)]
mod tests {
    use crate::part1;
    use crate::part2;

    #[test]
    fn test_part1() {
        let example1 = r#"2333133121414131402"#;
        let expected1 = "1928".to_string();

        assert_eq!(part1(&example1.to_string()), expected1);
    }

    #[test]
    fn test_part2() {
        let example1 = r#"2333133121414131402"#;
        let expected1 = "2858".to_string();

        assert_eq!(part2(&example1.to_string()), expected1);
    }
}
