use std::collections::{HashMap, HashSet};
use std::fs;

fn main() {
    let input = &fs::read_to_string("day23.txt").expect("Unable to read input file");
    println!("part1: {}", part1(input.lines().collect()));
    println!("part2: {}", part2(input.lines().collect()));
}

fn part1(lines: Vec<&str>) -> usize {
    get_3_lans(true, &get_connections(lines)).len()
}

fn part2(lines: Vec<&str>) -> String {
    let connections = get_connections(lines);
    get_max_lan(&connections)
}

fn get_3_lans(must_include_chief: bool, connections: &HashMap<String, HashSet<String>>) -> HashSet<String> {
    let mut lans = HashSet::new();
    for (&ref first, computers) in connections.iter() {
        for second in computers {
            for third in computers.intersection(connections.get(second).unwrap()) {
                if !must_include_chief || (first.starts_with("t") || second.starts_with("t") || third.starts_with("t")) {
                    let mut lan = vec![first.clone(), second.clone(), third.clone()];
                    lan.sort();
                    lans.insert(lan.join(","));
                }
            }
        }
    }
    lans
}

fn get_max_lan(connections: &HashMap<String, HashSet<String>>) -> String {
    let mut lan = HashSet::new();
    for (&ref computer, computers) in connections {
        let mut lan_finder = LanFinder { chosen: HashSet::new(), lan: HashSet::new() };
        lan_finder.chosen.insert(computer.to_string());
        lan_finder.get_lan(computer.clone(), computers, connections);
        if lan_finder.lan.len() > lan.len() {
            lan = lan_finder.lan;
        }
    }
    let mut computers: Vec<String> = lan.iter().map(|computer| computer.to_string()).collect();
    computers.sort();
    computers.join(",")
}

struct LanFinder {
    chosen: HashSet<String>,
    lan: HashSet<String>
}

impl LanFinder {
    fn get_lan(&mut self, candidate: String, allowed: &HashSet<String>, connections: &HashMap<String, HashSet<String>>) {
        let mut lan = HashSet::new();
        let candidate_connections = &connections[&candidate];
        let next_candidates = &candidate_connections.intersection(allowed).map(|computer| computer.clone()).collect::<HashSet<String>>();
        for computer in next_candidates {
            self.chosen.insert(computer.clone());
            self.get_lan(computer.clone(), next_candidates, connections);
            if self.lan.len() > lan.len() {
                lan = self.lan.clone();
            }
            self.lan.clear();
            if lan.len() > next_candidates.len() / 2 {
                break
            }
            self.chosen.remove(computer);
        }
        lan.insert(candidate);
        self.lan = lan.clone();
    }
}

fn get_connections(lines: Vec<&str>) -> HashMap<String, HashSet<String>> {
    let mut connections = HashMap::new();
    for line in lines {
        let (computer1, computer2) = line.split_once("-").unwrap();
        connections.entry(computer1.to_string()).or_insert(HashSet::new()).insert(computer2.to_string());
        connections.entry(computer2.to_string()).or_insert(HashSet::new()).insert(computer1.to_string());
    }
    connections
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn test_part1() {
        let lines = vec![
            "kh-tc",
            "qp-kh",
            "de-cg",
            "ka-co",
            "yn-aq",
            "qp-ub",
            "cg-tb",
            "vc-aq",
            "tb-ka",
            "wh-tc",
            "yn-cg",
            "kh-ub",
            "ta-co",
            "de-co",
            "tc-td",
            "tb-wq",
            "wh-td",
            "ta-ka",
            "td-qp",
            "aq-cg",
            "wq-ub",
            "ub-vc",
            "de-ta",
            "wq-aq",
            "wq-vc",
            "wh-yn",
            "ka-de",
            "kh-ta",
            "co-tc",
            "wh-qp",
            "tb-vc",
            "td-yn"
        ];
        assert_eq!(part1(lines), 7);
    }

    #[test]
    fn test_part2() {
        let lines = vec![
            "kh-tc",
            "qp-kh",
            "de-cg",
            "ka-co",
            "yn-aq",
            "qp-ub",
            "cg-tb",
            "vc-aq",
            "tb-ka",
            "wh-tc",
            "yn-cg",
            "kh-ub",
            "ta-co",
            "de-co",
            "tc-td",
            "tb-wq",
            "wh-td",
            "ta-ka",
            "td-qp",
            "aq-cg",
            "wq-ub",
            "ub-vc",
            "de-ta",
            "wq-aq",
            "wq-vc",
            "wh-yn",
            "ka-de",
            "kh-ta",
            "co-tc",
            "wh-qp",
            "tb-vc",
            "td-yn"
        ];
        assert_eq!(part2(lines), "co,de,ka,ta".to_string());
    }
}
