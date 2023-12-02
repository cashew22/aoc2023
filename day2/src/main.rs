fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let green_max = 13;
    let blue_max = 14;
    let red_max = 12;

    let mut total = 0;
    let mut total_power = 0;

    for (i, line) in input.lines().enumerate() {
        let game_id = i + 1;

        let mut line = line.replace(";", "");
        line = line.replace(",", "");
        let tokens: Vec<&str> = line.split_whitespace().collect();

        // Part 1
        let mut valid = true;
        for (j, token) in tokens.iter().enumerate() {
            if token.to_string() == "green" && tokens[j - 1].parse::<i32>().unwrap() > green_max {
                valid = false;
                break;
            } else if token.to_string() == "blue" && tokens[j - 1].parse::<i32>().unwrap() > blue_max {
                valid = false;
                break;
            } else if token.to_string() == "red" && tokens[j - 1].parse::<i32>().unwrap() > red_max {
                valid = false;
                break;
            }
        }

        if valid {
            total += game_id;
        }

        // Part 2
        let mut green = 0;
        let mut blue = 0;
        let mut red = 0;
        for (j, token) in tokens.iter().enumerate() {
            if token.to_string() == "green" {
                let cubes = tokens[j - 1].parse::<i32>().unwrap();
                if cubes > green {
                    green = cubes;
                }
            } else if token.to_string() == "blue" {
                let cubes = tokens[j - 1].parse::<i32>().unwrap();
                if cubes > blue {
                    blue = cubes;
                }
            } else if token.to_string() == "red" {
                let cubes = tokens[j - 1].parse::<i32>().unwrap();
                if cubes > red {
                    red = cubes;
                }
            }
        }

        total_power += green * blue * red;

    }

    println!("Day 2 part 1: total possible game ids: {}", total);
    println!("Day 2 part 2: total power: {}", total_power);

}
