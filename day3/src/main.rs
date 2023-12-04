fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let re = regex::Regex::new(r"\d+").unwrap();
    let re2 = regex::Regex::new(r"\*").unwrap();

    let mut part1_total = 0;
    let mut part2_total = 0;

    for (i, line) in input.lines().enumerate() {
        for cap in re.captures_iter(line) {
            let number = cap[0].parse::<i32>().unwrap();
            let start_index = cap.get(0).unwrap().start();
            let end_index = cap.get(0).unwrap().end() - 1;

            if (start_index > 0 && line.chars().nth(start_index - 1).unwrap() != '.') ||
                (end_index < line.len() - 1 && line.chars().nth(end_index + 1).unwrap() != '.') {
                part1_total += number;
                continue;
            }

            if i > 0 {
                let prev_line = input.lines().nth(i - 1).unwrap();
                let start_find = if start_index > 0 {start_index - 1} else {start_index};
                let end_find = if end_index < prev_line.len() - 1 {end_index + 2} else {end_index + 1};

                for i in start_find..end_find {
                    if prev_line.chars().nth(i).unwrap() != '.' {
                        part1_total += number;
                        break;
                    }
                }
            }

            if i < input.lines().count() - 1 {
                let next_line = input.lines().nth(i + 1).unwrap();
                let start_find = if start_index > 0 {start_index - 1} else { start_index};
                let end_find = if end_index < next_line.len() - 1 {end_index + 2} else {end_index + 1};

                for i in start_find..end_find {
                    if next_line.chars().nth(i).unwrap() != '.' {
                        part1_total += number;
                        break;
                    }
                }
            }
        }

        for cap in re2.captures_iter(line){
            let pos = cap.get(0).unwrap().start();
            let mut numbers: Vec<i32> = Vec::new();

            if i > 0 {
                let prev_line = input.lines().nth(i - 1).unwrap();
                for cap2 in re.captures_iter(prev_line) {
                    let number = cap2[0].parse::<i32>().unwrap();
                    let start = cap2.get(0).unwrap().start();
                    let end = cap2.get(0).unwrap().end() - 1;

                    if (start >= pos - 1 && start <= pos + 1) || (end >= pos - 1 && end <= pos + 1) {
                        numbers.push(number);
                    }
                }
            }

            for cap2 in re.captures_iter(line) {
                let number = cap2[0].parse::<i32>().unwrap();
                let start = cap2.get(0).unwrap().start();
                let end = cap2.get(0).unwrap().end() - 1;
                
                if pos + 1 == start || pos - 1 == end {
                    numbers.push(number);
                }
            }

            if i < input.lines().count() - 1 {
                let next_line = input.lines().nth(i + 1).unwrap();
                for cap2 in re.captures_iter(next_line) {
                    let number = cap2[0].parse::<i32>().unwrap();
                    let start = cap2.get(0).unwrap().start();
                    let end = cap2.get(0).unwrap().end() - 1;
                    
                    if (start >= pos - 1 && start <= pos + 1) || (end >= pos - 1 && end <= pos + 1) {
                        numbers.push(number);
                    }
                }
            }

            if numbers.len() == 2 {
                part2_total += numbers[0] * numbers[1];
            }
        }
    }
    println!("Day3 part1: {}", part1_total);
    println!("Day3 part2: {}", part2_total);
}
