use std::{collections::HashMap, time::Instant};

#[derive(Debug)]
struct Mapper {
    ranges: Vec<Range>,
}

impl Mapper {
    fn new() -> Self {
        Mapper { ranges: Vec::new() }
    }

    fn add_range(&mut self, dst: i64, src: i64, length: i64) {
        self.ranges.push(Range { dst, src, length });
    }

    fn map(&self, src: i64) -> i64 {
        for range in &self.ranges {
            if src >= range.src && src < range.src + range.length {
                return (range.dst - range.src) + src;
            }
        }
        src
    }
}

#[derive(Debug)]
struct Range {
    dst: i64,
    src: i64,
    length: i64,
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut seeds = Vec::new();
    let mut mapper_map = HashMap::new();

    let mut mapper_name = "";
    for line in input.lines() {
        if line.contains("seeds") {
            seeds = line
                .split_whitespace()
                .filter_map(|s| s.parse::<i64>().ok())
                .collect();
            continue;
        }

        if line.contains("map:") {
            mapper_name = line.split_whitespace().nth(0).unwrap();
            mapper_map.insert(mapper_name, Mapper::new());
            continue;
        }

        if line.to_string().is_empty() {
            continue;
        }

        let mapper = mapper_map.get_mut(mapper_name).unwrap();
        let tokens = line.split_whitespace().collect::<Vec<_>>();
        mapper.add_range(tokens[0].parse::<i64>().unwrap(), tokens[1].parse::<i64>().unwrap(), tokens[2].parse::<i64>().unwrap());
    }

    let mut lowest_location = std::i64::MAX;
    for seed in seeds.clone() {
        lowest_location = lowest_location.min(turbo_mapper(&mapper_map, seed));
    }

    println!("Day 5 part 1: lowest location: {}", lowest_location);
    
    let mut range_seeds = Vec::new();
    for pair in seeds.chunks(2) {
        for seed in pair[0]..pair[0]+pair[1] {
            range_seeds.push(seed);
        }
    }

    lowest_location = std::i64::MAX;
    for seed in range_seeds {
        lowest_location = lowest_location.min(turbo_mapper(&mapper_map, seed));
    }

    println!("Day 5 part 2: lowest location: {}", lowest_location);
}

fn turbo_mapper(mapper_map: &HashMap<&str, Mapper>, seed: i64) -> i64 {
    let start = Instant::now();
    let soil = mapper_map.get("seed-to-soil").unwrap().map(seed);
    let fertilizer = mapper_map.get("soil-to-fertilizer").unwrap().map(soil);
    let water = mapper_map.get("fertilizer-to-water").unwrap().map(fertilizer);
    let light = mapper_map.get("water-to-light").unwrap().map(water);
    let temperature = mapper_map.get("light-to-temperature").unwrap().map(light);
    let humidity= mapper_map.get("temperature-to-humidity").unwrap().map(temperature);
    let loc = mapper_map.get("humidity-to-location").unwrap().map(humidity);
    let duration = start.elapsed();
    println!("Time elapsed in turbo_mapper() is: {:?}", duration);
    loc
}
