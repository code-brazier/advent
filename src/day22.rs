use std::collections::HashMap;
use std::fs;

fn main() {
    let input = &fs::read_to_string("day22.txt").expect("Unable to read input file");
    let input = input.lines().map(|line| line.parse::<i64>().unwrap()).collect::<Vec<i64>>();
    println!("part1: {}", sum(&input));
    println!("part2: {}", get_best_price(&input));
}

fn sum(seeds: &Vec<i64>) -> i64 {
    seeds.iter()
        .map(|&seed|
            generate(seed, 2000))
        .sum()
}

fn generate(mut seed: i64, iterations: i32) -> i64 {
    for _ in 0..iterations {
        let result = seed * 64;
        seed = result ^ seed;
        seed = seed % 16777216;
        let result = seed / 32;
        seed = result ^ seed;
        seed = seed % 16777216;
        let result = seed * 2048;
        seed = result ^ seed;
        seed = seed % 16777216;
    }
    seed
}

fn get_price_trends(seed: i64, iterations: usize) -> HashMap<i64, i64> {
    let mut seeds: [i64; 5] = [0; 5];
    seeds[0] = generate(seed, 1);
    seeds[1] = generate(seeds[0], 1);
    seeds[2] = generate(seeds[1], 1);
    seeds[3] = generate(seeds[2], 1);
    seeds[4] = generate(seeds[3], 1);
    let mut price_trends = HashMap::new();
    for i in 5..iterations {
        seeds[i % 5] = generate(seeds[(i + 4) % 5], 1);
        let mut trend = seeds[(i + 2) % 5] % 10 - seeds[(i + 1) % 5] % 10 + 9;
        trend = trend * 100 + seeds[(i + 3) % 5] % 10 - seeds[(i + 2) % 5] % 10 + 9;
        trend = trend * 100 + seeds[(i + 4) % 5] % 10 - seeds[(i + 3) % 5] % 10 + 9;
        trend = trend * 100 + seeds[i % 5] % 10 - seeds[(i + 4) % 5] % 10 + 9;
        if !price_trends.contains_key(&trend) {
            price_trends.insert(trend, seeds[i % 5] % 10);
        }
    }
    price_trends
}

fn get_best_price(seeds: &Vec<i64>) -> i64 {
    let mut price_trends = HashMap::new();
    for &seed in seeds {
        let prices = get_price_trends(seed, 2000);
        for (trend, price) in prices {
            price_trends.entry(trend).and_modify(|p| *p += price).or_insert(price);
        }
    }
    *price_trends.values().max().unwrap()
}

#[cfg(test)]
mod tests {
    use crate::{generate, get_best_price, get_price_trends, sum};

    #[test]
    fn test_generate() {
        assert_eq!(generate(123, 1), 15887950);
        assert_eq!(generate(123, 2), 16495136);
        assert_eq!(generate(123, 3), 527345);
        assert_eq!(generate(123, 4), 704524);
        assert_eq!(generate(123, 5), 1553684);
        assert_eq!(generate(123, 6), 12683156);
        assert_eq!(generate(123, 7), 11100544);
        assert_eq!(generate(123, 8), 12249484);
        assert_eq!(generate(123, 9), 7753432);
        assert_eq!(generate(123, 10), 5908254);

        assert_eq!(generate(1, 2000), 8685429);
        assert_eq!(generate(10, 2000), 4700978);
        assert_eq!(generate(100, 2000), 15273692);
        assert_eq!(generate(2024, 2000), 8667524);
    }

    #[test]
    fn test_sum() {
        assert_eq!(sum(&vec!(1, 10, 100, 2024)), 37327623);
    }

    #[test]
    fn test_price_trends() {
        let price_trends = get_price_trends(123, 10);
        assert_eq!(price_trends.values().max().unwrap(), &6);
    }

    #[test]
    fn test_best_price() {
        assert_eq!(get_best_price(&vec!(1, 2, 3, 2024)), 23);
    }
}
